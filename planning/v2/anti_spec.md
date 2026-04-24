# Myco — Anti-Spec

Things that are retired, dropped, or wrong. Consult before re-litigating.
Terse by design. Sources: 5-reviewer consolidation audit (2026-04-20),
gap-review stale list, subsequent design locks.

## Retired keywords / syntax

| retired | replacement | why |
|---|---|---|
| `dyn` | `impl Contract` (static monomorph) + `some` (runtime sizing) | clean split of compile-time vs runtime heterogeneity |
| `param` | workflow-bound typed fields | CC1: all values enter from workflow |
| `slot` / `learn_slot` / `bind_slot` / `bind_slot_metadata` | `bind(path, Controller(...))` | controller is workflow-only, no `.myco` kind |
| `[*]` wildcard slot inputs | `Controller(reads=[...], writes=[...], input_contract=..., output_contract=...)` | explicit I/O spec |
| transparent-heuristic ABI | `Controller` source object | one workflow-side mechanism for pluggable behavior |
| structural introspection (`<:` predicate, §5.5/§8.5) | nothing | closure policies see values + hyperparameters only |
| `[t+1]` / `[t]` temporal subscripts | `d(x) = expr` (ODE) / `step(x) = expr` (discrete) | subscripts conflated kinds |
| `rate()` | `d(x) = expr` | same |
| `rule` keyword | `event` | disambiguate from rewrite rules |
| module-scope `initial:` / `temporal:` per-type | in-type-body `initial:` / `temporal:` | module-scope kept only for truly cross-entity relations |
| `const N: usize` | `N: val` | cleaner val-generic spelling |
| `assume_topology` | `bind_topology` | topology materialization is its own workflow verb |
| `has`-style field-presence filtering | `where x is T` narrowing | type-based narrowing |
| `property` declarations (`property sigma is PositiveDefinite`) | refinement types + capability contracts (`Invertible<_>`, `Differentiable`, `Monotone`) + `constraint` blocks | redundant with existing machinery; spec_new.md §6 already forbids user property-declaration surface. mock_sperry.myco flagged for rewrite |
| `DataContract` / "data contract" as distinct contract kind | plain contracts satisfied by a type's output fields | `Controller(..., input_contract=..., output_contract=...)` enforces access; no failure case found where a plain contract + output-type annotation is insufficient |
| `Line1D` / `Rectangle2D` / `Ball3D` dimensional-suffix geometry names | `Interval` / `Rectangle` / `Ball` (suffix-free authoritative names) | dimension is intrinsic to the mathematical object; suffix is noise. Stdlib catalog in §11.3 uses standard names |
| `Polar` / `Spherical` as geometry types | coord parameterization on `as` clause (`Disk as (r, θ)`, `Ball as (r, θ, φ)`) | coordinate systems are annotations on solid regions, not separate geometry types |
| `Sphere` as a solid 3D region | `Sphere` = S² (2-manifold, surface only); `Ball` = solid 3D region | mathematical convention; solid-vs-manifold distinction is load-bearing in §11.3 |
| `laplace` spelling | `laplacian` | one canonical spelling across stdlib and docs |
| `trace(f, junction, edge)` overload for directional limit at graph junctions | `limit_from(f, junction, edge)` | `trace` kept for manifold restriction (standard PDE trace operator); overloading two mathematically distinct operations on one name invites confusion |
| user `fn` declarations | parameterized relations with explicit output slots | user-authored reusable model structure adds graph obligations; expression-position functions are stdlib/compiler-owned atoms |
| `kernel` keyword or kernel kind | parameterized relation accepting two point arguments and an explicit scalar output slot | kernels are not a distinct construct; §6 and §28 state the design positively |
| `approximate A <-> B: under: ...` expr-infix form | `approximate { body: ... under: ... }` block form | chunk 04 earlier draft used an infix-over-equation syntax; block form scopes cleanly to a named body and is what §15.1 locks |
| `loss_of(residual)` intrinsic | `objective_terms(residual)` | "loss" overloaded planner cost, training objective, and likelihood; `objective_terms` is explicitly workflow-facing training decomposition |
| `basis` declaration for matrix axes | plain contracts / field-set shapes plus compiler-facing matrix facts | axis signatures do not need a new source construct; contracts already name fields and units |
| user-marked matrix role types (`LinearMap<...>`, `Covariance<...>`, `Precision<...>`) | matrix facts derived from relations, constraints, stdlib construction provenance, and provider validation | roles would be unchecked annotation mechanics; operations consume facts, not labels |
| shape expressions in runtime value position | structural-position shape expressions only | shapes are compile-time / plan-time metadata for tensor compatibility, not model values relations can observe |
| single canonical matrix-envelope representation | parallel envelope views: entry-wise, norm, spectral, structural | the views carry genuinely different evidence; coercion between them requires named rules |

## Retired annotations / attributes

| retired | replacement | why |
|---|---|---|
| `#[verified_externally]` | nothing | no proof-escape-hatch annotations |
| `#[inverse]` | capability contract (`Invertible<_>`) on stdlib expression atom | unified contract machinery |
| four-class invertibility metadata (`bijective` / `injective_restricted` / `lossy` / `opaque`) | capability contracts on stdlib expression atoms | same |
| all `#[...]` attribute annotations | nothing | `.myco` has no annotation surface |
| user-declared fn invertibility / differentiability / domain | compiler derives relation properties from body composition + stdlib atom contracts | no user property-declaration surface; refactor the relation if compiler can't derive the needed property |

## Dropped features

| retired | status | why |
|---|---|---|
| macros (declarative + derive) | deferred post-v2.1 | generics + contracts + refinements + `{conserved}` + `impl`/`some` cover the boilerplate use cases |
| homotopy continuation as language feature | workflow Python recipe | belongs on workflow side |
| stdlib physical constants (`R`, `Avogadro`, etc.) | workflow-injected via `bind(path, Constant(...))` or stdlib default bindings | physical constants are values; values live workflow-side |
| float and unit-qualified literal numerics in `.myco` value position | CC1: banned in value position; must enter through the workflow as universal bindings. Exception positions (unit defs, affine conversion bodies, structural positions) unchanged. Bare dimensionless integer literals in value position are legal via stdlib desugar to `integer<N: val>` universal (§4). | no two-trust-posture split; all `.myco` files obey one rule; integer-only carve-out keeps ergonomic arithmetic without introducing float-magic-number risk |
| concrete numeric matrix data in `.myco` value position | finite matrix assembly from existing graph values (`matrix[[a, b]; [c, d]]`) plus workflow-bound providers for concrete data; bare dimensionless integer entries retain the ordinary `integer<N>` desugar | matrix assembly preserves graph provenance; concrete numeric arrays remain workflow data |
| terrain-as-field on irregular domain boundaries (v2.1) | terrain-as-field on a flat domain (the supported v2.1 pattern) | irregular-boundary terrain treatment is an elegance and efficiency concern, not a correctness concern; the flat-domain plus terrain-field composition covers all practical v2.1 use cases; irregular boundaries deferred beyond v2.1 |
| dimensionless-ratio literal carve-out (`0.5`, `2.0` in a dimensionless expression) | CC1 applies uniformly: bind the ratio as a universal | earlier drafts allowed "obvious" dimensionless ratios inline; CC1 is now position-based not dimensionality-based, so no carve-out exists |
| universals carrying values (`universal R: Scalar<U> = 8.314`) | `universal R: Scalar<U>` declaration only; value from workflow | CC1 scope |
| contract composition alias (`contract C := A + B`) | nothing | multi-contract satisfaction (`: A + B + C`) + supertraits already cover the bundle case; alias adds a second spelling with no new expressive power |
| user-facing `Dual` numeric representation | hybrid AD boundary (§31) | symbolic/algorithmic derivatives are compiler-owned and runtime AD is backend-owned; user-facing `Dual` would duplicate machinery and risk conflicting with backend AD representation |
| general SMT / theorem-prover guarantee for all shape arithmetic | represented shape-expression AST with staged solver support | hard shape cases must be expressible, but v2.1 only guarantees a conservative automatic solver subset |

## Retired architectural framing

| retired | replacement | why |
|---|---|---|
| JAX-as-primary emitter | backend trait (burn-style) with capability advertising | no primary backend; trait-based |
| PyTorch-as-primary emitter | same | same |
| residual graph as core semantic object | e-graph three-layer split (equational core / envelope metadata / adjacent keyed state); residual = user-facing projection | chunk 04 recommitment |
| compiler auto-emitted admissibility projections | workflow picks projection flavor (`hard_clip` / `sigmoid` / `soft_clip`) | projection-free-compiler principle |
| compiler auto-selected solver | workflow selects | same principle |
| controller as `.myco` construct | workflow-only concept | strict `.myco` / Python split |
| "slot is gone" narrative / "v2.0 had X" retirement prose | none — use anti_spec.md instead of in-spec versioning | consolidation strips versioning prose |
| X-category bundling pole L'Hopital and `identify` as one rewrite shape | X1 (pole L'Hopital, removable-singularity operator substitution) / X2 (identify, quotient-induced value equality via Layer-3 site records) | different data paths: X1 rewrites an operator at a locus, X2 installs a Layer-1 merge mediated by Layer-3 adjacent keyed state. Bundling obscured the geometric-fact-in-Layer-3 / value-equality-in-Layer-1 split. Resolved 2026-04-22 |
| "structural-predicate-gated" as the X-category name | "site-gated strict" | collision with §16.4 structural tolerance and §17.4 structural shape; X fires on a site or geometric predicate owned by a geometry, not on structural envelope properties |
| "eight merge sources" as a monolithic framing (all sources directly write merges) | "eight authorization sources" with direct-writer vs rewrite-class-authorizer split: sources 1, 2, 3, 7, 8 directly write merges; sources 4 (`identify` via Layer-3 site records), 5 (stdlib inverses via E-group), 6 (`convert`) authorize rewrite classes that subsequently effect merges | clarifies that the e-graph's Layer-1 merge surface is narrower than the set of §17 sources; resolves the identify-as-merge-source vs identify-via-Layer-3 tension raised by opus_identify_review.md. Resolved 2026-04-22 |
| within-event index-order tiebreak | §10.4 three-case exhaustive analysis | ordering is not needed once the three cases are classified. The v2.1_in_progress "tiebreak by index order, overridable from Python" framing predates the three-case analysis; §10.4 is the replacement |
| "`deriv` always symbolic / no runtime cost" framing | three-mode lowering (symbolic / algorithmic / runtime) per §14.4 | runtime AD is the authorized fallback for SCCs too large to expand symbolically under the hybrid AD boundary (§31) |
| spec.md §12.3 "canonical evaluator" framing for residual | residual as user-facing projection from the e-graph via `cost_of`-guided extraction (§19) | canonical-evaluator narrative predates the three-layer e-graph; residual is a projection parameterized by cost preference. Subsumed by the broader "residual as core semantic object" retirement but called out for legacy-doc readers |
| Linear / Polynomial / General-nonlinear as first-class SCC taxonomy | matrix-fact dispatch and solver-strategy metadata under §21 lowering | SCC semantic class is only static / dynamic / stochastic / training; algebraic solver strategy is lowering detail |
| two-way plan representation (`forward-derived` vs `solver-block`) | four-way SCC lowering targets (§21.2) | the old two-way split is too coarse once stochastic and training SCCs are explicit |
| backend-specific "mask may be optional" semantics | uniform alive-mask semantics with backend-specific optimization hidden under lowering | semantics cannot depend on PyTorch/JAX-style branch behavior; backend may optimize but not change the plan surface |
| workflow must supply `MAX_CAPACITY` for dynamic collections | `.myco` declares N-max with workflow override up to ceiling | capacity is part of the source model's static bound; workflow can specialize within the declared ceiling |
| compiler-emitted fixed loss-function menu (`obs_loss`, `consistency_loss`, etc.) | residual catalog + workflow-selected objective helpers | training objective composition is workflow policy, not compiler policy |
| two-phase non-convergence penalty semantics as language feature | backend/runtime failure policy plus workflow-selected objective terms | solver non-convergence is backend/runtime behavior; Myco exposes diagnostics and residuals but does not hard-code convergence-penalty injection |
| automatic semantic fallback when required matrix facts are unknown | unmet-obligation diagnostics unless the user explicitly writes a different valid operation / workflow policy | unknown `positive_definite`, kernel-PD, scaling, or axis facts do not authorize opaque handoff or "make it work" behavior |
| treating dynamic topology dimensions as silently static tensor shapes | `ShapePhase` facts (`provider_validated`, `runtime_bounded`, `dynamic_unknown`) | runtime topology counts must carry evidence phase; static-specialized code cannot assume them without proof or provider validation |
| automatic cross-view envelope implication (`entrywise -> PSD`, `PSD -> entrywise`, `norm -> symmetric`, etc.) | named stdlib/compiler implication rules only | envelope views are parallel; no view silently proves another just because both are attached to the same e-class |
| matrix structures as a closed enum / fixed structural-subtype tree | open-world matrix fact-entailment lattice | `PositiveDefinite`, `Diagonal`, `Orthogonal`, etc. are facts/refinements with evidence and implication rules, not a finite list of cases |
| tensor `convert` as precision/layout/device tuning | `approximate` for value-changing precision; backend/provider facts for layout and device placement | `convert` is meaning-preserving type-layer machinery, not execution tuning |
| reshape by element count alone | reshape with a named index bijection that preserves axes, entry-unit laws, patterns, and provenance | same cardinality does not prove the same tensor meaning |
| dense-to-sparse `convert` by threshold or over-approximate pattern | dense-to-sparse only with proven/provider-validated `zero_pattern`; thresholding routes through `approximate` | sparsification can change values and must carry an envelope |
| ordinary-gradient-through-discontinuity semantics | regime-boundary records with explicit crossing policy | gradients flow inside regimes; crossings require one-sided, subgradient, saltation, estimator, relaxation, or strict rejection |
| default auto-smoothing of nonsmooth source models | strict default plus workflow-selected relaxation handlers | smoothing is either a `.myco` model claim or an explicit workflow surrogate, never compiler housekeeping |
| untracked relaxed training plans | relaxation ledger in `hypha explain` / plan IR | relaxed execution must be auditable against the hard source model |
| backend-dependent dynamic-topology semantics | one Myco shape-boundary model with backend capability-advertised lowerings | JAX-style masks, PyTorch-style symbolic dims, CPU dynamic maps, and replanning are execution strategies, not different languages |
| silent in-solve tensor shape mutation | `CapacityMask`, `EventReplan`, or `DynamicKeyed` crossing handlers | changing the vector space is a regime-boundary crossing, not an ordinary value update |
| matrix `@` operator as canonical matmul | ordinary `*` with shape / axis facts governing contraction | Myco keeps math spelling; elementwise product, if needed, is named (`hadamard`) |
| `inv(A)` as canonical primitive | `inverse(A)` | avoid terse alias as the normative spelling; `inverse(A) * b` may still rewrite to `solve(A, b)` |
| distinct `Scalar<U>` and rank-0 `Tensor<U, ()>` semantic types | `Scalar<U>` as normative spelling for `Tensor<U, ()>` | avoids conversion edges and duplicate envelope / AD / distribution machinery |

## Retired open questions (closed or structurally void)

| item | status |
|---|---|
| `dyn` trait-object semantics vs sized | void — `dyn` retired |
| `rule` keyword semantics for topology change | void — renamed to `event` |
| wildcard-slot / slot-declaration / slot-ABI questions | void — slot construct retired |
| structural-introspection iteration | void — introspection retired |
| `[*]` wildcard reachability | void — slot retired |
| homotopy continuation | void — not a language feature |
| `condition_weighted` deferred | resolved — ships via `condition_of` Levels I-III (chunk 04 O4.5) |
| `~` stochastic as e-graph merge | resolved — `~` is layer-2 distributional metadata, not a merge |
| MVN "deferred pending vector/matrix story" | reframed — consumes matrix facts on `Σ`; B5 type-mechanics question resolved by matrix facts |
| `d(x)` vs `step(x)` | resolved — both ship |

## Stale in legacy docs (do not import)

- `spec.md` §2.5, §4.11, §7, §5.5, §8.5, §6.3, §12, §13.2-13.3, §14.6, Appendix A/C — supersede wholesale
- `v2.1_in_progress.md` internal `rule`/`event` contradiction (§984-988 vs §1795-1800)
- `v2.1_in_progress.md` "NEW:" / "renamed from" / "API-break note" / "ships in v2.1" versioning prose
- `chunk 01` `assume_topology` occurrences (10 locations) — pre-verb-lock
- `chunk 03` §8 `condition_weighted` deferral — pre chunk-04
- `mock_potkay.myco` — uses `slot` + `[t+1]` + universals-with-values; full rewrite pending
- `mock_sperry.myco` — uses `property monotone: ...` (retired); rewrite to capability contracts pending
- `open_questions.md` §Spec Maintenance section — migration checklist, not spec prose
