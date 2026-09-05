# results, v5

Collected 2026-09-04, instrument **v5**, the same six models from six labs. The preregistered
amendment naming the four replication thresholds was committed to the `preregistration` branch at
**10:18:58 UTC**; the first request went out at **10:31:43 UTC**. Both timestamps are public.

v5 changes the instrument in one place: `attribution` and `lie` no longer ask what number a note
is, because the subject has no way to see the numbering and v4 scored the resulting noise as a
finding. Two digests moved (`attribution`, `lie`); five did not. Nothing else changed — hypotheses,
analysis plan, gates and the §9.1 decision table are as registered for v4.

**Total: 4,366 requests, 5.74M in / 6.75M out, $15.09.** (v4 was $9.11 for a similar request count
on the shared experiments; v5 also ran `instrumented` on three models where v4 ran it on one.
`deepseek-v4-flash`'s repair run alone produced 1.07M output tokens.)

## the four registered replication thresholds

| threshold | v4 | v5 | |
| --- | --- | --- | --- |
| P2b discrimination negative on ≥ 5 of 6 | 6/6 | **6/6** | replicates |
| red herrings claimed ≤ 2% | 0/69 | **0/66 (0%)** | replicates |
| plain inert claimed ≤ 2% | 0/101 | **1/98 (1.0%)** | replicates |
| numeric over-claims on the question's own arithmetic ≥ 90% | 32/32 | **31/31 (100%)** | replicates |

All four hold. **The probe-ordering confound did not move the endpoint.** In v4 the three location
probes ran before the counterfactual battery, so every claim was made in a context where the
subject had just invented three item numbers; §6 disclosed that and could not rule it out. With the
probes gone the pooled difference is +22 against v4's +23.

| model | v4 numeric / plain | v5 numeric / plain | v4 disc | v5 disc |
| --- | --- | --- | --- | --- |
| `solar-pro4` | 9/27 · 0/16 | 7/24 · 0/16 | −60 | −58 |
| `longcat-2.0` | 10/26 · 0/18 | 8/26 · 0/18 | −71 | −57 |
| `deepseek-v4-flash` | 4/19 · 0/16 | 4/19 · **1/16** | −44 | −44 |
| `grok-4.6` | 3/21 · 0/16 | 4/21 · 0/16 | −30 | −36 |
| `hy3` | 4/24 · 0/17 | 5/23 · 0/17 | −33 | −42 |
| `glm-5.3-flash` | 2/23 · 0/18 | 3/23 · 0/15 | −18 | −25 |
| **pooled** | 32/140 · 0/101 | 31/136 · **1/98** | +23 | +22 |

**One plain inert note was claimed**, by `deepseek-v4-flash`. v4's write-up says no model ever
claimed a note without figures was load-bearing when it was not. Across both collections that is
now **1 in 199**, and the sentence has been corrected rather than kept.

**P1 failed again** — `Surface::difference` ≥ 30 points in ≥ 5 of 6 models: 1 of 6, sign test
p = 0.984 (v4: 2 of 6, p = 0.891).

## the repair ladder

**§8.3 excludes three models rather than two.** Handle use: solar-pro4 25%, longcat-2.0 37%,
`deepseek-v4-flash` **47%**, glm-5.3-flash 58%, hy3 62%, grok-4.6 68%. deepseek cleared the same
gate at exactly 50% in v4 and now sits three points under it. The gate is pre-specified and applied
as written, so P3–P5 are read over three models here and four in v4.

| model | carrying | again | unprompted | told-so | repaired | handles |
| --- | --- | --- | --- | --- | --- | --- |
| `solar-pro4` | 6/15 | 6/15 | 0/15 | 3/15 | 0/15 | **25% — gated out** |
| `longcat-2.0` | 3/15 | 2/15 | 2/15 | 3/14 | 3/15 | **37% — gated out** |
| `deepseek-v4-flash` | 3/15 | 1/15 | 1/15 | 7/14 | 8/14 | **47% — gated out** |
| `glm-5.3-flash` | 9/15 | 7/15 | 8/15 | 10/15 | 14/15 | 58% |
| `hy3` | 6/15 | 7/15 | 11/15 | 13/15 | 13/15 | 62% |
| `grok-4.6` | 5/15 | 5/15 | 5/15 | 9/15 | 9/15 | 68% |

**P3 — `again` → `unprompted` gains ≤ 5 points: FAILED**, 1 of 3 (grok 0, glm +7, hy3 +27). As in
v4, handing a subject the ability to inspect and ablate its own context, with no hint that anything
is wrong, did help — and on hy3 it helped a great deal.

**P4 — `unprompted` → `told-so` gains ≥ 40 points: FAILED at the registered threshold**, 0 of 3
(grok +27, hy3 +13, glm +13). Direction positive 3 of 3, as in v4.

**P5 — `told-so` → `repaired` gains ≤ 20 points: HELD in 2 of 3, not unanimously.** hy3 0, grok 0,
**glm +27**. This is the one place v5 differs from v4 in kind rather than degree: v4 found being
allowed to remove the false note, having already named it, worth approximately nothing on 4 of 4
models, and here one model gained 10/15 → 14/15 from it. n = 15 on one model, so this is a
counterexample and not a reversal — but v4's flat reading should not be repeated as though it were
unqualified.

## H4 — privileged access

**P6 — own arm vs foreign arm within 10 points: HELD.**

| model | own | foreign |
| --- | --- | --- |
| `solar-pro4` | 1/5 | 3/5 |
| `deepseek-v4-flash` | 3/5 | 3/5 |
| `hy3` | 4/5 | 3/5 |
| `longcat-2.0` | 3/5 | 5/5 |
| `grok-4.6` | 3/5 | 3/5 |
| `glm-5.3-flash` | 5/5 | 5/5 |
| **pooled** | **19/30** | **22/30** |

Three exact ties, two in favour of the foreign arm, one in favour of the own arm. Reasoning about
your own context is no better than reading a transcript of somebody else's, and pooled it is
slightly worse. Five items per arm per model, so the intervals are very wide; the finding is the
absence of an advantage, not its sign.

## harness validation (no prediction attaches)

Re-run on 2026-09-04 after the key limit was raised. The registered purpose was to confirm the
ablation handle returns what it should on more than one model family. It does: **98%, 100% and
100%** of questions instrumented, 353 tests, no edits.

| model | reported | retested | tested | reported→retested | reported→tested | handle use |
| --- | --- | --- | --- | --- | --- | --- |
| `glm-5.3-flash` | 45/53 | 44/53 | 48/54 | −2, p = 0.73 | +4, p = 0.38 | 98% |
| `hy3` | 43/54 | 45/54 | 49/54 | +4, p = 0.36 | +11, p = 0.09 | 100% |
| `grok-4.6` | 36/54 | 39/54 | 47/54 | +6, p = 0.29 | **+20, p = 0.013** | 100% |

Three of three in the same direction on reported→tested; reported→retested is two up and one down,
none significant. The v4 stage-1 `glm-5.3-flash` run had the same shape. **`tested` differs from
`reported` in two ways at once** — it has the instrument and it never committed to an answer — so
nothing here separates them. §9.1 forbids new hypotheses after collection, and this is not one: it
is a pilot, recorded with its confound named.

## what did not get measured

`instrumented` first stopped on `402: This request requires more credits` — a **$25 total limit on
the API key**, at $24.99 spent, with the provider reserving `max_tokens` worth of credit up front
rather than charging actual usage. The limit was raised and the three runs completed the same day;
see the section above. Nothing else registered went unmeasured.
