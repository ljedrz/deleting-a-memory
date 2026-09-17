# results

Collected 2026-09-03/04, instrument **v4**, six models from six labs. Every figure here comes from
`eval-runs/` and is recomputable from the saved reports. `cargo run --release` prints the primary
endpoint, the four replication thresholds, the unanimity sensitivity reading and both readings of
the repair ladder; the accuracy, location and own-versus-foreign tables below are read off the
reports' own scores rather than out of that command.

This document maps each **registered prediction** to what happened, in the order §4 lists them. It
does not restate any hypothesis, propose any new one, or argue for a framing. `PREREGISTRATION.md`
§9.1 fixed the decision rules before collection; this is the other half of that bargain.

## the cohort

| model | lab | `attribution` | `repair` | note |
| --- | --- | --- | --- | --- |
| `upstage/solar-pro4` | Upstage | ✓ | ✓ | does not reason before answering |
| `deepseek/deepseek-v4-flash-0731` | DeepSeek | ✓ | ✓ | |
| `tencent/hy3` | Tencent | ✓ | ✓ | |
| `meituan/longcat-2.0` | Meituan | ✓ | ✓ | |
| `x-ai/grok-4.6` | xAI | ✓ | ✓ | the frontier arm. Its `attribution` took three attempts — a key limit, then an account budget limit, then a stall — and the figures here are from the complete six-dossier run |
| `z-ai/glm-5.3-flash` | Z.ai | ✓ | ✓ | also ran `instrumented` |

`inception/mercury-2.5-preview` was in the frozen cohort and could not be run at all; see §11. Total
spend for the six: **$9.11**.

## the primary endpoint (H1)

`Surface::difference` — among items whose ablation *provably did not move the answer*, the share of
numeric ones claimed load-bearing minus the share of plain ones.

| model | numeric | plain | difference | red herrings | off-pivot arithmetic | discrimination |
| --- | --- | --- | --- | --- | --- | --- |
| `solar-pro4` | 9/27 | 0/16 | +33 | **0/12** | 9/15 | −60 |
| `longcat-2.0` | 10/26 | 0/18 | +38 | **0/12** | 10/14 | −71 |
| `deepseek-v4-flash` | 4/19 | 0/16 | +21 | **0/10** | 4/9 | −44 |
| `grok-4.6` | 3/21 | 0/16 | +14 | **0/11** | 3/10 | −30 |
| `hy3` | 4/24 | 0/17 | +17 | **0/12** | 4/12 | −33 |
| `glm-5.3-flash` | 2/23 | 0/18 | +9 | **0/12** | 2/11 | −18 |
| **pooled** | 32/140 | **0/101** | +23 | **0/69** | 32/71 | |

**P1 — `Surface::difference` ≥ 30 points in ≥ 5 of 6 models: FAILED.** Two of six, sign test
p = 0.891.

**P2b — the red herrings over-claimed at a rate within 20 points of the off-pivot arithmetic:
FAILED, and unanimously in one direction.** Discrimination is negative on **6 of 6** models, sign
test **p = 0.031 two-sided** — the direction was not registered in advance for this collection, so
the two-sided figure is the right one here; the one-sided 0.016 belongs to v5, where it was. Not one
of the six claimed a single one of the sixty-nine red herrings — notes carrying figures for all
three options on a dimension with no bearing on the question. Every numeric over-claim in the entire
cohort, thirty-two of them, fell on a note the instrument files as off-pivot arithmetic; thirty-one
of those are genuinely a capacity, intake, rate, stock or threshold, and one — `longcat-2.0` on
`mill/records/guidance` — is a task-central note whose only digits are a year. See `PAPER.md` §4.3.

§4 registered the four-row outcome table for exactly this, and the row that occurred is the second:
*"dismissed / over-claimed → the cue is 'resembles the question's arithmetic', a different and much
more defensible behaviour, and H1 must be restated as that."* Recorded here as the registered
reading, not as a rescue.

Note the plain column, which is not part of any prediction: **0 of 101**. No model, at any
capability, claimed that a note without figures was load-bearing when it was not.

## the repair ladder (H2, H3)

Fifteen observations per rung — five dossiers, three independent ladders each. Where a turn was cut
the rung carries the smaller denominator. The contrasts below are **paired over the cells where both
rungs were measured**, as §3 and §7 register them, so they are not differences of these columns.

| model | carrying | again | unprompted | told-so | repaired | handle use |
| --- | --- | --- | --- | --- | --- | --- |
| `solar-pro4` | 5/15 | 5/15 | 0/15 | 1/15 | 2/15 | **27% — gated out** |
| `longcat-2.0` | 3/15 | 3/15 | 2/15 | 7/15 | 2/15 | **37% — gated out** |
| `deepseek-v4-flash` | 2/15 | 1/15 | 5/15 | 8/15 | 10/15 | 50% |
| `hy3` | 3/15 | 6/15 | 6/14 | 14/15 | 11/15 | 62% |
| `grok-4.6` | 4/15 | 3/15 | 4/15 | 9/15 | 9/15 | 67% |
| `glm-5.3-flash` | 8/15 | 6/15 | 9/15 | 10/15 | 10/14 | 60% |

**§8.3 excluded two of six models from H2 and H6** for handle use below 50%, and they are reported
as non-users rather than averaged in. `solar-pro4` is the sharper case: it made **16 edits off 6
tests**, rewriting its own context more often than it measured it, and its answers got *worse* with
the handles than without them (5/15 carrying → 0/15 unprompted). A model that edits without
measuring is a finding about giving edit authority to weak models, and is not evidence about H2.

**Correction, 2026-09-17.** P3 and P5 were first published as differences of the whole-rung
percentages in the table above. The registration reads all three contrasts as paired, and two cells
here have a cut turn, so the readings come apart on exactly those two: `hy3`'s `again` →
`unprompted` is **+7 paired** where the marginal reading gave ~0, and `glm`'s `told-so` →
`repaired` is **+7 paired** where it gave +5. The paired figures are used below. **P3's tally moves
from one of four to zero of four**; P4 and P5 are unchanged. `cargo run --release` now prints both
readings of the ladder, which is how this was found.

Among the four that cleared the gate:

**P3 — `again` → `unprompted` gains ≤ 5 points: FAILED.** Zero of four, p = 1.000. Measured:
deepseek +27, glm +20, grok +7, hy3 +7. Handing a subject the ability to inspect and ablate its own
context, with no hint that anything is wrong, **did** help — past the registered five points on all
four, and substantially on two.

**P4 — `unprompted` → `told-so` gains ≥ 40 points: FAILED at the registered threshold.** One of
four, p = 0.938. Measured: hy3 +50, grok +33, deepseek +20, glm +7. The *direction* is 4 of 4
(p = 0.062): being told helped every model that could be measured.

**P5 — `told-so` → `repaired` gains ≤ 20 points, i.e. H3 fails: HELD, 4 of 4, p = 0.062.** H3 was
registered in the expectation of failure (§10) after the first complete ladder contradicted it, and
it failed. Measured: deepseek +13, glm +7, grok 0, hy3 −20. Being asked to put the context right,
having already named the note, is worth approximately nothing. The handles were granted at
`unprompted` and never withdrawn, so this rung adds an instruction rather than a permission; see
`PAPER.md` §4.5.

## the secondary endpoints

**P6 — H4, own arm vs foreign arm: the interval half held 6 of 6, the point-estimate half 4 of 6.**
§4 registers P6 per model as |own − foreign| ≤ 10 points *with an interval containing 0*. Measured:
solar 1/5 vs 3/5 (−40), deepseek 1/5 vs 1/4 (−5), hy3 3/5 vs 3/5 (0), longcat 3/5 vs 3/5 (0), grok
3/5 vs 2/5 (+20), glm 4/5 vs 4/5 (0). Three exact ties, one pair five points apart, and two models
outside the ten-point band, one each way. Every per-model interval contains 0, and so does the
pooled one: 15/30 own against 16/29 foreign, −5 points, 95% CI −29 to +19.

§10 states H4's failure condition one-sidedly — it fails only if the own arm *beats* the foreign
arm by more than 10 points with an interval excluding 0 — and on that rule **H4 did not fail**. The
only model whose own arm led by more than ten points is grok, whose interval runs −32 to +60. The
two halves of the registration therefore do not ask the same question of this row; the conflict is
logged in `PREREGISTRATION.md` §11 rather than settled here in favour of whichever reads better.

What five items per arm supports is the absence of a *detected* own-context advantage, not
equivalence between the arms. This document previously read "there is no privileged access", which
is a claim about the world these counts cannot carry, and has been corrected rather than kept.

**Counterfactual accuracy against the majority baseline**, which bears on the thesis directly:

| model | accuracy | baseline |
| --- | --- | --- |
| `glm-5.3-flash` | 52/54 — 96% | 76% |
| `hy3` | 48/54 — 89% | 76% |
| `longcat-2.0` | 44/54 — 81% | 81% |
| `grok-4.6` | 47/54 — 87% | 69% |
| `deepseek-v4-flash` | 41/52 — 79% | 67% |
| `solar-pro4` | 39/54 — 72% | 80% |

Four of six beat their baseline, two do not. Reports about causal dependence are **not** generally
inaccurate, and any framing built on "models cannot report what their answers depend on" is
contradicted by this table. `RELATED.md` forbade that sentence before the data existed.

**Location — withdrawn. Reported here as a correction, not as a result:**

| model | items located correctly |
| --- | --- |
| `solar-pro4` | 2/18 |
| `deepseek-v4-flash` | 0/18 |
| `hy3` | 0/18 |
| `longcat-2.0` | 0/18 |
| `grok-4.6` | 0/18 |
| `glm-5.3-flash` | 0/18 |

**2 of 108.** This document originally read that as "models can say what their answer rests on but
not where it is". That reading is wrong and has been withdrawn.

`attribution` installs no handles, and the projector renders items as `label:` plus content with no
item numbers anywhere. The subjects were asked for a number they had never been shown. The majority
baseline for the row is 33% — always naming the single most common item number — and 1.9% is an
order of magnitude *below* it, which is what an unanswerable question looks like rather than a
missing ability.

nachalnik can address context items to a model (`inspect`'s `look` lists them by number); that
handle was simply not granted in this experiment. Whether a model can locate an item when it is
allowed to look is untested and open. See `PAPER.md` §4.2 and `PREREGISTRATION.md` §11.

## what the registered plan says happens now

Every confirmatory hypothesis failed except H4, which predicted a null and was not falsified: §10's
condition for sinking it was not met, though §4's per-model form of the same prediction held on four
of six models on the point estimate and six of six on the interval. `PREREGISTRATION.md` §11 records
why those are two different tests. One registered prediction produced a significant unanimous
result — P2b, in the direction opposite to the hypothesis it belonged to.

§9.1's standing rules apply: **no new hypotheses**, and the honest paper is the one the data
supports. It is not the paper this instrument was built to write, and §9.1 registered in advance
that this outcome was acceptable and that the instrument and the ground-truth method stand on their
own.
