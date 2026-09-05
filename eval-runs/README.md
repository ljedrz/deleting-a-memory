# eval-runs

Results from `nachalnik-eval`, kept out of the instrument's crate on purpose: the harness should not
know what this study measured with it.

```text
eval-runs/<model>/<UTC timestamp>-<experiments>-r<replicates>/
    meta.json     what was run, against what, at which commit
    summary.txt   exactly what the `bench` example printed
    report.json   the whole record: every question, every answer verbatim,
                  every copy's reply, every comparison, and the scores
```

The timestamp is the start of the run, in UTC, so the directories sort chronologically. `meta.json`
carries the commit the harness was built from, because a score is only comparable with another
score taken by the same questions.

`meta.json` names the harness commit each run was built from, and for 41 of the 65 runs that commit
no longer resolves: `nachalnik-eval` was split out into its own crate on 2026-09-04 and its history
remade with it. What a run asked is identified by the instrument version and digest inside its
`report.json`, which is what comparability rests on, and the commit `Cargo.toml` pins re-scores
every report here.

`meta.json` also carries an `instrument_version`, and on every run from 2026-09-03 it says `3` where
the instrument was v4 — a missed bump, left as it was recorded rather than edited after the fact.
The authoritative version is the one inside `report.json`, on each outcome's `instrument` beside its
digest, and that is what the analysis reads.

**No API key appears anywhere in here.** The endpoint does, because a run against Google's
OpenAI-compatible shim and a run against the same model somewhere else are not the same
measurement.

To reproduce one, read its `meta.json` and run:

```console
$ NACHALNIK_API_KEY=... NACHALNIK_BASE_URL=<endpoint> \
    cargo run -p nachalnik-eval --example bench -- -m <model> -r <replicates> --json out.json
```

`report.json` is enough to re-score a run without paying for it again: every answer is in it
verbatim, beside the `shape` it was asked in.
