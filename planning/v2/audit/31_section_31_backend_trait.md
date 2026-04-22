# Audit Report — §31 Backend Trait Surface

Audited against corpus as of 2026-04-22.

---

## Absorbed

Corpus content that already landed in spec_new.md §31.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §3 (Scope / In scope) and §4 (Design surface):**
  The six-responsibility framing of §4.1 (allocation/elementwise, linear algebra, distribution, autodiff primitives), the capability-advertising enumeration (§4.2: Cholesky, sparse, iterative solvers, SVD, `condest`, autodiff modes), and the three-option fallback policy (error / host / emulate) from §4.2 are all reflected in §31.1 of spec_new.md. The fallback names, semantics, and the `run.config.backend` knob match §4.2 exactly.

  Chunk 06 framing absorbed into §31.1 (`supports_complex`, `supports_forward_ad`, `supports_hamiltonian_monte_carlo`, `supports_sparse_matmul` as representative examples; error / host / emulate as the three policy modes). The `host` fallback description in §31.1 ("Correctness preserved at the cost of device-host traffic. Useful for CPU-only families") traces to chunk 06 §4.2.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11, settled item 36 / design blocker B6:**
  > "Backend abstraction factored into chunk 06 ... unified burn-style backend trait covering PPL inference, numerical linear algebra, GPU lowering, opaque callables, and AD ownership"
  > "CC4 Tier C routing ... chunk 02 collection aggregation GPU kernels, and `bind_controller` callable gradient-flow all share one underlying concern — workflow selects a backend at run-time configuration, compiler emits backend-agnostic IR, backend lowers to concrete kernels (burn-style trait pattern)."

  Absorbed into §31 summary and §31.6 preamble: "The backend is a trait the compiler targets: numerical execution, automatic differentiation, PPL handoff, opaque-callable runtime, plus capability advertising."

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11, B3 absorption note:**
  > "B3 — Tier C PPL backend protocol. ABSORBED (2026-04-20) into B6 / chunk 06 backend abstraction."
  > "Chunk 06 ... owns the unified backend trait surface; PPL protocol is §4.4 there."

  Absorbed into §31.2 (PPL Handoff Protocol) and §31.5 (Stochastic E-Class Serialization). The framing that the handoff is "a protocol, not a library call" (spec_new.md §31.2 line 4667) echoes chunk 04's rationale for factoring the PPL protocol into the backend chunk.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.4 (PPL backend protocol, was B3):**
  The compiler-emits / backend-returns skeleton matches §31.2. The direction that samples return without parametric envelope facts is captured in §31.2 ("Samples come back without envelope facts about the parametric form"). The spec_new.md commitment ("no envelope facts about the parametric form; downstream code treats them as opaque draws") traces to chunk 06 §4.4: "Clean answer: they enter as new envelope facts on existing e-classes, not as new merges." This is the settled direction, absorbed correctly.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.5 (Opaque callable protocol) and §4.6 (Mixed-backend policy):**
  The lean "same-backend-per-run for simplicity" from §4.5 and §4.6 is absorbed into §31.3 and surfaced more explicitly in §32.1 (Mixed-Backend Policy), which calls out the open status but correctly records the single-backend-per-run lean.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.7 (Versioning):**
  The plan to version the trait surface via Myco semantic versioning, have backends advertise which trait versions they implement, and key the plan cache on `(plan, trait_version, backend_identity)` is absorbed into §31.4 (Backend Versioning) essentially verbatim.

- **`planning/v2/v2.1_in_progress.md` lines 1785-1793 (Backend targets section):**
  > "Myco is a compiler — the backend is a lowering target, not a language decision."

  Absorbed into §31 preamble and §31.6 (No Primary-Backend Commitment). The no-primary-backend principle is stated clearly in spec_new.md lines 4738-4746.

- **`planning/v2/anti_spec.md` "Retired architectural framing" table (JAX-as-primary, PyTorch-as-primary):**
  The retirements "JAX-as-primary emitter | backend trait (burn-style) with capability advertising" and "PyTorch-as-primary emitter | same" are reflected correctly in §31.6's closing sentence (spec_new.md lines 4743-4746) naming the prior framing as retired.

- **`planning/v2/anti_spec.md` "Dropped features" row for `Dual<T>`:**
  > "user-facing `Dual` numeric representation | backend-owned AD | Part V commits backend-delegated AD (burn-style tensor tracks operations); user-facing `Dual` would duplicate backend machinery..."

  The anti_spec.md retirement of user-facing `Dual` is consistent with §31.3's description of gradient flow: the backend provides AD; the compiler does not expose `Dual` to users. §31.3 does not re-introduce this retired feature.

- **`planning/v2/v2.1_in_progress.md` lines 749-765 (Compilation and backend behavior for stochastic claims):**
  > "Backend routing and capability checks happen at workflow composition. If the world description includes stochastic claims the chosen backend cannot handle ... the compiler emits a capability error."

  Absorbed into §31.1's capability-mismatch diagnostic description ("Fail at plan-binding time with a capability-mismatch diagnostic (workflow-composition error tier, §19.4)"). The timing — workflow composition / plan-binding time — is consistent.

---

## Superseded

Corpus content replaced by a newer decision in spec_new.md §31. Should move to anti_spec.md if not already there.

- **`planning/v2/spec.md` §13.2 (JAX emitter as primary) and §13.3 (Backend interface):**
  > "The JAX backend is the primary implementation for v2. Other backends are specified here for interface design but implemented post-v2." (spec.md line 3066)
  > "The JAX emitter produces a Python module using: `jax.numpy` ... `jax.lax.scan` ..."

  Superseded by §31.6's no-primary-backend commitment. The old "JAX is primary" framing is replaced by the symmetric trait-based approach. The anti_spec.md already captures the retirement of "JAX-as-primary emitter" in the architectural-framing table. No further action needed there.

  `Recommend:` spec.md §13.2 and §13.3's JAX-primary framing is stale documentation; the anti_spec.md retirement entry covers it. Confirm that spec.md's §13 sections are not being imported into the spec_new.md consolidation without review.

- **`planning/v2/spec.md` §13.3 "Backend interface" method list:**
  > "Emit admissibility projections at slot boundaries" / "Emit parameter initialization" / "Emit numerical quadrature calls"

  The backend interface described in spec.md §13.3 is a JAX-centric emitter method list, not a trait-surface description. §31's trait approach is structurally different: a capability-advertising trait, not an emitter method table. The old method list is superseded; the trait surface and its capability-advertisement mechanism replace it.

  The anti_spec.md "JAX-as-primary emitter" entry covers the architectural framing. The specific method-list form of the interface is not individually retired; it is implicitly superseded.

  `Recommend:` No separate anti_spec.md entry is needed. The JAX-primary emitter retirement encompasses this.

- **`planning/v2/v2.1_in_progress.md` lines 1785-1790 (Backend targets with PyTorch-primary and JAX-secondary):**
  > "Primary target: PyTorch ... Secondary: JAX ... Long-term watch: Enzyme + Rust."

  Superseded by §31.6's symmetric no-primary treatment. The v2.1_in_progress framing predates the backend-abstraction chunk (06) which explicitly retired the primary-backend model. The anti_spec.md retirement entry covers this.

  `Recommend:` No new anti_spec.md entry needed. Confirm that v2.1_in_progress.md's backend-targets section is treated as stale by the consolidation (anti_spec.md already marks v2.1_in_progress stale items). The anti_spec.md "Stale in legacy docs" section already flags v2.1_in_progress "NEW:" / "ships in v2.1" prose.

- **`planning/v2/spec.md` §14.7 (truncated backpropagation, gradient checkpointing):**
  > "gradient checkpointing via `jax.checkpoint`"

  JAX-specific gradient management is superseded by the backend-agnostic gradient-flow story in §31.3 (backend provides the AD facility; the compiler does not specify JAX primitives). This is implicitly covered by the JAX-emitter retirement.

---

## Homeless

Corpus content relevant to §31 that is not in spec_new.md §31 and not retired in anti_spec.md. This is the highest-value bucket.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.3 — AD ownership fork (Options A, B, C):**

  Chunk 06 §4.3 presents three options for AD ownership: Option A (Myco owns AD — symbolic `deriv` extended to every tensor operation; backend trait stays small); Option B (delegate to backend AD — JAX `grad`, PyTorch autograd, Enzyme; backend handles backward pass); Option C (Hybrid — Myco owns AD for compile-time analysis, backend owns AD for runtime execution). Chunk 06 §4.3 records "Lean: Option C. Matches Myco's broader symbolic-analysis-plus-concrete-execution pattern. But this is a real fork that deserves explicit decision."

  §31 does not document this fork. §32's summary (spec_new.md line 4750-4759) mentions "AD ownership fork (Myco-owned / backend-delegate / hybrid, leans hybrid)" and lists it as an open item, but does not cite the three-option structure, the consequences of each option (Option A's analysis advantages vs. implementation cost; Option B's opacity to condition-number estimation; Option C's two-system consistency obligation), or why Option C is the recommended lean.

  This matters because: the anti_spec.md "Retired architectural framing" row for `` `deriv` always symbolic / no runtime cost" framing" (line 73) explicitly references "three-mode lowering (symbolic / algorithmic / runtime) per §14.4" and notes "runtime AD is the authorized fallback for SCCs too large to expand symbolically, gated on B6 backend-AD ownership." This presupposes Option C (hybrid) is the resolution, but the fork is not formally closed in spec_new.md.

  `Recommend:` Add to §32 (or §31.3) a subsection or note documenting the three AD-ownership options and recording the Option C lean explicitly, including the consequences of each and the consistency obligation. This is not open design — chunk 06 already records the lean with rationale. The commitment text in spec_new.md should make the lean formal enough that anti_spec.md's §14.4 reference to "runtime AD is the authorized fallback, gated on B6 backend-AD ownership" is grounded in a spec section, not just a chunk report.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.1 — Minimum backend trait API (which ops are mandatory vs. capability-advertised):**

  Chunk 06 §4.1 poses the key design question: "where's the minimum? Should every backend implement Cholesky? Or is Cholesky capability-advertised? The line determines how fat the trait is." It lists four mandatory primitive sets (allocation/elementwise, linear algebra primitive set, distribution primitive set, autodiff primitive set if backend owns AD) but does not resolve the mandatory-vs-optional line for individual operations.

  §31 commits the four-responsibility framing (numerical execution, AD, PPL handoff, opaque-callable runtime) and notes "concrete signatures land in chunk 06." This is correct staging, but the open question about the mandatory-vs-optional line is not captured in §31 or §32.

  `Recommend:` Add to §32 an explicit open item for "minimum backend trait API" tracking the mandatory-vs-optional question and citing chunk 06 §4.1. Without this, reviewers of §31 have no pointer to where the trait's minimum API question lives.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.4 — PPL protocol specifics (compiler-emitted content and backend-returned content):**

  Chunk 06 §4.4 specifies what the compiler emits to the backend:
  - Envelope metadata (layer-2 facts: family, parameters, bounds)
  - Structural declarations (joint syntax)
  - Coupling annotations (independence claims, copula structure)
  - Log-density assembly recipe (how to build `log_pdf` from parts spanning Tiers A/B/C)

  And what the backend returns:
  - Sample values (with shape and provenance metadata)
  - Gradient estimates (score function, reparameterized, or via backend AD per §4.3)
  - MCMC traces (chains, acceptance stats, convergence diagnostics)
  - Diagnostic metadata (effective sample size, R-hat, divergence warnings)

  §31.2 covers the high-level protocol description: "log-density callable, parameter shape and bounds, observation data, inference kind: MCMC, VI, importance, etc." and "samples plus diagnostics." But the specific message schema — particularly gradient estimates, MCMC traces, ESS, R-hat, divergence warnings — is absent. These are settled design content from chunk 06 §4.4 (not open questions), and they are material to anyone implementing a compliant backend.

  §31.5 (Stochastic E-Class Serialization) covers what the compiler serializes but does not describe the return side (sample values with provenance, gradient estimates, diagnostic metadata).

  `Recommend:` Expand §31.2 to enumerate the return-side message schema from chunk 06 §4.4 (sample values with provenance, gradient estimates, MCMC traces, diagnostic metadata including ESS, R-hat, divergence warnings). This is locked design from chunk 06, not open work. Alternatively, call out the return schema as a §32 open item if it is intentionally deferred. Currently it is neither in §31 nor in §32.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.4 — PPL protocol open questions:**

  Chunk 06 §4.4 records two open questions not captured in §31 or §32:
  1. "Does the backend see the whole stochastic model at once, or per-factor? (Affects what optimizations the backend can do — JIT-compile the full model vs. build it incrementally.)"
  2. "How do backend-returned samples participate in further graph computation? (Clean answer: they enter as new envelope facts on existing e-classes, not as new merges.)"

  Question 2 has a preferred answer in chunk 06 but is not recorded as decided in spec_new.md. Question 1 is genuinely open.

  `Recommend:` Add both questions to §32 as explicit PPL-protocol open items. Record chunk 06's preferred answer for Q2 ("they enter as new envelope facts on existing e-classes, not as new merges") as a design lean pending lock. The current §32 summary mentions only "PPL protocol specifics (message schema, inference-kind enumeration)" — it should name these specific questions.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.5 — Opaque callable gradient-flow specifics (questions not yet decided):**

  Chunk 06 §4.5 identifies four unsettled questions about the opaque callable protocol:
  1. Which backend runs the callable (same as the rest of the graph, or separate)?
  2. How does gradient flow work when the callable is inside a training-time SCC?
  3. Can a neural controller with Matrix/Tensor I/O use a different backend than the main numerical workload?
  4. Portability: can a callable trained against one backend run against another?

  Chunk 06 records a lean for Q1 and Q3 ("same-backend-per-run for simplicity") but Q2 and Q4 are unresolved.

  §31.3 (Opaque-Callable Runtime) states that the backend "threads gradients back through Python for training emission (§25)" and "manages any memory / device-residency needed for the interop." This addresses Q2 at a high level (backend handles gradient flow) but does not resolve the specific case of a callable inside a training-time SCC and how backend AD interacts with the SCC's residual graph.

  §32's summary mentions "Gradient-flow semantics for `bind_controller` callables" as an open item, but does not enumerate the four specific questions from chunk 06 §4.5.

  `Recommend:` Add to §32 the four specific callable questions from chunk 06 §4.5, recording Q1/Q3 as having a lean (same-backend-per-run) and Q2/Q4 as genuinely open. The cross-backend callable interop question (Q4) is already noted in the open_questions.md Tier 2 ("Compiler Internals") section as "Cross-backend callable interop. §31.6 locks that Myco commits to no primary backend; §23.3 locks that trained callables reuse across workflows via plain contracts. What's unresolved: if workflow A trains a callable on backend X, can workflow B bind the same callable when running on backend Y?"). That note references §31.6 and §32.1, which is correct; it should also be reflected in §32's own open-items list.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.4 — Framework-specific adapters (NumPyro-style, Pyro-style, Turing.jl-style, Stan-style):**

  Chunk 06 §4.4 notes: "Framework-specific adapters: NumPyro-style, Pyro-style, Turing.jl-style, Stan-style. Each wraps the same protocol differently."

  §31.2 does not mention framework-specific adapters or the adapter pattern over the PPL handoff protocol. This is a consequential design surface: the backend trait defines one protocol, and each concrete PPL framework gets an adapter. This is how "backends can satisfy the trait symmetrically" (§31 opening) actually works for PPL-heavy backends.

  `Recommend:` Add a sentence or brief note to §31.2 acknowledging that framework-specific adapters (NumPyro-style, Pyro-style, Stan-style, etc.) each wrap the same protocol. This communicates how the no-primary-backend commitment interacts with the diversity of real PPL frameworks. If the adapter pattern is itself deferred to chunk 06, note that in §32.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §5 — JAX-like vs PyTorch-like dynamic-topology lowering interaction with backend:**

  Chunk 04 §5 ("Lowering to execution") notes that JAX-like backends require N-max slots with validity masks (XLA requires static shapes), while PyTorch-like backends can use per-timestep allocation with dynamic shapes. This is a backend-capability difference with consequence for the alive-mask lowering described in §21 (spec_new.md). The backend trait must either expose a "static-shape vs dynamic-shape" capability flag, or the compiler must have a separate lowering strategy per backend.

  §31.1 lists capability examples (`supports_complex`, `supports_forward_ad`, `supports_hamiltonian_monte_carlo`, `supports_sparse_matmul`) but does not include `supports_dynamic_shapes` or similar. This gap means the dynamic-topology backend-capability interaction is unaddressed in the trait surface.

  `Recommend:` Add `supports_dynamic_shapes` (or an equivalent capability flag) to the §31.1 capability examples, or add a note acknowledging that static-shape vs dynamic-shape backend behavior is a capability difference that backends must advertise. If this is deferred to chunk 06 for trait-signature design, note it in §32.

- **`planning/v2/open_questions_deprecated_use_spec_new.md` — B6 / backend-related open items:**

  The deprecated open questions file's "Tier 2 — Compiler Internals" section records the cross-backend callable interop question (see above Homeless item on §4.5 Q4). The framing there cites §31.6 and §32.1, which is accurate, but §32 itself does not contain this open item in its body (only in §32's summary referencing "Gradient-flow semantics for `bind_controller`"). The cross-run portability dimension (training on backend X, running on backend Y) is distinct from the gradient-flow semantics dimension, and deserves its own entry.

  `Recommend:` Confirm that §32's open-items body is extended (in a future chunk 06 pass) to enumerate the cross-run portability question separately from the gradient-flow question.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §8 (Open questions Q1-Q8):**

  Chunk 06 §8 records eight consolidated open questions for this chunk: Q1 (AD ownership), Q2 (minimum API vs capability-advertised), Q3 (default fallback policy), Q4 (PPL protocol form), Q5 (opaque callable gradient-flow), Q6 (mixed-backend policy lean), Q7 (versioning), Q8 (first concrete backend).

  §32 in spec_new.md mentions Q1, Q5, Q6, and Q8 either explicitly or implicitly. Q2 (minimum API line), Q3 (default fallback policy), Q4 (PPL protocol form), and Q7 (versioning) are not surfaced as named open questions in §32.

  Q3 is particularly notable: the default fallback policy (error / host / emulate) is described in §31.1 without naming which is the default. Chunk 06 §4.2 states "Open question: default policy. `'error'` is safest (no silent performance catastrophes); `'host'` is most permissive." The spec_new.md §31.1 says "Conservative default" under the `error` bullet, which implies `error` is the default — but this is not explicitly locked, and chunk 06 treats it as open.

  `Recommend:` Either explicitly lock `error` as the default fallback mode in §31.1 (removing the ambiguity), or move the default-policy question to §32 as an explicit open item. Do not leave "Conservative default" as the only signal; it reads as decided without being formally locked.

---

## Conflicts

Direct contradictions between spec_new.md §31 and corpus.

- **§31 summary (spec_new.md line 4607-4611) vs. chunk 06 §4.1 minimum API framing:**

  §31's summary states: "The minimum trait API covers four responsibilities, numerical execution, automatic differentiation, PPL handoff, and opaque-callable runtime." This framing commits autodiff as part of the minimum API. But chunk 06 §4.3 records the AD-ownership fork as open: under Option A (Myco-owned AD), the backend trait stays small (no autodiff primitives required); under Option B or C (backend-delegated or hybrid), autodiff primitives enter the minimum API. The spec_new.md §31 summary resolves this ambiguity by implicitly assuming Options B or C (AD in the minimum API), while §32 still lists the AD ownership fork as open.

  This is a structural inconsistency: §31's summary pre-commits a design decision that §32 explicitly calls open.

  `Recommend:` Qualify the §31 summary's "four responsibilities" formulation. Rather than listing autodiff as a minimum-API responsibility, write it as "the minimum trait API covers four responsibilities depending on the AD-ownership model: numerical execution, PPL handoff, and opaque-callable runtime are always present; autodiff primitives join the minimum API when the backend owns AD (the hybrid and full-delegation options; §32, AD ownership open)." This preserves the four-responsibility framing without falsely resolving the open fork.

- **§31.3 (Opaque-Callable Runtime) "threads gradients back through Python for training emission" vs. §24.2 gradient-flow description referencing "the backend's AD facility (§31)":**

  §31.3 says the backend "threads gradients back through Python for training emission (§25)." §24.2 (spec_new.md lines 3901-3906) says: "Loss gradients from `observe` (§13.8) flow through the model graph to the controller's output, into the controller's parameters, via the backend's AD facility (§31)."

  The two descriptions are compatible in intent but pull in opposite directions on where gradient flow is described as "living." §31.3 says the backend threads gradients through Python; §24.2 says the backend's AD facility does it and cross-references §31. If §31 is the authority for opaque-callable gradient flow, then §31.3 should say more than one sentence about the mechanism — particularly how the backend AD facility interacts with the training-time SCC boundary (the unresolved Q2 from chunk 06 §4.5). Currently the cross-references form a loop (§24.2 → §31; §31.3 → §25) without resolving the mechanism at either end.

  `Recommend:` In §31.3, expand the gradient-flow description to clarify the direction of cross-references: §31 owns the backend-side AD mechanism; §24.2 owns the model-side gradient-flow semantics; §25 owns training emission mechanics. The current one-sentence treatment in §31.3 ("threads gradients back through Python for training emission (§25)") is insufficient as the authoritative backend-AD description that §24.2 leans on. If the mechanism is genuinely deferred pending the AD-ownership lock (§32), say so explicitly in §31.3 rather than leaving a forward reference to §25 as the apparent resolution.

- **§31.1 "Conservative default" for the `error` fallback vs. chunk 06 §4.2 "Open question: default policy":**

  §31.1 describes the three fallback modes and labels `error` with the phrase "Conservative default." This phrasing implies `error` is the decided default. Chunk 06 §4.2 explicitly marks default policy as open: "Open question: default policy. `'error'` is safest (no silent performance catastrophes); `'host'` is most permissive." The question is unresolved in chunk 06.

  This is a direct conflict: §31.1 implies a decision that chunk 06 explicitly defers.

  `Recommend:` Remove "Conservative default" from the §31.1 `error` bullet and either (a) formally lock `error` as the default by recording the decision rationale, or (b) move the default-policy selection to §32 as an explicit open item. The current state misleads readers into thinking the default is decided.
