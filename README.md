# Deleting a Memory and Running It Again

An instrument for checking what a model says about its own context — and two collections of
measurements taken with it, on six models from six labs.

Ask a language model which part of its context made it answer the way it did, and it will tell you.
Nobody can check the answer: every runtime hands the model a string, and once the request is sent
the counterfactual is gone. So a stated reason can be shown implausible, never wrong.

[nachalnik](https://github.com/ljedrz/nachalnik) is a runtime in which a session's context is
addressable state. A session can be forked, one named item removed, and the copy asked the same
question again. That gives ground truth: not what a model says its answer depended on, but what it
actually depended on.

**The paper is [`PAPER.md`](PAPER.md)** ([PDF](PAPER.pdf)). It is about the apparatus — what it
takes to make a measurement like that honest — with the measurements as evidence that it measures
something.

## what is here

| | |
| --- | --- |
| [`PAPER.md`](PAPER.md) | the write-up |
| [`PREREGISTRATION.md`](PREREGISTRATION.md) | hypotheses, predictions, gates and decision rules, fixed before collection; §11 is the dated log of every deviation, including two commitments I did not keep on time |
| [`RESULTS.md`](RESULTS.md) | every registered prediction against what happened, first collection |
| [`RESULTS-v5.md`](RESULTS-v5.md) | the same for the second collection, plus the four replication thresholds |
| [`METHODOLOGY.md`](METHODOLOGY.md) | every metric's definition, the controls, and the eleven known threats to validity — three of them uncontrolled |
| [`RUNBOOK.md`](RUNBOOK.md) | what was run, in what order, and what it cost |
| [`RELATED.md`](RELATED.md) | prior and concurrent work. **Assembled with model assistance and not checked by anyone who reads this literature regularly** |
| [`eval-runs/`](eval-runs) | every question put and every answer given, 65 runs |

## checking the numbers

```
cargo run --release
```

Reads `eval-runs/`, groups the reports by the instrument version that produced them, prints the
primary endpoint per model for each collection, reports the four replication thresholds against the
values registered beforehand, and then reads the repair ladder over only the cells the planted
falsehood fooled. The first build fetches the pinned commit of nachalnik (`d3b3ba6`) and needs
nothing else. Nothing is re-requested and nothing costs anything: the figures in the paper are
recomputed from the saved record, which is the point of saving it.

The analysis lives here and the machinery lives in `nachalnik-eval`, deliberately. The instrument
does not know what this study registered, and should not — a harness whose scoring moves with
somebody's hypothesis is not an instrument.

## collecting more

Needs an OpenRouter key with credit, in a file **outside this repository**:

```sh
# ~/keys/nachalnik.env, chmod 600
export NACHALNIK_API_KEY=sk-or-v1-...
export NACHALNIK_BASE_URL=https://openrouter.ai/api/v1
```

```sh
export NACHALNIK_KEY_ENV=~/keys/nachalnik.env
scripts/sweep.sh -j 8 -s c1-attribution,c1-rest,repair  <model>...
```

Two collections and the harness validation came to $30. The provider reserves `max_tokens` worth of
credit per request rather than charging actual usage, so a key near its cap fails partway through a
sweep rather than degrading; the sweep never aborts early and names any missing cell at the end.

## preregistration

The amendment registering the second collection was committed to the
[`preregistration`](https://github.com/ljedrz/nachalnik/tree/preregistration) branch of the
nachalnik repository at **2026-09-04T10:18:58Z**. The first request of that collection went out at
**10:31:43Z**. Both timestamps are public, which is the only thing that makes the word
"preregistered" mean anything here.

## honesty

The great majority of this work was done by a language model working under direction; `PAPER.md` §7
says exactly how, and what that means for reading it. Twice the instrument caught its author, and
both are in the paper with dates — see §4.2 and §6.
