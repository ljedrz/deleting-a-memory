# results

Collected 2026-09-03/04, instrument **v4**, six models from six labs. Every figure here comes
from `eval-runs/`, and every one is recomputable from the saved reports with
`cargo run --example pool`.

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
test **p = 0.016**. Not one of the six claimed a single one of the sixty-nine red herrings — notes
carrying figures for all three options on a dimension with no bearing on the question. Every numeric
over-claim in the entire cohort, thirty-two of them, fell on a note belonging to the question's own
arithmetic that did not decide it.

§4 registered the four-row outcome table for exactly this, and the row that occurred is the second:
*"dismissed / over-claimed → the cue is 'resembles the question's arithmetic', a different and much
more defensible behaviour, and H1 must be restated as that."* Recorded here as the registered
reading, not as a rescue.

Note the plain column, which is not part of any prediction: **0 of 101**. No model, at any
capability, claimed that a note without figures was load-bearing when it was not.

## the repair ladder (H2, H3)

Fifteen observations per rung — five dossiers, three independent ladders each.

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

Among the four that cleared the gate:

**P3 — `again` → `unprompted` gains ≤ 5 points: FAILED.** One of four, p = 0.938. Measured:
deepseek +27, glm +20, grok +7, hy3 ~0. Handing a subject the ability to inspect and ablate its own
context, with no hint that anything is wrong, **did** help two of the four.

**P4 — `unprompted` → `told-so` gains ≥ 40 points: FAILED at the registered threshold.** One of
four, p = 0.938. Measured: hy3 +50, grok +33, deepseek +20, glm +7. The *direction* is 4 of 4
(p = 0.062): being told helped every model that could be measured.

**P5 — `told-so` → `repaired` gains ≤ 20 points, i.e. H3 fails: HELD, 4 of 4, p = 0.062.** H3 was
registered in the expectation of failure (§10) after the first complete ladder contradicted it, and
it failed. Measured: deepseek +13, glm +5, grok 0, hy3 −20. Being allowed to remove the false note,
having already named it, is worth approximately nothing.

## the secondary endpoints

**P6 — H4, own arm vs foreign arm within 10 points: HELD.** Reasoning about your own context scored
the same as reasoning about a transcript of someone else's, on every model: solar 1/5 vs 3/5,
deepseek 1/5 vs 1/4, hy3 3/5 vs 3/5, longcat 3/5 vs 3/5, grok 3/5 vs 2/5, glm 4/5 vs 4/5. Three
exact ties, one pair five points apart, one each way. There is no privileged access.

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

Every confirmatory hypothesis failed except H4, which predicted a null and got one. One registered
prediction produced a significant unanimous result — P2b, in the direction opposite to the
hypothesis it belonged to.

§9.1's standing rules apply: **no new hypotheses**, and the honest paper is the one the data
supports. It is not the paper this instrument was built to write, and §9.1 registered in advance
that this outcome was acceptable and that the instrument and the ground-truth method stand on their
own.
