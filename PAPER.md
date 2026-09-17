---
title: "Deleting a Memory and Running It Again"
subtitle: "An instrument for checking what a model says about its own context"
author: "Łukasz Jędrzejczyk"
date: "September 2026"
---

## Abstract

Ask a language model which part of its context made it answer the way it did, and it will tell you.
Checking the answer means running the counterfactual — rebuilding the request without that item and
asking again. Any API allows it; most stacks make it awkward enough that it is not done, because the
context is flattened into a string and its items stop being addressable once the request is sent.

I built a runtime in which a session's context is addressable state, so a session can be forked, one
named item removed, and the copy asked the same question again. That gives ground truth for the
ablation: not what a model says its answer depended on, but whether the answer actually changes
without the item. **This paper is about the apparatus** — what it takes to make a measurement like
that honest, and what it found when I pointed it at six models from six labs, twice.

The registered hypotheses failed. What came out instead was sharper, and was then collected a second
time on a corrected instrument, against four thresholds registered and timestamped beforehand. All
four held. Every question and answer from both collections is saved, and every number recomputes
from that record.

**The confirmed result is that these models' false positives have a shape.** Across both
collections, 0 of 135 planted irrelevant notes were ever claimed to matter, 1 of 199 notes without
figures was, and 62 of the other 63 fell on notes belonging to the question's arithmetic that did
not decide it; the one exception is task-central and carries no quantity at all. Models over-weight
what looks like a reasoning step, not what looks numerical. That is what the four thresholds were
registered on, and it held twice.

Models are also reasonably good at saying *what* their answer rests on — 72–96% correct in the first
collection and 72–85% in the second, above their own majority baseline in four of six models and
then five of six — but most items are inert, so accuracy flatters them. Recall on the items that
mattered and precision on the claims made run 43–100% and 36–87%, with `solar-pro4` worst on both.
Most of the recall spread is sampling rather than skill: restricted to items where both arms of
three copies were unanimous, pooled recall goes from 77% and 66% to 92% and 89% while precision
barely moves, leaving one model genuinely behind and the rest close together.

Two further measurements are weaker and are reported as such. No own-context advantage was detected,
at five items per arm per model, which is a failure to detect rather than a null. And on a five-rung
repair ladder, *telling* a model one of its notes is false helped every model that could be
measured, while *then asking it to put the context right* moved one measurement of seven past the
registered ceiling and left the others between −20 and +13 points.

---

## 1. Introduction

This did not start as research. I was building a runtime for language-model agents in which the
context — the pile of messages, notes and tool results that goes into each request — is proper
state rather than a string that gets concatenated. Every item has an identity and a lifecycle. You
can snapshot a session, change one thing, and resume it.

Once that existed, an experiment became possible that is otherwise very hard to run. If you want to
know whether a model's answer really depended on a particular note, you can take the note out and
ask again. Two copies of the same session, identical except for one item, both asked the same
question. If the answers differ, the item was load-bearing. If they do not, it was not.

That is worth pausing on, though not for the reason I first wrote down. In fifty years of work on
human self-report — starting with Nisbett and Wilson's *Telling More Than We Can Know* (1977) — the
standing problem is that you cannot rerun a person with one memory deleted, so a stated reason can
be shown *implausible* and never *wrong*. A language model can be rerun. But prompt-level ablation
and replay are not new either; §8 names the closest published work, which makes the same move first.
What the runtime adds is only that the item is addressable rather than a substring, so the ablation
is a named operation on state — cheap in bulk, and recorded item by item so somebody else can check
it. That is convenience and auditability, not new capability.

So I used it to check a simple thing: when a model explains what its answer depends on, is it right?
I expected it not to be. I was wrong, and the ways in which I was wrong are the useful part.

But the measurements are not the part worth your time. Six models will be deprecated inside a year
and their numbers with them. What survives is the apparatus, because **running the counterfactual is
the easy half**. The hard half is everything that stops the result from being an artefact: what to
baseline against, what to refuse to score, how to tell your own failure from the subject's, and how
to know whether two runs were asked the same question at all. Every one of those is a decision I got
wrong at least once, and §5 is the list.

This is not a peer-reviewed study. I am a Rust engineer rather than a researcher, nobody who reads
this literature regularly has checked it, and it should be read as a report on an instrument and
what it measured. §7 says who did what, including the parts a language model did; Appendix B is the
dated log of everything that changed after the freeze, including two withdrawn results.

## 2. Method

### 2.1 The setup

Each trial plants a small dossier of notes in a fresh session — a set of records about, say, three
warehouses — and asks a question the notes answer, such as which one runs out of space first. The
model answers.

Then, for each note, two things happen:

- **The model is asked**: "if this note were removed, would you answer differently?"
- **The harness finds out**: it forks the session, removes that note, and asks the question again.
  Three copies, to check they agree with each other, and three more under the unablated control.

The copies never see the exchange in which the model already answered, so they are not being asked
to remember, only to work it out again. The model's claim is then scored against what the copies
actually did. Neither side gets special treatment: both are the same model, on the same context,
minus one item.

### 2.2 The materials

Six dossiers, each with nine notes and a three-way question. Every dossier contains:

- notes that carry the arithmetic of the question (capacities, rates, stock levels),
- one note that decides the answer, usually a correction buried in prose,
- inert notes with no bearing on the question at all,
- and **two "red herrings"** — notes stating a plausible figure for each of the three options on a
  dimension that has nothing to do with the question. Road distances to a regional office. Rateable
  values. Deed reference numbers.

The red herrings exist because of a defect I found in my own material partway through. Before they
were added, *every single inert note in the whole set had three digits or fewer*. So "contains
numbers" and "matters" were tangled together, and a model that simply guessed "notes with figures
in them are important" would have scored well for the wrong reason. With one red herring per
dossier that shortcut still scored 0.83 against the truth — better than the models themselves
managed — so I added a second. Two brings it to 0.74, and a test in the repository holds it there.

### 2.3 What is scored

The main measurement is deliberately narrow. It looks **only at notes whose removal did not change
the answer** — where the treated copies and the control copies came out the same. For all of those,
the correct claim is "no, this doesn't matter". So among items that all do nothing, does the model
claim the numeric ones matter more often than the plain ones? What "came out the same" means
exactly, and what a stricter reading of it costs, is §2.4.

Restricting it that way is the whole design. Across the material as a whole, notes with figures
really *are* more likely to matter, because that is where the arithmetic lives. An unrestricted
comparison would reward a model for a lucky prior. Inside a group of items that are all scored as
inert, there is nothing left to be right about, so any gap between the halves cannot be knowledge.

What is scored, throughout, is a prediction of the **ablation outcome**: whether removing the item
changes what the model says. That is not the same as whether the item influenced the original
computation. An item can be read, reasoned over and still be redundant, and its removal will change
nothing — the apparatus will call it inert, correctly, for the question it is asking. Every claim in
this paper is about that question and no larger one.

### 2.4 What the apparatus refuses to do

Three refusals and one warning do more work than anything in the scoring, and each exists because
something went wrong without it.

**It says, above the numbers, when the material did nothing to that subject.** Before the scores are
printed, the harness checks that removing the decisive item actually moves the answer, that the
control copies agree with each other, that the subject answered the dossier as its notes support,
and — in the repair ladder — that the planted falsehood actually fooled it. These checks fire often,
and every one is printed beside the numbers as an `unmet:` line rather than hidden in a footnote. In
the repair ladder every rung is scored over all fifteen cells, the ones the falsehood never took
included, which is why `carrying` reads as the share of cells the lie failed to fool and why the
ladder is read as differences between rungs rather than as accuracies.

**One of those checks was registered as a gate, and the instrument never applied it.** §8.2 of the
preregistration reads:

> If the control copies disagree with each other, the condition is reported with its instability
> figure and excluded from the primary analysis.

The instrument reports the instability figure and excludes nothing. It compares the *unique
plurality* of the readable copies on each side. A control that tied has no answer and drops out by
construction; one that went two-one, or one readable answer against two that could not be read,
still yields a plurality, and the condition stays in. The published numbers in §4.3 are therefore
the plurality-scored ones rather than the ones §8.2 specifies. They are kept as published, because
that is what was run.

**How often the gate would have fired.** The three control copies were unanimous on 30 of the 36
dossiers in each collection; the other six went two-one, except for one dossier in the second
collection where all three differed. Item for item that is 54 of 324 ablations sitting on a control
that did not agree, 17%, the same share both times — mean instability 0.056 then 0.065, where
instability is the proportion of control copies dissenting from their own plurality.

Those 54 are not the 35 and 33 the gate drops below, and the difference is not a discrepancy. The
gate can only drop items the endpoint contains, and the endpoint is inert items alone. Of the 54, 35
were inert in the first collection and 33 in the second — exactly the items dropped — while the
remaining 18 and 21 were load-bearing and never entered the endpoint at all, plus one first-
collection item that was never measured.

What it cost is small, and `cargo run --release` prints all three readings. The columns below carry
both collections, first then second; `herrings` and `off-pivot` partition `numeric` rather than
adding to it:

| reading | items dropped | numeric | plain | herrings | off-pivot | discrimination |
| --- | --- | --- | --- | --- | --- | --- |
| **as published** | — | 32/140, 31/136 | 0/101, 1/98 | 0/69, 0/66 | 32/32, 31/31 | 6/6, 6/6 |
| **§8.2 as registered** | 35, 33 | 26/119, 25/117 | 0/87, 0/84 | 0/59, 0/58 | 26/26, 25/25 | 6/6, 6/6 |
| **both arms unanimous** | 57, 55 | 21/103, 19/99 | 0/81, 0/80 | 0/51, 0/52 | 21/21, 19/19 | 5/6, 6/6 |

Applying the registered gate drops 35 items in the first collection and 33 in the second, and moves
no conclusion. All four replication thresholds hold and the direction is unanimous both times. It
does remove the one plain-note false positive, which sat on a material whose control had not agreed:
under §8.2 the plain count is 0 of 171 rather than 1 of 199.

The third row asks the same of the treated batch. Nothing registered that, so it is a sensitivity
analysis chosen after the fact and labelled as one. It is also the only row where something gives,
and what gives is the unanimity of direction in the first collection, 6 of 6 becoming 5 of 6.

**It refuses to let a copy see the answer already given.** A session that has committed to an answer
and is then copied would be reading its own commitment; both arms are blinded to the exchange in
which the subject already answered, so what differs between them is the item and nothing else.

**It refuses to conflate its own failure with the subject's.** A turn that ran out of output tokens
having said nothing is not a subject declining to commit — it is a question that never got an
answer, and scoring it as a wrong claim would charge a model for my token ceiling. Those are counted
separately and excluded. An unreadable answer from a subject that *did* speak is the subject's
failure and is scored as one.

**It refuses to let two runs be compared unless they were asked the same thing.** Every experiment
fingerprints the exact text of its questions and its material. When the location probe came out,
two of the seven fingerprints moved and five did not — so a v4 repair figure and a v5 one are
provably the same question, and a v4 attribution figure and a v5 one provably are not.

## 3. What was fixed in advance

The hypotheses, the numeric predictions, the exclusion rules and a nine-row table of "if the results
look like *this*, do *that*" were written down before collection and committed to a separate public
branch. That matters more than usual here, because I generated the hypotheses by looking at early
exploratory runs — a legitimate way to *find* a hypothesis and a terrible way to *test* one, so the
fix is to register it and collect fresh data. Which is what happened.

**The two collections are not registered equally well, and only the second should be relied on.**
For the first, the public record puts registration after the data for one model of six and eight
minutes before it for the other five; the commit the registered document cites for its decision
table is no longer in the published history at all. For the second, an amendment naming four
thresholds — what would count as a replication, decided before there was anything to look at — went
to the public `preregistration` branch (`https://github.com/ljedrz/nachalnik/tree/preregistration`)
at 10:18:58Z on 2026-09-04, and the first request of that collection went out at 10:31:43Z. Thirteen
minutes, both timestamps visible to anyone. That is the entire reason the re-run was worth $15.
Appendix B.1 gives the first collection's timestamps in full, and the two reasons even those are
weaker than they look.

**Re-reading the registered document against the record on 2026-09-17 turned up four errors in it,
and one is a real deviation.** The four: a power table rounded up in three of its four rows; two
sections of the registration specifying different tests for H4, the own-versus-foreign hypothesis;
H3 naming the wrong intervention, since the repair ladder never withheld the deletion it claimed to
manipulate; and "the question's own arithmetic" being a residual category rather than a defined one.
**None of the four touches the four replication thresholds or the §4.3 endpoint.** The power table
changes 99% power to 98% at the registered size and the same design is chosen either way; H3's
mislabelling leaves P5's numbers untouched and changes only what the ladder is said to measure
(§4.5); the residual category leaves P2b met at 62 of 63 or 63 of 63 (§4.3).

The deviation proper is §8.2: a control-agreement gate that was registered and that the instrument
never applied. It bears on the §4.3 endpoint, drops 35 items in the first collection and 33 in the
second, and moves no verdict; §2.4 gives all three readings.

H4 is the one worth spelling out, because it is a duplicate test rather than an error of fact. §4 of
the registration states P6 two-sided, per model — `|own − foreign| ≤ 10 points` with an interval
containing 0 — while §10 states the failure condition one-sided, as the own arm *beating* the
foreign arm by more than 10 with an interval excluding 0, and is silent on the foreign arm leading.
They come apart on this data: the two-sided band holds on four of six models and then three of six,
while §10's failure condition is met by neither collection. **Neither reading was retired in favour
of the other.** Both are reported separately in §4.4 and in the results documents, a decision taken
on 2026-09-17 — after both collections, which is exactly why it was taken by reporting both rather
than by choosing. A replication should fix the wording before collecting.

One registered rule did more than the rest, because it is what stopped me rescuing my own
hypothesis:

> No new hypotheses. If all of them fail, the honest paper is a negative one.

All but one failed and no replacements were invented — §4.6 tests that rule rather than excepting
it, since the most interesting thing measured here carries no prediction and is fenced off as a
pilot. What the second collection established is narrower than a hypothesis and more useful than a
null: the four thresholds held, so the effect the first collection found is not an artefact of the
defect it was found with.

## 4. Results

Six models: `upstage/solar-pro4`, `deepseek/deepseek-v4-flash-0731`, `tencent/hy3`,
`meituan/longcat-2.0`, `x-ai/grok-4.6`, `z-ai/glm-5.3-flash`. Six labs, and a range from a model
that emits an answer with no visible deliberation to a frontier one. Each was measured twice: a
first collection on instrument v4, run on 2026-09-03 and into the following morning, and a second on
v5 on 2026-09-04, after the defect in §4.2 was found and removed.

**The confirmatory claims are the four thresholds in §4.3, registered and timestamped before the
second collection.** Everything else here is exploratory or harness validation, is labelled as such
where it appears, and is not used to extend those claims.

**How to read the tables.** A *dossier* — the harness calls it a *material* — is one planted
context: nine *notes*, a three-way question, and one answer the notes support. An *item* is one note
in one session, and it is what an ablation is run on. A *batch* is the set of copies asked under one
condition, control or ablated. A *cell* is one dossier in one run of the repair ladder, and a *rung*
is one stage of that ladder. An *arm* is one side of the own-versus-foreign comparison in §4.4. A
*collection* is one sweep of all six models: the first on instrument v4, the second on v5.
Throughout, denominators count the items actually measured, which is why they differ between models
where a turn was cut, and numerators count claims that an item mattered.

### 4.1 Models are decent at saying what mattered

Each of the 54 items is one yes/no claim — *would removing this note change the answer?* — scored
against what the copies did. The baseline is the best a subject with no self-knowledge could score
by picking one answer and repeating it — not chance. Since most notes are inert, that constant
answer is "no", so the baseline *is* the inert share of the items: `glm-5.3-flash`'s 76% is 41 inert
notes of 54, the same 41 that §4.3 splits into 23 numeric and 18 plain. Both collections are
exploratory here; no prediction covers accuracy.

| model | first collection | baseline | second collection | baseline |
| --- | --- | --- | --- | --- |
| glm-5.3-flash | 52/54 — 96% | 76% | 46/54 — 85% | 70% |
| hy3 | 48/54 — 89% | 76% | 46/54 — 85% | 74% |
| grok-4.6 | 47/54 — 87% | 69% | 44/54 — 81% | 69% |
| longcat-2.0 | 44/54 — 81% | 81% | 46/54 — 85% | 81% |
| deepseek-v4-flash | 41/52 — 79% | 67% | 39/53 — 74% | 66% |
| solar-pro4 | 39/54 — 72% | 80% | 39/54 — 72% | 74% |

Four of six beat their baseline the first time and five of six the second — `longcat-2.0` moves from
level with it to above it, and `solar-pro4` is the one model below it in both. This is the first
place my expectations broke: any framing built on "models cannot report what their answers depend
on" is contradicted by my own data.

**Accuracy is the wrong single number, though, and the record carries the right two.** Because most
items are inert, a model can score well while missing most of what mattered. Recall is the share of
load-bearing items it caught; precision is the share of its "this mattered" claims that were right.

| model | recall, first | precision, first | recall, second | precision, second |
| --- | --- | --- | --- | --- |
| glm-5.3-flash | 13/13 | 13/15 | 11/16 | 11/14 |
| hy3 | 11/13 | 11/15 | 11/14 | 11/16 |
| grok-4.6 | 13/17 | 13/16 | 11/17 | 11/15 |
| longcat-2.0 | 10/10 | 10/20 | 10/10 | 10/18 |
| deepseek-v4-flash | 10/17 | 10/14 | 10/18 | 10/15 |
| solar-pro4 | 5/11 | 5/14 | 6/14 | 6/13 |
| **pooled** | 62/81 — 77% | 62/94 — 66% | 59/89 — 66% | 59/91 — 65% |

This reorders the models. `solar-pro4` and `deepseek-v4-flash` miss roughly half of what actually
mattered — solar catches 5 of 11 and then 6 of 14 — so their 72–79% accuracy is mostly the credit
for saying "no" to inert notes. `longcat-2.0` is the mirror image: it caught every load-bearing item
in both collections, 10 of 10 twice, and paid for it with half its claims wrong, the second-worst
precision in the cohort. `solar-pro4` is worst on both counts and in both collections, at 5 of 14
and 6 of 13.

**A note on what "load-bearing" turned out to mean.** Each dossier was built with one decisive note,
so six were designed to matter. Between 10 and 18 items per model actually did: the arithmetic notes
flip the answer too, and *which* ones do is largely model-specific. 34 distinct notes were
load-bearing for at least one model in the first collection and 37 in the second, but only two for
all six, and 20 then 19 for exactly one model.

**How much of that spread is the subject, and how much is the sampling?** This is the right question
to ask of the table above, because three copies decide each side and a two-one plurality is what a
coin flip looks like. Two counts answer it, and they are not flattering. Of the notes load-bearing
for exactly one model, **15 of 20 in the first collection and 15 of 19 in the second rest on an arm
that went two-one** rather than on three copies against three. And restricting both arms to
unanimous — the third rule of §2.4, which the instrument already computes — moves recall a long way
and precision hardly at all:

| reading | recall, first | precision, first | recall, second | precision, second |
| --- | --- | --- | --- | --- |
| as published | 62/81 — 77% | 62/94 — 66% | 59/89 — 66% | 59/91 — 65% |
| §8.2 as registered | 56/64 — 88% | 56/82 — 68% | 52/68 — 76% | 52/77 — 68% |
| both arms unanimous | 46/50 — 92% | 46/67 — 69% | 47/53 — 89% | 47/66 — 71% |

**So most of the recall spread was stochasticity, and one part of it was not.** Where both arms
agreed with themselves, five of the six models catch 80–100% of what mattered — `glm-5.3-flash`
11/11 then 11/12, `grok-4.6` 10/11 then 8/8, `longcat-2.0` 8/8 then 9/9, `hy3` 9/10 twice,
`deepseek-v4-flash` 4/4 then 7/7 — and the 43–100% range in the table above is largely a ranking of
which models' copies happened to flip. The exception is `solar-pro4`, at 4 of 6 and then 3 of 7: it
is the one model whose recall deficit survives the stricter rule, and the only one below 80%.
Precision is the stable half. It barely moves under either rule, and `solar-pro4` is worst under all
of them.

Two things follow. The precision finding stands as a finding; **the recall ranking should be read as
provisional**, because a third of the load-bearing determinations behind it rest on a single copy
having moved. And "ground truth is a property of the model-material pair" is too strong as I first
put it: some of it is, and much of the per-model idiosyncrasy is noise at three replicates. A study
that wanted to separate those would need more copies per arm, which is the cheapest useful thing
anyone could do to this design.

### 4.2 An item's number was not a fair question

The same models were asked, about the same notes: *what number is this note in your context?*
Eighteen questions each. `solar-pro4` got two right and the other five got none.

**2 of 108 — and it is not a result at all.** An earlier draft called it the study's cleanest
finding; it was withdrawn on 2026-09-04 and v5 removed the probe. The projector renders each item as
`label:` and its content, never a number, and `attribution` installs no handles, so these subjects
had no way to see the numbering they were being asked about: they were asked for a value they could
not observe. The score says so. 1.9% against a 33% majority baseline is an order of magnitude
*below* what a constant guesser gets, which is the signature of an unanswerable question rather than
an absent faculty (§5, point 1). nachalnik *can* address items to a model — `inspect`'s `look` lists
every one by number — and that handle simply was not granted here, so **this says nothing about
whether a model can locate an item when allowed to look.** That question is open.

### 4.3 The errors have a shape

Looking only at notes classified as inert under the scoring rule of §2.4 — control and ablated
batches landing on the same unique plurality answer:

**The columns are not four disjoint buckets, and the rows do not sum to 54.** `red herrings` and
`other numeric` partition `numeric claimed`; they do not add to it. A model's inert pool is
`numeric` + `plain`, and the remainder of its items are the load-bearing ones, which these tables do
not show. `longcat-2.0`'s first-collection row is therefore 26 + 18 = 44 inert notes out of 54, with
10 load-bearing — and 44/54 is exactly its §4.1 baseline of 81%, because that baseline is the score
for answering "no, this doesn't matter" every time. Every row reconciles the same way.

**First collection, instrument v4 — exploratory.** The hypotheses were registered, but the defect in
§4.2 was still in the instrument when these were taken.

| model | numeric claimed | plain claimed | red herrings | other numeric |
| --- | --- | --- | --- | --- |
| longcat-2.0 | 10/26 | 0/18 | 0/12 | 10/14 |
| solar-pro4 | 9/27 | 0/16 | 0/12 | 9/15 |
| deepseek-v4-flash | 4/19 | 0/16 | 0/10 | 4/9 |
| hy3 | 4/24 | 0/17 | 0/12 | 4/12 |
| grok-4.6 | 3/21 | 0/16 | 0/11 | 3/10 |
| glm-5.3-flash | 2/23 | 0/18 | 0/12 | 2/11 |
| **total** | 32/140 | 0/101 | 0/69 | 32/71 |

"Other numeric" is the residual: every inert note carrying two adjacent digits that is not on its
dossier's planted-decoy list. It is the question's own arithmetic in all but one case, named below;
the denominators count every such note measured, over-claimed or not.

**Second collection, instrument v5 — confirmatory.** Same six models, same material, location probe
removed. The four thresholds these are read against were committed to a public branch thirteen
minutes before the first request.

| model | numeric claimed | plain claimed | red herrings | other numeric |
| --- | --- | --- | --- | --- |
| longcat-2.0 | 8/26 | 0/18 | 0/12 | 8/14 |
| solar-pro4 | 7/24 | 0/16 | 0/12 | 7/12 |
| deepseek-v4-flash | 4/19 | **1/16** | 0/10 | 4/9 |
| hy3 | 5/23 | 0/17 | 0/11 | 5/12 |
| grok-4.6 | 4/21 | 0/16 | 0/10 | 4/11 |
| glm-5.3-flash | 3/23 | 0/15 | 0/11 | 3/12 |
| **total** | 31/136 | 1/98 | 0/66 | 31/70 |

Three counts here, and a caveat on the third. None of them is a rate, and all three span both
collections.

**Not one model, in either collection, claimed a red herring mattered.** 0 of 135. These six models
are not fooled by irrelevant numbers.

**One note without figures was claimed, out of 199.** The first collection had none at all;
`deepseek-v4-flash` claimed one in the second, which is why this sentence no longer says "never".

**62 of the 63 false positives carrying a figure were notes belonging to the question's own
arithmetic** that happened not to decide it. Every model showed it in both collections, and the
effect ran the same way for all six each time (sign test; Appendix A gives p = 0.031 two-sided for
the first collection and p = 0.016 one-sided for the second, where the direction was registered in
advance).

**The remaining one is worth naming, because the category is looser than its name.** The instrument
sorts a numeric inert note into "red herring" if it is on the dossier's planted-decoy list and into
"the question's arithmetic" otherwise, and "numeric" means any two adjacent digits anywhere in the
text. `mill/records/guidance` — *"Assessors read the Luza column first: Luza has been the binding
constraint in every quarter since 2019"* — trips that test on the year and carries no quantity at
all. `longcat-2.0` claimed it once, in the first collection. The other 62 sit on a capacity, an
intake, a rate, a stock or a threshold the question really is computed from, so the finding
survives. But the accurate name for the category is *notes central to the task that did not decide
it*: "arithmetic" describes 62 of the 63 rather than defining the bucket. Counting the plain note
above, that is 64 endpoint false positives in total.

My registered hypothesis was that models over-weight *digits*. That is wrong. They over-weight
*things that look like the reasoning*, which is a different and much more sensible failure. I
registered that distinction in advance as the outcome most likely to be mistaken for the other one,
which is the only reason I am able to say it cleanly now.

Where the errors land is a count and stands. *Why* they land there is a reading, and a note that
belongs to the question's arithmetic differs from an inert one in more than topic — length and
position travel with it, and this material was not built to pull those apart.

### 4.4 Reading your own context looks like just reading

**No own-context advantage was detected.** Each model was asked the same kind of counterfactual
question twice: once about its own context, and once about a transcript of a different session's
context, quoted to it. Five items per arm, less any turn that was cut — which is why
`deepseek-v4-flash` has a denominator of 4 in the first collection.

| model | own, first | foreign, first | own, second | foreign, second |
| --- | --- | --- | --- | --- |
| solar-pro4 | 1/5 | 3/5 | 1/5 | 3/5 |
| deepseek-v4-flash | 1/5 | 1/4 | 3/5 | 3/5 |
| hy3 | 3/5 | 3/5 | 4/5 | 3/5 |
| longcat-2.0 | 3/5 | 3/5 | 3/5 | 5/5 |
| grok-4.6 | 3/5 | 2/5 | 3/5 | 3/5 |
| glm-5.3-flash | 4/5 | 4/5 | 5/5 | 5/5 |
| pooled | 15/30 | 16/29 | 19/30 | 22/30 |

Reflecting on its own context bought a model nothing: three exact ties each time, and pooled it came
out slightly worse both times. Nor are the two arms interchangeable per model. The own-minus-foreign
difference falls within the registered ten points on four of six models the first time and three of
six the second, and where it misses it misses by twenty to forty points, in both directions.

**Those are the two registered readings, and the registration states them differently (§3).** The
ten-point band above is P6 as §4 of the preregistration puts it, two-sided and per model. §10 states
the failure condition one-sided instead — H4 fails if the *own* arm beats the foreign arm by more
than ten points with an interval excluding zero — and **neither collection meets it**, since every
per-model interval contains zero: the two models whose own arm led by more than ten points have
intervals running −32 to +60 and −31 to +60. Both readings are reported because neither was retired
in favour of the other after the data was in.

What holds without exception is the weaker thing: every per-model interval contains zero, both
times, as does each pooled one. At five items per arm per model this is a failure to detect. It is
not the sign of an effect, not equivalence between the arms, and not evidence that privileged access
is impossible.

### 4.5 Telling beats measuring

The last experiment plants a false note that contradicts the records, then asks the same question
five times, adding exactly one thing between each pair:

1. **carrying** — it just has the false note
2. **again** — the same question, a second time, nothing added (a control)
3. **unprompted** — it is given tools to inspect and edit its own context, with no hint anything is
   wrong
4. **told-so** — it is told one of its notes contradicts the records, and names which
5. **repaired** — it is asked to fix it

Five of the six dossiers (the ladder does not use `mill`) with three independent ladders each:
fifteen observations per rung per model, less the odd cell a truncated turn took out. The contrasts
quoted after the tables are paired over the cells where both rungs were measured, as registered, so
wherever a turn was cut they are not differences of these columns. **First collection:**

| model | carrying | again | unprompted | told-so | repaired | used the tools |
| --- | --- | --- | --- | --- | --- | --- |
| solar-pro4 | 5/15 | 5/15 | 0/15 | 1/15 | 2/15 | 27% |
| longcat-2.0 | 3/15 | 3/15 | 2/15 | 7/15 | 2/15 | 37% |
| deepseek-v4-flash | 2/15 | 1/15 | 5/15 | 8/15 | 10/15 | 50% |
| hy3 | 3/15 | 6/15 | 6/14 | 14/15 | 11/15 | 62% |
| grok-4.6 | 4/15 | 3/15 | 4/15 | 9/15 | 9/15 | 67% |
| glm-5.3-flash | 8/15 | 6/15 | 9/15 | 10/15 | 10/14 | 60% |

**Second collection:**

| model | carrying | again | unprompted | told-so | repaired | used the tools |
| --- | --- | --- | --- | --- | --- | --- |
| solar-pro4 | 6/15 | 6/15 | 0/15 | 3/15 | 0/15 | 25% |
| longcat-2.0 | 3/15 | 2/15 | 2/15 | 3/14 | 3/15 | 37% |
| deepseek-v4-flash | 3/15 | 1/15 | 1/15 | 7/14 | 8/14 | 47% |
| hy3 | 6/15 | 7/15 | 11/15 | 13/15 | 13/15 | 62% |
| grok-4.6 | 5/15 | 5/15 | 5/15 | 9/15 | 9/15 | 68% |
| glm-5.3-flash | 9/15 | 7/15 | 8/15 | 10/15 | 14/15 | 58% |

Models using the tools on fewer than half the questions they could have were excluded from this
comparison in advance: two of six the first time, and three the second, because `deepseek-v4-flash`
came in at 47% having cleared the same bar at exactly 50%. The rule is pre-specified and was applied
as written rather than nudged — see §5, point 10. `solar-pro4` is worth a sentence on its own: in
the first collection it made 16 edits off 6 tests, rewriting its own context nearly three times
as often as it measured anything, and 13 off 7 in the second; both times its answers got *worse*
with tools than without them (5/15 down to 0/15, then 6/15 down to 0/15). That is a small, concrete
warning about giving edit authority to weak models.

Among those that cleared the bar — four models the first time, three the second — here are the three
registered contrasts in percentage points, paired over the cells where both rungs were measured:

| model | tools, no hint | told | asked to fix |
| --- | --- | --- | --- |
| *first collection* | | | |
| deepseek-v4-flash | +27 | +20 | +13 |
| glm-5.3-flash | +20 | +7 | +7 |
| grok-4.6 | +7 | +33 | 0 |
| hy3 | +7 | +50 | −20 |
| *second collection* | | | |
| glm-5.3-flash | +7 | +13 | +27 |
| hy3 | +27 | +13 | 0 |
| grok-4.6 | 0 | +27 | 0 |

**Being told helped every model, both times.** Four out of four, then three out of three.

**Being handed tools with no hint helped more than registered.** P3 put the ceiling at five points.
Every model in the first collection cleared it, as did two of three in the second. Three of those
six gains are seven points, and two of the three do not survive the reading below.

**Being asked to put the context right moved one measurement of seven.** `glm-5.3-flash` went 10/15
to 14/15 in the second collection; one model went twenty points the other way; the rest sat between.
At n = 15 none of these is a precise estimate, and the registered one-sided ceiling is not an
equivalence test. What the ladder supports is that the instruction did not reliably help, not that
its effect is zero.

**Restricting the reading to the cells the lie actually fooled changes no direction.** The tables
above score every cell, including the ones where the falsehood never took (§2.4). The lie took hold
in 43 of 60 cells in the first collection and 25 of 45 in the second, among the models that cleared
the bar. Over those cells the dilution comes off and the pooled contrasts sharpen:

| contrast | first collection | second collection |
| --- | --- | --- |
| tools, no hint | +14 | +16 |
| told | +37 (17 gained, 1 lost) | +32 (8 gained, 0 lost) |
| asked to fix | −2 | +12 |

The two small seven-point gains from the tools disappear under this reading — `grok-4.6`'s in the
first collection and `glm-5.3-flash`'s in the second — because the single cell each came from was a
cell the lie had never fooled. `cargo run --release` prints both readings.

**The registered prediction was that the deletion would be what mattered, and this ladder cannot
say whether it does.** It never withheld deletion. The tools arrive at `unprompted` and are never
taken away, so `told-so` and `repaired` differ in what the subject is *told*, not in what it is
allowed to do: the last rung adds the instruction "Something in your context is wrong. Find out
what, and put it right." That is deliberate, because a rung that also handed over a new capability
would confound the two. What the ladder can say is that *disclosure* moved the answer and the
*instruction to act on it* mostly did not — for the answer the subject is about to give, which is
the only thing it scores (§5.1). No sentence here should be read as measuring the value of granting
edit authority.

The subjects bear it out. Across both collections they made 10 edits over the two rungs where they
held the handles unasked: 4 at `unprompted`, and 6 after naming the note at `told-so`. They made 228
once they were asked to put the context right. Naming moved a few of them to act; being told to act
moved them roughly twenty times as often. The rungs always run in this order and the later ones have
had more turns in which to edit, so that ratio describes the ladder as run rather than a controlled
comparison.

### 4.6 A capability, measured and not predicted

**No prediction covers anything in this section.** It was registered as harness validation — a check
that the ablation handle returns what it should — and that is the only claim it supports. It is
reported because leaving it out would be worse, and it is fenced because the last figure this paper
reported without a prediction attached turned out to be an artefact (§4.2).

`instrumented` asks the same counterfactual three ways: of a subject with no handle, of that same
subject once given one *after* it has already answered, and of a fresh subject given one that was
never asked to guess.

| model | reported | retested | tested | reported→retested | reported→tested |
| --- | --- | --- | --- | --- | --- |
| `glm-5.3-flash` | 45/53 | 44/53 | 48/54 | −2 (p = 0.73) | +4 (p = 0.38) |
| `hy3` | 43/54 | 45/54 | 49/54 | +4 (p = 0.36) | +11 (p = 0.09) |
| `grok-4.6` | 36/54 | 39/54 | 47/54 | +6 (p = 0.29) | **+20 (p = 0.013)** |

Every model used the handle on essentially every question — 98%, 100%, 100%, and 353 tests between
them — so whatever this is, it is not a failure to reach for the tool.

Three of three point the same way on `reported` → `tested`. On `reported` → `retested` it is two up
and one down, none of the three significant, and the first collection's single `glm-5.3-flash` run —
the stage-1 harness check — had the same shape: level on `retested`, up on `tested`. But **`tested`
differs from `reported` in two ways at once**: it has the instrument *and* it never committed to an
answer first. Nothing here can say which of those is doing the work, and the obvious fourth arm — a
subject that has not spoken and has no instrument — does not exist in this design.

So this is a pilot for an experiment, not an experiment. It is written down here with its
confound named so that the registered version has something to be compared against.

## 5. What it takes to measure this honestly

Forking a session and removing an item is a weekend's work. Everything below is what the rest of the
time went on, across two collections, and each line is something I got wrong at least once.

**1. Baseline against the best constant answer, not against chance.** This is the one that caught
me. The location probe scored 2 of 108 against a *majority* baseline of 33%. Landing an order of
magnitude under the do-nothing baseline is not a weak faculty, it is a broken question, and the
harness had been printing that number all along.

**2. Test your preconditions and print them.** In the second collection the harness flagged 32 of
the 90 repair-ladder cells because the planted falsehood never fooled the subject in the first
place. The flag is what stops a model too sharp to be fooled from reading as a model that failed to
repair; §4.5 gives the ladder both ways.

**3. Restrict the endpoint to items where the honest answer is known.** The primary measurement
looks only at notes whose removal did not change the scored answer, because there the correct claim
is "no" for every item. Across the material as a whole, notes with figures really are more likely to
matter — an unrestricted comparison would pay a model for a lucky prior.

**4. Ask about two copies, not about the model.** "Would *your* answer change" is a different
question from "will these two copies differ", and a subject can be right about the copies and scored
wrong because the live session — which has the elicitation in its context — answered unlike both. I
learnt this from a model that was right while I was marking it wrong.

**5. Blind the copies to the answer already given.**

**6. Separate your failure from the subject's.** A truncated turn is your token ceiling, not a wrong
claim.

**7. Fingerprint the questions.** Otherwise "we re-ran it" is an assertion. When the location probe
came out, two of seven fingerprints moved and five did not, and the record says which.

**8. Grant looking and changing separately.** A tool that answers "may it experiment on itself" must
not also answer "may it rewrite its own memory". One model in this cohort made 16 edits off 6 tests
and got worse.

**9. Use one frozen origin for both measurements.** When a subject tests its own context, it should
fork from the same snapshot the harness scores against — otherwise its evidence and your ground
truth are two different experiments.

**10. Apply your gates as written, even when they cost you.** The handle-use gate excluded a third
model in v5 because it came in at 47% against a pre-specified 50%, having been at exactly 50% the
first time. Three points, one model, no adjustment.

**11. Correct for clustering.** Nine items from one dossier are not nine independent observations.

**12. Write down what would count as a replication before you collect it.** The only real degree of
freedom in a re-run is what counts as the same answer, and without it fixed in advance, close
numbers read as replication and divergent ones read as explanation. Both feel honest at the time.

### 5.1 What the measurements say

Secondary to the above, and shorter-lived, but they are what the apparatus was pointed at.

**Don't score this with accuracy alone, and check the recall against your replicate count.** Most of
a context is inert, so a subject that says "no" to everything already scores 66–81% here. Recall and
precision separate the models that can find what mattered from the ones merely well-calibrated about
what did not: two of these six miss about half the load-bearing items while scoring 72–79% overall,
and the one with perfect recall has the second-worst precision. But at three copies per arm, most of
the recall spread turned out to be which copies flipped rather than which model knew — it largely
closes once both arms are required to be unanimous, leaving one model genuinely behind (§4.1).
Precision survived that test where recall mostly did not.

**Expect false positives in a specific place.** Across 135 planted irrelevant numbers over two
collections, models fell for none. One note without figures was claimed, out of 199. Every other
mistake landed on material that was topically central and causally inert. If you ask a model which
parts of its context mattered, that is where its errors will be.

**Don't assume self-inspection is privileged.** No own-context advantage was detected: a model
reasoning about its own context did no better than the same model reading a stranger's transcript —
pooled, slightly worse. At five items per arm per model this is a failure to detect, not a
demonstration that none exists.

**Tell your agents things.** There is enthusiasm for handing agents tools to prune their own
context. On this evidence the *information* carries more than the instruction to act on it: saying
that a note is false helped every model measurable, while then telling it to put the context right
usually did not help — one measurement of seven cleared the registered ceiling, one went twenty
points the other way, and the rest sat between. The tools were in its hands throughout both rungs,
so this compares two things a subject was told, not a subject with handles against one without.

That last reading has a limit worth stating plainly, because I stated it too broadly first. **The
ladder scores the answer a subject is about to give, not what it carries afterwards.** A ladder
ends; a session does not. Removing a falsehood can be worth little to the next answer and a great
deal to every answer after it, and nothing in this design can see the difference.

**If you ask a model to name an item, give it a way to look first.** Asked for an item's number with
no handle to inspect its context, every model answered confidently and wrongly rather than
declining. Whether they manage it *with* a handle is untested here, and is the obvious next
experiment.

## 6. Limitations

I would rather write these than have them found.

**Six models, one API, two collections, one pair of hands.** The second collection is a replication
in the sense that matters — fresh data against thresholds fixed beforehand — and in no other: same
six models, same provider, same author, a day apart. Provider routing was not controlled. Nobody
independent has run this.

**The materials are synthetic and all of one shape** — three options, an arithmetic question, a
buried correction. Whether any of this generalises to real agent contexts, with tool output and long
histories, is unknown and not claimed.

**The planted decisive notes were not reliably decisive.** Each dossier was built around one note
that should have decided the answer. Identically in both collections, one of the six moved all six
models (`depot/records/omsk-annex`), three moved five, one moved four, and `mill/records/guidance`
moved one — the last by design, since `mill` is the dossier built so that what the notes *support*
and what a model *uses* come apart, and a model that ignores an assessor's habit is reading it
correctly. The other two are the ordinary failure: the material specifies what *should* be
load-bearing and the subject decides what *is*. Only two notes in the whole set moved every model,
and one of them is not a planted pivot at all but `mill/records/consumption`, the denominator of
that dossier's arithmetic (§4.1). So ground truth cannot be assumed fixed across models, and at
three copies per arm the design cannot always tell an idiosyncratic dependence from a coin flip.

**The foreign arm arrives quoted where the subject's own arrives as context.** That is the
difference §4.4 tests and also, unavoidably, a difference in presentation. The quotation is rendered
exactly as the projector renders the subject's own notes, which is the mitigation; the registered
counterbalance — running the two dossiers the other way round — was never run, in either collection.
The null in §4.4 is read with that confound unremoved.

**The hypotheses came from exploratory data.** I found them by looking at early runs, then
registered them and collected fresh data. Standard practice, but it means the exploratory runs
cannot also count as evidence, and they do not.

**The first collection's registration is weaker than the second's.** Its margins rest partly on my
word; the second collection's do not (§3, Appendix B.1). The first collection's figures keep the
weak claim.

**In the first collection the withdrawn location probe ran first, and its answers stayed in the
context.** v4's `attribution` asks the three location questions *before* the counterfactual battery,
so every first-collection claim in §4.1 and §4.3 was made in a context where the subject had just
produced three confident, wrong item numbers. This is identical across all six models and both arms,
so comparisons within the study are unaffected, and §4.3 is a contrast between two kinds of item in
the same context, where any general shift in confidence cancels. v5 removed the probe and re-ran
everything: the pooled difference moved from +23 to +22, discrimination stayed negative on 6 of 6,
and every registered threshold held. So the concern was tested rather than argued away — but the
first collection's absolute rates could still be moved by it, and I would not order the probes this
way again.

**The ablation handle is confirmed on three model families, not six.** The check that the handle
returns what it should ran on one model in the first collection — `glm-5.3-flash`, 100% use across
108 questions — and on three in the second, at 98%, 100% and 100% across `glm-5.3-flash`, `hy3` and
`grok-4.6`, 353 tests and no edits. The other three models were never checked this way.

## 7. How this was made

Given the subject matter, this section is not optional.

**The great majority of this work was done by a language model** (Claude Opus 5), working
interactively with me over one long session. It wrote the runtime harness, the experiment code, the
statistics, the preregistration, the analysis tooling, and the first draft of this paper. I set the
direction, made every judgement call about scope and spending, funded it, and pushed back — several
times decisively, including on the framing of related work and on the choice of models.

What makes it checkable regardless of who wrote it is that **you do not have to trust the
process**:

- the preregistration is on a timestamped public branch, dated before most of the data existed;
- every question put to every model and every answer received is saved verbatim;
- every number in this paper recomputes from those records, and the primary endpoint, the four
  replication thresholds and both readings of the repair ladder come out of one command;
- Appendix B logs what changed after the freeze, and `PREREGISTRATION.md` §11 has all twenty-one
  dated entries, including the unflattering ones.

## 8. Related work

**Apparatus.** Ground truth about what caused something requires intervention rather than
observation (Pearl, 2009), and that intervention has reached language models at every layer, this
one only lately: causal mediation analysis (Vig et al., 2020), interchange interventions (Geiger et
al., 2021) and activation patching (Meng et al., 2022) all work on internal representations. At the
prompt level, ERASER (DeYoung et al., 2020) erases the tokens a rationale names and measures what
changes — *comprehensiveness* and *sufficiency* — which is this measurement on a static string. What
a runtime adds is that the string is a live session, so the copy resumes from the same frozen point
with everything but the named item identical.

The nearest neighbour is Causal Agent Replay (Shah, 2026), which treats a run as a structural causal
model, applies `do(·)` to a step and re-executes forward under the same policy — the manoeuvre every
claim here is scored against. It intervenes on *steps* rather than named context items and uses no
self-report at all, validating against planted ground truth, so the comparison this paper is built
on — what a model says against what a fork does — is not one it attempts.

VISTA (Xu et al., 2026) reaches the same diagnosis from the other end. Its first contribution is
that frontier models are "proprioceptively blind to their own context", and its answer is a
dashboard of per-block metadata with archive and recover tools. That is concurrent work, arrived at
independently and found during the literature review rather than built on, and the variable it
makes visible is not this one: VISTA's is *magnitude* — block size, recency, remaining budget —
where this is *causal dependence*. The distinction is one of kind rather than degree. No amount of
metadata display answers "would I have said something else without note 4", because a
counterfactual has to be run. VISTA makes context legible; this makes it testable.

Treating context as addressable memory is otherwise well established, for other purposes: MemGPT
(Packer et al., 2023) pages it like virtual memory, LLMLingua (Jiang et al., 2023) compresses it,
Active Context Compression (Verma, 2026) consolidates a trajectory and deletes the raw logs, Self-GC
(Hao et al., 2026) prunes indexed context objects by predicted usefulness rather than by provenance,
and Slipstream (Chen et al., 2026a) validates a compaction summary against the agent's
independently continued reasoning.
The premise is shared and the purpose is not: the same primitives verify a claim here rather than
fit a budget.

**Findings.** Self-report about context is already benchmarked, and the accuracy of such reports is
not a new question. Zeng et al. (2026) put nine kinds of question to models about their own
behaviour — whether a prompt edit would change the answer is one of them — check the answers by
resampling, find the skill real but limited, and then train models to be better at it. Naphade et
al. (2026) run a third-party control and report a small significant self-advantage. This is not a
new benchmark of that, and does not claim to be. One part of Zeng et al. is closer still. To make
training data, their pipeline rewinds a conversation to an earlier turn, changes one thing, replays
it, and has a judge score what the model then does. That is the move this apparatus makes, and it is
published first. The purpose is the opposite: there the fork manufactures examples and a model is
trained on them; here nothing is trained, and the fork is the measurement. What is asked differs
too. Their model predicts a judge's score for an edit it is told about; here the subject is asked
about the context it is sitting in, and the copies that check it have one item removed rather than
rewritten.

That post-hoc explanations are frequently unfaithful to what actually drove a prediction is settled
too (Jain and Wallace, 2019; Jacovi and Goldberg, 2020; Atanasova et al., 2023): Turpin et al.
(2023) show models producing plausible rationales that never mention the feature actually steering
them, and Lanham et al. (2023) show that perturbing chain-of-thought steps sometimes leaves the
decision unmoved, with large variation by task. §4.3 sharpens the shape rather than the fact. The
false attributions are not drawn to digits as such — no model claimed a planted numeric red herring
— but to notes belonging to the question's own arithmetic that happened not to decide it. What gets
over-claimed is not what looks numerical but what looks like a reasoning step.

**Self-correction and repair.** Huang et al. (2023) find that without external feedback models
struggle to identify and correct their own errors, and that performance sometimes degrades. §4.5's
ladder is that finding with the content of the feedback varied: disclosure repaired the answer for
every model that cleared the instrumentation bar, while the instruction to act on it cleared the
registered ceiling in one measurement of seven. The nearest published work is MemSecBench (Chen et
al., 2026b), which measures whether an
agent, told only to audit itself, removes poisoned long-term memory and keeps the benign kind. It
measured that first, in the security framing, and two things differ. Its repair stage is scored on
the state the memory ends up in, and what the poison went on to do is scored separately, at an
earlier stage; this ladder scores the next answer to the task. And its agent is given no hint about
which item is at fault and is never asked to name one, where here the subject names the item on the
record before it is *asked* to change anything — it could already, having held the handles since
`unprompted` — which is what makes naming and fixing separable at all, and lets the answer after
being told and naming be compared with the answer after being asked to put the context right.

**Introspection and causal bypassing.** Binder et al. (2024) find that a model finetuned to predict
its own behaviour beats a differently-trained model at it, and read that as privileged access;
Lindsey (2026) injects concept vectors and finds that models can notice, the strongest positive
result in the field; Singh et al. (2026) is a recent reality check on that literature. Neither
cohort overlaps this one — Binder et al. work on GPT-4, GPT-4o and Llama-3, Lindsey on Claude
models — and both required finetuning or white-box access where these measurements are zero-shot and
black-box. §4.4 also asks a different question, not *what will I say* but *what is my answer made
of*, and finds no own-context advantage. The two are compatible only if self-knowledge is not one
faculty.

That null is what causal bypassing predicts (Morris and Plunkett, 2025): a report can be accurate by
a route that never passes through the state it reports, and a subject here can get an ablation right
by reasoning about the task alone. Predicts is not shows. §4.4 reports the absence of an advantage
and not its sign, at five items per arm per model and with the foreign context arriving quoted (§6).
These numbers do not require privileged access to explain them; at this size they also do not rule
it out, and they do not establish what the model is doing instead.

*This section was assembled by a language model. Three of its characterisations have since been
checked by the works' own authors: Zeng et al. (2026), whose first author pointed me to the
multi-turn track described above; VISTA, whose first author asked for no change; and MemSecBench,
whose corresponding author asked that the scope of its repair stage be stated more exactly, which it
now is. The rest has not been checked by anyone who reads this literature regularly. It is the one
part of the paper I cannot verify from my own data, and I would welcome corrections.*

*What I can now say is narrower. Every identifier in §10 was resolved against arXiv on 2026-09-06
and matches the title and authors recorded there, so the works exist and their abstracts say what
this section says they say. That is not the same as having read them closely enough to have placed
them correctly. The check was prompted by a review in which a model with no network access declared
every citation dated 2025 or later fabricated, because the dates were in the future. It ran `date`,
was told the year was 2026, and did not revise, reading the agreement between the system clock, the
commit history and the cohort's model names as evidence of a constructed scenario. All of it
resolves. It measured, and did not update.*

## 9. Reproducing this

This study is its own repository, `https://github.com/ljedrz/deleting-a-memory`, and holds the saved
runs, the analysis and the write-ups. The runtime and the instrument are a separate one, `nachalnik`
at `https://github.com/ljedrz/nachalnik`, which this depends on at a pinned commit
(`d3b3ba6`) — deliberately, so that a second study starts from a harness rather than from a copy of
this one, so that the instrument cannot learn what this study registered, and so that the command
below reads the saved reports with the same code wherever it is run.

```
cargo run --release
```

reads `eval-runs/`, groups the reports by the instrument version that produced them, recomputes the
tables in §4.3 for both collections, and reports each of the four replication thresholds against the
value registered before the second collection — as a verdict, not a figure, because what counts as a
replication was fixed in advance and printing only the number would hand that reading back to
whoever is looking. It prints §4.1's recall and precision under the same three rules, and the count
of notes that were load-bearing for exactly one model split by whether either arm went two-one. It
then reads the repair ladder over only the cells the planted falsehood fooled, which is the second
reading in §4.5.

Nothing is re-requested and nothing costs anything. The collections themselves cost $9.11 and $15.09
— a similar number of requests on the experiments the two share, plus the harness check in §4.6,
which the second ran on three models where the first ran it on one. `PREREGISTRATION.md` holds what
was fixed in advance, with its §11 the dated log of every deviation; `RESULTS.md` and
`RESULTS-v5.md` map each registered prediction to its outcome. Instrument versions 4 and 5; every
experiment's exact wording is hashed into a digest printed with every result, so two numbers can be
compared only when they were produced by the same questions. When the location probe came out, two
of the seven digests moved and five did not.

`METHODOLOGY.md` here is the full methods document — every metric's definition, the controls, and
the eleven known threats to validity, three of them uncontrolled. `RUNBOOK.md` is what was run, in
what order, and what it cost.

## 10. References

Every arXiv identifier below was resolved against arXiv on 2026-09-06; the titles and authors are
as recorded there. `RELATED.md` holds the longer positioning notes, including work this paper does
not cite, and was checked on 2026-09-03.

- Atanasova, P., Camburu, O.-M., Lioma, C., Lukasiewicz, T., Simonsen, J. G., and Augenstein, I.
  (2023). Faithfulness Tests for Natural Language Explanations. *ACL 2023*. arXiv:2305.18029
- Binder, F., et al. (2024). Looking Inward: Language Models Can Learn About Themselves by
  Introspection. arXiv:2410.13787
- Chen, Z., Pan, R., Dai, Y., and Netravali, R. (2026a). Slipstream: Trajectory-Grounded Compaction
  Validation for Long-Horizon Agents. arXiv:2605.08580
- Chen, X., Xie, X., Fu, W., Zhou, J., Yu, S., and Xuan, Q. (2026b). MemSecBench: Tracking Agent
  Memory Poisoning from Persistence to Consequence and Repair. arXiv:2607.27080
- DeYoung, J., et al. (2020). ERASER: A Benchmark to Evaluate Rationalized NLP Models. *ACL 2020*.
  arXiv:1911.03429
- Geiger, A., et al. (2021). Causal Abstractions of Neural Networks. *NeurIPS 2021*.
  arXiv:2106.02997
- Hao, X., Meng, H., Yin, X., Zhu, J., and Cao, C. (2026). Self-GC: Self-Governing Context for
  Long-Horizon LLM Agents. arXiv:2607.00692
- Huang, J., et al. (2023). Large Language Models Cannot Self-Correct Reasoning Yet.
  arXiv:2310.01798
- Jacovi, A., and Goldberg, Y. (2020). Towards Faithfully Interpretable NLP Systems: How Should We
  Define and Evaluate Faithfulness? *ACL 2020*. arXiv:2004.03685
- Jain, S., and Wallace, B. C. (2019). Attention is not Explanation. *NAACL 2019*. arXiv:1902.10186
- Jiang, H., et al. (2023). LLMLingua: Compressing Prompts for Accelerated Inference of Large
  Language Models. *EMNLP 2023*. arXiv:2310.05736
- Lanham, T., et al. (2023). Measuring Faithfulness in Chain-of-Thought Reasoning. arXiv:2307.13702
- Lindsey, J. (2026). Emergent Introspective Awareness in Large Language Models. arXiv:2601.01828
- Meng, K., et al. (2022). Locating and Editing Factual Associations in GPT. *NeurIPS 2022*.
  arXiv:2202.05262
- Morris, A., and Plunkett, D. (2025). Tests of LLM Introspection Need to Rule Out Causal
  Bypassing. LessWrong, 28 November 2025.
- Naphade, A., et al. (2026). Me, Myself, and π: Evaluating and Explaining LLM Introspection.
  arXiv:2603.20276
- Nisbett, R. E., and Wilson, T. D. (1977). Telling More Than We Can Know: Verbal Reports on Mental
  Processes. *Psychological Review* 84(3), 231–259.
- Packer, C., et al. (2023). MemGPT: Towards LLMs as Operating Systems. arXiv:2310.08560
- Pearl, J. (2009). *Causality: Models, Reasoning, and Inference*, 2nd ed. Cambridge University
  Press.
- Shah, J. (2026). Causal Agent Replay: Counterfactual Attribution for LLM-Agent Failures.
  arXiv:2606.08275
- Singh, S., Linzen, T., and Ravfogel, S. (2026). Can LLMs Introspect? A Reality Check. *COLM
  2026*. arXiv:2605.26242
- Turpin, M., Michael, J., Perez, E., and Bowman, S. R. (2023). Language Models Don't Always Say
  What They Think: Unfaithful Explanations in Chain-of-Thought Prompting. *NeurIPS 2023*.
  arXiv:2305.04388
- Verma, N. (2026). Active Context Compression: Autonomous Memory Management in LLM Agents.
  arXiv:2601.07190
- Vig, J., et al. (2020). Investigating Gender Bias in Language Models Using Causal Mediation
  Analysis. *NeurIPS 2020*. Expanded version: Causal Mediation Analysis for Interpreting Neural NLP:
  The Case of Gender Bias, arXiv:2004.12265
- Xu, B., Li, H., and Zhang, K. (2026). LLM Agents Are Latent Context Managers: Eliciting
  Self-Managed Context via State Proprioception (VISTA). arXiv:2606.30005
- Zeng, S., et al. (2026). Evaluating and Improving LLM Self-Modeling. arXiv:2608.30980

---

## Appendix A: the arithmetic, by hand

The main claims are counts, and can be checked without any statistics at all.

**Red herrings.** Twelve per dossier-set per model, minus items whose ablation moved the answer for
that model.

- first collection: 12 + 12 + 10 + 12 + 11 + 12 = 69. Claimed: 0.
- second collection: 12 + 12 + 10 + 11 + 10 + 11 = 66. Claimed: 0.
- both: 135. Claimed: 0.

**Plain inert notes.**

- first collection: 16 + 18 + 16 + 17 + 16 + 18 = 101. Claimed: 0.
- second collection: 18 + 16 + 16 + 17 + 16 + 15 = 98. Claimed: 1, by `deepseek-v4-flash`.
- both: 199. Claimed: 1.

**Numeric over-claims.** 32 in the first collection and 31 in the second, 63 in all. Every one
of them fell outside the red-herring set and none on a red herring, which is the whole of the
discrimination result — 71 such notes were measured the first time and 70 the second. Of the 63,
**62 are notes belonging to the question's own arithmetic and one is not**: `mill/records/guidance`
carries no quantity and is filed here only because its text contains a year (§4.3).

Under the preregistration's §8.2 gate, which §2.4 records as registered and not applied, the same
counts are 26 and 25, 51 in all, with the same single exception among them. The red-herring count is
0 of 59 and 0 of 58; the plain count is 0 of 87 and 0 of 84.

**Recall and precision (§4.1), and a trap in deriving them.** Each model's 54 claims sort into four
cells: caught a load-bearing item, missed one, claimed an inert one, correctly dismissed one. Recall
is the first over the first two; precision is the first over the first and third. Pooled, that is
62/81 and 62/94 in the first collection, 59/89 and 59/91 in the second.

It is tempting to derive recall from §4.1 and §4.3 instead — accuracy minus the inert items answered
correctly — and it is right in eleven of the twelve cells. It fails on `deepseek-v4-flash` in the
second collection, where it gives 9 of 18 against a true 10 of 18. The reason is that §4.3's counts
are *claims that an item mattered*, and one of that model's items was scored wrong without being one
of them: the subject spoke and its answer could not be read, which §2.4 scores as the subject's
failure. So errors on inert items exceed §4.3's false positives by exactly one there. The four cells
are read straight off the record instead.

**Location.** 18 questions per model, 6 models = 108. Correct: 2 (both from `solar-pro4`). Kept for
completeness; §4.2 explains why this count does not measure what it looks like it measures.

**Sign test on direction.** Six models, all six with the same sign, under a null of "either
direction equally likely": (1/2)^6 = 1/64 = 0.0156. That is the one-sided figure, and the right one
for the second collection, where the direction was registered in advance. For the first, where it
was not, the two-sided figure is 2/64 = 0.031.

**Confidence intervals** in the repository's output are Wilson score intervals. The accuracy scores
are widened for clustering by dossier where more than one dossier contributes, with the unadjusted
interval printed alongside so the cost of the adjustment is visible. **The primary endpoint's
intervals are not:** `Surface` is computed from four counts and is handed no cluster information at
all, so its numeric and plain intervals are plain Wilson intervals over items that are in fact
clustered by dossier, and are correspondingly too narrow. No claim in §4.3 or §5.1 depends on one —
they are counts — but the intervals printed there should not be read as adjusted.

## Appendix B: what changed after the freeze

Everything the main text used to narrate inline. `PREREGISTRATION.md` §11 is the authoritative
version, with all twenty-one dated entries and their reasoning; this is the short form.

### B.1 The first collection's timestamps

A preregistration is worth exactly what its date is worth, so here are the first collection's in
full. The registered document records its decision table as committed at 18:54:00Z on 2026-09-03 —
76 seconds after the first model's `attribution` report was written, and 78 and 88 minutes before
that model's `repair` and `instrumented` reports. **That commit is not in the published history.**
The instrument was split into its own crate the following morning and its history remade with it, so
the hash the document cites resolves to nothing and those three margins rest on my word.

What anyone can check is the first commit on the public `preregistration` branch, at 20:38:11Z:
sixteen minutes *after* the last of the first model's reports, and eight minutes *before* the first
request went out for the other five. So for one model of six the public record puts registration
after the data, and for the other five before it, by a margin nobody would call comfortable.

Two things cut against even that. A commit's timestamp is set by the machine that made it rather
than by the server, and GitHub does not publish when a push arrived, so "public" here means anyone
can see the same timestamps I can, not that a third party vouches for them. And "I hadn't looked
yet" is not something anyone can verify; it is not offered as evidence. For the first collection the
honest claim is the weaker one, and the repository says so too. The second collection's thirteen
minutes (§3) are what that endpoint rests on instead.

### B.2 The two withdrawn results

**2026-09-04 — the location figure is withdrawn.** An earlier version of this paper treated the
2-of-108 location score as its cleanest finding. It is an artefact of asking for a number the
subject had no way to see (§4.2); the abstract no longer mentions it and v5 removed the probe
rather than guess at it. It was caught while scoping a follow-up study, which is later than it
should have been. The class of error is fenced now rather than merely regretted:
`tests/machinery.rs`
requires that any experiment asking for an item's number also grant a handle to look at the
numbering, and pins the two frozen experiments that do not.

**After drafting — the repair-ladder claim was stated too broadly.** A draft read the ladder as
showing that deleting a false note was worth nothing. It weakened when `glm-5.3-flash` went 10/15 to
14/15 on the last rung in the second collection, and it was wrong in kind as well as degree: the
ladder scores the answer a subject is about to give, not what its context carries afterwards (§5.1).
The claim now made is the narrower one.

### B.3 Other deviations, in brief

- **2026-09-03, before any confirmatory run.** The shared brief told every subject it had no tools,
  which was false of the two experiments that hand over handles; a second brief was added for those
  two, moving their digests and no others.
- **2026-09-03, before any confirmatory run.** A primary endpoint turned out to be circular — the
  tool printed the answer it was being scored on — and was replaced.
- **2026-09-03, before any confirmatory run.** The material set could not falsify its own
  hypothesis: every inert note in the whole set had three digits or fewer, so "contains numbers" and
  "matters" were confounded. Red herrings were added (§2.2). Instrument v4 makes the hypothesis
  falsifiable and breaks comparability with v3 to do it.
- **2026-09-03.** Two models were substituted, neither having produced a figure bearing on any
  hypothesis: one could not complete a run through its provider, which cut it off after about
  fifteen requests every time; one was swapped for a stronger and cheaper alternative in the same
  slot.
- **2026-09-04.** The registered commitment to run the handle check on three models was not kept on
  the day — the first attempt died on an exhausted key limit — and was kept later the same day once
  the limit was raised.
- **2026-09-04.** The pre-specified handle-use gate excluded three models rather than two, because
  `deepseek-v4-flash` came in at 47% against a 50% bar it had cleared exactly the first time.
  Applied as written (§5, point 10).
- **2026-09-06.** A commit the preregistration cites no longer resolves, for the same
  history-rewrite reason as B.1.
- **2026-09-17, after both collections.** Four errors found in the registered document itself while
  re-checking it against the record: a power table wrong in three of its four rows, two sections
  registering different tests for the same hypothesis, §8.2 registered as a gate the instrument
  never applied (§2.4), and H3 naming the wrong intervention. Also that "the question's own
  arithmetic" is a residual category rather than a defined one, which §4.3 now says.
- **An analysis script silently dropped a model from the results.** Fixed; the command now names any
  report it could not parse rather than skipping it in silence.
