# Audit Report — §21 Lowering

Audited against corpus as of 2026-04-22.

---

## Absorbed

Corpus content that already landed in spec_new.md §21.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §4 (temporal handling):**
  > "`y[t]` and `y[t+1]` are distinct ground terms — they are not claimed equal anywhere. ... temporal is not a special axis. It is just indexing that produces distinct terms."

  Absorbed into §21.3: "Temporal indexing produces distinct e-graph ground terms, not a templated family. `y[1]`, `y[2]`, `y[3]` are three different e-classes" and the bullet "Merges on `y[5]` are permanent ... but do not propagate to `y[6]` except through `step(y)` or `d(y)`."

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §5 (referential truth + lowering to execution):**
  > "JAX-like (big sparse, static shapes): the compiler allocates N-max slots and derives the alive-mask from 'which entities have active relations at timestep T.' Mask falls *out of* the graph; it is not stored on the graph."

  Absorbed into §21.4: N-max slots plus alive mask as the dynamic-topology lowering form; the alive mask as a "layer-3 adjacent-keyed-state fact, not an e-graph deletion."

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §5 ("Things do not know they are dead"):**
  > "Entity existence at timestep `T` is defined by *whether any relation at T references the entity*. No alive/dead flag. No tombstoning."
  > "Under monotonicity, the entity's e-classes continue to exist equationally"

  Absorbed into §21.4: "Under monotonicity (§16.2), the entity's e-classes continue to exist equationally ... Dead entities 'do not know they are dead' (§0 principle 5)."

- **`planning/v2/v2.1_in_progress.md` §1778-1783 (static/dynamic classification):**
  > "Compiler mechanically classifies model modules. If the type graph contains `some`-sized collections or events: dynamic topology (solver factory). Otherwise: fully static (fixed factor graph, everything proven at compile time)."

  Absorbed into §21.1 as the module-level static vs dynamic split with classification happening before SCC decomposition.

- **`planning/v2/v2.1_in_progress.md` §1795-1800 (dynamic-topology lowering):**
  > "`[Fish; some]` compiles to a **fixed-size array with validity mask**. The workflow layer must supply `MAX_CAPACITY` when binding the experiment."

  Absorbed into §21.4: N-max declared at collection site, workflow override via `bind_topology` up to a compile-enforced ceiling, mask flips on retirement, overflow as runtime error.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 (Tier A/B/C distributional propagation, item 24):**
  > "Three-tier distributional propagation: Tier A closed-form / Tier B approximate / Tier C opaque"

  Absorbed into §21.2 stochastic SCC lowering: "Tier A closed-form marginals resolve at compile time; Tier B approximate rewrites pre-materialize their error-bounded form; Tier C hands off opaquely."

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §1 preamble (Pattern to commit):**
  > "burn-style `trait Backend { type Tensor; type Distribution; fn matmul(...); fn sample(...); ... }`. Every backend-dependent op routes through the trait."

  §21 framing treats the backend as an abstract target of lowering; the concrete Tier-C PPL handoff in §21.2 and the runtime primitives in §21.4 presuppose the trait-based surface from chunk 06. (Detailed trait surface itself lives in §31 per the cross-reference in §21.2.)

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 (item 31 on `condition_of`):**
  > "problem conditioning (III) is runtime `condest`-style estimate"

  Aligned with §21.2 dynamic-SCC lowering scope: per-tick computation includes assembled-Jacobian paths that consume Level III `condest`. (Not directly quoted in §21 but the four-way split is compatible.)

---

## Superseded

Corpus content replaced by a newer decision in spec_new.md §21. Should move to anti_spec.md if not already there.

- **`planning/v2/spec.md` §13.2 "JAX emitter (primary)":**
  > "The JAX emitter produces a Python module using: `jax.numpy` for array operations, `jax.lax.scan` for rollout, `jax.nn` for smooth projections ... The JAX backend is the primary implementation for v2."

  Superseded by §21's backend-agnostic lowering path plus the chunk 06 burn-style trait. Already retired in anti_spec.md:
  > "JAX-as-primary emitter | backend trait (burn-style) with capability advertising | no primary backend; trait-based"
  No further action needed.

- **`planning/v2/v2.1_in_progress.md` §1785-1790 (Backend targets):**
  > "Primary target: **PyTorch** (`torch.compile(dynamic=True)` supports symbolic-shape kernels ...). Secondary: **JAX** for static-topology models where XLA optimization shines."

  Superseded by the same retirement. Already in anti_spec.md:
  > "PyTorch-as-primary emitter | same | same"
  No further action needed.

- **`planning/v2/v2.1_in_progress.md` §1802-1804 (per-backend mask policy):**
  > "On PyTorch: `torch.compile(dynamic=True)` can handle actual resizing without recompilation, so masking may be optional. On JAX: masking is mandatory (XLA requires static shapes). The compiler emits the appropriate lowering per backend."

  Superseded. §21.4 commits to N-max plus alive mask as *the* dynamic-topology lowering, independent of backend. Per-backend optional-mask policy is retired along with the JAX/PyTorch primary framing.

  `Recommend:` This specific "mask may be optional on PyTorch" framing is not explicitly in anti_spec.md. The general retirement covers it, but a one-line entry would help legacy-doc readers:
  "per-backend mask-optionality framing for `[T; some]` | N-max + alive mask is the uniform lowering (§21.4)".
  Low urgency.

- **`planning/v2/spec.md` §13.1 "Plan representation" (backend-agnostic plan derived directly from residual graph):**
  > "The plan is backend-agnostic. From the closed residual graph, the emitter derives: Forward computation steps for derived nodes (topologically ordered), Solver blocks for square implicit components (SCCs) ..."

  Superseded by §21's four-way SCC lowering (static / dynamic / stochastic / training) with explicit class dominance. The older two-way split (forward-derived vs solver-block) is subsumed by the four-way classification. Not in anti_spec.md explicitly, but follows from the broader "residual graph as core semantic object" retirement since the plan is derived from the residual projection now, not treated as the canonical intermediate.

- **`planning/v2/spec.md` §13.3 "Backend interface" obligations ("Emit admissibility projections at slot boundaries"):**
  > "The interface requires: ... Emit admissibility projections at slot boundaries"

  Superseded. Already retired in anti_spec.md:
  > "compiler auto-emitted admissibility projections | workflow picks projection flavor (`hard_clip` / `sigmoid` / `soft_clip`) | projection-free-compiler principle"
  No further action needed. §21 correctly omits automatic projection emission from the lowering surface.

---

## Homeless

Corpus content that is relevant to §21, not accounted for in spec_new.md §21, and not already committed to anti_spec.md. This is the highest-value bucket.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.2 (capability advertising and fallback policy):**
  > "Workflow knob: `run.config.backend.fallback = \"error\" | \"host\" | \"emulate\"`."
  > "Fallback policy options: Error ... Host fallback ... Emulate (synthesize the op from available primitives)"

  §21 describes lowering targets but never acknowledges that an SCC may request a primitive the selected backend does not advertise. Stochastic SCCs in §21.2 route to "backend PPL primitives (§31)"; dynamic SCCs assembling Jacobians consume `condest`. Neither subsection says what lowering does when the backend lacks a capability. Chunk 06 §4.2 fixes this with a fallback knob and enumerated policies.

  `Recommend:` Add a bullet (or cross-reference to §31) in §21.2 stating that SCC lowering consults backend capability advertising and applies the workflow-configured fallback policy. Without this, §21 silently assumes every backend implements every primitive.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.3 (AD ownership fork):**
  > "Option C — Hybrid. Myco owns AD for compile-time analysis (symbolic `deriv` for Level I/II condition bounds, envelope propagation, closure-policy ranking); backend owns AD for runtime execution."
  > "Lean: Option C."

  §21.2 training-SCC lowering says "differentiability propagates through contained stdlib atoms via their `Differentiable` contracts" but does not state who owns the runtime AD pass. The chunk-06 hybrid lean (compile-time symbolic, runtime backend-owned) is settled direction and directly affects what §21.2 training lowering emits.

  `Recommend:` Add a sentence to §21.2 training-SCC bullet stating that the runtime gradient pass is backend-owned (per chunk 06 §4.3) while the symbolic derivative machinery used for closure-policy ranking and envelope analysis remains compiler-owned. This is settled direction per chunk 06's §7 return path item 1.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.5 (opaque callable protocol) and §4.6 (mixed-backend policy):**
  > "Lean: v2.1 commits to same-backend-per-run for simplicity. Cross-backend callable interop is v2.2+."
  > "Lean: v2.1 commits to single-backend-per-run. SCC-level is v2.2."

  §21 is silent on whether different SCCs can lower to different backends and on which backend runs `bind_controller` callables. The chunk 06 leans (single-backend-per-run; callable in the same backend as the rest of the graph) are directly relevant to §21.2's "Lowered to backend PPL primitives" phrasing for stochastic SCCs and to training SCCs containing opaque callables.

  `Recommend:` Add a §21 preamble or §21.2 note stating: "All SCCs in one run lower to a single workflow-selected backend; opaque callables attached via `bind_controller` execute in the same backend. Cross-backend and SCC-level dispatch are v2.2+." This is a settled lean from chunk 06 §4.6 and affects how a reader understands §21.2.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 O4.3 (per-residual training emission):**
  > "Overconstrained relations must survive extraction with their *original relation names* so training emission can expose them per-residual. Standard CSE-style canonicalization would collapse them."

  §21.2 training-SCC bullet says "Loss exposure per residual (§25) enables workflow-selected scalar combinations" but does not state the lowering-side constraint: the training emitter receives relations with their declared names preserved, not CSE-canonicalized. The constraint lives in §19.2 (per audit of §19) but has a concrete lowering consequence that §21.2 should note.

  `Recommend:` Add a bullet under §21.2 training SCCs clarifying that the training emitter receives relations with their extraction-preserved names (as required by §19.2 for per-residual loss exposure). This is stable design from chunk 04 O4.3.

- **`planning/v2/spec.md` §13.2 "Long rollout stability":**
  > "For temporal rollouts, the emitter supports gradient checkpointing via `jax.checkpoint` on the scan function to trade compute for memory. Truncated backpropagation through time (limiting the temporal gradient horizon) is configurable in section 14.7."

  Checkpointing and truncated BPTT are workflow-configurable lowering choices. §21.3 covers temporal ground-term lowering to storage slots and mentions rolling buffers for streaming runs but does not address gradient checkpointing policy for dynamic+training combinations. The JAX-specific `jax.checkpoint` call is backend detail (retired with JAX-as-primary), but the backend-agnostic capability (workflow-configurable checkpointing / TBPTT horizon) is missing from §21.

  `Recommend:` Add a bullet to §21.3 noting that workflow configuration controls gradient-checkpoint granularity and TBPTT horizon for dynamic+training combinations; §31 backend trait must expose a checkpoint primitive. Settled concept from spec.md §13.2; only the JAX-specific spelling is retired.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 O4.7 (incremental saturation):**
  > Phase 4 item "O4.7 incremental saturation"

  §21.1 says classification happens before SCC decomposition at compile time. Dynamic modules with event-time topology add relations at runtime (new entities, new equalities). §21.3 / §21.4 describe allocation and alive-mask updates but do not state whether saturation runs once at compile time or incrementally as events fire. The chunk 04 O4.7 open item suggests this is acknowledged but open.

  `Recommend:` Add a forward reference from §21.4 (or §21 preamble) to §35 O4.7 stating that incremental saturation under event-time topology mutations is open design. Currently §21 reads as if saturation runs once and alive-mask updates are the only dynamic-topology runtime concern.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 P1 (mesh discretization):**
  > "P1 | Open-questions §B.3 item 9 | Mesh discretization stencils — architectural call: e-graph rewrite vs pre-e-graph codegen"

  Spatial operator lowering (`grad`, `laplacian`, etc. on discretized fields) is neither treated in §21 nor explicitly deferred there. The rewrite-vs-codegen question is active design. §21.1's static-module path covers standalone algebraic programs; dynamic-module path covers event loops; neither says where mesh discretization enters.

  `Recommend:` Add a bullet to §21 (either preamble or a new §21.5 stub) cross-referencing §35 P1 and stating that spatial operator lowering to stencils is open design. Without this, a reader with a PDE model cannot locate the lowering path.

- **`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §4 (primitive list consumed by lowering):**
  > "`cholesky(PosDef<U, n>) -> LowerTriangular<U, n>` ... `solve(Matrix<U, n, n>, Vector<U, n>) -> Vector<U, n>` — dispatches on structural subtype (Cholesky for PosDef, triangular solve for Triangular, LU otherwise)."

  Dynamic SCCs whose residual assembles a square linear system lower through these primitives. §21.2 dynamic-SCC bullet says "Intra-SCC ordering resolved by the residual graph's topology" but does not mention structural-subtype dispatch on assembled systems. The dispatch is a lowering-time decision.

  `Recommend:` Add a bullet to §21.2 dynamic-SCC section noting that linear-solve residuals dispatch to structural-subtype-specialized backend primitives (Cholesky for PosDef, triangular solve for Triangular, general LU otherwise) per §31 and the Matrix stdlib surface. Settled from chunk 05 §4.

---

## Conflicts

Direct contradictions between spec_new.md §21 and any corpus document.

- **§21.4 "Workflow override via `bind_topology` ... up to a compile-enforced ceiling" vs. `v2.1_in_progress.md` §1796-1797:**

  §21.4 says N-max is declared at the collection and the workflow can override "up to a compile-enforced ceiling." The v2.1_in_progress framing is the inverse:
  > "`[Fish; some]` compiles to a fixed-size array with validity mask. The workflow layer **must** supply `MAX_CAPACITY` when binding the experiment."

  In the older framing, the `.myco` declaration says "can grow/shrink" and the workflow supplies the capacity; in §21.4, the `.myco` declaration sets N-max and the workflow may override up to a ceiling. These are different locus-of-authority stories. The §21.4 framing is the newer one and consistent with CC1 (values come from workflow but structural positions stay in `.myco`), but the legacy "workflow must supply MAX_CAPACITY" phrasing is still live in v2.1_in_progress.

  `Recommend:` Retire the "workflow must supply MAX_CAPACITY" framing in anti_spec.md:
  "workflow-must-supply MAX_CAPACITY framing for `[T; some]` | `.myco` declares N-max at collection; `bind_topology` overrides up to compile-enforced ceiling (§21.4) | CC1 compatibility".

- **§21.2 "an SCC inherits the most expensive class among its members" vs. `v2.1_in_progress.md` §1660-1666 (`some` bind-static inference):**

  §21.2 states class dominance: any stochastic member promotes the SCC to stochastic. v2.1_in_progress says the compiler infers bind-static vs event-time status for `some`-sized collections based on whether any event targets them. These are compatible in principle, but §21.1's module-level static-vs-dynamic split is coarser than the v2.1_in_progress SCC-level "bind-static (no validity mask updates, static Jacobian after bind)" distinction. The older framing allows a module containing `some`-sized collections with no events to skip mask-update machinery even though events exist elsewhere; §21.1 classifies the whole module as dynamic if *any* event exists.

  `Recommend:` Clarify in §21.1 whether the mask-update machinery is per-collection (switchable off for bind-static `some` collections with no targeting events) or module-wide. If per-collection, add a bullet to §21.4 stating that bind-static `some` collections skip mask-update emission. If module-wide, add an explicit retirement to anti_spec.md:
  "bind-static vs event-time per-collection mask-skip inference | module-level dynamic classification applies mask machinery uniformly (§21.1) | simpler compile-time story".

- **§21.3 "streaming runs use rolling buffers sized to the maximum temporal-lookback depth the module references" vs. `v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §5 ("Memory freed per user retention policy"):**

  §21.3 commits to two storage shapes: T slots for bounded runs, rolling buffer at max lookback for streaming. Chunk 04 §5 says the PyTorch-like lowering uses "per-timestep allocation only for entities with active references. Memory freed per user retention policy." The chunk 04 framing has user-configurable retention policy; §21.3's rolling buffer is sized mechanically by lookback depth with no user knob.

  `Recommend:` Reconcile. Either §21.3 should state that the rolling-buffer size is the mechanical minimum and user retention policy (from chunk 04 §5) can extend it, or chunk 04's "user retention policy" should be marked superseded. The likely resolution is that §21.3 is correct for ground-term storage (keyed by lookback) and chunk 04's retention policy applies to e-graph node garbage collection, not storage slots. A clarifying sentence in §21.3 would resolve the apparent conflict.
