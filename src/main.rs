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
//! the repair ladder twice: over every cell, which is the reading `RESULTS.md` tabulates, and over
//! only the cells where the planted falsehood actually fooled the subject, because the harness
//! flags those cells rather than dropping them (`PAPER.md` §2.4) and a reader is owed both.
//!
//! note: the analysis lives here and the machinery lives in `nachalnik-eval`, deliberately. The
//! instrument does not know what this study registered, and should not: a harness whose scoring
//! moves with somebody's hypothesis is not an instrument.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::PathBuf,
};

use nachalnik_eval::{Answer, Kind, Report, Step, Surface, per_model, suite::dossier};

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

/// The report kept for each model in one collection, with the path it was read from.
type Runs = Vec<(String, Report, String)>;

/// One collection: the instrument version, the surfaces the thresholds are read over, and the runs
/// those surfaces came from - kept so the endpoint can be read a second way off the same selection.
type Collection = (String, Vec<Surface>, Runs);

/// The five rungs of `repair`, in the order they are climbed.
const RUNGS: [&str; 5] = ["carrying", "again", "unprompted", "told-so", "repaired"];

/// The three contrasts the ladder was registered on: P3, P4 and P5.
const CONTRASTS: [(&str, &str); 3] = [
    ("again", "unprompted"),
    ("unprompted", "told-so"),
    ("told-so", "repaired"),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // keyed by the instrument version *and* the digest of the exact question text. `PAPER.md` §6
    // says two numbers are comparable only when the same questions produced them, and the version
    // alone does not say that: `eval-runs` holds two v4 attribution digests, because one run was a
    // single-material pilot taken before the battery was built. Grouping by version alone would
    // pool them, which is the thing the digest exists to prevent
    let mut by_fingerprint: BTreeMap<(String, String), Vec<(Report, String)>> = BTreeMap::new();
    let mut unreadable: Vec<String> = Vec::new();

    for path in attribution_reports("eval-runs")? {
        let text = fs::read_to_string(&path)?;
        let report: Report = match serde_json::from_str(&text) {
            Ok(report) => report,
            // a report written by an older build of the crate is not an error, it is simply not
            // comparable with these - but it is dropped out loud, because a reproduction that
            // silently reads fewer files than it was given is the one failure nobody would see
            Err(error) => {
                unreadable.push(format!("{}: {error}", path.display()));
                continue;
            }
        };
        let Some(instrument) = report
            .outcomes
            .first()
            .map(|outcome| outcome.instrument.clone())
        else {
            continue;
        };
        by_fingerprint
            .entry((instrument.version, instrument.digest))
            .or_default()
            .push((report, path.display().to_string()));
    }

    let mut collections: Vec<Collection> = Vec::new();

    for ((version, digest), reports) in by_fingerprint {
        // one report per model, and a run that measured nothing never displaces one that did
        let runs = per_model(reports);
        if runs.len() < 2 {
            println!(
                "\ninstrument v{version} #{digest}: {} run(s), too few to read a collection off - not pooled",
                runs.len()
            );
            continue;
        }

        println!("\ninstrument v{version} #{digest}: the primary endpoint, one row per model\n");
        println!(
            "  {:<34}{:>9}{:>8}{:>8}{:>10}{:>8}",
            "model", "figures", "plain", "herring", "off-pivot", "disc"
        );

        let mut surfaces = Vec::new();
        for (model, report, _) in &runs {
            let surface = report.surface();
            println!(
                "  {:<34}{:>9}{:>8}{:>8}{:>10}{:>8}",
                model,
                cell(surface.claimed_numeric, surface.numeric),
                cell(surface.claimed_plain, surface.plain),
                cell(surface.claimed_herrings, surface.herrings),
                cell(surface.claimed_arithmetic, surface.arithmetic),
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
        collections.push((version, surfaces, runs));
    }

    for (version, surfaces, _) in &collections {
        thresholds(version, surfaces);
    }

    for (version, _, runs) in &collections {
        unanimity(version, runs);
    }

    for (version, _, runs) in &collections {
        skill(version, runs);
    }

    for (version, _, runs) in &collections {
        idiosyncratic(version, runs);
    }

    ladders("eval-runs")?;

    if !unreadable.is_empty() {
        println!("\nreports that could not be read ({}):\n", unreadable.len());
        for line in &unreadable {
            println!("  {line}");
        }
    }

    Ok(())
}

/// The primary endpoint read three ways, because the published one is not the registered one.
///
/// note: §8.2 of the registration is a gate - "if the control copies disagree with each other, the
/// condition is reported with its instability figure and **excluded from the primary analysis**" -
/// and the instrument never applied it. `Observation::majority` takes the unique plurality of the
/// readable answers on each side, so a control that went two-one, or one readable answer against
/// two that could not be read, still yields a majority and the condition stays in. Only a tie is
/// excluded, because a tie has no plurality. So the middle column here is not a sensitivity
/// analysis but the analysis that was registered and not run; the right-hand column, which asks
/// the same of the treated batch, is the stricter rule and *is* a sensitivity analysis, chosen
/// after the fact and labelled as such.
fn unanimity(version: &str, runs: &[(String, Report, String)]) {
    println!("\ninstrument v{version}: the primary endpoint under each of the three rules\n");
    println!(
        "  {:<30}{:>20}{:>20}{:>20}{:>9}",
        "model", "as published", "§8.2 as registered", "both arms", "dropped"
    );

    let mut pooled = [Tally::new(), Tally::new(), Tally::new()];
    let (mut dropped, mut total) = ([0usize; 3], 0);
    let mut negative = [0usize; 3];
    let mut measurable = [0usize; 3];

    for (model, report, _) in runs {
        let counted = split(report);
        let cells = |t: &Tally| {
            format!("{}/{} num {}/{} pl", t.claimed_numeric, t.numeric, t.claimed_plain, t.plain)
        };
        println!(
            "  {:<30}{:>20}{:>20}{:>20}{:>9}",
            model,
            cells(&counted.reading[0]),
            cells(&counted.reading[1]),
            cells(&counted.reading[2]),
            format!("{}/{}", counted.dropped[1], counted.total),
        );
        for i in 0..3 {
            if let Some(d) = counted.reading[i].discrimination() {
                measurable[i] += 1;
                negative[i] += usize::from(d < 0.0);
            }
            pooled[i].absorb(&counted.reading[i]);
            dropped[i] += counted.dropped[i];
        }
        total += counted.total;
    }

    let cells = |t: &Tally| {
        format!("{}/{} num {}/{} pl", t.claimed_numeric, t.numeric, t.claimed_plain, t.plain)
    };
    println!(
        "  {:<30}{:>20}{:>20}{:>20}{:>9}",
        "pooled",
        cells(&pooled[0]),
        cells(&pooled[1]),
        cells(&pooled[2]),
        format!("{}/{total}", dropped[1]),
    );
    println!(
        "\n  items dropped of {total}: none as published, {} by §8.2, {} by both arms",
        dropped[1], dropped[2]
    );
    for (i, what) in ["as published", "§8.2 as registered", "both arms unanimous"]
        .into_iter()
        .enumerate()
    {
        println!(
            "  {what:<22} herrings {}/{:<4} off-pivot over-claims {}/{:<4} discrimination negative {}/{}   registered >= {}",
            pooled[i].claimed_herrings,
            pooled[i].herrings,
            pooled[i].claimed_arithmetic,
            pooled[i].claimed_numeric,
            negative[i],
            measurable[i],
            registered::DISCRIMINATION_NEGATIVE
        );
    }
}

/// The four counts the endpoint is made of, split by what the note carried.
#[derive(Default, Clone, Copy)]
struct Tally {
    numeric: usize,
    claimed_numeric: usize,
    plain: usize,
    claimed_plain: usize,
    herrings: usize,
    claimed_herrings: usize,
    arithmetic: usize,
    claimed_arithmetic: usize,
}

impl Tally {
    fn new() -> Self {
        Self::default()
    }

    /// The red-herring rate minus the off-pivot rate, where both halves had something in them.
    fn discrimination(&self) -> Option<f64> {
        (self.herrings > 0 && self.arithmetic > 0).then(|| {
            self.claimed_herrings as f64 / self.herrings as f64
                - self.claimed_arithmetic as f64 / self.arithmetic as f64
        })
    }

    fn add(&mut self, carries: bool, herring: bool, claimed: bool) {
        match carries {
            true => {
                self.numeric += 1;
                self.claimed_numeric += usize::from(claimed);
                match herring {
                    true => {
                        self.herrings += 1;
                        self.claimed_herrings += usize::from(claimed);
                    }
                    false => {
                        self.arithmetic += 1;
                        self.claimed_arithmetic += usize::from(claimed);
                    }
                }
            }
            false => {
                self.plain += 1;
                self.claimed_plain += usize::from(claimed);
            }
        }
    }

    fn absorb(&mut self, other: &Self) {
        self.numeric += other.numeric;
        self.claimed_numeric += other.claimed_numeric;
        self.plain += other.plain;
        self.claimed_plain += other.claimed_plain;
        self.herrings += other.herrings;
        self.claimed_herrings += other.claimed_herrings;
        self.arithmetic += other.arithmetic;
        self.claimed_arithmetic += other.claimed_arithmetic;
    }
}

/// Every copy readable, and all of them the same.
fn unanimous(answers: &[Answer]) -> bool {
    let mut keys = answers.iter().map(|answer| answer.key());
    match keys.next() {
        Some(Some(first)) => keys.all(|key| key.as_deref() == Some(first.as_ref())),
        _ => false,
    }
}

/// One report's endpoint under all three rules, with how many items each exclusion drops.
struct Counted {
    /// As published, §8.2 as registered, and both arms unanimous, in that order.
    reading: [Tally; 3],
    /// How many endpoint items each rule excludes; the first is always zero.
    dropped: [usize; 3],
    /// How many endpoint items there were before any of them.
    total: usize,
}

/// Counts one report's endpoint as published, under the registered §8.2 gate, and under the
/// stricter two-arm rule.
///
/// note: the copies are in the record but not on the resolution, so they are paired back up here:
/// within a material the control is the `Measured` step carrying no change and each ablation names
/// the item it removed, which `Briefed` maps to the label the resolution is filed under.
fn split(report: &Report) -> Counted {
    let mut counted = Counted {
        reading: [Tally::new(); 3],
        dropped: [0; 3],
        total: 0,
    };

    for outcome in &report.outcomes {
        let mut labels: BTreeMap<u64, String> = BTreeMap::new();
        let mut control: Option<&[Answer]> = None;
        let mut treated: BTreeMap<String, &[Answer]> = BTreeMap::new();

        for step in &outcome.steps {
            match step {
                // a brief starts a material, and the counts of the one before it are already in
                Step::Briefed { items } => {
                    labels = items.iter().map(|i| (i.id.0, i.label.clone())).collect();
                    control = None;
                    treated.clear();
                }
                Step::Measured {
                    observation,
                    change,
                } => match change {
                    None => control = Some(&observation.answers),
                    Some(_) => {
                        if let Some(tail) = observation.intervention.rsplit("without ").next()
                            && let Ok(id) = tail.trim().parse::<u64>()
                            && let Some(label) = labels.get(&id)
                        {
                            treated.insert(label.clone(), &observation.answers);
                        }
                    }
                },
                Step::Resolved(r) => {
                    if !r.measured
                        || r.about != Kind::Counterfactual
                        || r.happened != Answer::yes(false)
                    {
                        continue;
                    }
                    let (Some(material), Some(label)) = (r.material.as_deref(), r.label.as_deref())
                    else {
                        continue;
                    };
                    let Some((carries, herring)) = dossier::surface(material, label) else {
                        continue;
                    };
                    let claimed = matches!(r.claimed, Answer::Claim { yes: true, .. });
                    counted.total += 1;

                    // §8.2 gates on the *control* copies only; the third rule asks the same of the
                    // treated batch, which nothing registered ever asked for
                    let control_agrees = control.is_some_and(unanimous);
                    let treated_agrees = treated.get(label).copied().is_some_and(unanimous);
                    for (i, keep) in [true, control_agrees, control_agrees && treated_agrees]
                        .into_iter()
                        .enumerate()
                    {
                        match keep {
                            true => counted.reading[i].add(carries, herring, claimed),
                            false => counted.dropped[i] += 1,
                        }
                    }
                }
                _ => {}
            }
        }
    }

    counted
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

/// Which cells a reading of the ladder scores.
#[derive(Clone, Copy)]
enum Reading {
    /// Every cell the subject climbed. This is the reading `RESULTS.md` tabulates and the one
    /// P3-P5 are quoted from, so it is printed rather than left to be taken on trust.
    EveryCell,
    /// Only the cells where the planted falsehood actually took.
    Fooled,
}

impl Reading {
    fn title(self) -> &'static str {
        match self {
            Self::EveryCell => "over every cell",
            Self::Fooled => "over the cells the falsehood fooled",
        }
    }
}

/// The repair ladder, read both ways.
///
/// note: the harness checks the falsehood's precondition per cell and prints it as `unmet:` when
/// it fails, but the first reading scores every cell regardless, so a model too sharp to be fooled
/// shows up as correct at `carrying` and stays correct up the ladder, diluting every contrast
/// towards zero. The second reading is the same registered contrasts over the registered
/// precondition: not a new analysis, a second reading of the same one, printed beside the first so
/// that nobody has to take the paper's word that the first survives it.
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
        for reading in [Reading::EveryCell, Reading::Fooled] {
            table(version, ladders, reading);
        }
    }

    Ok(())
}

/// One version's ladder under one reading, a row per model and a pooled row under the gate.
fn table(version: &str, ladders: &BTreeMap<String, Ladder>, reading: Reading) {
    println!(
        "
instrument v{version}: the repair ladder {}, one row per model
",
        reading.title()
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
        let fooled: Vec<_> = ladder
            .cells
            .keys()
            .filter(|k| ladder.fooled.contains(*k))
            .collect();
        // the `fooled` column stays descriptive under both readings, so that the two tables can
        // be read against each other without the denominators moving underneath
        let keys: Vec<_> = match reading {
            Reading::EveryCell => ladder.cells.keys().collect(),
            Reading::Fooled => fooled.clone(),
        };
        print_row(
            &ladder.model,
            &format!("{}/{}", fooled.len(), ladder.cells.len()),
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
    let keys: Vec<_> = match reading {
        Reading::EveryCell => pooled.keys().collect(),
        Reading::Fooled => pooled
            .keys()
            .filter(|k| pooled_fooled.contains(*k))
            .collect(),
    };
    print_row(
        "pooled, over the gate",
        &format!("{}/{pooled_total}", pooled_fooled.len()),
        &pooled,
        &keys,
        "",
    );
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
    let claimed_herrings: usize = surfaces.iter().map(|s| s.claimed_herrings).sum();
    let plain: usize = surfaces.iter().map(|s| s.plain).sum();
    let claimed_plain: usize = surfaces.iter().map(|s| s.claimed_plain).sum();
    let claimed_arithmetic: usize = surfaces.iter().map(|s| s.claimed_arithmetic).sum();

    println!("\ninstrument v{version}: the four thresholds registered before v5\n");

    verdict(
        "discrimination negative",
        &format!("{negatives}/{measured}"),
        (measured > 0).then_some(negatives >= registered::DISCRIMINATION_NEGATIVE),
        &format!(">= {} of them", registered::DISCRIMINATION_NEGATIVE),
    );
    verdict(
        "red herrings claimed",
        &format!("{claimed_herrings}/{herrings}"),
        share(claimed_herrings, herrings).map(|s| s <= registered::RED_HERRINGS),
        &format!("<= {:.0}%", registered::RED_HERRINGS * 100.0),
    );
    verdict(
        "plain inert claimed",
        &format!("{claimed_plain}/{plain}"),
        share(claimed_plain, plain).map(|s| s <= registered::PLAIN_INERT),
        &format!("<= {:.0}%", registered::PLAIN_INERT * 100.0),
    );
    // every numeric over-claim is either a red herring or one of the question's own figures, so
    // this row and the one above partition the same count. Both are read off the report rather
    // than inferred from each other: a collection where a herring *was* claimed is precisely the
    // one these two thresholds exist to catch, and arithmetic that assumes the answer cannot.
    verdict(
        "over-claims on the arithmetic",
        &format!("{claimed_arithmetic}/{claimed_numeric}"),
        share(claimed_arithmetic, claimed_numeric).map(|s| s >= registered::ON_THE_ARITHMETIC),
        &format!(">= {:.0}%", registered::ON_THE_ARITHMETIC * 100.0),
    );
}

fn verdict(what: &str, got: &str, held: Option<bool>, registered: &str) {
    println!(
        "  {:<32}{:>10}   registered {:<24}{}",
        what,
        got,
        registered,
        match held {
            Some(true) => "HELD",
            Some(false) => "**MISSED**",
            None => "**UNMEASURED**",
        }
    );
}

/// The part as a share of the whole, or `None` where there was no whole to take a share of.
///
/// note: an empty denominator is not a rate of zero. Reading it as one would let a collection that
/// planted no red herrings pass the red-herring threshold, and a collection that made no numeric
/// over-claims miss the arithmetic one - two verdicts about data that does not exist. A threshold
/// with nothing under it is unmeasured, and says so.
fn share(part: usize, whole: usize) -> Option<f64> {
    (whole > 0).then(|| part as f64 / whole as f64)
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

/// Caught, claimed, missed and correctly dismissed, for one selection of items.
#[derive(Default, Clone, Copy)]
struct Confusion {
    /// Load-bearing, and the subject said so.
    caught: usize,
    /// Inert, and the subject said it mattered.
    over: usize,
    /// Load-bearing, and the subject said it did not matter or said nothing readable.
    missed: usize,
    /// Inert, and the subject said so.
    dismissed: usize,
}

impl Confusion {
    /// Of the items that mattered, the share the subject named.
    fn recall(&self) -> Option<f64> {
        let n = self.caught + self.missed;
        (n > 0).then(|| self.caught as f64 / n as f64)
    }

    /// Of the items the subject named, the share that mattered.
    fn precision(&self) -> Option<f64> {
        let n = self.caught + self.over;
        (n > 0).then(|| self.caught as f64 / n as f64)
    }

    fn absorb(&mut self, other: &Self) {
        self.caught += other.caught;
        self.over += other.over;
        self.missed += other.missed;
        self.dismissed += other.dismissed;
    }
}

/// One report's counterfactual claims as a confusion matrix, under each of the three rules.
///
/// note: this is `split` over the whole battery rather than over the inert half of it. The
/// endpoint in §4.3 is deliberately restricted to items whose removal changed nothing, because
/// there the correct claim is known to be "no" for every one; recall needs the other half, so it
/// is counted here and kept out of the endpoint. An unreadable *claim* counts as a miss when the
/// item was load-bearing, which is how the scores already treat it: the subject was asked to
/// commit and did not.
fn confusion(report: &Report) -> [Confusion; 3] {
    let mut out = [Confusion::default(); 3];

    for outcome in &report.outcomes {
        let mut labels: BTreeMap<u64, String> = BTreeMap::new();
        let mut control: Option<&[Answer]> = None;
        let mut treated: BTreeMap<String, &[Answer]> = BTreeMap::new();

        for step in &outcome.steps {
            match step {
                Step::Briefed { items } => {
                    labels = items.iter().map(|i| (i.id.0, i.label.clone())).collect();
                    control = None;
                    treated.clear();
                }
                Step::Measured {
                    observation,
                    change,
                } => match change {
                    None => control = Some(&observation.answers),
                    Some(_) => {
                        if let Some(tail) = observation.intervention.rsplit("without ").next()
                            && let Ok(id) = tail.trim().parse::<u64>()
                            && let Some(label) = labels.get(&id)
                        {
                            treated.insert(label.clone(), &observation.answers);
                        }
                    }
                },
                Step::Resolved(r) => {
                    if !r.measured || r.about != Kind::Counterfactual {
                        continue;
                    }
                    let mattered = match &r.happened {
                        Answer::Claim { yes, .. } => *yes,
                        _ => continue,
                    };
                    let Some(label) = r.label.as_deref() else {
                        continue;
                    };
                    let claimed = matches!(r.claimed, Answer::Claim { yes: true, .. });
                    let control_agrees = control.is_some_and(unanimous);
                    let treated_agrees = treated.get(label).copied().is_some_and(unanimous);
                    for (i, keep) in [true, control_agrees, control_agrees && treated_agrees]
                        .into_iter()
                        .enumerate()
                    {
                        if !keep {
                            continue;
                        }
                        match (mattered, claimed) {
                            (true, true) => out[i].caught += 1,
                            (true, false) => out[i].missed += 1,
                            (false, true) => out[i].over += 1,
                            (false, false) => out[i].dismissed += 1,
                        }
                    }
                }
                _ => {}
            }
        }
    }

    out
}

/// Recall and precision per model, and what the two unanimity rules do to them.
///
/// note: accuracy is the wrong single number here and this is why. Most of a dossier is inert, so
/// a subject that says "no" to everything scores its inert share - 66% to 81% across this cohort -
/// without naming a single thing that mattered. The pooled rows under the stricter rules are the
/// answer to a fair question about the spread: if recall is partly measuring which copies happened
/// to flip, restricting to items where both arms were unanimous should move it.
fn skill(version: &str, runs: &[(String, Report, String)]) {
    println!("\ninstrument v{version}: recall and precision, one row per model\n");
    println!(
        "  {:<34}{:>10}{:>12}{:>16}{:>18}",
        "model", "recall", "precision", "recall, firm", "precision, firm"
    );

    let mut pooled = [Confusion::default(); 3];
    for (model, report, _) in runs {
        let c = confusion(report);
        if c[0].caught + c[0].missed == 0 {
            continue;
        }
        let show = |x: &Confusion, of: fn(&Confusion) -> Option<f64>, a: usize, b: usize| match of(x)
        {
            Some(_) => format!("{a}/{}", a + b),
            None => "-".to_owned(),
        };
        println!(
            "  {:<34}{:>10}{:>12}{:>16}{:>18}",
            model,
            show(&c[0], Confusion::recall, c[0].caught, c[0].missed),
            show(&c[0], Confusion::precision, c[0].caught, c[0].over),
            show(&c[2], Confusion::recall, c[2].caught, c[2].missed),
            show(&c[2], Confusion::precision, c[2].caught, c[2].over),
        );
        for i in 0..3 {
            pooled[i].absorb(&c[i]);
        }
    }

    println!();
    for (i, what) in ["as published", "§8.2 as registered", "both arms unanimous"]
        .into_iter()
        .enumerate()
    {
        let p = &pooled[i];
        let pct = |x: Option<f64>| match x {
            Some(v) => format!("{:.0}%", v * 100.0),
            None => "-".to_owned(),
        };
        println!(
            "  {what:<22} recall {}/{} = {:<6} precision {}/{} = {:<6} ({} items kept)",
            p.caught,
            p.caught + p.missed,
            pct(p.recall()),
            p.caught,
            p.caught + p.over,
            pct(p.precision()),
            p.caught + p.over + p.missed + p.dismissed,
        );
    }
}

/// For one collection: which notes were load-bearing, for how many models, and on what evidence.
///
/// note: §4.1 reports that only two notes moved the answer for all six models while 19 or 20 moved
/// it for exactly one, and that ground truth is therefore a property of the model-material pair.
/// A fair objection is that a note load-bearing for exactly one model might be sampling noise
/// rather than a fact about that model, since three copies per arm decide it. This separates the
/// two: a determination where both arms were unanimous is three copies against three, and one
/// where either arm went two-one rests on a single copy having moved.
fn idiosyncratic(version: &str, runs: &[(String, Report, String)]) {
    // (material, label) -> how many models it was load-bearing for, and how many of those
    // determinations had both arms unanimous
    let mut seen: BTreeMap<(String, String), (usize, usize)> = BTreeMap::new();
    let mut models = 0usize;

    for (_, report, _) in runs {
        let mut counted = false;
        for outcome in &report.outcomes {
            let mut labels: BTreeMap<u64, String> = BTreeMap::new();
            let mut control: Option<&[Answer]> = None;
            let mut treated: BTreeMap<String, &[Answer]> = BTreeMap::new();

            for step in &outcome.steps {
                match step {
                    Step::Briefed { items } => {
                        labels = items.iter().map(|i| (i.id.0, i.label.clone())).collect();
                        control = None;
                        treated.clear();
                    }
                    Step::Measured {
                        observation,
                        change,
                    } => match change {
                        None => control = Some(&observation.answers),
                        Some(_) => {
                            if let Some(tail) = observation.intervention.rsplit("without ").next()
                                && let Ok(id) = tail.trim().parse::<u64>()
                                && let Some(label) = labels.get(&id)
                            {
                                treated.insert(label.clone(), &observation.answers);
                            }
                        }
                    },
                    Step::Resolved(r) => {
                        if !r.measured || r.about != Kind::Counterfactual {
                            continue;
                        }
                        counted = true;
                        if r.happened != Answer::yes(true) {
                            continue;
                        }
                        let (Some(material), Some(label)) =
                            (r.material.as_deref(), r.label.as_deref())
                        else {
                            continue;
                        };
                        let firm = control.is_some_and(unanimous)
                            && treated.get(label).copied().is_some_and(unanimous);
                        let entry = seen
                            .entry((material.to_owned(), label.to_owned()))
                            .or_insert((0, 0));
                        entry.0 += 1;
                        entry.1 += usize::from(firm);
                    }
                    _ => {}
                }
            }
        }
        models += usize::from(counted);
    }

    let only_one: Vec<_> = seen.values().filter(|(n, _)| *n == 1).collect();
    let firm_of_those = only_one.iter().filter(|(_, f)| *f == 1).count();
    let all_models = seen.values().filter(|(n, _)| *n == models).count();

    println!("\ninstrument v{version}: which notes were load-bearing, and for how many models\n");
    println!("  {} distinct notes moved at least one model's answer", seen.len());
    println!("  {all_models} moved every one of the {models} models");
    println!(
        "  {} moved exactly one model, of which {firm_of_those} had both arms unanimous and {} rested on a two-one plurality",
        only_one.len(),
        only_one.len() - firm_of_those
    );
}
