# runbook

What to run, in what order, and what each thing costs. `METHODOLOGY.md` says why; this says how.

---

## the argument the runs are for

> Self-knowledge in language models is not a retrieval problem. It is an instrumentation problem.

Three conditions on the *same* question. The first is all any other harness can do; the second and
third need a context that can be snapshotted, ablated and rewritten.

| | condition | experiment | what it answers |
| --- | --- | --- | --- |
| **C1** | *report* — "what does your answer rest on?" | `attribution`, `privilege`, `lie`, `recursion`, `feedback` | is a model's account of its own causal structure true? |
| **C2** | *test* — the same question, with a handle that forks its context | `instrumented` | does it reach for evidence, and does evidence beat its theory? |
| **C3** | *repair* — the same, plus the ability to change what it finds | `repair` | does being able to fix a context produce a better **answer**? |

The headline number is the accuracy difference between `instrumented`'s three stages, and the
headline outcome is `repair`'s three task answers. Everything else is the C1 baseline that makes
them mean something.

---

## running it

```console
$ export NACHALNIK_API_KEY=sk-or-...          # OpenRouter, or any OpenAI-compatible key
$ export NACHALNIK_BASE_URL=https://openrouter.ai/api/v1
$ cargo run -p nachalnik-eval --example bench -- -m <model> -r 3 --json out.json
```

Google AI Studio speaks the same dialect; its base URL is
`https://generativelanguage.googleapis.com/v1beta/openai`. A local ollama does too, and costs
nothing: `http://localhost:11434/v1` with any key at all.

Useful flags: `-e <name>` to run one experiment (repeatable), `-r N` for replicates, `--swap` for
`privilege` with its dossiers counterbalanced, `--temperature T` (default 0).

Then tabulate:

```console
$ cargo run -p nachalnik-eval --example compare -- eval-runs/*/*/report.json
```

`compare` groups runs by instrument digest and refuses to put two instruments in one table without
saying so.

### what a run costs

Per model, at `-r 3`, measured on `gemini-3.7-flash`:

| | requests | input tokens |
| --- | --- | --- |
| the five C1 experiments, `-r 3` | ~181 | ~116k |
| `instrumented`, v4 default (6 dossiers, 54 items, `-r 1`) | ~700 | ~500k |
| `instrumented`, one dossier (`-e instrumented`, 7 items) | ~70 | ~45k |
| `repair`, v4 default (5 dossiers, 5 rungs, 3 ladders each) | ~222 | ~530k |
| `repair`, one ladder per dossier (`-l 1`, a probe) | **74 measured** | 177k in / 384k out |
| `attribution`, v4 default (6 dossiers, 54 notes — the H1 endpoint, no tools) | ~264 | ~190k |
| `attribution`, one dossier (`-e attribution` after `.on()`, a probe) | **44 measured** | 32k in / 96k out |

**`repair -l 1` measured 74 requests, not the ~35 the probes suggested.** Tool-call turns are
requests too, and a subject that looks, tests three times and then edits spends five requests on
one rung. Multiply by three ladders for the default. Every earlier figure in this table came from
runs that died partway and undercounted for that reason.

The ladder is an order of magnitude more expensive than it was at v2, and deliberately: v2 asked
four items per stage, which puts a 3/4 result between 30% and 95% and cannot distinguish any
hypothesis from chance. `PREREGISTRATION.md` §6 has the arithmetic. Budget roughly **£1-15 per
model** depending on the model, and run the one-dossier probe first.

**Watch the `cut` column.** A figure there means a turn ran out of room before the subject
answered anything: those claims are untested rather than wrong, and the run should be repeated with
a larger `--max-tokens`. The default is 32,768 and it needs to clear the *thinking* - measured,
`deepseek/deepseek-v4-flash-0731` spent 15,374 reasoning tokens on a single question about
repairing its own context.

Output is small in the `out` column and **not** small in reality: a reasoning model's thinking is
billed as output and Google's shim reports it only as the residual of `total_tokens` (see the
`usage_of` note in `nachalnik-utils`). Budget for the output side being an order of magnitude above
what the `out` column says.

### saving results

```text
eval-runs/<model>/<UTC start>-<what>-r<N>/
    meta.json     model, endpoint, replicates, params, commit
    summary.txt   what the runner printed
    report.json   the whole record: every question, every answer verbatim, every comparison
```

Untracked on purpose, and in `.git/info/exclude` so a stray `git add -A` cannot sweep them in.
`git add -f eval-runs` when you decide they should be in the repository. **No API key goes in
there.**

---

## the order to run things in

> **v4.** The primary endpoint is now H1, which lives in C1 and needs no tools:
> `Surface::difference` over items that provably do nothing. `PREREGISTRATION.md` §3 has the
> hypothesis set, §11 has why it changed. Section 12 is closed, so confirmatory collection may
> begin.

1. **One cheap probe first.** `-e repair -l 1` is ~74 requests and exercises both handles. Read
   `handles:` in the output: below 50% the model is not instrumenting, the preregistered gate
   (§8.3) takes it out of H2 and H6, and everything there is measuring a model that does not use
   tools. H1 is unaffected — it asks nothing of the handles.
2. **`attribution`, per model, `-r 3`.** The home of the primary endpoint: **six dossiers**, 54
   notes, no tools. Read the `surface:` line, and read the bracketed split beside it before
   reading the difference — see P2b. One dossier is a probe and nothing more: measured on
   `deepseek/deepseek-v4-flash-0731`, `depot` alone left seven inert items and an interval a
   hundred points wide.
3. **The rest of the C1 baseline**, per model, `-r 3`: `recursion`, `lie`, `privilege`, `feedback`.
   `privilege` carries H4 and `lie` carries the calibration prediction.
4. **`repair`**, per model, at the default three ladders per dossier — `-r` is the ablation copy
   count and does not touch it. Fifteen task answers per rung. Read the rungs as a row, not as an
   accuracy: `carrying` → `again` → `unprompted` → `told-so` → `repaired`, and read the `paired:`
   lines rather than the rung accuracies. **`again` → `unprompted` is H2's endpoint** and a null
   there is the prediction; a gain invalidates everything above it and would be the better result.
5. **`instrumented`**, per model, at the default `-r 1`. The expensive one, and no longer the
   headline: six dossiers, 54 items a stage, twelve sessions. It is a check that the handles work.
   Run it last, and drop it first if the budget bites.
6. **`privilege --swap`**, on at least two models, once the rest is in. The counterbalance.

Models worth having, in rough order of value: one frontier model, one mid-tier, one small
open-weights model, and one older generation of the same family as the frontier one. The
generational series is what shows whether the findings are about *models* or about *a* model —
measured so far, `gemini-3.8`, `3.7` and `3.5-flash` agree claim for claim, and a 3B model fails
the manipulation checks entirely. **Every figure behind H1 comes from three models that were run
because they were to hand**, before the hypothesis existed and on material that could not falsify
it — which is why the six registered models are all new to the question, and why re-running those
three at v4 buys nothing one of the six does not buy better.

---

## the staging order

One model at a time, cheapest first. Two reasons and the second is the real one: a defect found on
a cheap model costs cents to rediscover, and a run that dies halfway through a sweep leaves the
whole sweep un-analysable. Every stage below is a complete model, saved before the next begins.

**Re-costed 2026-09-03 against a measured run, and the figures went up.** `repair -l 1` on
`deepseek/deepseek-v4-flash-0731` came in at **74 requests, 177,251 in and 384,363 out** — a shape
of 2,395 in and 5,194 out per request, against the 1,921/3,144 the earlier table assumed. Output is
65% higher than estimated because reasoning is billed as output and does not appear in the answer.
Every dollar figure below is that measured shape applied to the old per-request price, and should
be read as a **lower bound** until stage 1 confirms it.

| what | requests per model | measured? |
| --- | --- | --- |
| the C1 five, `-r 3` (`attribution` over six dossiers carries H1) | ~420 | 44 × 6 measured, rest estimated |
| `repair`, 3 ladders (H2, H3) | ~222 | 74 × 3, measured |
| `instrumented`, 6 dossiers (H6, a harness check) | ~700 | estimated |

### plan A — the hypotheses, without the harness check

C1 five plus `repair`, per model, derived from two measured runs rather than estimated:

| | requests | input | output |
| --- | --- | --- | --- |
| `attribution`, six dossiers, `-r 3` (**H1**) | 264 | 193k | 575k |
| `recursion`, `lie`, `privilege`, `feedback`, `-r 3` | ~166 | ~121k | ~361k |
| `repair`, three ladders (**H2**, H3) | 222 | 532k | 1,153k |
| **per model** | **~652** | **~846k** | **~2,089k** |

`attribution` is 40% of the requests and 20% of the tokens: no tools, no ladder, short answers. That
is why the primary endpoint is the cheap half of the plan.

| # | stage | ~cost |
| --- | --- | --- |
| 1 | `upstage/solar-pro4` | ~$0.03 |
| 2 | `deepseek/deepseek-v4-flash-0731` | ~$0.44 |
| 3 | `z-ai/glm-5.3-flash` | ~$0.59 |
| 4 | `tencent/hy3` | ~$1.22 |
| 5 | `meituan/longcat-2.0` | ~$2.76 |
| 6 | `x-ai/grok-4.6` | ~$14.23 |

**~$19.5, plus ~$1 for `instrumented` on the two cheapest models** as the harness check it now is —
call it **~$21 for six models**. Two of the six were swapped on 2026-09-03: `mercury` for
`solar-pro4` because mercury cannot be run, and `claude-sonnet-5` for `grok-4.6` on cost. Both are
logged in `PREREGISTRATION.md` §11 with their grounds, and neither model had been run.

The two secondary arms — `x-ai/grok-4.6` and the `moonshotai/kimi-k2` generational series — are
**cut**, not deferred. They were scoped for the old hypothesis set, and six models across five labs
answers "about models or about a model" better than three generations of one model does.

Stage 1 is `inception/mercury-2.5-preview` and it is also the re-cost: it is the cheapest model,
it is architecturally the odd one out (a diffusion LM, so its token shape is nobody else's), and
if it comes in more than 50% over this table, stop and re-derive before stage 2.

### plan B — everything on everybody

~1,120 requests per model: the frontier arm alone comes to **~$65** and the sweep to **~$90**. That
is $58 of extra spend to run a demoted endpoint on six models instead of two, and the demoted
endpoint is the one whose tool prints the scored answer. **Not recommended**, and written down so
the choice is visible rather than implied.

**Six models is the registered cohort and it is enough.** §12.1 froze six before any of this, and
the arithmetic has not changed: six unanimous is a sign test at `(1/2)^6 = 0.016`, which carries a
model-level claim. More models would buy a smaller p-value on a claim that is already significant,
and that is not what this budget is short of.

Re-cost after stage 1 rather than trusting the table. **Stop and re-cost if stage 1 comes in more
than 50% over**, and run stage 6 last regardless of what the order says elsewhere.

Within a model: `attribution` first (it is now the primary endpoint and the cheapest thing here),
then `repair`, then the rest of the C1 five, then `instrumented` if it is being run at all.

## reading a report

Read in this order, and stop at the first thing that is wrong:

1. **The unmet checks**, printed above the scores. A subject whose material moves nothing has
   measured nothing about its self-knowledge, however confident its claims were.
2. **`the session and a copy of it`** in the notes. Where they disagree, the ablations are still
   sound - copies against copies - but the subject's own answer was reached some other way.
3. **`guessing would get`** beside every accuracy. A battery where nothing moves is one where
   "no" scores a hundred percent.
4. **The interval and the p-value.** n is in the tens; most single-experiment figures will not
   reach significance, and that is the honest result rather than a defect.
5. **Then the figures.**

---

## state of play, 2026-09-03

Where a fresh reader (or a fresh context) picks this up.

**The thesis moved, twice, and both times because something was checked.** It is now: *a model's
account of what its answer depends on reads the surface of its context rather than the structure,
and that is demonstrable rather than arguable because a runtime treating context as state can
delete the item and run the world again.* `RELATED.md` has the full wording and the "must not say"
list; `PREREGISTRATION.md` §3 has the hypotheses and §11 is the dated log of what
changed and why.

**What the old primary was.** "A subject that can ablate its own context makes better claims about
its own dependence than one reporting from the prompt." The test tool replies
`without [9]: omsk becomes kirov, moved: true`, and `moved` *is* the scored claim — so it measured
whether a model can copy a field out of a tool result. It is demoted to H6, a check that the
handles work, and kept because that is a real thing to show.

**What the new primaries are.**

- **H1, the surface cue.** Among items whose ablation *provably does not move the answer*, does the
  subject claim the ones carrying figures matter more often than the ones that do not? Pilot
  reanalysis: 34 independent cells over three models of one family, the claim predicted 94% of the
  time by "does the note carry a number of two or more digits", against 76% for the claim
  predicting the truth. Subjects claimed 16 of 18 numeric notes mattered and **0 of 16** plain ones.
  Every error a false positive; no false negatives at all.
- **H2, measuring without updating.** First complete `repair` ladder, five dossiers on
  `deepseek/deepseek-v4-flash-0731`: `carrying` 1/5 → `again` 0/5 → `unprompted` **0/5** →
  `told-so` 3/5 → `repaired` 3/4, at a 75% instrumentation rate with 18 tests and 12 looks. Handed
  the tools and no hint, it measured its own context and answered wrongly five times out of five.
  Told a note was false, it recovered three of five **with no edit at all** — the act log confirms
  every edit falls at `repaired` and none at `told-so`.

**Instrument v4 is what made H1 testable, and it broke comparability.** Every inert note used to
carry three digits or fewer, so the cue and the truth were confounded across all six dossiers. Two
numeric red herrings per dossier now break that; one was too few (the figures shortcut scored 0.83,
beating the subjects' own 0.76). Every digest moved, the five report-only experiments included, so
**no v4 figure is comparable with a v3 or v2 one on any experiment.**

> **STAGE 1 ABORTED, 2026-09-03: `inception/mercury-2.5-preview` cannot complete a run.**
> Every experiment dies partway with an HTTP **502** from Inception whose body is a canned refusal
> — *"I'm sorry, but I can't share details of my architecture or training process"* — which no
> question in the suite asks for. Characterised over six probes, ~$0.01 all told:
>
> | run | requests | outcome |
> | --- | --- | --- |
> | `attribution -r 3` | 17 | died |
> | `attribution -r 3` again | 17 | died, identically |
> | `recursion -r 3` | 16 | died, after filing 2 resolutions |
> | `recursion -r 1` | 14 | **completed, 6/6, p=0.02** |
> | `recursion -r 1` again, back to back | 14 | **completed, 6/6** |
>
> So: not content (it dies in three different experiments, and completed several ablation copies
> before going); not cumulative (two 14-request runs back to back both passed); not concurrency
> (`Ablation` issues replicates sequentially — `for _ in 0..self.replicates`). What is left is a
> short-window request or token ceiling somewhere around fifteen, on the provider's side, which
> resets in about a minute. The refusal text appears to be Inception's canned response when their
> gateway cuts a caller off, and is a red herring.
>
> **Mercury is a capable subject when it runs**: 6/6 on `recursion`, twice, against a 50% baseline.
> The decision about whether to pace it, drop it, or replace it is open and is the user's — see the
> state of play below. **No further collection until it is settled.**

> **HOLD LIFTED 2026-09-03.** Stage 1 ran on `z-ai/glm-5.3-flash` (mercury could not be run; see
> above) and the plan was reviewed against its figures. The remaining five models are authorised
> and running: `solar-pro4`, `deepseek-v4-flash-0731`, `hy3`, `longcat-2.0`, `grok-4.6`, three
> steps each.
>
> **`instrumented` is dropped from the sweep.** It carries H6, which is a check that the handle
> returns what it should rather than a hypothesis about models, and glm already answered it: 100%
> instrumentation over 108 questions, and `reported` → `retested` at **+0 points**. One harness
> check is a harness check; five would be paying frontier prices for a tool that prints the scored
> answer. glm's run stands as the record that the handles work.
>
> Every figure from stage 1 went against every registered hypothesis. Per §9.1 rows C, D and E
> that changes nothing about the plan, and the plan was not changed.

**The immediate next action:** `-e attribution -r 3` on the cheapest model. It is the primary
endpoint, one dossier, nine notes, no tools, and it costs pennies. Read the `surface:` line. Then
stage 1 of plan A.

**Standing findings worth not rediscovering.**

- Reasoning tokens are the bulk of the bill and appear in no answer. Measured 2,395 in / 5,194 out
  per request, against the 1,921/3,144 that was assumed; cost estimates that assume otherwise come
  in 40–65% low.
- Tool-call turns are requests. `repair -l 1` is 74 requests, not the ~35 the rung count suggests.
- Every defect found against a live model was found by a probe costing under a cent, and none by a
  test. The ones found offline were all found by *building the next thing*: the brief that
  contradicted the handles, the grant that outlived its session, the fixture rule table that
  silently stopped testing new notes, and the material that could not falsify its own hypothesis.
- Two reviews by another model found the tautology at the heart of the old primary and the
  confound in the material. Neither was found by any test here.

**Credentials** are not in this repository and must not be. The runner reads `NACHALNIK_API_KEY`
and `NACHALNIK_BASE_URL` from the environment.

## what is not done

- **`privilege --swap` has never been run.** The counterbalance exists and is unused, so the
  foreign arm's presentation confound is bounded by argument rather than by data. See
  `METHODOLOGY.md` §9.
- **Option order is not counterbalanced.** The three answers appear in a fixed order in every
  probe. The record keeps the exact question asked, so a later run with shuffled options is a
  direct A/B against everything already recorded.
- **The instrumented stages are ordered, not randomised.** `reported` always comes before
  `retested` on the same subject. `tested` is the fresh-subject arm that exists to bound that
  order effect; a fully randomised design would need two more subjects per run.
- **`mill` has not been read out on its own.** The falsification dossier ran in both collections'
  `attribution` as one of the six, but the dissociation it was built to show — a subject naming the
  buried correction as decisive while the ablation names the salience note — has not been tabulated
  separately from the pooled endpoint. Whether it appeared is in `report.json` and unread.
