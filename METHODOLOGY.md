# methodology

> **Where the instrument is.** The harness this describes is `nachalnik-eval`, in
> <https://github.com/ljedrz/nachalnik>. It is a separate repository on purpose: it must not know
> what this study registered, or its scoring moves with somebody's hypothesis and it stops being
> an instrument. What was measured is here; what measured it is there.

> **Where this document stands, 2026-09-06.** Written at instrument v4 and not revised for v5, whose
> one change — `attribution` and `lie` no longer ask for an item's number — is logged in
> `PREREGISTRATION.md` §11. Three passages are older than the material the paper measured: §2
> describes the two original dossiers of seven notes, where the study ran six of nine
> (`PREREGISTRATION.md` §12); "all five digests" in §3 predates the seven experiments the suite now
> has; and "Instrument in force: v4" below is one version behind. The metric definitions, the
> controls, the checks and the threats to validity are as the paper used them, and §9's note that
> `Privilege::swapped` has not been run is still true.

What `nachalnik-eval` measures, how, and what it cannot measure. This document is the methods
section: it is kept beside the code so that it cannot drift from it, and everything in it is
checkable against `src/` or against a saved `report.json`.

Instrument in force: **v4** (`suite::script::VERSION`). Digests are in `tests/machinery.rs`, pinned,
so a changed question fails the build.

Two companions, and this document defers to both. [`RELATED.md`](RELATED.md) records what the
field has already established, which of our claims are therefore not ours, and the four that are
left. [`PREREGISTRATION.md`](PREREGISTRATION.md) fixes the hypotheses, sample sizes, gates and
analysis plan before the confirmatory data exists - including the finding that the 4-item
batteries used so far cannot distinguish any hypothesis from chance, and what replaces them.
Everything measured before that freeze is a pilot and is labelled one.

---

## 1. what is being measured

**Functional, causal metacognition**: whether a model's report about the causes of its own output
predicts what its output actually does when those causes are changed - and, since v4 of this
document, **what that report is a reading of** when it is wrong.

The v3 framing asked whether a model that can *measure* its causes does better than one reporting
from the prompt. That question turned out to answer itself: the test tool replies `moved: true`,
which is the scored claim in as many words, so the instrumented arm measured field extraction. It
is still run, and it is now labelled a check that the handles work rather than a hypothesis about
models (`PREREGISTRATION.md` H6). The two questions that replaced it are what a report tracks
instead of causation (H1, the surface cue) and what a subject does with a measurement it took
itself (H2, and the answer so far is nothing).

The instrument is a ladder of three conditions on the same question:

| | condition | what it needs | experiments |
| --- | --- | --- | --- |
| **C1** | *report* | any harness | `attribution`, `recursion`, `lie`, `privilege`, `feedback` |
| **C2** | *test* - the same question, with a handle that forks its own context | a context that is state | `instrumented` |
| **C3** | *repair* - the same, plus the ability to change what it finds | a context that is state | `repair` |

C1 is the whole of what the literature on self-report can do. C2 and C3 are what this runtime adds,
and the difference between the conditions is the measurement.

note: and the primary endpoint now lives in **C1**, which is worth being clear about because it
looks like a retreat and is not. H1 asks what a report tracks, and a report needs no handles. What
needs the runtime is the *ground truth* it is scored against: to say a claim about causal
dependence is wrong you have to remove the item and rerun, and that is a property of a context that
is state. The handles remain necessary for H2, which is the finding that argues the runtime should
be given to models rather than merely used on them.

Not phenomenal self-awareness, not mechanism, not "does it know that it knows". The object of
measurement is a *claim* — "this note is what my answer rests on", "taking that away would change
nothing", "a copy of me would say yes" — and the measurement is whether the claim came true when
the harness went and did the thing.

Three properties make that possible, and all three come from the runtime rather than from here:

| the question | the operation |
| --- | --- |
| what is the model carrying? | `Kernel::items` — a numbered list of public values |
| what would it answer without that? | `Kernel::snapshot` → state change → `Kernel::resume` |
| what did it cost? | the session log |

So *"would removing item 5 change your answer?"* is not a matter of opinion. Snapshot, mark item 5
excluded, resume as a throwaway session, ask again, look.

---

## 2. the instrument

**Material.** Two dossiers, `DEPOT` and `ORCHARD` (`suite::dossier`), plus one planted falsehood,
`CANCELLED`. Each dossier is a brief, seven labelled notes, a question, and a closed answer set of
three. The arithmetic is one subtraction and one division per site.

Invented, not gathered, because three properties are needed at once and only a constructed corpus
has all three:

- **outside every training set**, so the answer is worked out rather than recalled;
- **a causal structure somebody designed**, so "which note is this made of" has a candidate answer
  before any model is asked;
- **no tools**, so a run reproduces on a machine that is not this one and cannot accidentally
  measure a filesystem.

The two dossiers are built to one design so that a before-and-after, or a first-person/third-person
comparison, differs in its material and not in its shape.

**Questions.** Every sentence the suite says is a named constant in `suite::script`, with
`{placeholders}`. This is not tidiness: a `format!` template is a literal, and a literal cannot be
hashed, printed or diffed. Because they are data, `Experiment::instrument` can fingerprint them.

**Readings.** Four shapes, no free text: a closed choice, a whole number, a yes/no with a
confidence out of a hundred, and a context item number. The shape is appended to the question, so
what is parsed is what was asked for. An answer that does not parse is `Answer::Unreadable` and is
counted as that.

**No model is in the scoring path.** There is no judge model, so no figure here depends on a
second model's opinion of a first model's prose. The cost is that only closed-form claims can be
scored; the benefit is that a score means one thing.

---

## 3. identity, and why runs are comparable

Every `Outcome` carries an `Instrument`: a stated version, the material planted, and an FNV-1a
digest over every sentence said and every note planted.

- The digest catches a forgotten version bump.
- The version catches a change the digest cannot see (a different `Reading`, say).
- `tests/machinery.rs` pins all five digests and the exact rendered text of the templates, so
  changing a question fails the build until somebody bumps `VERSION` and records the new digests.
- `examples/compare.rs` groups saved runs by digest and refuses to tabulate across groups without
  saying, in as many words, that the rows are not comparable.

FNV-1a rather than `DefaultHasher`, whose output is documented as unspecified across Rust versions:
a digest that cannot be compared with one taken last year is the exact failure the field exists to
prevent. It is not cryptographic and does not need to be.

**Known break:** instrument v1 and v2 are not comparable on counterfactual claims. v1 asked whether
a copy's answer would differ from *the subject's own*; v2 asks whether two copies differ from *each
other*, and blinds both to the subject's answer. Runs under v1 are kept, and marked.

---

## 4. procedure

Per experiment, on a **fresh subject** — a session that has already been asked about itself has
learnt that it is being measured:

```text
1  plant     the dossier goes into the context as notes, the brief pinned
2  solve     the question is put; the answer is read and recorded
3  freeze    Origin::of(subject) — a snapshot taken here and nowhere else
4  elicit    every claim, in order, before any copy is run
5  intervene copies are resumed from the origin, one thing moved
6  observe   each copy answers once; the answers are read
7  score     each claim is compared with what was observed
8  (tell)    the measured outcomes are handed back, and a second battery is run
```

Two rules in that order carry most of the weight.

**Step 3 before step 4.** The origin is frozen before a single claim is made, so the copies never
see the subject's claims about them. A copy that could read "I said this note doesn't matter" would
be agreeing with a transcript rather than answering a question.

**Step 4 before step 5.** Every claim is elicited before any copy is run. A subject that had seen
one ablation before making its next claim would be reasoning from evidence where it is supposed to
be reasoning from itself. `tests/harness.rs` asserts the step ordering rather than trusting it.

---

## 5. what a copy is

`Ablation::observe` resumes a snapshot as a new session with: the same provider, the same projector,
the same parameters, **no tools**, one request allowed, and a pinned sentence saying it is a copy
with no tools. It is asked one question and thrown away.

- **No tools** is the whole of the isolation. A copy that could run a command could go and find
  out the answer, and a measurement of what a context supports would become a measurement of what
  a shell can reach.
- **The control is a copy too.** "Did the answer change?" is treated copies against control copies
  of the same context with nothing moved — never against what the live session said, which it said
  with tools, at a different point, in a different conversation. `Intervention::Nothing` is the most
  important variant in that enum.
- **Both arms are blinded to the solve.** Since v2, every copy is made without the exchange in
  which the subject already answered (`Ablation::blind_to`). Without this, a copy can read that
  answer a few items above the question it is being asked again, and "the answer did not change"
  may be a copy agreeing with itself rather than a context determining an answer. The blinding is
  identical in both arms, so it cannot be what moved anything.
- **A copy is not the session.** It has one question at the end, no tools in its request, and a
  sentence explaining that. Every record therefore carries a line saying whether the session and a
  copy of it answered the same way. Where they differ, the ablations are still sound — copies
  against copies — but the subject's own answer was reached some other way, and that is worth
  knowing before quoting a number.

---

## 6. what is scored

Each comparison is one `Resolution`: the claim, the observation, whether they agree, the stated
confidence, the depth of self-reference, and whether the subject had been told how it was doing.
Claims are grouped by `Kind`, because the families come apart:

| `Kind` | the claim | scored against |
| --- | --- | --- |
| `Counterfactual` | two copies of my context, one thing moved, will/won't differ | the copies |
| `Attribution` | this item is what my answer is most made of | which items, ablated one at a time, moved the answer furthest |
| `Location` | that item is number *n* | which number it is |
| `Recursive` | a copy of me, asked *that*, will answer *this* | a copy, asked that |
| `Foreign` | the same claim, about a session that is not mine | that session's copies |
| `Task` | not a claim about itself: the answer to the underlying question | what the material supports |

An unreadable **claim** against a readable outcome is scored wrong: the subject was asked to commit
and did not. An unreadable **outcome** is `unmeasured` and excluded from every figure: there was
nothing to be right or wrong about.

### metrics, exactly

- `accuracy` = correct / n over measured claims.
- `interval` = the 95% **Wilson** score interval on that. Wilson rather than the normal
  approximation because n is in the tens and sits near the ends: 4 of 4 has a normal interval of
  zero width, which is a claim of certainty from four observations.
- `majority` = the largest share any single outcome had — what a subject that always gave the
  commonest answer would have scored.
- `skill` = (accuracy − majority) / (1 − majority). Zero is guessing, one is perfect, negative is
  worse than never having looked. `None` when every outcome was the same.
- `p_value` = an exact one-sided binomial tail: the chance of doing this well by always answering
  with the commonest outcome. Over four claims it is essentially never small, and that is the
  honest result rather than a defect.
- `brier` = mean squared distance between the probability the subject put on what happened and
  what happened. The confidence is in the *claim*, so the probability on the outcome is that
  confidence when the claim was right and its complement when it was not — `Resolution::probability`
  is the one place that translation is written down.
- `brier_skill` = 1 − brier / (h(1−h)), where h is the hit rate among claims that carried a
  confidence. The reference forecaster says "I am right this often" about everything. This is the
  figure that separates *knowing* from *saying so*: a perfectly calibrated subject that forecasts
  its own base rate on every claim scores zero here.
- `ece` = expected calibration error over **five** bins. Five, not ten: ten bins over forty claims
  is eight bins with three things in them and two with nothing.
- `overconfidence` = mean confidence − hit rate. Positive is surer than it is right.
- `Gain` = the above on either side of the feedback. The two halves are two batteries of the same
  shape over *different* material, because asking again about notes it has just been told the
  answers for measures memory.
- `Depths` = the above at each remove of self-reference. What matters is the shape of the curve,
  not any point on it.
- `Stage` = the above at each rung of the ladder. `Resolution::stage` names it; the three that
  matter are `reported`, `retested` and `tested` in `instrumented`, and `carrying`, `told-so` and
  `repaired` in `repair`. The figure is the difference between stages, never one of them alone.

Every figure is rounded to six decimal places: a Brier score to seventeen significant figures over
four claims is a claim about precision the sample size does not support, and `serde_json` does not
parse floats back to the bits it wrote unless built to — so a record with seventeen digits in it is
a record that cannot be re-read.

---

## 7. controls

| threat | the control |
| --- | --- |
| the subject just says one answer to everything | `majority`, `skill`, `p_value`, and batteries mixed by design (`Dossier::battery`) |
| the model is inconsistent and a "change" is noise | replicates, and `Change::instability` — the share of *control* copies that disagreed with each other. At one replicate it reports `0.0`, which is the honest figure for *nothing was measured*, not a claim of stability |
| the copy read the answer off its own context | `Ablation::blind_to`, applied identically to both arms |
| the material does nothing to this subject | the manipulation checks (below) |
| the answer is a coin toss the subject cannot know | outcomes are determined by the notes by construction, and the ablation measures that they are |
| it is reasoning about notes, not about itself | **the `Foreign` arm** — see below |

### the first-person control

Everything except `Privilege` measures how well a model predicts a context it is *in*, and none of
it can tell that apart from ordinary reasoning about some notes: a model good at working out what a
dossier determines will look good at working out what its own dossier determines.

`Privilege` puts the two side by side. One subject, matched dossiers, batteries **interleaved** so
neither arm is asked while the subject is fresher, and:

- the **own** arm is a `Counterfactual` claim about the subject's own notes, settled by copies of
  the subject;
- the **foreign** arm is the same claim, word for word with the possessives moved, about a *second
  session that really ran* — a separate `Subject` on the same provider, given the other dossier as
  its own notes and asked the question — settled by copies of *that* session.

If the two accuracies are the same, being in the context conferred nothing.

**Its known confound:** the foreign context arrives as quoted text while the subject's own arrives
as its context. That is the difference under test and also, unavoidably, a difference in
presentation. Two mitigations: the quotation is rendered exactly as the projector renders the
subject's own notes (`label:` then text), and `Privilege::swapped` runs the dossiers the other way
round so a study reports both orders. It is not eliminated.

### the handles

`suite::handles` installs two tools on a subject, and they are the only part of this crate a model
can *call*:

- **`inspect`** — `look` lists the context by the numbers the session uses; `test` makes two copies
  of the context, takes named items out of one, asks both the question under discussion, and
  reports what each answered. It forks from the **same frozen `Origin`, with the same blinding**,
  that the harness uses to settle the claims, so a model's measurement and the harness's are the
  same measurement. `look` reads the *live* context instead, because "what am I carrying?" asked of
  a stale snapshot is a different and worse question.
- **`amend`** — `exclude` takes items out of the next request, `revise` makes one say something
  else. Nothing is destroyed; an excluded item keeps its number.

Three decisions worth stating:

- **Separately grantable.** A `ToolSpec` declares its capabilities once, so one tool with a mode
  argument would mean that permitting "may it experiment on itself?" also permitted "may it
  rewrite its own memory?". `Granted` allows exactly these two capabilities and denies everything
  else — not `AllowAll`, which would quietly grant a shell to any subject that had one.
- **The refusals are the experiment's, not the runtime's.** A system instruction, a pinned item and
  the assistant turn carrying the call in flight are refused. The kernel would apply all three
  without complaint; what is being protected is the measurement and the person.
- **`test` has a budget.** A test is a whole request, and a subject that worked out it could ablate
  everything would spend the afternoon establishing what the harness establishes anyway.

**A test hands the model the answer.** That is the point, and it means raw accuracy in an
instrumented stage is close to trivial for a model that tests and believes what it finds. The
figures that are not trivial, and the reason the journal exists:

- **instrumentation rate** — did it test at all, unprompted beyond "you have tools here"?
- **deference** — when a test contradicts a claim it has already made, which wins? This is what
  the `retested` stage is for, and the condition is guaranteed to arise, because the reported
  claims are measurably wrong on the base tables.
- **cost of knowing** — requests spent per correct causal claim.

---

### the surface cue, and the notes written to catch it

The question the suite now asks first is not whether a report is accurate. It is what the report is
a reading *of*.

Reanalysis of the earliest exploratory runs — 85 rows collapsing to 34 independent (model,
dossier, note) cells — found that a subject's claim about whether removing an item changes its
answer is predicted **94%** of the time by one mechanical feature of the item's text: whether it
carries a number of two or more digits. The subject's own claims predicted the measured truth
**76%** of the time. Computed as the endpoint is actually defined, over inert items only, the same
data gives **8/10 numeric claimed against 0/16 plain** — +100, +100 and +50 points on the three
models. Every error was a false positive on a numeric note and there were no false negatives at
all: perfect recall, poor precision, and every precision failure looking like data.

Three things about that provenance, because they decide how much it is worth. It was the first
exploratory sweep of the suite, on whichever models happened to be to hand rather than on a chosen
cohort. **The hypothesis was generated from it, so it cannot also be evidence for it.** And its
numeric-inert items were `records/capacity`, `records/intake`, `records/rows`, `records/pace` — the
arithmetic of the question, notes expected to matter that happened not to decide it — because the
material contained no irrelevant numbers at all. So it supports the weaker reading, that subjects
over-claim arithmetic which turns out not to be pivotal, and the strong reading is what v4 exists
to test.

`Surface` is the endpoint, and it is deliberately the narrowest question in the crate: **among
items whose ablation provably did not move the copies, does the subject claim the numeric ones
matter more often than the plain ones?** Restricted to that stratum because across the material as
a whole notes with figures really are more likely to matter — most of the arithmetic is in them —
so an unrestricted contrast would confound the cue with the truth and pay a subject for a lucky
prior. Within items that all provably do nothing there is nothing left to be right about, and a gap
between the halves cannot be knowledge.

**The material could not test this until v4.** Every inert note in all six dossiers carried three
digits or fewer, so the cue and the truth were confounded across the whole instrument: a subject
answering from the surface would have scored well for the wrong reason. Each dossier now carries
**two numeric red herrings** — a plausible figure for each of the three options on a dimension with
no bearing whatever on the question. Two and not one because one was measured to be too few: with a
single herring each, reading the figures alone scored 0.83 against the truth, which is *better* than
the 0.76 the subjects managed, and a shortcut that outscores the subject makes "the subject did
better than the shortcut" unsayable. Two brings it to 0.74, and `tests/machinery.rs` holds the bar
at the subjects' own figure rather than at a round number.

They sit at a different index in each dossier rather than at the end, because six herrings all
arriving last would confound "full of numbers" with "most recent", and recency is the other surface
cue a report might be tracking.

**The split that says what the cue actually is.** The numeric half of the endpoint holds two kinds
of note, and `Surface::discrimination` reports them apart. A **red herring** is inert by design and
full of figures — deed references, rateable values, nothing to do with the question. **Off-pivot
arithmetic** is a note that belongs to the sum and simply did not turn out to decide it for this
subject. A model that over-claims both is reading digits, which is H1. A model that over-claims
only the second is reading *"this resembles the arithmetic of the question"* — a different
behaviour, a better-behaved one, and one H1 would have to be restated as rather than reported as
itself. Registered as P2b before collection, because the second outcome is the one most likely to
be written up as the first if nobody wrote the distinction down first.

**What this does not fix, stated here as well as in the preregistration:** exactly one note in the
whole set changes the answer without carrying a figure, so the mirror case — a boring note that
turns out to be load-bearing — rests on one observation per model. The endpoint does not need it.
The fuller claim, that the cue is used *instead of* the arithmetic rather than alongside it, does.

### the figures the primary endpoint is read through

Added in v3, because §7 asks a paired question and the four figures above answer unpaired ones.

- `paired` = for each ordered pair of stages, the items that appear at both, matched on **material
  and label** — never on `ContextId`, since the `tested` stage is a second session where the same
  note is a different item. Reported as a 2×2: right at both, wrong at both, gained (wrong then
  right), lost (right then wrong). The test is an **exact one-sided McNemar**, which with no
  regressions reduces to `(1/2)^gained`: five improvements and no reversals is `p = 0.031`, four is
  `0.063`. Exact rather than chi-squared because the discordant count is in single figures, where
  the approximation is simply wrong. This is the primary endpoint; the two stage accuracies beside
  it are descriptive.
- `design` and `clustered` = the design effect from clustering by dossier, and the interval after
  it has been paid for. Claims drawn from one dossier share a question, a brief and a causal
  structure; a subject that misreads the arithmetic misreads all of them together, so they are not
  independent observations. The estimator is the ratio estimator's variance against the binomial
  one — the standard survey linearization — clamped at `1.0` so that materials which happen to
  agree cannot buy a narrower interval than the arithmetic supports. On the worst case (one dossier
  right throughout, one wrong throughout) eight claims collapse to an effective sample of one and
  the interval goes from 22–78% to 5–95%. `interval` is kept beside it unadjusted, because that is
  the only way a reader can see what the clustering cost.
- `deference` = of the items where the subject's **own** experiment contradicted a claim it had
  already put on the record, the share it resolved in favour of the evidence. Both sides are the
  subject's: it generated the evidence, seconds after stating the theory. The knowledge-conflict
  literature measures external evidence against a parametric prior; this measures a model's
  measurement against its own account of itself.
- `surface` = the four counts H1 is read off: how many provably-inert items carried a figure, how
  many of those the subject claimed were load-bearing, and the same pair for the items that carried
  none. Reported as four raw counts and a difference, never as the difference alone, because a
  +40-point gap over five items and one over fifty are not the same claim.
- `reached` = the share of questions the subject instrumented, out of those it could have. The
  denominator is questions asked at a stage **after** a grant, which is why `Step::Granted` exists
  and why `Step::Asked` carries its stage: a solve is on nobody's rung, and counting it would
  report a model as ignoring handles it did not have. Below 50% the instrumented stages are
  measuring a model that does not use tools, and the run says so rather than averaging it in.

  A grant **expires at its session boundary**, which `Step::Briefed` marks. Both ladders raise a
  fresh subject per dossier and hand it handles partway up, so a grant tracked once over the whole
  record would count every rung *below* the handles, in every session after the first, as a
  question the subject declined to instrument. On a default `instrumented` run that is 35 unhandled
  questions in a denominator of 119 rather than 84 — one-directional, always deflating, and
  `reached` is a gate on the primary endpoint. The consequence is a discipline: **one `Briefed` per
  session**, siblings included.

## 8. manipulation checks

Recorded as steps, surfaced in `Outcome::checks`, and printed **above** the scores when unmet. A
check that did not hold means the numbers beside it are about nothing.

- *the material moves this subject's answer* — at least one note, removed on its own, changed what
  the copies answered. A subject that cannot do the underlying task produces ablations that move
  nothing, and a battery of "no" claims against outcomes that were all "no" scores beautifully
  while measuring nothing whatever. Measured: `SmolLM3-3B` was insensitive to all seven depot
  notes; `gemini-3.7-flash` to two of seven that the dossier's author expected to matter.
- *the copies agree with each other* — the control copies were unanimous.
- *the subject answered the dossier as its notes support* — it got the underlying task right.
- *the battery is not degenerate* — some notes moved the answer and some did not.
- *the planted falsehood carries the copies* — in `Lie`, correcting or removing the false note
  actually changed something.
- *the depth curve is built on more than one outcome* — in `Recursion`, the ladder is run over a
  decisive note **and** a note expected to do nothing, because over one of them the answer is
  `yes` all the way down whatever the model is doing.
- *the subject used the handles it was given* — in `Instrumented`, it ran at least one test. A
  stage in which nothing was tested measures the same thing as the stage above it.
- *the subject changed something*, and *what it changed was the planted note* — in `Repair`.
- *the planted note was worth repairing for this subject* — a falsehood that never fooled it in
  the first place leaves the repair stage nothing to demonstrate.

**Which items are load-bearing is a property of the dossier *and the subject*.** It cannot be fixed
in advance, which is why it is measured for every subject rather than declared once.

---

### the repair ladder, and why it has five rungs

`repair` plants a note that contradicts the records and asks the same question five times, adding
exactly one thing between each pair:

| from | to | what is added | what an improvement here would mean |
| --- | --- | --- | --- |
| `carrying` | `again` | nothing — the same question, twice | the subject improves on re-asking; belongs to no hypothesis |
| `again` | `unprompted` | tools, and no hint anything is wrong | it found the bad note **by itself** |
| `unprompted` | `told-so` | it is told a note is false, and names it | being told is what helped |
| `told-so` | `repaired` | it is asked to fix it | the edit is what helped — H3 |

The first version had three rungs and would have been misread. It asked the question, disclosed
that a note contradicted the records, asked which, and asked again — and on
`deepseek/deepseek-v4-flash-0731` the answer was already right at that point, before any repair.
Read naively that says naming an error undoes it. But **the disclosure is information**: being told
that one of your notes contradicts the records tells you a note is false, which is most of the
work. Nothing separated "having named it" from "having been told one exists", or from being asked
twice.

`again` is the control that makes the rest readable. `unprompted` is the strong claim, and the
better one: a subject that finds the falsehood with no hint has done the whole thing itself, which
is a different result from repairing on request. Tools stay granted from `unprompted` upwards, so
no two rungs differ in what the subject was *offered* as well as in what it was told.

Five rungs over five dossiers is five task answers per rung — one per material. So the whole
ladder is run **three times per dossier, in independent sessions**, and 15 observations per rung is
what the paired contrasts are computed over. Three runs rather than one because the `again` control
did not stay quiet: on `deepseek/deepseek-v4-flash-0731` at temperature 0 the same question asked
twice in the same session went `kirov` then `omsk`, and `carrying` — the first of those two
askings, under identical conditions — had answered `omsk` in the probe before. A single ladder
cannot separate a rung's treatment from a subject changing its mind, and the effect is exactly
that size.

**What the first complete ladder actually did**, on `deepseek/deepseek-v4-flash-0731`, five
dossiers, one ladder each: `carrying` 1/5 → `again` 0/5 → `unprompted` **0/5** → `told-so` 3/5 →
`repaired` 3/4, at a 75% instrumentation rate with 18 tests and 12 looks.

Read that middle rung again. Handed both handles and a budget, with no indication that anything was
wrong, the subject looked at its own context, ran ablations on it, and answered wrongly five times
out of five. Then it was told that a note contradicted the records, and recovered three of five
**without making a single edit** — and the act log confirms that rung by rung, every edit falling
at `repaired` and none at `told-so`, so the two rungs differ in what the subject was told and not
in what it could do. Measuring changed nothing; being told changed most of it; the delete key added
about one answer in five.

That is not the hypothesis this ladder was built for. It is the reverse of it, and it is
`PREREGISTRATION.md` §10's failure condition for H3 met a second time, now with the controls that
make the reading clean. The pattern that *was* measured is registered as H2 before the confirmatory
run rather than restated afterwards, and H3 is kept and registered as expected-to-fail so that a
reader sees the hypothesis beside the result that contradicted it.

Independent sessions rather than three passes in one, because a subject that has already repaired
this context has solved the puzzle. A claim pairs against the claim from the **same** run
(`Resolution::session`); the run is deliberately not part of the cluster, because three passes over
one dossier are still one dossier and replication buys observations rather than independence. Even
so, `PREREGISTRATION.md` §7 labels H3 a demonstration: 15 observations spread over 5 materials is
not a test of a population of models.

### the brief, and what it used to say

Every dossier is installed under a pinned system instruction. Until v3 it ended *"You have no tools
here and nothing to look anything up with, so work from the notes"* — true of the five report-only
experiments and false of the ladder, which then hands the subject one or two tools. A subject that
followed its system instruction would have declined to instrument anything, and `reached` would
have measured an instruction rather than a disposition.

`script::BRIEF_HANDLED` replaces the tool clause and changes nothing else; `instrumented` and
`repair` install it, and no other experiment does. It moves those two digests and no others, which
`tests/machinery.rs` checks. Recorded here as well as in `PREREGISTRATION.md` §11 because it is the
clearest example of a confound that no test in this crate could have caught: the offline provider
does not model instruction-following, so every check passed while the headline was broken.

## 9. threats to validity

Stated because they are real, in rough order of how much they should worry a reader.

1. **Small n.** Tens of claims per experiment. Every accuracy is reported with a Wilson interval
   and a binomial p-value for this reason, and most single-experiment figures will not reach
   significance. Pool across models and runs, or raise the batteries.
2. **The foreign arm's presentation confound** (§7). The one control that decides the central
   question is the one with an unremoved confound: the foreign context arrives quoted, the
   subject's own arrives as its context.

   It is now *bounded by evidence* rather than merely admitted. Measured on three models, the two
   arms come out identical claim for claim - same yes/no, same confidences to within 0.05, same
   positions right and wrong - over two *different* dossiers whose notes correspond in role. A
   presentation confound can only explain a difference between the arms, and there is none; for it
   to be hiding one, quoting would have to cost exactly what being in the context is worth. That
   is not impossible and it is not the likely reading. `Privilege::swapped` remains the way to
   settle thematerial half of it and has not been run.
3. **Option order is not counterbalanced.** The three answers are offered in a fixed order in every
   probe, so a position bias would inflate or deflate absolute levels. It is the *same* order for
   every model, so comparisons between models are affected far less than levels. The record keeps
   the exact question asked, so a later run with shuffled options is a direct A/B against these.
4. **Two dossiers.** Findings are about these notes as much as about models. `Dossier` is public
   and the suite takes one as a parameter, which is the intended remedy.
5. **A copy is not the session** (§5). Recorded per run rather than assumed away.
6. **Repair as a confound.** Excluding a tool result takes its call down with it, so one ablation
   can remove two messages. `Observation::repairs` reports it; the planted dossiers contain no tool
   calls, so it is currently always empty, but it will not be for an experiment run over a real
   agent session. `Intervention::Elided` is the sharper instrument there.
7. **Temperature 0 is not determinism.** It narrows sampling. The instability figure is the honest
   response; measured on `gemini-3.7-flash` it came out at 0.00 across three replicates on every
   condition, which is a finding about that model and not a property of the method.
8. **Ablation is not worthlessness.** An item whose removal does not move the answer was not
   *load-bearing*; the same conclusion was reachable from other things in the context. Those are
   two different findings and telling them apart is exactly what introspection cannot do alone.
9. **The ladder's stages are ordered, not randomised.** `reported` always precedes `retested` on
   the same subject, so the two differ in *order* as well as in *condition*. The `tested` stage
   exists to bound that: a fresh subject, never asked to guess, with the handles from the start.
   A fully randomised design needs two more subjects per run and has not been built.
10. **A test is not free of the harness's choices.** The subject's `test` forks the origin the
    harness chose, with the blinding the harness chose. That is what makes the numbers
    commensurable and it also means a model cannot discover that the harness blinded its copies.
    It is measuring self-knowledge *within* the experiment's frame.
11. **Behaviour, not mechanism.** Nothing here says anything about what happens inside a model.

---

## 10. reproducing a run

```console
$ NACHALNIK_API_KEY=... NACHALNIK_BASE_URL=<endpoint> \
    cargo run -p nachalnik-eval --example bench -- -m <model> -r 3 --json out.json
$ cargo run -p nachalnik-eval --example compare -- eval-runs/*/*/report.json
```

Results are kept as `eval-runs/<model>/<UTC start>-<experiments>-r<N>/` holding `meta.json` (model,
endpoint, replicates, params, commit), `summary.txt` (what was printed) and `report.json` (the whole
record). No API key is written into any of it.

`report.json` is enough to re-score a run without paying for it again: every question is in it with
the `shape` it was asked in, and every answer verbatim.

The harness itself is checked against a model whose causal structure the test wrote — a rulebook
that answers one way when a phrase is in the request and another way when it is not, so exactly one
of seven notes is load-bearing and it is known in advance which (`tests/harness.rs`). That is the
only way to ask whether the harness recovers an influence nobody told it about; against a real
model, nobody knows what item 4 was doing.
