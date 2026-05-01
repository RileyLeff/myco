# Audit Report — §20 SCC Decomposition and Component Classification

Audited against corpus as of 2026-04-22.

§20 is a 15-line skeleton (spec_new.md:3502-3515). It commits to
(a) decomposition of the residual graph into SCCs and (b) a four-way
SCC class (static / dynamic / stochastic / training). Most of the
substantive corpus material on SCCs is not reflected yet; this
audit therefore runs long on Homeless.

---

## Absorbed

Corpus content already reflected in spec_new.md §20.

- **`planning/v2/spec.md` §12.5 (line 2796):**
  > "the planner identifies **strongly connected components** (SCCs). An SCC is a maximal set of quantities where each depends (directly or transitively) on every other."

  Absorbed into §20's opening sentence: "the compiler decomposes the residual graph into strongly-connected components."

- **`planning/v2/v2.1_in_progress.md` §945-947 ("SCC detection, settled"):**
  > "Automatic algebraic loop discovery. No solve blocks. Status: settled."

  Absorbed: §20 treats SCC discovery as automatic with no user-side solve-block annotation.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §1235:**
  > "SCCs — derived from the e-graph at lowering time; the decomposition is not itself in the graph."

  Absorbed structurally. §20 places SCC decomposition after constraint collection (spec_new.md §16.1 layer-3 `SCC decomposition results keyed on SCC identifier` carries the class assignment).

- **`planning/v2/spec_new.md` §19.3 (line 3431-3434):**
  > "Each residual SCC is tagged `static` / `dynamic` / `stochastic` / `training`. The tag determines lowering strategy and backend dispatch."

  §20 is the locus where this four-way tag is defined. §19.3 forward-references it consistently.

---

## Superseded

Corpus content replaced by a newer decision in spec_new.md §20. Should move to anti_spec.md if not already there.

- **`planning/v2/spec.md` §12.3 four-way classification (computational redundancy / square implicit / underdetermined / overconstrained):**
  > "Computational redundancy ... Square implicit component ... Underdetermined residual ... Overconstrained residual."

  This is the legacy **equation-count** classification over residual components. §20's four-way **execution-class** classification (static / dynamic / stochastic / training) is orthogonal to it; the equation-count classification moved to §8.6 / §19.3's three-way overdetermination tag (redundant / provably inconsistent / conditionally inconsistent). Not superseded in the sense of being replaced; the two axes coexist (§19.3 makes this explicit). No anti_spec action needed.

- **`planning/v2/v2.1_in_progress.md` §961-964 ("Four-way component classification, settled"):**
  > "Computational redundancy, square implicit, underdetermined residual, overconstrained residual. Status: settled."

  Same note as above. The v2.1_in_progress four-way was equation-count; §20's four-way is execution-class. `Recommend:` spec_new.md §20 should disambiguate — the same phrase ("four-way classification") now means two different things.

- **`planning/v2/spec.md` §12.5 "Solver classification" (Linear / Polynomial / General nonlinear):**
  > "**Linear**: all relations are linear ... **Polynomial**: relations are polynomial ... **General nonlinear**: the default case."

  Not explicitly retired in anti_spec.md. The linear/polynomial/nonlinear taxonomy is a solver-dispatch decision, orthogonal to the static/dynamic/stochastic/training execution classes. It has not been formally re-stated anywhere in spec_new.md. `Recommend:` Either add to anti_spec.md (if retired) or absorb into §20 / §21 as a solver sub-classification. The chunk 05 matrices report (line 112-114) notes "No structured solver selection ... SCC solver dispatch exists conceptually but Cholesky-for-PSD vs LU-for-general is not formalized in the language." Gap confirmed.

---

## Homeless

Corpus content relevant to §20, not accounted for in spec_new.md §20, and not already committed to anti_spec.md. Primary bucket given the skeletal state of §20.

- **`planning/v2/spec.md` §12.2 step 4-7 (planner procedure):**
  > "4. Identify strongly connected components (SCCs) ... 5. Classify each coupled component ... 6. Eliminate what is eliminable: acyclic derivations become derived nodes, square implicit components become solver blocks 7. Leave the rest as residual blocks"

  §20 does not describe the decomposition procedure beyond "after constraint collection." The acyclic-becomes-derived vs square-becomes-solver distinction is load-bearing for §21 lowering.

  `Recommend:` Add a bullet to §20 stating the pipeline: SCCs form; single-node SCCs (acyclic) lower as forward evaluations; multi-node (coupled) SCCs lower through the class-dispatched path.

- **`planning/v2/spec.md` §9.5 / §12.5 "Hierarchical SCC decomposition" (lines 2306-2417):**
  > "When an SCC contains `deriv` expressions whose results feed back into the same SCC, the compiler attempts to split the monolithic SCC into a nested inner/outer structure that avoids Hessian computation entirely."

  Also affirmed in `v2.1_in_progress.md` §949-952: "When `deriv(A, g_s)` feeds back into its own SCC: inner physics Newton + IFT boundary + outer root-find over control variables. Status: settled."

  `spec_new.md` mentions hierarchical SCC decomposition (§2825, §2830-2851) as a Part II principle but §20 does not. The full detection algorithm (control vs state variable partition, decomposability check, error E0952) lives in legacy spec.md §12.5 and is not restated in spec_new.md.

  `Recommend:` Add a subsection §20.x "Hierarchical decomposition" that (a) states the triggering condition (`deriv` feedback into the same SCC), (b) describes the P / D / X / Y partition algorithm, (c) specifies the failure diagnostic. This is settled design (chunk 04 and spec.md both affirm), just not restated in spec_new.md.

- **`planning/v2/spec.md` §12.5 "Multiple SCCs and gradient chains" (lines 2824-2828):**
  > "When multiple SCCs exist in a single timestep ... and they depend on each other through shared quantities, the emitter generates nested `custom_root` calls. Gradients flow through the full chain via composed implicit differentiation."

  §20 commits to the four-way class but is silent on inter-SCC dependencies and gradient composition. Part II's overview (§2844-2851) mentions tiered nesting but not the IFT chaining rule.

  `Recommend:` Add a bullet to §20 describing inter-SCC ordering: acyclic between SCCs, topological-sort on the condensation DAG, IFT provides exact gradients through the chain.

- **`planning/v2/spec.md` §12.5 "Binding-dependent loops" (lines 2882-2889):**
  > "Different bindings may produce different SCCs from the same model ... In multi-experiment training (section 17), different experiments may produce different SCC configurations for the same model."

  A stable property of SCC classification: it is **post-binding**, not a static-model property. Not stated in §20 or in Part II.

  `Recommend:` Add to §20 that SCC decomposition runs on the dependency graph **after** workflow bindings are applied; the same `.myco` model may yield different SCCs under different bindings. This is load-bearing for multi-experiment training (§25).

- **`planning/v2/spec.md` §12.5 "SCC + slot participation" (lines 2802-2803):**
  > "If a slot's output feeds into an SCC, the slot is part of that SCC."

  `slot` retired, but the semantic claim applies to `bind_controller`: when a controller's output feeds an SCC, the controller opaque factor joins that SCC, and the solver wraps the controller call at every iteration.

  `v2.1_in_progress.md` §902-906 restates this for opaque callables:
  > "When the callable's declared inputs include quantities inside an SCC, the variable joins that SCC as an opaque factor. Symbolic reasoning through the callable is blocked; gradient flow at train time uses the callable's native autodiff. SCC structure is determined by the planner."

  `Recommend:` Add to §20 that opaque controller factors (§24 `bind_controller`) participate in SCC membership; the planner does not special-case them. This is settled and was covered in legacy spec but not restated.

- **`planning/v2/spec.md` §12.5 "Solver convergence failure" (lines 2846-2875):**
  > "In `train` mode, non-convergence must not crash the training loop. The emitter generates a fallback ... convergence penalty ... The implicit function theorem (`custom_root`) computes exact gradients only at a true root. When the solver does not converge, the last iterate is not a root, so IFT gradients are not mathematically valid."

  The two-phase gradient regime (convergence penalty gradient early, IFT gradient after convergence) is settled design for training-classified SCCs. §20 names "training" as a class but does not specify convergence-failure semantics.

  `Recommend:` Add to §20 (or forward-reference to §25 training emission) the convergence-penalty / detached-solver-path gradient handling for training-classified SCCs. This is material that legacy spec treats in detail.

- **`planning/v2/spec.md` §12.6 "Knowledge envelopes" (lines 2891-2925):**
  > "`realization`: `explicit(expr)` ... `implicit(residual_block)` ... or `opaque(provider)`."

  The realization field (explicit / implicit / opaque) is a per-quantity classification orthogonal to the per-SCC classification. It is not restated in spec_new.md §20 or §19.

  `Recommend:` If the knowledge-envelope realization tagging is still in v2.1 scope, reference it from §20 or §19.3. If retired, add to anti_spec.md. Current state is ambiguous.

- **`planning/v2/spec_new.md` §16.1 internal contradiction with §20:**
  > "SCC decomposition results keyed on SCC identifier; carries class assignment (algebraic / stochastic / training / fixed-point / iterative-solve / stepper; §20)." (spec_new.md:2911-2912)

  §16.1 names **six** SCC classes; §20 names **four** (static / dynamic / stochastic / training). "algebraic", "fixed-point", "iterative-solve", "stepper" from §16.1 have no correspondent in §20, and "static" / "dynamic" from §20 have no correspondent in §16.1. This is the single most urgent inconsistency in the §20 neighborhood.

  `Recommend:` Pick one taxonomy and make §16.1 and §20 match. The §16.1 list reads like a **solver dispatch** taxonomy (algebraic closed-form / fixed-point iteration / numerical solve / time stepper), which is a finer dispatch than the §20 execution-class tag. If both are real, distinguish them explicitly: one for execution role (static / dynamic / stochastic / training) and one for solver strategy (algebraic / fixed-point / iterative-solve / stepper). Currently §20 names only the first and §16.1 mashes both. Flagged also under Conflicts below.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 O4.3 / CC3:**
  > "Overconstrained relations must survive extraction with their *original relation names* so training emission can expose them per-residual. Standard CSE-style canonicalization would collapse them."

  §19.2 addresses this as a name-preservation constraint on extraction. §20 does not state that the **training** class specifically depends on per-residual identity for loss exposure; the cross-reference from the SCC class to the extraction constraint is missing.

  `Recommend:` Add a bullet to §20's training-class description noting the per-residual name-preservation requirement (CC3 / O4.3) is a load-bearing invariant for training emission.

- **`planning/v2/v2.1_in_progress.md` §1772:**
  > "Intra-entity SCCs vmapped on GPU. Inter-entity coupling through shared resources."

  A lowering consequence: SCCs that are per-entity (one SCC per leaf, say) lower differently from SCCs that span entities. §20 and §21 are silent on this vectorization aspect of SCC lowering.

  `Recommend:` Note in §20 (or §21) that per-entity SCCs vmap; cross-entity SCCs emit scalar solver code. This is settled per v2.1_in_progress and relevant to the dispatch story.

- **`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` lines 443-457 ("Dynamic topology × matrix shapes — DEFERRED"):**
  > "SCC decomposition of a matrix whose shape changes mid-run is nontrivial. For v2.1: tensor shapes are compile-time known. Document this limitation explicitly; defer dynamic-shape matrices to future work."

  A stated v2.1 limitation on SCCs under dynamic topology. Not captured in spec_new.md.

  `Recommend:` Add a caveat to §20 (or §21 dynamic topology) that SCCs containing matrix-shaped quantities require compile-time-known tensor shapes; dynamic-shape matrix SCCs are deferred post-v2.1.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §1622-1623 "SCC-role predicates":**
  > "SCC-role predicates — 'apply only to e-classes that are the solved root of a Newton SCC'"

  Listed as a fact source for site-scoped rewrite predicates (O4.2). Implies SCC membership and SCC-role assignments are queryable from the rewrite predicate language. §20 does not mention this surface.

  `Recommend:` Add a bullet to §20 that SCC class and membership are exposed to site-scoped rewrite predicates as one of the fact sources (§17 predicate language), enabling predicates like "fire only on e-classes that are Newton-SCC roots."

---

## Conflicts

Direct contradictions between spec_new.md §20 and any corpus document (including spec_new.md itself).

- **§20 four-way class vs §16.1 six-way class (internal spec_new.md conflict):**

  §20:3510-3513:
  > "Each SCC receives a four-way classification: **static** (fully resolved pre-run), **dynamic** (timestepped), **stochastic** (distributional ...), **training** (gradient-optimized)."

  §16.1:2911-2912:
  > "class assignment (algebraic / stochastic / training / fixed-point / iterative-solve / stepper; §20)."

  These are two different enumerations in the same spec_new.md document, both claiming to be the §20 classification. The §16.1 list looks like solver-strategy tags (algebraic / fixed-point / iterative-solve / stepper) plus the pair that aligns (stochastic / training), while the §20 list looks like execution-role tags. They are not reconcilable as-is.

  `Recommend:` Resolve before Phase 2 closes. Two plausible shapes: (a) pick §20's four-way as authoritative and update §16.1 to match; (b) pick §16.1's six-way as authoritative and split §20's "static" into "algebraic + fixed-point" and "dynamic" into "iterative-solve + stepper." Option (a) is simpler; option (b) captures more solver-dispatch information. The solver sub-dispatch could alternatively live in §21 lowering.

- **§20 vs `planning/v2/spec.md` §12.5 "Linear / Polynomial / General nonlinear":**

  §20's classes are static / dynamic / stochastic / training; legacy spec.md §12.5 offers Linear / Polynomial / General nonlinear as the solver classification for square SCCs. Neither is a subset or superset of the other: a "dynamic" SCC might be linear or nonlinear; a "static" SCC might be polynomial.

  `Recommend:` Decide whether Linear / Polynomial / General nonlinear is a live v2.1 sub-classification under §20's "static" or "dynamic" classes (dispatching to LU vs Newton vs symbolic root-finding) and either restate it in §21 lowering or retire it explicitly in anti_spec.md.

- **§20 "stochastic" class vs `planning/v2/spec_new.md` §13.2 tiered stochastic SCC framing:**

  §20 treats "stochastic" as one class. spec_new.md §2836-2842 (Part II overview) describes **tiered** stochastic SCCs: "Tier A stochastic closed-form SCCs ... may nest within deterministic SCCs. Tier B lossy-model SCCs may contain Tier A subcomponents. Numerical solve SCCs may nest around stochastic kernels (§13.8)."

  Not strictly a contradiction, but §20's flat four-way class does not acknowledge the tiering. The stochastic class is actually Tier-A / Tier-B / Tier-C dispatch at lower layers.

  `Recommend:` State in §20 that the "stochastic" class decomposes into the three tiers (closed-form / approximate / opaque) per §13.2's stochastic capability contracts. This aligns §20's summary with Part II's structural claim.
