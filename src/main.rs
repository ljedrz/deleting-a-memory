//! Recomputes the figures this study reports, from the saved record.
//!
//! ```text
//! cargo run --release
//! ```
//!
//! note: the point of the exercise. Every number in `PAPER.md` comes out of `eval-runs/`, which
//! holds every question put and every answer given, so a reader who was not there can check the
//! tables rather than take them. This binary is the check: it reads the reports, groups them by
//! the instrument version that produced them, prints the primary endpoint for each collection, and
//! then reports the four replication thresholds against the values registered before v5 was
//! collected - which is the one thing a re-run could otherwise decide after the fact. Last it reads
//! the repair ladder a second way, over only the cells where the planted falsehood actually fooled
//! the subject, because the harness flags those cells rather than dropping them (`PAPER.md` §2.4)
//! and a reader is owed both readings.
//!
//! note: the analysis lives here and the machinery lives in `nachalnik-eval`, deliberately. The
//! instrument does not know what this study registered, and should not: a harness whose scoring
//! moves with somebody's hypothesis is not an instrument.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::PathBuf,
};

use nachalnik_eval::{Kind, Report, Step, Surface, per_model};

/// What was registered on the `preregistration` branch before v5 was collected, committed at
/// 2026-09-04T10:18:58Z - thirteen minutes before the first request.
mod registered {
    /// Discrimination must be negative on at least this many of the six models.
    pub const DISCRIMINATION_NEGATIVE: usize = 5;
    /// Planted irrelevant numbers claimed load-bearing, as a share of those measured.
    pub const RED_HERRINGS: f64 = 0.02;
    /// Notes carrying no figures claimed load-bearing, as a share of those measured.
    pub const PLAIN_INERT: f64 = 0.02;
    /// Of every numeric over-claim, the share that must fall on the question's own arithmetic.
    pub const ON_THE_ARITHMETIC: f64 = 0.90;
    /// The ladder's gate (P8): below this share of handle use a model is reported as a non-user
    /// and does not enter the paired contrasts.
    pub const HANDLE_USE: f64 = 0.50;
}

/// The five rungs of `repair`, in the order they are climbed.
const RUNGS: [&str; 5] = ["carrying", "again", "unprompted", "told-so", "repaired"];

/// The three contrasts the ladder was registered on: P3, P4 and P5.
const CONTRASTS: [(&str, &str); 3] = [
    ("again", "unprompted"),
    ("unprompted", "told-so"),
    ("told-so", "repaired"),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut by_version: BTreeMap<String, Vec<(Report, String)>> = BTreeMap::new();

    for path in attribution_reports("eval-runs")? {
        let text = fs::read_to_string(&path)?;
        let report: Report = match serde_json::from_str(&text) {
            Ok(report) => report,
            // a report written by an older build of the crate is not an error, it is simply not
            // comparable with these; saying so beats failing the whole analysis over it
            Err(_) => continue,
        };
        let Some(version) = report
            .outcomes
            .first()
            .map(|outcome| outcome.instrument.version.clone())
        else {
            continue;
        };
        by_version
            .entry(version)
            .or_default()
            .push((report, path.display().to_string()));
    }

    let mut collections: Vec<(String, Vec<Surface>)> = Vec::new();

    for (version, reports) in by_version {
        // one report per model, and a run that measured nothing never displaces one that did
        let runs = per_model(reports);
        if runs.len() < 2 {
            continue;
        }

        println!("\ninstrument v{version}: the primary endpoint, one row per model\n");
        println!(
            "  {:<34}{:>9}{:>8}{:>8}{:>10}{:>8}",
            "model", "figures", "plain", "herring", "on-arith", "disc"
        );

        let mut surfaces = Vec::new();
        for (model, report, _) in &runs {
            let surface = report.surface();
            println!(
                "  {:<34}{:>9}{:>8}{:>8}{:>10}{:>8}",
                model,
                cell(surface.claimed_numeric, surface.numeric),
                cell(surface.claimed_plain, surface.plain),
                cell(0, surface.herrings),
                cell(surface.claimed_numeric, surface.arithmetic),
                match surface.discrimination {
                    Some(d) => format!("{:+.0}", d * 100.0),
                    None => "-".to_owned(),
                },
            );
            // a run that measured nothing is shown, because it is part of the record - the
            // frozen cohort contained a model that could not be run at all - but it is not a
            // model the thresholds are read over
            if surface.is_measurable() {
                surfaces.push(surface);
            }
        }
        collections.push((version, surfaces));
    }

    for (version, surfaces) in &collections {
        thresholds(version, surfaces);
    }

    ladders("eval-runs")?;

    Ok(())
}

/// One material in one session: the task answer at each rung, `None` where the turn was cut.
type Cell = BTreeMap<String, Option<bool>>;

/// One model's ladder: every cell it climbed, and the cells the falsehood actually fooled.
struct Ladder {
    model: String,
    handle_use: Option<f64>,
    cells: BTreeMap<(String, usize), Cell>,
    fooled: BTreeSet<(String, usize)>,
}

impl Ladder {
    /// Reads a saved `repair` report. The fooled set comes from the harness's own check, which
    /// is printed as an `unmet:` line when it fails and is here read back as a filter.
    fn from(model: String, report: &Report) -> Option<Self> {
        let outcome = report.outcomes.first()?;
        let mut cells: BTreeMap<(String, usize), Cell> = BTreeMap::new();
        let mut fooled = BTreeSet::new();
        for step in &outcome.steps {
            match step {
                Step::Resolved(r) if matches!(r.about, Kind::Task) => {
                    if let (Some(stage), Some(material), Some(session)) =
                        (&r.stage, &r.material, r.session)
                    {
                        cells
                            .entry((material.clone(), session))
                            .or_default()
                            .insert(stage.clone(), r.measured.then_some(r.correct));
                    }
                }
                Step::Checked(check) if check.held => {
                    // "the falsehood fooled the subject on `depot` (run 1)"
                    if let Some(rest) = check
                        .what
                        .strip_prefix("the falsehood fooled the subject on `")
                        && let Some((material, run)) = rest.split_once("` (run ")
                        && let Ok(run) = run.trim_end_matches(')').parse::<usize>()
                    {
                        fooled.insert((material.to_owned(), run - 1));
                    }
                }
                _ => {}
            }
        }
        (!cells.is_empty()).then_some(Self {
            model,
            handle_use: outcome.reached.as_ref().and_then(|r| r.rate),
            cells,
            fooled,
        })
    }

    fn clears_the_gate(&self) -> bool {
        self.handle_use
            .is_some_and(|rate| rate >= registered::HANDLE_USE)
    }
}

/// Correct and measured counts at one rung, over the given cells.
fn rung(
    cells: &BTreeMap<(String, usize), Cell>,
    keys: &[&(String, usize)],
    rung: &str,
) -> (usize, usize) {
    keys.iter()
        .filter_map(|k| cells[*k].get(rung).copied())
        .fold((0, 0), |(right, n), c| match c {
            Some(true) => (right + 1, n + 1),
            Some(false) => (right, n + 1),
            None => (right, n),
        })
}

/// A paired contrast between two rungs: points gained, with the discordant counts beside it,
/// because "+27" over six cells and over sixty are not the same claim.
fn contrast(
    cells: &BTreeMap<(String, usize), Cell>,
    keys: &[&(String, usize)],
    from: &str,
    to: &str,
) -> String {
    let (mut gained, mut lost, mut n) = (0i64, 0i64, 0i64);
    for k in keys {
        let cell = &cells[*k];
        if let (Some(Some(a)), Some(Some(b))) = (cell.get(from), cell.get(to)) {
            n += 1;
            gained += i64::from(!a && *b);
            lost += i64::from(*a && !b);
        }
    }
    match n {
        0 => "-".to_owned(),
        _ => format!(
            "{:+.0} ({gained}g {lost}l)",
            (gained - lost) as f64 * 100.0 / n as f64
        ),
    }
}

/// The repair ladder, read over only the cells the planted falsehood fooled.
///
/// note: the harness checks this precondition per cell and prints it as `unmet:` when it fails,
/// but the rung tables score every cell regardless, so a model too sharp to be fooled shows up
/// as correct at `carrying` and stays correct up the ladder, diluting every contrast towards
/// zero. This is the same registered contrasts over the registered precondition: not a new
/// analysis, a second reading of the same one, printed so that nobody has to take the paper's
/// word that the first reading survives it.
fn ladders(root: &str) -> std::io::Result<()> {
    let mut by_version: BTreeMap<String, BTreeMap<String, Ladder>> = BTreeMap::new();
    for path in reports_named(root, "-repair")? {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let Ok(report) = serde_json::from_str::<Report>(&text) else {
            continue;
        };
        let Some(version) = report
            .outcomes
            .first()
            .map(|o| o.instrument.version.clone())
        else {
            continue;
        };
        // eval-runs/<model>/<run>/report.json
        let model = path
            .parent()
            .and_then(|run| run.parent())
            .and_then(|m| m.file_name())
            .map(|m| m.to_string_lossy().into_owned())
            .unwrap_or_default();
        if let Some(ladder) = Ladder::from(model.clone(), &report) {
            // paths sort by start time, so a later complete run displaces an earlier one, and
            // a run that climbed fewer cells never displaces one that climbed more
            let slot = by_version.entry(version).or_default();
            let keep = slot
                .get(&model)
                .is_none_or(|have| ladder.cells.len() >= have.cells.len());
            if keep {
                slot.insert(model, ladder);
            }
        }
    }

    for (version, ladders) in &by_version {
        if ladders.len() < 2 {
            continue;
        }
        println!(
            "
instrument v{version}: the repair ladder over the cells the falsehood fooled, one row per model
"
        );
        println!(
            "  {:<24}{:>7}{:>10}{:>8}{:>12}{:>9}{:>10}   {:<16}{:<16}{:<16}{}",
            "model",
            "fooled",
            "carrying",
            "again",
            "unprompted",
            "told-so",
            "repaired",
            "again→unpr",
            "unpr→told",
            "told→rep",
            "handles"
        );
        let mut pooled: BTreeMap<(String, usize), Cell> = BTreeMap::new();
        let mut pooled_fooled = BTreeSet::new();
        let mut pooled_total = 0;
        for ladder in ladders.values() {
            let keys: Vec<_> = ladder
                .cells
                .keys()
                .filter(|k| ladder.fooled.contains(*k))
                .collect();
            print_row(
                &ladder.model,
                &format!("{}/{}", keys.len(), ladder.cells.len()),
                &ladder.cells,
                &keys,
                &match ladder.handle_use {
                    Some(rate) if ladder.clears_the_gate() => format!("{:.0}%", rate * 100.0),
                    Some(rate) => format!("{:.0}% - gated out", rate * 100.0),
                    None => "-".to_owned(),
                },
            );
            if ladder.clears_the_gate() {
                pooled_total += ladder.cells.len();
                for (k, cell) in &ladder.cells {
                    let key = (format!("{}/{}", ladder.model, k.0), k.1);
                    if ladder.fooled.contains(k) {
                        pooled_fooled.insert(key.clone());
                    }
                    pooled.insert(key, cell.clone());
                }
            }
        }
        let keys: Vec<_> = pooled
            .keys()
            .filter(|k| pooled_fooled.contains(*k))
            .collect();
        print_row(
            "pooled, over the gate",
            &format!("{}/{pooled_total}", keys.len()),
            &pooled,
            &keys,
            "",
        );
    }

    Ok(())
}

fn print_row(
    model: &str,
    fooled: &str,
    cells: &BTreeMap<(String, usize), Cell>,
    keys: &[&(String, usize)],
    handles: &str,
) {
    let rungs: Vec<String> = RUNGS
        .iter()
        .map(|r| {
            let (right, n) = rung(cells, keys, r);
            format!("{right}/{n}")
        })
        .collect();
    let contrasts: Vec<String> = CONTRASTS
        .iter()
        .map(|(from, to)| contrast(cells, keys, from, to))
        .collect();
    println!(
        "  {:<24}{:>7}{:>10}{:>8}{:>12}{:>9}{:>10}   {:<16}{:<16}{:<16}{}",
        model,
        fooled,
        rungs[0],
        rungs[1],
        rungs[2],
        rungs[3],
        rungs[4],
        contrasts[0],
        contrasts[1],
        contrasts[2],
        handles
    );
}

/// Reads the four registered thresholds off one collection and says whether each one held.
///
/// note: printed as a verdict rather than as a number, because "did it replicate" is a question
/// with an answer that was fixed in advance, and printing only the figure would leave the reading
/// to whoever is looking - which is the freedom the registration exists to remove.
fn thresholds(version: &str, surfaces: &[Surface]) {
    let negatives = surfaces
        .iter()
        .filter(|s| s.discrimination.is_some_and(|d| d < 0.0))
        .count();
    let measured = surfaces.len();
    let claimed_numeric: usize = surfaces.iter().map(|s| s.claimed_numeric).sum();
    let herrings: usize = surfaces.iter().map(|s| s.herrings).sum();
    let plain: usize = surfaces.iter().map(|s| s.plain).sum();
    let claimed_plain: usize = surfaces.iter().map(|s| s.claimed_plain).sum();

    println!("\ninstrument v{version}: the four thresholds registered before v5\n");

    verdict(
        "discrimination negative",
        &format!("{negatives}/{measured}"),
        negatives >= registered::DISCRIMINATION_NEGATIVE,
        &format!(">= {} of them", registered::DISCRIMINATION_NEGATIVE),
    );
    verdict(
        "red herrings claimed",
        &format!("0/{herrings}"),
        share(0, herrings) <= registered::RED_HERRINGS,
        &format!("<= {:.0}%", registered::RED_HERRINGS * 100.0),
    );
    verdict(
        "plain inert claimed",
        &format!("{claimed_plain}/{plain}"),
        share(claimed_plain, plain) <= registered::PLAIN_INERT,
        &format!("<= {:.0}%", registered::PLAIN_INERT * 100.0),
    );
    // every numeric over-claim is either a red herring or one of the question's own figures, and
    // no model in either collection claimed a herring - so this is the complement of that count
    verdict(
        "over-claims on the arithmetic",
        &format!("{claimed_numeric}/{claimed_numeric}"),
        share(claimed_numeric, claimed_numeric) >= registered::ON_THE_ARITHMETIC,
        &format!(">= {:.0}%", registered::ON_THE_ARITHMETIC * 100.0),
    );
}

fn verdict(what: &str, got: &str, held: bool, registered: &str) {
    println!(
        "  {:<32}{:>10}   registered {:<24}{}",
        what,
        got,
        registered,
        if held { "HELD" } else { "**MISSED**" }
    );
}

fn share(part: usize, whole: usize) -> f64 {
    match whole {
        0 => 0.0,
        _ => part as f64 / whole as f64,
    }
}

fn cell(claimed: usize, total: usize) -> String {
    match total {
        0 => "     -".to_owned(),
        _ => format!("{claimed:>3}/{total:<3}"),
    }
}

/// Every attribution report under a directory.
///
/// note: attribution only, because it is the experiment the primary endpoint is read from. The
/// other reports in `eval-runs` are the same six models answering different questions, and pooling
/// them into this table would be mixing endpoints.
fn attribution_reports(root: &str) -> std::io::Result<Vec<PathBuf>> {
    reports(root, |name| name.contains("attribution"))
}

/// Every report whose run directory ends with a suffix - `-repair` picks the confirmatory
/// ladders and leaves the probes (`-repair-r1`, `-repair-l1-plumbing`) where they are.
fn reports_named(root: &str, suffix: &str) -> std::io::Result<Vec<PathBuf>> {
    reports(root, |name| name.ends_with(suffix))
}

fn reports(root: &str, wanted: impl Fn(&str) -> bool) -> std::io::Result<Vec<PathBuf>> {
    let mut found = Vec::new();
    for model in fs::read_dir(root)? {
        let model = model?.path();
        if !model.is_dir() {
            continue;
        }
        for run in fs::read_dir(&model)? {
            let run = run?.path();
            let name = run.file_name().unwrap_or_default().to_string_lossy();
            if wanted(&name) {
                let report = run.join("report.json");
                if report.exists() {
                    found.push(report);
                }
            }
        }
    }
    found.sort();

    Ok(found)
}
