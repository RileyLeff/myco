# Audit Report — §25 Training Emission

Audited against corpus as of 2026-04-22.

§25 is 28 lines (spec_new.md lines 4016-4043). The prose commits four
mechanisms (warm-start, projection flavor, per-residual exposure,
constraint enforcement) by name but states none of their operational
contracts. Expect a heavy Homeless bucket.

---

## Absorbed

Corpus content that already landed in spec_new.md §25.

- **`planning/v2/v2.1_in_progress.md` §1048-1055 (warm-started solvers settled):**
  > "Iterative solvers (Newton, fixed-point) emitted by the compiler accept the previous tick's solution as their initial guess. When state is continuous, warm-start is almost always a win; when it isn't (cold-start evaluation, regime changes), the workflow can disable it."

  Absorbed into §25 summary: "warm-start semantics drawn from `assume_constant` initial values or `learn_constant` priors." The wording is tighter and also binds warm-start to `assume_constant` / `learn_constant` as value sources, but the settled mechanism is carried.

- **`planning/v2/v2.1_in_progress.md` §1057-1074 (projection flavor as workflow choice):**
  > "The compiler does **not** auto-emit projection: the choice of projection flavor (hard clip, sigmoid reparameterization, soft clip) is a training-dynamics decision that varies by problem."

  Absorbed: §25 summary, "Workflow selects projection flavor (`hard_clip`, `sigmoid`, `soft_clip`)."

- **`planning/v2/v2.1_in_progress.md` §1076-1096 (per-residual exposure settled):**
  > "Overdetermined components (the overconstrained residuals from section 12.3) expose each residual individually, named by the producing relation."

  Absorbed: §25 summary, "Per-residual loss exposure: users attach losses to named residuals."

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 (O4.3 / CC3, TRACKED):**
  > "Overconstrained relations must survive extraction with their *original relation names* so training emission can expose them per-residual."

  Absorbed into §25's per-residual loss exposure requirement.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.3 (Option C lean):**
  > "Myco owns AD for compile-time analysis ...; backend owns AD for runtime execution (actual gradient values for training / inference)."

  Absorbed implicitly: §25 does not mention AD ownership, but §24.2 covers it for controllers and §21 covers backend dispatch. §25's "gradient-trainable code" framing is consistent with the Option C lean.

- **`planning/v2/spec.md` §12.3 + §13.2 (consistency losses from overconstrained residuals):**
  > "For overconstrained components (whether closed by a policy or left as residuals), the compiler emits consistency losses from the extra equations."

  Absorbed into §25's per-residual exposure commitment and the summary's tie to §20 training-classified SCCs.

---

## Superseded

Corpus content replaced by a newer decision in spec_new.md §25. Should
move to anti_spec.md if not already there.

- **`planning/v2/spec.md` §13.2 JAX-emitter auto-emitted projection:**
  > "Admissibility projections and constraint penalty terms ... Inject admissibility projections at slot boundaries where domain restrictions are not statically proven"

  Superseded by §25: projection flavor is a workflow choice, not a compiler-auto-emitted operator. The retirement is already recorded in anti_spec.md:
  > "compiler auto-emitted admissibility projections | workflow picks projection flavor (`hard_clip` / `sigmoid` / `soft_clip`) | projection-free-compiler principle"
  No further action needed.

- **`planning/v2/spec.md` §12.5 two-phase gradient regime with convergence penalty:**
  > "In `train` mode, non-convergence must not crash the training loop. The emitter generates a fallback: if the solver exceeds `max_iterations`, it returns the last iterate and adds a **convergence penalty** to the loss (proportional to the final residual norm). ... The emitter handles this by **detaching the solver path**: gradients from the observation loss do not flow through the non-converged SCC via the implicit function theorem."

  §25 does not restate this. The four-item commitment (warm-start, projection flavor, per-residual exposure, constraint enforcement) makes no claim about solver-non-convergence behavior in training mode. Whether this is superseded or simply not re-specced at §25's summary level is ambiguous. It is not in anti_spec.md.

  `Recommend:` Check whether the two-phase convergence-penalty regime survives into v2.1. If retired, add to anti_spec.md; if retained, §25 should name it (one sentence) or forward-reference the SCC-convergence subsection that owns it.

- **`planning/v2/spec.md` §13.2 auto-emitted loss-function menu:**
  > "`obs_loss()`, `consistency_loss()`, `physics_residual_loss()`, `constraint_violation_loss()`, `admissibility_loss()`, `soft_penalty_loss()`, `loss_components()`, `total_loss()` — weighted aggregation"

  Superseded by §14.2 `loss_of` named-field return plus workflow-side aggregation (`bind_loss(...)` per §14.2). The compiler no longer emits a fixed loss-function menu; users compose scalar loss from `loss_of(residual)` field access. Not in anti_spec.md.

  `Recommend:` Add to anti_spec.md under "Retired architectural framing": "compiler-emitted fixed loss-function menu (`obs_loss`, `consistency_loss`, `physics_residual_loss`, etc.) | `loss_of(residual)` named-field return plus workflow-side `bind_loss` aggregation | §14.2 + §25".

- **`planning/v2/spec.md` §17 per-experiment `set_weight` API for study-level loss:**
  > "`exp_a.set_weight(1.0)` ... `exp_b.set_weight(5.0)`"

  The specific Python API (`exp.set_weight(...)`) is a pre-chunk-09 workflow-layer surface. Chunk 09 locks bind / observe / run verb families but does not name study weighting. Not in anti_spec.md.

  `Recommend:` No action at §25; multi-experiment weighting surface is workflow-side and open per chunk 09. Flag so it does not re-enter §25 by accident.

---

## Homeless

Corpus content that is relevant to §25, not accounted for in spec_new.md
§25, and not already committed to anti_spec.md. This is the highest-
value bucket.

- **`planning/v2/v2.1_in_progress.md` §1082-1096 — stdlib loss helpers (`soft_penalty`, `augmented_lagrangian`):**
  > "Stdlib ships two loss helpers in v2.1: **`soft_penalty(weights)`** — default. Sums `w_i · r_i²` over residuals. ... **`augmented_lagrangian(weights, mu, lambda_init, mu_schedule)`** — opt-in for brittle-penalty regimes. Adds a linear dual term `λ_i · r_i` with `λ_i` updated after each training step."

  §25 names "per-residual loss exposure" but does not name the two v2.1 stdlib helpers that consume `model.residuals`. This is settled design from the v2.1_in_progress training-emission block and from `open_questions_deprecated_use_spec_new.md:497-503`.

  `Recommend:` Add a bullet to §25 naming `soft_penalty` and `augmented_lagrangian` as the two v2.1 stdlib loss helpers, or forward-reference Part IV / workflow section for the stdlib surface. Both consume the per-residual exposure API and both are locked.

- **`planning/v2/v2.1_in_progress.md` §1089-1093 — two API shapes for `augmented_lagrangian`:**
  > "Two API shapes by paradigm: PyTorch-style mutable `lambdas.update(residuals)` or JAX-style pure `(params, lambdas, opt_state) = update(params, lambdas, opt_state, residuals)`."

  §25 does not mention dual-state semantics or the mutable-vs-pure API split. The two shapes are workflow-side but the compiler must expose `model.residuals` in a form consumable by both.

  `Recommend:` Flag as a workflow-side obligation that §25 should cross-reference. The dual-state ownership question is settled (helper maintains `λ_i`), but §25 currently omits it.

- **`planning/v2/v2.1_in_progress.md` §1076-1080 — `model.residuals` workflow surface:**
  > "`model.residuals` is a workflow-visible list of `Residual` objects with names and runtime values."

  §25 commits "per-residual loss exposure" but does not name the `model.residuals` surface or the `Residual` object shape. This is the concrete API the two stdlib loss helpers consume.

  `Recommend:` Either name `model.residuals` in §25 or cross-reference the Part IV / §31 section that owns the workflow API. Without a named surface, §25's "attach losses to named residuals" has no hook.

- **`planning/v2/v2.1_in_progress.md` §1057-1060 — refinement-type bounds as workflow-visible metadata:**
  > "When an unknown has bounds from its refinement type (`type Fraction : Scalar<ratio> { 0 <= self <= 1 }`, `type Conductance { self >= 0 }`, etc.), the compiler surfaces those bounds as workflow-visible metadata on the unknown."

  §25 mentions projection-flavor selection but does not state that the projection surface is populated by refinement-type bounds. The compiler's obligation (surface bounds as metadata) is settled but not in §25.

  `Recommend:` Add a bullet to §25 stating that projection targets are the unknowns carrying refinement-type bounds, surfaced as workflow-visible metadata. Ties §25 to §3 / §6 refinement types explicitly.

- **`planning/v2/v2.1_in_progress.md` §1104-1106 — what is NOT auto-emitted:**
  > "What is not auto-emitted by the compiler. Projection flavor choice, loss aggregation, dual-variable updates, annealing schedules. These are training-dynamics choices that belong in the workflow."

  §25 states the positive side (workflow selects projection flavor) but does not enumerate the four non-emitted items. The negative list is load-bearing for understanding where the compiler / workflow boundary sits.

  `Recommend:` Either add the four-item negative list to §25 or fold into the Summary. Currently a reader sees only the compiler's obligations, not the boundary.

- **`planning/v2/v2.1_in_progress.md` §1098-1102 — deferred beyond v2.1 (homotopy, pre-training):**
  > "Homotopy continuation (blending controller output with a baseline via annealing `α`) and pre-training against a hand-coded heuristic are entirely workflow-layer patterns that the controller-binding API already supports — no language features needed."

  `anti_spec.md` has "homotopy continuation as language feature | workflow Python recipe | belongs on workflow side" (already retired). Pre-training against a hand-coded heuristic is not in anti_spec.md and not in §25.

  `Recommend:` No §25 change. Pre-training-against-heuristic can either be added to anti_spec.md ("Dropped features") or left as a workflow recipe; either way §25 does not need to mention it.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 O1 — training-mode consistency-loss substitution:**
  > "O1 — train-mode consistency-loss substitution (shape-only; per-residual exposure follows CC3 / O4.3)"
  > Line 2033: "In train mode: overconstrained lhs = rhs becomes loss += w * (lhs - rhs)²"

  §25 names per-residual loss exposure but does not state the substitution rule (overconstrained `lhs = rhs` becomes `(lhs - rhs)²` in training mode). This is the rewrite that converts an e-graph equality into a training residual. Chunk 04 has it locked.

  `Recommend:` Add to §25 a one-line statement of the substitution: in training-classified SCCs, each unsatisfied equality lowers to a named squared residual. This is the mechanical contract behind "per-residual loss exposure."

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.5 — opaque callable gradient-flow in training-time SCCs:**
  > "How does gradient flow work when the callable is inside a training-time SCC? Backend AD through the callable requires the callable to live in the same AD frame as the rest of the computation."

  §25 addresses training SCCs but does not acknowledge the callable-in-SCC gradient-flow concern. Chunk 06 tracks this as an open item; §24.2 covers the controller case via "differentiable black box" framing, but §24.2 does not tie to §25's training emission explicitly.

  `Recommend:` Add a §25 cross-reference to §24.2 for controller-in-training-SCC semantics. Do not duplicate; the bind_controller section owns gradient flow through callables.

- **`planning/v2/v2.1_chunk_reports/12_cost_field_unification.md` §load-bearing Q5:**
  > "Training-emission aggregation in §25 presumes `loss_of` named fields today. A change to option (a) requires rewriting §25's aggregation contract."

  §25 summary uses "Per-residual loss exposure" without naming `loss_of` fields (`data_fit`, `constraint_violation`, `regularization`). Chunk 12 is the authoritative tracker of the cost-field-unification open item; §25's aggregation depends on field names that are not locked.

  `Recommend:` Add a `*Open.*` stanza to §25 flagging the dependency on chunk 12. Do not resolve (the question is open across three surfaces; chunk 12 is the place for the decision). The flag keeps §25 honest about what it presumes.

- **`planning/v2/spec.md` §14.7 rollout stability (gradient checkpointing, truncated BPTT):**
  > "Gradient checkpointing: trades compute for memory by recomputing intermediate states during the backward pass ... Truncated BPTT: limits the temporal gradient horizon."

  §25 is the training-emission section but does not acknowledge long-rollout gradient regime (checkpointing, truncated BPTT). These are workflow-configurable but affect training-emission correctness (what gradient the user actually receives).

  `Recommend:` Add to §25 a pointer to the rollout-stability subsection (likely §24 or §31 workflow). If rollout stability is retired as a language concern, add to anti_spec.md.

- **`planning/v2/spec.md` §17 study-level training (multi-experiment joint learning):**
  > "The optimizer minimizes the joint loss: `L = sum over experiments k: w_k * (obs_loss_k + consistency_loss_k + physics_residual_loss_k + constraint_penalty_k + admissibility_loss_k)`"

  §25 addresses per-residual exposure within a single model but does not cross-reference multi-experiment training. Chunk 09 locks the workflow layer as data-orchestration-only but does not address study-level training specifically. The sum-over-experiments pattern is settled per spec.md §17.5 ("Study-level training can start as a Python-side pattern").

  `Recommend:` Add a §25 cross-reference to the multi-experiment workflow section (or explicitly note that study-level joint loss is workflow-side composition, out of §25 scope). Without this, §25's training-emission contract is ambiguous about whether it covers single-experiment or multi-experiment.

- **`planning/v2/spec.md` §16.4 — physics residual factor from learn_trajectory + temporal equation (PINN):**
  > "The temporal equation becomes a **physics residual factor** ... a loss term penalizing deviation between the trajectory's values and what the temporal equation predicts. This is the PINN (physics-informed neural network) pattern."

  §25 does not name this interaction. Whether the PINN pattern is part of §25 training emission or lives at the §24 `learn_trajectory` verb level is ambiguous. The physics-residual-factor mechanism is settled in chunk 04 and spec.md §16.4.

  `Recommend:` Either name the physics-residual-factor case in §25 (it is another path from "equation" to "loss term") or forward-reference §24's `learn_trajectory` semantics. Currently §25 is silent on PINN-style training, which is a primary v2.1 use case.

---

## Conflicts

Direct contradictions between spec_new.md §25 and any corpus document.

- **§25 warm-start source attribution vs. v2.1_in_progress warm-start framing:**

  §25 summary: "warm-start semantics drawn from `assume_constant` initial values or `learn_constant` priors." v2.1_in_progress §1048-1055: warm-start is "the previous tick's solution as their initial guess." These are two different warm-start concepts. The v2.1_in_progress version is between-timesteps (solver initial guess from prior timestep's solution); the §25 version appears to be between-runs or at-initialization (initial guess from `assume_constant` / `learn_constant` values). The two concepts are compatible but not the same mechanism.

  `Recommend:` Clarify §25's warm-start wording. Three distinct warm-start sources exist in training: (a) previous-timestep solution within a rollout (v2.1_in_progress), (b) value supplied by `assume_constant` at run start, (c) prior mean from `learn_constant` before optimization. §25 conflates (b)/(c) with (a), or silently drops (a). Rewrite to enumerate the sources, or pick the load-bearing one.

- **§25 "constraint enforcement discharges at compile time where possible, otherwise at runtime" vs. chunk 04 CC1 + §8.1 `constraint` blocks:**

  §25 summary says constraint enforcement "discharges at compile time where possible, otherwise at runtime [via projection]." Chunk 04 and §8.1 frame `constraint` blocks as producing residuals that go into `loss_of(residual).constraint_violation`, which is a training-time loss term, not a "runtime projection." The two mechanisms (training-time penalty vs. runtime projection) are both valid and both different from compile-time discharge. §25's summary collapses them into a single "runtime projection otherwise" phrase.

  `Recommend:` Rewrite §25's constraint-enforcement bullet to distinguish three discharge regimes: (a) compile-time (statically proven via refinement types / e-graph rewriting), (b) training-time penalty (via `loss_of(residual).constraint_violation`), (c) runtime projection (via workflow-selected `hard_clip` / `sigmoid` / `soft_clip`). Regime (b) is the training-specific path; regime (c) is the simulate-mode / guard-definedness path. §25 currently conflates (b) and (c).
