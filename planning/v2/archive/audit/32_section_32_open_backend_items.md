# Audit Report — §32 Open Backend Items

Audited against corpus as of 2026-04-22.

---

## Absorbed

Corpus content that already landed in spec_new.md §32.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.6 ("Mixed-backend policy"):**
  > "Lean: v2.1 commits to single-backend-per-run. SCC-level is v2.2. Op-level probably never ships."

  Absorbed into §32.1: "Current lean: single-backend-per-run. If a workflow needs specialized handling for one SCC, the intended escape hatch is to run the specialized SCC in isolation and pass its samples / outputs into the main run via workflow-layer glue, rather than to implement cross-backend marshalling in the compiler."

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.6, three-option enumeration (single-backend-per-run / SCC-level / op-level):**
  > Arguments for single-backend-per-run: simpler capability negotiation, no cross-backend data marshalling, deterministic reproducibility. Arguments for mixed: allow a specialized backend per SCC.

  Absorbed into §32.1 arguments-for / arguments-against enumeration.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.8 / §8 Q8 ("First concrete backend"):**
  > "Q8. First concrete backend to implement against (burn? NumPy-on-CPU reference? JAX?) — not a design question strictly, but affects trait shape."

  Absorbed into §32.2 as the three-option list (burn-style Rust tensor stack, NumPy reference, JAX-alike) with the note that choice does not affect the trait-surface design.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.3 preamble (AD ownership fork named as open):**
  > "Option A — Myco owns AD ... Option B — Delegate to backend AD ... Option C — Hybrid. Lean: Option C."

  The §32 preamble names AD ownership as open with the three-way fork and hybrid lean, matching chunk 06 §4.3's lean but not resolving it. The naming of the fork as open is absorbed.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.4 preamble (PPL protocol named as open):**
  > "Open questions: Does the backend see the whole stochastic model at once, or per-factor? How do backend-returned samples participate in further graph computation?"

  The §32 preamble names PPL protocol specifics (message schema, inference-kind enumeration) as open. The higher-level recognition that this is open is absorbed.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.5 preamble (`bind_controller` gradient-flow named as open):**
  > "`bind_controller` ... Unsaid: Which backend runs the callable? How does gradient flow work when the callable is inside a training-time SCC?"

  The §32 preamble names gradient-flow semantics for `bind_controller` callables as open. Absorbed at the naming level.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11, B6 entry:**
  > "B6 — Backend abstraction ... Primary open questions: AD ownership fork ... minimum backend trait API vs capability-advertised optional ops, fallback policy (error / host / emulate), PPL backend protocol concrete form ... opaque callable gradient-flow semantics, mixed-backend policy (lean single-backend-per-run for v2.1), versioning."

  The full enumeration of open questions in B6 maps to the §32 preamble list. The named items (AD ownership, PPL protocol, gradient-flow, mixed-backend, first-backend) are absorbed at the level of naming.

- **`planning/v2/spec_dev_notes.md` §31 / §32 completion notes:**
  > "§32 Open Backend Items — 2026-04-21 complete for in-scope items: 32.1 mixed-backend policy ... 32.2 first concrete backend ... does not affect trait-surface design."

  Confirms §32's two subsections are the designated in-scope content; no further material was dropped during consolidation.

- **`planning/v2/open_questions_deprecated_use_spec_new.md` Compiler Internals (Tier 2), cross-backend callable interop paragraph:**
  > "§31.6 locks that Myco commits to no primary backend; §23.3 locks that trained callables reuse across workflows via plain contracts. What's unresolved: if workflow A trains a callable on backend X (e.g., PyTorch), can workflow B bind the same callable when running on backend Y (e.g., JAX)?"

  The narrow intra-run scope of §32.1 (single-backend-per-run) is consistent with and absorbs the statement that §32.1 caps intra-run scope. The cross-run interop question is tracked separately (see Homeless).

---

## Superseded

Corpus content replaced by a newer decision in spec_new.md §32 or in §31. Should move to anti_spec.md if not already there.

- **`planning/v2/spec.md` §13.2 ("JAX emitter (primary)") and §13.3 final sentence:**
  > "The JAX backend is the primary implementation for v2. Other backends are specified here for interface design but implemented post-v2."
  > "The JAX emitter produces a Python module using: `jax.numpy`, `jax.lax.scan`, `jax.lax.custom_root`, ..."

  Superseded by §31.6 ("no primary-backend commitment — trait-symmetric; burn / JAX / PyTorch / CPU reference all first-class") and anti_spec.md ("JAX-as-primary emitter | backend trait (burn-style) with capability advertising | no primary backend; trait-based"). The primary-JAX framing is retired.

  `Recommend:` The "JAX-as-primary emitter" and "PyTorch-as-primary emitter" retirements are already in anti_spec.md. The spec.md §13.2-§13.3 text remains stale legacy content. No additional action needed beyond the existing anti_spec.md entry; spec.md §13 is listed in anti_spec.md's "Stale in legacy docs" section as supersede-wholesale.

- **`planning/v2/v2.1_in_progress.md` lines 1785-1793 ("Backend targets"):**
  > "Primary target: PyTorch (`torch.compile(dynamic=True)` ...). Secondary: JAX ... Long-term watch: Enzyme + Rust ..."

  Superseded by §31.6 no-primary-backend commitment and by chunk 06 §2's reframing of Enzyme-via-LLVM as "one possible backend implementation, not a committed architecture." The primary-PyTorch / secondary-JAX rank ordering is retired. The Enzyme mention is reframed, not retired.

  `Recommend:` The v2.1_in_progress.md "Backend targets" paragraph (around line 1785) is already noted as stale in anti_spec.md's general stale-doc listing. No new anti_spec.md entry is needed specifically for the backend-targets paragraph; the wholesale-supersede note covers it.

- **`planning/v2/spec.md` §13.3 backend interface (slot-based):**
  > "Emit admissibility projections at slot boundaries."
  > "Emit parameter initialization."

  The slot construct is retired (anti_spec.md). The backend interface in §31 uses the `bind_controller` / capability-advertising / burn-style trait framing, which replaces the slot-based interface in §13.3. Superseded by §31.3.

  `Recommend:` Already covered by anti_spec.md's slot retirement and the wholesale spec.md §13 supersede note. No additional action needed.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §2 stale-doc mentions (Enzyme + Rust framing):**
  > "`v2.1_in_progress.md:1789` mentions 'long-term Enzyme + Rust path for LLVM-level AD' as an implementation direction. No interface lock."

  Reframed (not retired) by chunk 06 §6: "Enzyme-via-LLVM direction ... can be framed as 'one possible backend implementation,' not a committed architecture." The primary-direction framing is superseded; the option remains live.

  `Recommend:` No anti_spec.md entry needed for Enzyme itself; the framing change is handled by the v2.1_in_progress stale-doc note. If the chunk 06 return path locks Option C (hybrid AD), Enzyme becomes the natural "backend owns execution-time AD" implementation candidate and should be mentioned positively in §31 or §32 at that point.

---

## Homeless

Corpus content relevant to §32 that is not enumerated in §32, not committed to §31, and not retired. Highest-value bucket.

### H1: AD ownership Option A/B/C details not enumerated in §32

**`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.3 ("AD ownership — the central fork"):**
> Full pros/cons enumeration for Options A (Myco-owned), B (backend-delegate), and C (hybrid), including:
> - Option A con: "Every decomposition needs a hand-derived adjoint. Multi-year undertaking."
> - Option B con: "Gradient quantities opaque to Myco's analysis — condition-number estimation can't see through them."
> - Option C pro: "Analysis stays rigorous; execution stays fast."
> - Option C con: "Two AD systems, consistency obligation between them."
> - Lean: Option C.

§32's preamble names the fork (Myco-owned / backend-delegate / hybrid, leans hybrid) but contains none of the decision-relevant reasoning. The three-option analysis is the substance of what makes this an open item requiring resolution rather than a trivial choice. §32 provides no guidance for what resolving it would mean or what trade-offs govern the decision.

`Recommend:` Add a §32.3 subsection "AD Ownership Fork" enumerating the three options at summary level and stating the lean (Option C, hybrid) with the two key trade-offs: (a) Option C preserves Myco's compile-time analysis precision for condition bounds and envelope propagation while delegating runtime-execution AD to the backend; (b) Option C requires two AD systems with a mathematical-consistency obligation between them. Chunk 06 §4.3 is the design venue. The subsection need not reproduce the full pros/cons table but should state what the decision entails.

### H2: PPL protocol message schema specifics not enumerated in §32

**`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.4 ("PPL backend protocol"):**
> Compiler emits: envelope metadata, structural declarations, coupling annotations, log-density assembly recipe.
> Backend returns: sample values, gradient estimates, MCMC traces, diagnostic metadata (ESS, R-hat, divergence warnings).
> Open: does the backend see the whole model at once or per-factor? How do returned samples enter the graph (as envelope facts)?

§32's preamble names "PPL protocol specifics (message schema, inference-kind enumeration)" as open. The chunked design work in §4.4 has already sketched the compiler-emitted fields and backend-returned fields at a level that is more than just naming the question. The message schema shape (eight items across two sides of the protocol) is absent from §32 entirely.

`Recommend:` Add a §32.4 subsection "PPL Backend Protocol" summarizing the known shape of the handoff (compiler-emits / backend-returns structure from chunk 06 §4.4) and calling out the two remaining open questions explicitly: (a) whole-model vs per-factor visibility, (b) how returned samples re-enter the e-graph. This converts §32 from a naming-only placeholder into a subsection that can be resolved or delegated to chunk 06 with clear criteria.

### H3: `bind_controller` gradient-flow open details not enumerated in §32

**`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.5 ("Opaque callable protocol"):**
> Unsaid: Which backend runs the callable? How does gradient flow work inside a training-time SCC? Can a neural controller with Matrix/Tensor I/O use a different backend? Portability: can a callable trained on one backend run on another?
> Lean: same-backend-per-run. Cross-backend callable interop is v2.2+.

**`planning/v2/spec_dev_notes.md` (§24.2 completion note):**
> "`bind_controller` gradient-flow semantics (parameter registration at composition; backward pass through backend AD; opaque-fn fallback for non-differentiable controllers; cross-run weight persistence)"

§32's preamble names "gradient-flow semantics for `bind_controller` callables" as open. §31.3 covers the opaque-callable runtime at the trait level. §24.2 in spec_new.md already states parameter registration and backward-pass routing. The unresolved question — what happens when the callable is a non-differentiable function in a training-time SCC, and what the fallback contract is — is not enumerated in §32 and not fully resolved in §31.3.

`Recommend:` Add to §32 (as a bullet under the preamble or as §32.5) an explicit statement of the remaining open for `bind_controller` gradient flow: specifically, the opaque-fn fallback policy when a callable in a training-time SCC is non-differentiable (stop-gradient? compile error? workflow opt-in?). The lean from chunk 06 §4.5 (same-backend-per-run) is already in §32.1; the gradient-flow fallback is the residue.

### H4: Backend versioning policy not in §32 (absent from §31 too)

**`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.7 ("Backend versioning") and §8 Q7:**
> "Backends advertise semantic version. `run.config.backend` includes version pin option. Compiler warns on major-version mismatch; errors on incompatible trait-surface change."
> "Q7. Versioning strategy?"

**`planning/v2/spec_dev_notes.md` §31 completion note:**
> "31.4 backend versioning — trait surface Myco-versioned, backend implementations backend-versioned; plan cache keys on `(plan, trait_version, backend_identity)`."

§31.4 locks the versioning policy (trait Myco-versioned, implementations backend-versioned, cache keyed on triple). This is not a §32 open item. However, chunk 06 §8 lists Q7 as still open. The spec_dev_notes entry for §31.4 implies this was resolved in §31, not §32.

`Recommend:` Verify that §31.4 in spec_new.md contains the versioning policy text. If it does, remove versioning from the §32 open-item scope (it is resolved in §31). If §31.4 is still a stub, add versioning to §32 as an open item. The spec_dev_notes entry suggests §31.4 is complete, so this is likely a cleanup-only action (no §32 change needed, but chunk 06 Q7 should be marked resolved).

### H5: Capability advertising / fallback policy not in §32

**`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.2 ("Capability advertising") and §8 Q2-Q3:**
> "Open question: default policy. 'error' is safest (no silent performance catastrophes); 'host' is most permissive."
> "Q2. Minimum backend trait API vs. capability-advertised optional? Q3. Default fallback policy: error / host / emulate?"

**`planning/v2/spec_dev_notes.md` §31.1 completion note:**
> "31.1 capability advertising + 3 fallback modes (error / host / emulate) with fallback scoped per-run via `run.config.backend`; emulate mode's substitutions enter the approximation-error layer (§16)."

§31.1 locks the three fallback modes and per-run scoping. The default-fallback-policy question (Q3) is resolved in §31.1 (the three modes exist; default is configurable per-run). This is not a §32 open item if §31.1 is complete.

`Recommend:` Same action as H4: verify §31.1 is fully written (not a stub). If so, chunk 06 Q2 and Q3 should be marked resolved, and they need not appear in §32. If §31.1 is a stub, add fallback-default-policy as an open in §32.

### H6: Cross-backend callable interop is §32-adjacent but not enumerated

**`planning/v2/open_questions_deprecated_use_spec_new.md` Tier 2 Compiler Internals:**
> "Cross-backend callable interop. §31.6 locks that Myco commits to no primary backend; §23.3 locks that trained callables reuse across workflows via plain contracts. What's unresolved: if workflow A trains a callable on backend X (e.g., PyTorch), can workflow B bind the same callable when running on backend Y (e.g., JAX)? Weight-format translation, gradient-plumbing compatibility, and advertised-capability reconciliation all need to be specified. The single-backend-per-run policy (§32.1) caps intra-run scope; cross-run interop is the open question."

§32.1 explicitly caps scope to intra-run. Cross-run interop is described in open_questions_deprecated as a separate, post-§32.1 open. It is not enumerated in §32 even as a named future item.

`Recommend:` Add a one-sentence forward reference in §32.1 (or in the §32 preamble) noting that cross-run callable interop across different backends (weight-format translation, gradient-plumbing) is a separate open item tracked in §35 Other Opens. This prevents it from being silently assumed resolved when §32.1 is locked.

### H7: Minimum backend trait API surface not in §32

**`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.1 ("Backend trait minimum API") and §8 Q2:**
> "Open question: where's the minimum? Should every backend implement Cholesky? Or is Cholesky capability-advertised? The line determines how fat the trait is."

The question of what operations belong in the required minimum vs. the optional capability-advertised set is not enumerated in §32. §31 covers the capability-advertising mechanism; the boundary decision (what is required vs. optional) is the load-bearing open. Chunk 06 §8 lists it as Q2.

`Recommend:` Add a §32 bullet (or §32.6 subsection) naming the minimum-vs-optional boundary as an open item with the specific example: whether dense Cholesky is required of every backend or capability-advertised. This is the decision that determines how fat the backend trait is, and it has downstream consequences for which backends can claim conformance with a minimal implementation. Chunk 06 §4.1 is the design venue.

---

## Conflicts

Direct contradictions between spec_new.md §32 and corpus documents.

- **§32.1 "escape hatch is workflow-layer glue" vs. chunk 06 §4.6 SCC-level option named as v2.2:**

  §32.1 states the escape hatch for a specialized SCC is "to run the specialized SCC in isolation and pass its samples / outputs into the main run via workflow-layer glue." Chunk 06 §4.6 states "SCC-level dispatch ... Different SCCs can be annotated with different backends ... Requires SCC-boundary data movement. Lean: v2.1 commits to single-backend-per-run. SCC-level is v2.2."

  These are consistent in outcome (single-backend-per-run for v2.1) but differ in framing. §32.1 describes the workflow-glue pattern as the escape hatch, implying that the user manually splits the run. Chunk 06 §4.6 describes SCC-level dispatch (annotations, compiler-managed boundary data movement) as a future feature (v2.2). The two are not in conflict if "workflow-layer glue" is interpreted as user-managed and SCC-level dispatch is compiler-managed. However, §32.1 does not clarify this distinction.

  A reader of §32.1 could interpret "workflow-layer glue" as an invitation to build an ad-hoc cross-backend data-exchange pattern that would later conflict with the v2.2 SCC-level dispatch design (which implies compiler-managed data movement, not user-managed). If those two become mutually exclusive, §32.1's escape hatch could become a compatibility trap.

  `Recommend:` Add a parenthetical in §32.1 clarifying that the workflow-glue escape hatch is user-managed isolation (two separate Myco runs, outputs passed via Python), not a within-run SCC handoff. This disambiguates from the v2.2 SCC-level dispatch concept and prevents the escape hatch from being implemented in a way that conflicts with the future feature.

- **§32 preamble ("leans hybrid") vs. `anti_spec.md` "user-facing `Dual` numeric representation" retirement:**

  `anti_spec.md` retires user-facing `Dual`:
  > "user-facing `Dual` numeric representation | backend-owned AD | Part V commits backend-delegated AD (burn-style tensor tracks operations); user-facing `Dual` would duplicate backend machinery and risks conflicting with backend AD representation."

  The anti_spec.md entry says "Part V commits backend-delegated AD." §32's preamble says the AD ownership fork "leans hybrid" (Option C), which is distinct from purely backend-delegated (Option B). Under Option C, Myco owns compile-time symbolic AD and the backend owns runtime-execution AD. Under Option B, the backend owns both. The anti_spec.md retirement of `Dual` is motivated by Option B reasoning ("Part V commits backend-delegated AD") but the current lean is Option C (hybrid).

  This is a substantive conflict: the anti_spec.md entry implies Option B is decided, while §32 and chunk 06 §4.3 treat Option B vs Option C as still open (lean C).

  `Recommend:` Correct anti_spec.md's "user-facing `Dual`" retirement entry to state that `Dual` is retired because Myco's compile-time analysis uses symbolic `deriv` (not `Dual` types), and the runtime-execution AD is backend-owned in all three options. The retirement reason is independent of which option wins; the current phrasing ("Part V commits backend-delegated AD") implies Option B is locked, which contradicts §32 and chunk 06. Replace "Part V commits backend-delegated AD" with a reason that is option-neutral: "user-facing `Dual` is redundant with backend AD machinery in all three AD-ownership options and risks conflicting with backend AD representation."
