# related work

What other people have already shown, what that leaves for us, and — the part that matters most —
what we are no longer entitled to say. Checked 2026-09-03. Anything published after that date is
not in here, and this file should be re-checked before the paper is submitted anywhere.

The short version: **the C1 rung is largely anticipated, and one of the three sentences in our
working thesis belongs to somebody else.** C2 and C3 appear to be open. The positioning below is
what survives.

---

## 1. self-report about context dependence — anticipated

| work | what it did |
| --- | --- |
| [Evaluating and Improving LLM Self-Modeling](https://arxiv.org/abs/2608.30980) (2026-08) | nine self-modeling formats, **including "would this prompt edit change your answer?" and "which prompt component was influential?"**; ground truth by sampling under original and modified prompts; a *self-modeling skill* score measured against a dummy predictor that uses only base rates |
| [Me, Myself, and π: Evaluating and Explaining LLM Introspection](https://arxiv.org/abs/2603.20276) (2026-03) | IntrospectBench; defines *policy-introspection*; runs a **third-party control** — other frontier models predicting the same model — and reports a significant self-advantage (p = 0.021) |
| [Looking Inward](https://arxiv.org/abs/2410.13787) (2024-10) | fine-tuned self-prediction beats a stronger model trained on the same behaviour; fails on complex and out-of-distribution tasks |
| [Language Models Fail to Introspect About Their Knowledge of Language](https://arxiv.org/abs/2503.07513) | a clean negative result in a narrow domain |
| [Evidence for Limited Metacognition in LLMs](https://arxiv.org/abs/2509.21545) | confidence calibration and knowledge-boundary probing; models overestimate what they know |

This is our `attribution`, `counterfactual` and `location` families, our copy-vs-copy ground truth,
and our `skill`-over-baseline metric — arrived at independently, published first, one month before
this file. **We cannot present C1 as a new benchmark.** Two things about our C1 remain worth
reporting, and only these two:

- **`location`.** We ask *where* an item is, by the address the runtime actually uses. Nobody above
  asks a model for a handle to its own context. Our pilot is 12/12 wrong in the same way — the
  note's ordinal, never the item id — which is a claim about the *interface*, not about accuracy.
- **A different third-person control.** IntrospectBench asks whether model M beats model M′ at
  predicting M. We ask whether M reasoning about *its own* context beats M reasoning about a
  *transcript of someone else's*, holding the model fixed. That isolates the possession of the
  context rather than the identity of the predictor, it is cheaper, and our pilot finds **no
  advantage at all** (own 3/5 = foreign 3/5, identical claim for claim, three models) where they
  find a small significant one. A real tension, on different tasks, worth stating carefully and
  not overselling.

## 2. "models cannot perceive their own context" — concurrent work, on a different variable

[VISTA: LLM Agents Are Latent Context Managers](https://arxiv.org/abs/2606.30005) (2026-06) states as
its first contribution that it identifies "context proprioception as the missing interface, since
LLM agents cannot read their own context state from the prompt", gives the agent a dashboard of
per-block metadata plus archive/recover tools, and shows the interface closes the gap.

That is close to the general shape of the idea this work started from, arrived at independently and
found during the literature review rather than built on. **Concurrent work, cited as such** — which
is the ordinary situation when two people notice the same gap, and neither an embarrassment nor a
claim to be staked. Priority over the general observation is not what either paper should turn on.
What distinguishes them is substantive, and it is which variable is invisible:

- VISTA's gap is **magnitude**: token counts, budget, block sizes. Median relative error 0.43–0.84
  without the dashboard, 0.00 with it. They report explicitly that *temporal order* is recalled
  fine either way.
- Ours is **causal dependence**: which item the answer rests on. A dashboard does not show that.
  No amount of metadata display answers "would I have said something else without note 4" — that
  question can only be answered by running the counterfactual.

So: VISTA makes context *legible*. This work makes it *testable*. Reading a dashboard is
perception; forking a context, removing a named item and comparing two answers is experiment — and
only the second produces ground truth about what an answer depended on. That is the gap this
occupies, and it is a difference in kind rather than in degree: no quantity of metadata display
answers a counterfactual, because a counterfactual has to be *run*.

## 3. self-editing context — anticipated for budget and for security, open for task repair

| work | what the edit is for |
| --- | --- |
| [Active Context Compression](https://arxiv.org/abs/2601.07190) (2026-01) | the agent consolidates a trajectory and deletes the raw logs; measured on token savings at equal accuracy |
| [Less Context, Better Agents](https://arxiv.org/abs/2606.10209) (2026-06) | pruning and summarization configurations for long-horizon tool use |
| [MemSecBench](https://arxiv.org/abs/2607.27080) (2026-07) | the agent runs a "neutral security self-check" and deletes poisoned memory; scored on whether the malicious semantics survive (86.3%) **and** whether benign memory is preserved (62.5% pass both) |

MemSecBench is the closest thing to our `repair` rung and the difference is the whole point:

- It scores the **state of the memory**, not the **answer to the task**. Ours scores the task.
- Its agent is never told which item caused the behaviour, and is never asked. Ours is asked to
  name it, on the record, *before* it is allowed to change anything.

That ordering is what produces the result we care about, and nobody above can produce it: the
subject **names the false note correctly and still answers wrongly**, until it is given the ability
to remove it. Identification and correction come apart, and the gap between them is measured in
task accuracy rather than in security state.

*[2026-09-06: the data went the other way. Told that one of its notes was false, every model that
cleared the instrumentation gate answered correctly at that rung, before it was allowed to remove
anything; removal then added little in six of seven measurements and a great deal in the seventh.
`PAPER.md` §4.5 carries the reading the record supports.]*

## 4. self-correction — the debate our C3 speaks to

[LLMs Cannot Self-Correct Reasoning Yet](https://arxiv.org/abs/2310.01798) defines intrinsic
self-correction and finds it does not work; performance sometimes degrades. There is a live
counter-literature. Our `told-so` stage reproduces the negative finding and adds a mechanism: the
reason saying-what-is-wrong changes nothing is that the wrong thing is **still in the window**.
`repaired` is the same subject, the same knowledge, one context edit later, and it is right. That
reframes a failure of reasoning as a missing capability of the runtime, which is a contribution to
that debate rather than another data point in it.

*[2026-09-06: not what was measured. `told-so` is where the answers came right, with the false note
still in the window; `repaired` mostly added nothing. Tools with no hint helped unevenly. See
`PAPER.md` §4.5 and §8.]*

## 5. evidence against prior belief — the outward version exists, the inward one does not

The knowledge-conflict literature ([survey](https://arxiv.org/abs/2403.08319),
[Adaptive Chameleon or Stubborn Sloth](https://arxiv.org/abs/2305.13300), and much since) measures
what happens when **external** evidence contradicts a model's parametric prior. Findings are
mixed-to-stubborn: strong models over-trust their weights and show confirmation bias.

Our **deference** measure is that question turned inward. The evidence is not external and not
retrieved: the model generated it, by running an experiment on itself, seconds after stating the
theory it contradicts. As far as this search goes, nobody has measured that. It is the most
distinctive single quantity in the crate and it needs to be foregrounded rather than buried under
accuracy.

## 6. introspection proper, and the confound that saves us

[Emergent Introspective Awareness in LLMs](https://arxiv.org/abs/2601.01828) (Lindsey, Anthropic)
injects concept vectors and asks the model to notice; Claude Opus 4/4.1 do best, and the capability
looks emergent rather than trained. This is introspection about *activations* and is a different
layer from ours; it is the strongest positive result in the field and should be cited as the
contrast case, not as a competitor.

The methodological critique attached to it matters more to us:
[tests of LLM introspection need to rule out causal bypassing](https://www.lesswrong.com/posts/LD8yupMtE6btAE3R9/tests-of-llm-introspection-need-to-rule-out-causal-bypassing)
— a model can report a state accurately by a path that never routes through the state. Applied to
us: a subject can predict an ablation correctly by reasoning about the *task* ("the Omsk annex is
the decisive record, so removing it must change the answer") with no self-access whatsoever.

**Our `privilege` arm is a bypass detector, and the pilot says the bypass is what is happening.**
Own-context and foreign-context prediction agree claim for claim. That is not a defect in the
experiment; it is the experiment working, and it is the argument for the rest of the paper. If
introspective reports about context dependence are task reasoning wearing a first-person pronoun,
then trying to train better introspection is solving the wrong problem, and the right move is to
stop asking and start measuring.

## 7. methods we should be following and mostly are not yet

- [Adding Error Bars to Evals](https://arxiv.org/abs/2411.00640) (Miller, 2024) — the standard we
  are held to. Treat items as a sample from a super-population; **cluster-adjusted standard errors
  can be 3× the naive ones**; use paired analysis for paired comparisons; plan the sample size
  before running. Our Wilson intervals assume independent items drawn from one dossier, which is
  exactly the case the paper warns about.
- [Questionable practices in machine learning](https://arxiv.org/abs/2407.12220) — the taxonomy our
  process is trying to avoid. Its central point is that most of these practices are innocuous
  *before* looking at the test data and only become questionable afterwards, so preregistration is
  the only way to certify which side of the line you were on.
- [Science of AI Evaluation Requires Item-level Benchmark Data](https://arxiv.org/abs/2604.03244) —
  release per-item records, not aggregates. `report.json` already does this, which is one thing we
  got right early.

---

## what is left, precisely

Revised 2026-09-03, after one round of external review and a reanalysis of the pilots. Four claims,
in descending order of confidence that they are unclaimed:

1. **Confabulation about one's own context, with ground truth.** A subject's account of what its
   answer depends on, scored against what the answer *actually* depended on — established by
   removing the named item from a live session and running it again. This is Nisbett and Wilson's
   *Telling More Than We Can Know* (1977) with the hole in it filled: in human work you can never
   rerun a person with one memory deleted, so the stated reason can be doubted but never
   disproved. Here it can. The runtime is what makes the counterfactual available, and that is the
   whole of its role in this claim.
2. **The cue the report is actually reading.** Not "reports are inaccurate" — benchmarked elsewhere
   — but *what they track instead*. Measured on pilots: whether the note carries a number of two or
   more digits predicts the claim 94% of the time, against 76% for the claim predicting the truth,
   with the errors all false positives on numeric notes. A surface feature of the text, computable
   without running the model, that explains the model's self-report better than the model explains
   its own behaviour.
3. **Measurement without belief revision.** A subject handed the ability to inspect and ablate its
   own context, with no hint that anything is wrong, that uses the ability and then answers as
   though it had not. Distinct from sycophancy (deferring to *user* authority) and from anchoring
   (sticking to prior *context*): both sides here are self-generated, the claim and the refutation
   alike. Measured 0 of 5 at a 75% instrumentation rate on the first complete ladder.
4. **The naming/fixing dissociation**, kept and expected to fail. Correct identification of the
   offending item, scored separately from the task answer, with removal granted afterwards. The
   first complete ladder had the subject recover 3 of 5 from the disclosure alone with no edit, so
   the interesting dissociation is not between naming and fixing but between *measuring* and
   *being told*.

Removed from this list: **the digest-pinned instrument**. Hash-pinning prompt wording is
engineering hygiene rather than a contribution, as an external reviewer said plainly, and it is
reclassified as a methods detail. It stays load-bearing for replication and claims nothing.

## the thesis, revised to fit

The v3 version put the weight on a capability — give models handles and they do better — and the
endpoint that carried it was a tautology. This one puts the weight on a limit:

> A model's account of what its answer depends on is not self-knowledge but task reasoning in the
> first person, and it reads the surface of its context rather than the structure: a note full of
> figures is claimed as a reason whether or not it is one. This is demonstrable rather than
> arguable, because a runtime that treats context as addressable state can delete the item and run
> the world again — the counterfactual no experiment on people has ever had. And the failure
> survives instrumentation: a subject given the same primitives, that runs the ablation itself and
> reads the result, answers as though it had not. It updates when it is *told*, not when it
> *measures*.

Note what changed and why. The claim is no longer "models cannot report" (too strong, and
benchmarked elsewhere), no longer "give them an interface" (which is perception, and which VISTA
covers concurrently for a different variable), and no longer "the handles make it better" (true,
uninteresting, and measured by a tool that prints the answer). It is: **the report is a reading of
the wrong thing, and here is the ground truth that proves it.**

## what we must not say

- Not "the first benchmark of LLM self-knowledge about context." It is not.
- Not "models cannot report what their answers depend on." Our own pilot has them at 76% against
  the truth with perfect recall. The defensible statements are *surface-driven* (94% predicted by
  digit count), *unprivileged* (own = foreign), *unaddressed* (location 12/12 wrong in the same
  way), and *miscalibrated where it matters* (`lie`: 1/4 right at ~95–100% confidence).
- Not "we identify context proprioception as the missing interface" as though nobody else had
  looked: VISTA states that contribution for magnitude, concurrently and independently, and is
  cited as related work. The claim to make is the one about *causal dependence*, which a dashboard
  cannot show at any resolution, and about ground truth, which only a counterfactual produces.
- Not "models can self-repair poisoned context." MemSecBench measured that first, in the security
  framing — and our own ladder says they largely do not until told.
- Not "instrumentation improves self-knowledge" from the `instrumented` result. The tool returns
  the scored answer; that number is a check on the harness.
- Not any claim of privileged self-access from a C1 result until the bypass control is reported
  alongside it. The bypass is the default explanation and our own data supports it.
- Not "the digit-count effect is established." 34 cells, three models, one family, and the
  predictor was chosen by fitting six candidates to those cells. It is the registered hypothesis,
  not the finding.
