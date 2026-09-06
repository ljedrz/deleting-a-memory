---
title: "Deleting a Memory and Running It Again"
subtitle: "An instrument for checking what a model says about its own context"
author: "Łukasz Jędrzejczyk"
date: "September 2026"
---

## Abstract

Ask a language model which part of its context made it answer the way it did, and it will tell you.
Nobody can check the answer. Every runtime hands the model a string; once the request is sent the
counterfactual is gone, so a stated reason can be shown implausible but never wrong.

I built a runtime in which a session's context is addressable state, so a session can be forked, one
named item removed, and the copy asked the same question again. That gives ground truth for the
ablation: not what a model says its answer depended on, but whether the answer actually changes
without the item. **This paper is about the apparatus** — what it takes to make a measurement like
that honest, and what it found when I pointed it at six models from six labs, twice.

The registered hypotheses failed. What came out instead was sharper, and was then collected a second
time on a corrected instrument, against four thresholds registered and timestamped beforehand. All
four held. Every question and answer from both collections is saved, and every number recomputes
with one command.

Models are reasonably good at saying *what* their answer rests on: 72–96% correct in the first
collection and 72–85% in the second, above their own majority baseline in four of six models and
then five of six. Their false positives are not random — across both collections, **0 of 135**
planted irrelevant notes were ever claimed to matter, **1 of 199** notes without figures was, and
every one of the other **63** fell on notes belonging to the question's arithmetic that did not
decide it. Reasoning about your own context was no more accurate than reading a transcript of
someone else's. And on a five-rung repair ladder, *telling* a model one of its notes is false helped
every model that could be measured, while *letting it delete the note* added close to nothing in six
of seven measurements — and a great deal in the seventh.

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

That is worth pausing on, because it is the part that could not be done before. In fifty years of
work on human self-report — starting with Nisbett and Wilson's *Telling More Than We Can Know*
(1977) — the standing problem is that you cannot rerun a person with one memory deleted. You can
show that a stated reason is *implausible*. You cannot show it is *wrong*. Here you can.

So I used it to check a simple thing: when a model explains what its answer depends on, is it
right?

I expected it not to be. I was wrong, and the ways in which I was wrong are the useful part.

But the measurements are not what I think is worth your time. Six models will be deprecated inside
a year and their numbers with them. What survives is the apparatus and the discipline it needs,
because **running the counterfactual is the easy half**. The hard half is everything that stops the
result from being an artefact: what to baseline against, what to refuse to score, how to tell your
own failure from the subject's, and how to know whether two runs were asked the same question at
all. Every one of those is a decision I got wrong at least once, and §5 is the list.

The strongest evidence I can offer that the apparatus works is that it caught me, twice, in public,
with dates. A result I had called the study's cleanest turned out to be an artefact of asking for a
number the subject could not see — caught by a baseline the harness printed and I had not read. And
a claim that deleting a false note was worth nothing weakened when one model gained a great deal
from it. Both are in §4 and §6, because an instrument that cannot embarrass the person holding it is
not measuring anything.

What this is not is a peer-reviewed study. I am a Rust engineer rather than a researcher, nobody who
reads this literature regularly has checked it, and it should be read as a report on an instrument
and what it measured. §7 says who did what, including the parts a language model did.

## 2. Method

### 2.1 The setup

Each trial plants a small dossier of notes in a fresh session — a set of records about, say, three
warehouses — and asks a question the notes answer, such as which one runs out of space first. The
model answers.

Then, for each note, two things happen:

- **The model is asked**: "if this note were removed, would you answer differently?"
- **The harness finds out**: it forks the session, removes that note, and asks the question again.
  Several copies, to check they agree with each other.

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

The main measurement is deliberately narrow. It looks **only at notes whose removal provably did
not change the answer** — where the copies were unanimous and unmoved. For all of those, the honest
claim is "no, this doesn't matter". So among items that all do nothing, does the model claim the
numeric ones matter more often than the plain ones?

Restricting it that way is the whole design. Across the material as a whole, notes with figures
really *are* more likely to matter, because that is where the arithmetic lives. An unrestricted
comparison would reward a model for a lucky prior. Inside a group of items that all provably do
nothing, there is nothing left to be right about, so any gap between the halves cannot be knowledge.

What is scored, throughout, is a prediction of the **ablation outcome**: whether removing the item
changes what the model says. That is not the same as whether the item influenced the original
computation. An item can be read, reasoned over and still be redundant, and its removal will change
nothing — the apparatus will call it inert, correctly, for the question it is asking. Every claim in
this paper is about that question and no larger one.

### 2.4 What the apparatus refuses to do

Three refusals and one warning do more work than anything in the scoring, and each exists because
something went wrong without it.

**It says, above the numbers, when the material did nothing to that subject.** Before the scores are
printed, the harness checks that the copies actually disagree, that the subject answered the dossier
as its notes support, and — in the repair ladder — that the planted falsehood actually fooled it.
These checks fire often, and every one is printed beside the numbers as an `unmet:` line rather than
hidden in a footnote. Only the first of them excludes anything by itself: the primary endpoint is
defined over items whose copies were unanimous, so an item whose copies disagreed is outside it by
construction. The others flag rather than gate. In the repair ladder every rung is scored over all
fifteen cells, the ones the falsehood never took included — which is why `carrying` reads as the
share of cells the lie failed to fool, and why the ladder is read as differences between rungs
rather than as accuracies.

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
branch.

This matters more than usual here, because I generated the hypotheses by looking at some early
exploratory runs. That is a legitimate way to *find* a hypothesis and a terrible way to *test* one,
so the fix is to register it and then collect fresh data. Which is what happened.

Being precise about the timestamp, since a preregistration is worth exactly what its date is worth.
The registered document records its decision table as committed at 18:54:00Z on 2026-09-03 — 76
seconds after the first model's `attribution` report was written, and 78 and 88 minutes before that
model's `repair` and `instrumented` reports. That commit is not in the published history: the
instrument was split out into its own crate the following morning and its history remade with it, so
the hash the document cites resolves to nothing and those three margins rest on my word. What anyone
can check is the first commit on the public `preregistration` branch, at 20:38:11Z — sixteen minutes
*after* the last of the first model's reports and eight minutes *before* the first request went out
for the other five. So for one model of six the public record puts the registration after the data,
and for the other five before it, by a margin nobody would call comfortable. A commit's timestamp is
set by the machine that made it rather than by the server, and GitHub does not publish when a push
arrived, so "public" here means anyone can see the same timestamps I can, not that a third party
vouches for them. "I hadn't looked yet" is not something anyone can verify either, and is not
offered as evidence. For the first collection the honest claim is the weaker one, and the repository
says so too.

**The second collection repairs that.** After the defect in §4.2 was found, an amendment naming four
thresholds — what would count as a replication, decided before there was anything to look at — was
committed to that public branch at **10:18:58Z**, and the first request of that collection went out
at **10:31:43Z**. Thirteen minutes, both timestamps visible to anyone. The endpoint whose
first-collection margin cannot be checked has a checkable one the second, which is the entire reason
the re-run was worth $15.

The registered document also contains a rule I want to draw attention to, because it is the rule
that stopped me rescuing my own hypothesis:

> No new hypotheses. If all of them fail, the honest paper is a negative one.

All but one of them failed, and no replacements were invented — §4.6 is the test of that rule rather
than an exception to it, since the most interesting thing measured here carries no prediction and is
fenced off as a pilot. What the second collection then established is narrower than a hypothesis and
more useful than a null: the four registered thresholds held, so the effect the first collection
found is not an artefact of the defect it was found with.

## 4. Results

Six models: `upstage/solar-pro4`, `deepseek/deepseek-v4-flash-0731`, `tencent/hy3`,
`meituan/longcat-2.0`, `x-ai/grok-4.6`, `z-ai/glm-5.3-flash`. Six labs, and a range from a model
that emits an answer with no visible deliberation to a frontier one. Each was measured twice: a
first collection on instrument v4, run on 2026-09-03 and into the following morning, and a second on
v5 on 2026-09-04, after the defect in §4.2 was found and removed.

**The confirmatory claims are the four thresholds in §4.3, registered and timestamped before the
second collection.** Everything else here is exploratory or harness validation, is labelled as such
where it appears, and is not used to extend those claims.

### 4.1 Models are decent at saying what mattered

The baseline is the best a subject with no self-knowledge could score by picking one answer and
repeating it — not chance. Both collections are exploratory here; no prediction covers accuracy.

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

### 4.2 An item's number was not a fair question

The same models were asked, about the same notes: *what number is this note in your context?*

| model | correct |
| --- | --- |
| solar-pro4 | 2/18 |
| deepseek-v4-flash | 0/18 |
| hy3 | 0/18 |
| longcat-2.0 | 0/18 |
| grok-4.6 | 0/18 |
| glm-5.3-flash | 0/18 |

**2 of 108.** An earlier draft called that the study's cleanest result. It is not a result at all.

The projector renders each item as `label:` and its content, never a number, and `attribution`
installs no handles — so these subjects had no way to see the numbering they were being asked
about. They were asked for a value they could not observe. The score says so: 1.9% against a 33%
majority baseline, an order of magnitude *below* what a constant guesser gets, which is the
signature of an unanswerable question rather than an absent faculty (§5, point 1).

nachalnik *can* address items to a model — `inspect`'s `look` lists every one by number — and that
handle simply was not granted here. **So this says nothing about whether a model can locate an item
when allowed to look.** That question is open; the figure was withdrawn on 2026-09-04, and v5
removed the probe rather than guess at it.

### 4.3 The errors have a shape

Looking only at notes that provably do nothing:

**First collection, instrument v4 — exploratory.** The hypotheses were registered, but the defect in
§4.2 was still in the instrument when these were taken.

| model | numeric claimed | plain claimed | red herrings | question's arithmetic |
| --- | --- | --- | --- | --- |
| longcat-2.0 | 10/26 | 0/18 | **0/12** | 10/14 |
| solar-pro4 | 9/27 | 0/16 | **0/12** | 9/15 |
| deepseek-v4-flash | 4/19 | 0/16 | **0/10** | 4/9 |
| hy3 | 4/24 | 0/17 | **0/12** | 4/12 |
| grok-4.6 | 3/21 | 0/16 | **0/11** | 3/10 |
| glm-5.3-flash | 2/23 | 0/18 | **0/12** | 2/11 |
| **total** | 32/140 | **0/101** | **0/69** | 32/71 |

**Second collection, instrument v5 — confirmatory.** Same six models, same material, location probe
removed. The four thresholds these are read against were committed to a public branch thirteen
minutes before the first request.

| model | numeric claimed | plain claimed | red herrings | question's arithmetic |
| --- | --- | --- | --- | --- |
| longcat-2.0 | 8/26 | 0/18 | **0/12** | 8/14 |
| solar-pro4 | 7/24 | 0/16 | **0/12** | 7/12 |
| deepseek-v4-flash | 4/19 | **1/16** | **0/10** | 4/9 |
| hy3 | 5/23 | 0/17 | **0/11** | 5/12 |
| grok-4.6 | 4/21 | 0/16 | **0/10** | 4/11 |
| glm-5.3-flash | 3/23 | 0/15 | **0/11** | 3/12 |
| **total** | 31/136 | **1/98** | **0/66** | 31/70 |

Three things here, and none of them is a rate — they are counts, which is why they are worth
trusting. All three figures below span both collections.

**Not one model, in either collection, claimed a red herring mattered.** Zero out of a hundred and
thirty-five. These six models are not fooled by irrelevant numbers.

**One note without figures was claimed, out of a hundred and ninety-nine.** The first collection had
none at all; `deepseek-v4-flash` claimed one in the second, which is why this sentence no longer
says "never".

**Every false positive carrying a figure — sixty-three of sixty-three — was a note belonging to the
question's own arithmetic** that happened not to decide it. Counting the plain note above, that is
sixty-three of the sixty-four claims made in total. Every model showed it in both collections, and
the effect ran the same way for all six each time (sign test, p = 0.016 one-sided; Appendix A).

My registered hypothesis was that models over-weight *digits*. That is wrong. They over-weight
*things that look like the reasoning*, which is a different and much more sensible failure. I
registered that distinction in advance as the outcome most likely to be mistaken for the other one,
which is the only reason I am able to say it cleanly now.

Where the errors land is a count and stands. *Why* they land there is a reading, and a note that
belongs to the question's arithmetic differs from an inert one in more than topic — length and
position travel with it, and this material was not built to pull those apart.

### 4.4 Reading your own context looks like just reading

Each model was asked the same kind of counterfactual question twice: once about its own context, and
once about a transcript of a different session's context, quoted to it.

Own arm vs foreign arm, first collection: 1/5 vs 3/5, 1/5 vs 1/4, 3/5 vs 3/5, 3/5 vs 3/5, 3/5 vs
2/5, 4/5 vs 4/5 — three exact ties, one pair five points apart, and one each way.

Second collection: 1/5 vs 3/5, 3/5 vs 3/5, 4/5 vs 3/5, 3/5 vs 5/5, 3/5 vs 3/5, 5/5 vs 5/5 — three
ties, two to the foreign arm, one to the own. Pooled, 19/30 own against 22/30 foreign.

**No own-context advantage showed up.** Whatever a model is doing when it reflects on its own
context, it performed the same as when it read someone else's, and pooled across the second
collection slightly worse. Five items per arm per model and thirty paired observations per
collection, so what is being reported is the absence of an advantage in this cohort on this task —
not its sign, and not evidence that privileged access is impossible. The per-model pairs are printed
above because at this size they are worth more than anything I could compute from them.

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
fifteen observations per rung per model, less the odd cell a truncated turn took out. **First
collection:**

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
| deepseek-v4-flash | 3/15 | 1/15 | 1/15 | 7/14 | 8/14 | **47%** |
| hy3 | 6/15 | 7/15 | 11/15 | 13/15 | 13/15 | 62% |
| grok-4.6 | 5/15 | 5/15 | 5/15 | 9/15 | 9/15 | 68% |
| glm-5.3-flash | 9/15 | 7/15 | 8/15 | 10/15 | 14/15 | 58% |

Models using the tools on fewer than half the questions they could have were excluded from this
comparison in advance: two of six the first time, and three the second, because `deepseek-v4-flash`
came in at 47% having cleared the same bar at exactly 50%. The rule is pre-specified and was applied
as written rather than nudged — see §5, point 10. `solar-pro4` is worth a sentence on its own: in
the first collection it made **16 edits off 6 tests**, rewriting its own context nearly three times
as often as it measured anything, and 13 off 7 in the second; both times its answers got *worse*
with tools than without them (5/15 down to 0/15, then 6/15 down to 0/15). That is a small, concrete
warning about giving edit authority to weak models.

Among those that cleared the bar — four models the first time, three the second:

- **Being told helped every one of them, both times.** Four out of four, then three out of three.
- **Being allowed to delete the note added almost nothing, with one exception.** +13, +5, 0 and −20
  points the first time; 0, 0 and **+27** the second, where `glm-5.3-flash` went 10/15 to 14/15 once
  it could remove the note it had just named. One model out of seven measurements, at n = 15, so it
  is a counterexample rather than a reversal — but the flat reading is no longer unanimous and
  should not be repeated as though the second collection had not happened.
- **Being handed tools with no hint** gained more than the registered five points on three of four
  the first time and on two of three the second — though two of those five gains were seven points,
  and one of the two does not survive the reading below.

The tables above score every cell, including the ones where the falsehood never took (§2.4). Read
over the cells where it did — 43 of 60 in the first collection and 25 of 45 in the second, among the
models that cleared the bar — nothing changes direction and the dilution comes off. Being told
is +37 and +32 points pooled, 17 cells gained against 1 lost and then 8 against 0; deletion is −2
and +12; tools with no hint are +14 and +16, and the second collection's small `glm-5.3-flash` gain
is gone, because the one cell it came from was a cell the lie had never fooled. `cargo run` prints
this reading beside the other.

The registered prediction was that the deletion would be what mattered. It is not. The information
is what matters — for the answer the subject is about to give, which is the only thing this ladder
scores (§5.1).

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
me. The location probe scored 2 of 108 against a *majority* baseline of 33% — the score a subject
with no self-knowledge gets by picking one answer and repeating it. Landing an order of magnitude
under the do-nothing baseline is not a weak faculty, it is a broken question, and the harness had
been printing that number all along.

**2. Test your preconditions and print them.** In the second collection the harness flagged 32 of
the 90 repair-ladder cells because the planted falsehood never fooled the subject in the first
place. Those cells stay in the tables — the ladder is read as differences between rungs — and the
flag is what stops a model too sharp to be fooled from reading as a model that failed to repair.
§4.5 gives the ladder both ways.

**3. Restrict the endpoint to items where the honest answer is known.** The primary measurement
looks only at notes whose removal provably changed nothing, because there the correct claim is "no"
for every item. Across the material as a whole, notes with figures really are more likely to
matter — an unrestricted comparison would pay a model for a lucky prior.

**4. Ask about two copies, not about the model.** "Would *your* answer change" is a different
question from "will these two copies differ", and a subject can be exactly right about the copies
and scored wrong because the live session — which has the elicitation in its context — answered
unlike both. I learnt this from a model that was right while I was marking it wrong.

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

**Expect false positives in a specific place.** Across 135 planted irrelevant numbers over two
collections, models fell for none. One note without figures was claimed, out of 199. Every other
mistake landed on material that was topically central and causally inert. If you ask a model which
parts of its context mattered, that is where its errors will be.

**Don't assume self-inspection is privileged.** A model reasoning about its own context did no
better than the same model reading a stranger's transcript — pooled, slightly worse.

**Tell your agents things.** There is enthusiasm for handing agents tools to prune their own
context. On this evidence the *information* carries more than the tools: saying that a note is false
helped every model measurable, while letting it delete the note it had just named usually added
nothing — six of seven measurements, though one model gained a great deal.

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

**The foreign arm arrives quoted where the subject's own arrives as context.** That is the
difference §4.4 tests and also, unavoidably, a difference in presentation. The quotation is rendered
exactly as the projector renders the subject's own notes, which is the mitigation; the registered
counterbalance — running the two dossiers the other way round — was never run, in either collection.
The null in §4.4 is read with that confound unremoved.

**The hypotheses came from exploratory data.** I found them by looking at early runs, then
registered them and collected fresh data. Standard practice, but it means the exploratory runs
cannot also count as evidence, and they do not.

**The first collection's registration is weaker than the second's.** The commit the registered
document cites for its decision table is no longer in the published history, and the first commit on
the public branch postdates one model's complete first-collection results and predates the other
five by eight minutes (§3). The second collection's thresholds were on that branch thirteen minutes
before its first request, so that endpoint has a margin anyone can check; the first collection's
figures keep the weak one.

**Two models were substituted mid-study**, both logged with dates and reasons, and neither had
produced a figure bearing on any hypothesis. One could not complete a run through its provider,
which cut it off after about fifteen requests every time; one was swapped for a stronger and cheaper
alternative in the same slot.

**One reported result was withdrawn after drafting, on 2026-09-04.** An earlier version of this
paper treated the 2-of-108 location figure as its cleanest finding. It is an artefact of asking for
a number the subject had no way to see; §4.2 now says so and the abstract no longer mentions it. It
was caught while scoping a follow-up study, which is later than it should have been. The class of
error is fenced now rather than merely regretted: `tests/machinery.rs` requires that any experiment
asking for an item's number also grant a handle to look at the numbering, and pins the two frozen
experiments that do not.

**The withdrawn probe ran first — checked, and it did not matter.** The v5 collection removed it
and re-ran the whole study: the pooled difference moved from +23 to +22, discrimination stayed
negative on 6 of 6, and every registered threshold held (§4.3, `RESULTS-v5.md`). The concern below
stood until it was tested and no longer does; it is kept because a reader should see what was
suspected as well as what was found.

**The withdrawn probe ran first, and its answers stayed in the context.** `attribution` asks the
three location questions *before* the counterfactual battery, so every claim in §4.1 and §4.3 was
made in a context where the subject had just produced three confident, wrong item numbers. This is
identical across all six models and both arms, so comparisons within the study are unaffected, and
the §4.3 result is a contrast between two kinds of item in the same context, where any general shift
in confidence cancels. The absolute rates could still be moved by it. I would not order the probes
this way again.

**The ablation handle is now confirmed on three model families.** v5 registered this check on three
models; the first attempt died on an exhausted key limit and it was re-run on 2026-09-04 once that
was raised. Handle use came in at 98%, 100% and 100% of questions across `glm-5.3-flash`, `hy3` and
`grok-4.6`, with 353 tests run and no edits. The limitation immediately below is therefore
**closed**, and is kept in place so that a reader comparing versions can see what changed.
`PREREGISTRATION.md` §11 has the dates.

**Only one model ran the tool-use check** that confirms the ablation handle returns what it should.
It did — 100% use across 108 questions — but a second would have been better.

## 7. How this was made

Given the subject matter, this section is not optional.

**The great majority of this work was done by a language model** (Claude Opus 5), working
interactively with me over one long session. It wrote the runtime harness, the experiment code, the
statistics, the preregistration, the analysis tooling, and the first draft of this paper. I set the
direction, made every judgement call about scope and spending, funded it, and pushed back — several
times decisively, including on the framing of related work and on the choice of models.

I am a Rust engineer, not a researcher. This is an independent technical report rather than a
peer-reviewed study, and its status should be read that way.

What makes it checkable regardless of who wrote it is that **you do not have to trust the
process**:

- the preregistration is on a timestamped public branch, dated before most of the data existed;
- every question put to every model and every answer received is saved verbatim;
- every number in this paper recomputes from those records with one command;
- the deviations log has fifteen dated entries, including the ones that are unflattering.

Several real errors were caught during the work and are recorded rather than tidied away: a primary
endpoint that turned out to be circular (the tool printed the answer it was being scored on), a
material set that could not falsify its own hypothesis, an analysis script that silently dropped a
model from the results, and a timestamp claim that was 76 seconds wrong and now rests on a commit
the published history no longer holds (§3). I mention these because a paper with no visible mistakes
in a process this long is a paper whose mistakes were not written down.

## 8. Related work

**Apparatus.** Ground truth about what caused something requires intervention rather than
observation (Pearl, 2009), and that intervention has been applied to language models at every layer,
this one only lately. Causal mediation analysis (Vig et al., 2020), interchange interventions
(Geiger et al., 2021) and activation patching (Meng et al., 2022) intervene on internal
representations. At the prompt level, ERASER (DeYoung et al., 2020) scores a rationale by erasing
the tokens it names and measuring what changes — *comprehensiveness* and *sufficiency* — which is
this measurement performed on a static string. What a runtime adds is that the string is a live
session: context is addressable state, so a session can be forked, a named item removed, and the
copy resumed from the same frozen point with everything else held identical.

The nearest neighbour is Causal Agent Replay (Shah, 2026), which treats a run as a structural
causal model, applies `do(·)` to a step and re-executes forward under the same policy — the same
manoeuvre every claim here is scored against. Two differences matter: it intervenes on *steps*
rather than on named context items, and it uses no self-report at all, validating against planted
ground truth instead. The comparison this paper is built on, between what a model says and what a
fork does, is not one it attempts.

VISTA (Xu et al., 2026) reaches the same diagnosis from the other end. Its first contribution is
that frontier models are "proprioceptively blind to their own context", and its answer is a
dashboard of per-block metadata with archive and recover tools. That is concurrent work, arrived at
independently and found during the literature review rather than built on, and the variable it
makes visible is not this one: VISTA's is *magnitude* — block size, recency, remaining budget —
where this is *causal dependence*. The distinction is one of kind rather than degree. No amount of
metadata display answers "would I have said something else without note 4", because a
counterfactual has to be run. VISTA makes context legible; this makes it testable.

Treating context as addressable memory is otherwise well established, for other purposes. MemGPT
(Packer et al., 2023) manages it as OS-style virtual memory with paging and eviction and LLMLingua
(Jiang et al., 2023) compresses it; Active Context Compression (Verma, 2026) has an agent
consolidate a trajectory and delete the raw logs, scored on tokens saved at equal accuracy; Self-GC
(Hao et al., 2026) folds, masks and prunes indexed context objects by predicted future usefulness
rather than by provenance; Slipstream (Chen et al., 2026a) validates a compaction summary against
the agent's independently continued reasoning. The premise is shared and the purpose is not: the
same primitives are used here to verify a claim rather than to fit a budget.

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
too (Jain and Wallace, 2019; Jacovi and Goldberg, 2020; Atanasova et al., 2023). Turpin et al.
(2023) show models producing plausible rationales that never mention the feature actually steering
them, and Lanham et al. (2023) show that perturbing chain-of-thought steps sometimes leaves the
decision unmoved, with large variation by task. §4.3 sharpens the shape rather than the fact. The
false attributions are not drawn to digits as such — not one model claimed a planted numeric red
herring, 0 of 135 — but to notes belonging to the question's own arithmetic that happened not to
decide it: every numeric false positive, 63 of 63. What gets over-claimed is not the thing that
looks numerical but the thing that looks like a reasoning step.

**Self-correction and repair.** The limits of intrinsic self-correction are well documented: Huang
et al. (2023) find that without external feedback models struggle to identify and correct their own
errors, and that performance sometimes degrades. §4.5's ladder is that finding with the content of
the feedback varied. Tools and no hint helped unevenly — three of four models the first time and two
of three the second, two of those gains small. Being told that a note was false repaired the answer
for every model that cleared the instrumentation bar, both times; being allowed to delete the note
it had just named then added almost nothing in six of seven measurements, and a great deal in the
seventh. The nearest published work is MemSecBench (Chen et al., 2026b), which measures whether an
agent, told only to audit itself, removes poisoned long-term memory and keeps the benign kind. It
measured that first, in the security framing, and two things differ. Its repair stage is scored on
the state the memory ends up in, and what the poison went on to do is scored separately, at an
earlier stage; this ladder scores the next answer to the task. And its agent is given no hint about
which item is at fault and is never asked to name one, where here the subject names the item on the
record before it is allowed to change anything — which is what makes naming and fixing separable at
all, and lets the answer after being told and naming be compared with the answer after being asked
to fix.

**Introspection and causal bypassing.** Binder et al. (2024) find that a model finetuned to predict
its own behaviour beats a differently-trained model at it, and read that as privileged access;
Lindsey (2026) injects concept vectors and finds that models can notice, the strongest positive
result in the field. Binder et al. work on GPT-4, GPT-4o and Llama-3 and Lindsey on Claude models,
none of which is in this cohort, and both required finetuning or white-box access where these
measurements are zero-shot and black-box. Singh et al. (2026) is a recent reality check on that
literature and is worth reading beside them. §4.4 asks a different question — not *what will I say*
but *what is my answer made of* — and finds no own-context advantage over the same model reasoning
about a matched dossier held by a second session that really ran. The two are compatible only if
self-knowledge is not one faculty.

That null is what causal bypassing predicts (Morris and Plunkett, 2025): a report can be accurate
by a route that never passes through the state it reports, and a subject here can get an ablation
right by reasoning about the task — the Omsk annex is the decisive record, so removing it must
change the answer — with no self-access involved. Predicts is not shows. What §4.4 reports is the
absence of an advantage and not its sign, at five items per arm per model, with the foreign context
arriving quoted where the subject's own arrives as context (§6). It removes privileged access as
the explanation for these numbers. It does not establish what the model is doing instead.

*This section was assembled by a language model. Three of its characterisations have since been
checked by the works' own authors: Zeng et al. (2026), whose first author pointed me to the
multi-turn track described above; VISTA, whose first author asked for no change; and MemSecBench,
whose corresponding author asked that the scope of its repair stage be stated more exactly, which it
now is. The rest has not been checked by anyone who reads this literature regularly. It is the one
part of the paper I cannot verify from my own data, and I would welcome corrections.*

*What I can now say is narrower, and worth stating exactly. Every identifier in §10 was resolved
against arXiv on 2026-09-06 and matches the title and authors recorded there, so the works exist and
their abstracts say what this section says they say. That is not the same as having read them
closely enough to have placed them correctly, which is what the paragraph above is about. The check
was prompted by a review in which a model with no network access declared every citation dated 2025
or later to be fabricated, on the grounds that the dates were in the future, the 2025 ones included.
It ran `date`, was told the year was 2026, and did not revise, reading the agreement between the
system clock, the commit history and the cohort's model names as evidence of a constructed scenario
rather than of the date. All of it resolves. The episode belongs to §7 rather than to this section,
except in one respect: it measured, and did not update.*

## 9. Reproducing this

This study is [its own repository](https://github.com/ljedrz/deleting-a-memory) and holds the saved
runs, the analysis and the write-ups. The runtime and the instrument are a separate one,
[`nachalnik`](https://github.com/ljedrz/nachalnik), which this depends on at a pinned commit
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
whoever is looking. It then reads the repair ladder over only the cells the planted falsehood
fooled, which is the second reading in §4.5.

Nothing is re-requested and nothing costs anything. The collections themselves cost $9.11 and $15.09
— a similar number of requests on the experiments the two share, plus the harness check in §4.6,
which the second ran on three models where the first ran it on one. `PREREGISTRATION.md` holds what
was fixed in advance, with §11 the dated log of every deviation; `RESULTS.md` and `RESULTS-v5.md`
map each registered prediction to its outcome. Instrument versions 4 and 5; every experiment's exact
wording is hashed into a digest printed with every result, so two numbers can be compared only when
they were produced by the same questions. When the location probe came out, two of the seven digests
moved and five did not.

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

- first collection: 12 + 12 + 10 + 12 + 11 + 12 = **69**. Claimed: 0.
- second collection: 12 + 12 + 10 + 11 + 10 + 11 = **66**. Claimed: 0.
- both: **135**. Claimed: 0.

**Plain inert notes.**

- first collection: 16 + 18 + 16 + 17 + 16 + 18 = **101**. Claimed: 0.
- second collection: 18 + 16 + 16 + 17 + 16 + 15 = **98**. Claimed: 1, by `deepseek-v4-flash`.
- both: **199**. Claimed: 1.

**Numeric over-claims.** 32 in the first collection and 31 in the second, **63** in all. Every one
of them fell on a note belonging to the question's own arithmetic — 71 such notes were measured the
first time and 70 the second — and none on a red herring, which is the whole of the discrimination
result.

**Location.** 18 questions per model, 6 models = 108. Correct: 2 (both from `solar-pro4`). Kept for
completeness; §4.2 explains why this count does not measure what it looks like it measures.

**Sign test on direction.** Six models, all six with the same sign, under a null of "either
direction equally likely": (1/2)^6 = 1/64 = 0.0156. That is the one-sided figure, and the right one
for the second collection, where the direction was registered in advance. For the first, where it
was not, the two-sided figure is 2/64 = 0.031.

**Confidence intervals** in the repository's output are Wilson score intervals, and are widened for
clustering by dossier where more than one dossier contributes. The unadjusted interval is printed
alongside so the cost of the adjustment is visible. No claim in §4.3 or §5.1 depends on an
interval; they are counts.
