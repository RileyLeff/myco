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
| `kernel` keyword or kernel kind | parameterized relation accepting two input domains and an explicit scalar output slot; point-point same-locus kernels are one specialization | kernels are not a distinct construct; §6 and §28 state the design positively |
| source-level `kernel_apply` / `kernel_operator` / `convolve` as the primary kernel-operator construct | ordinary `integrate` / `sum` expressions recognized by compiler facts | kernel operators are normal Myco math; optional helpers may only be transparent desugaring once real use demands them |
| radius-only compact support as the complete support model | predicate-shaped `support` / `zero_when` facts plus structured summaries such as `metric_radius`, `graph_hop_radius`, boxes, and directionality | locality can be metric, graph, directional, anisotropic, or domain-specific; radius is an optimization summary, not the ontology |
| source-level sparse storage claims (`CSR<K>`, `block_sparse W`, `use_neighbor_list`) | planner-owned lowerings from exact pattern / index facts | storage layout is execution policy; `.myco` describes the mathematical object and its evidence-backed facts |
| `approximate A <-> B: under: ...` expr-infix form | `approximate { body: ... under: ... }` block form | chunk 04 earlier draft used an infix-over-equation syntax; block form scopes cleanly to a named body and is what §15.1 locks |
| `loss_of(residual)` intrinsic | `objective_terms(residual)` | "loss" overloaded planner cost, training objective, and likelihood; `objective_terms` is explicitly workflow-facing training decomposition |
| `basis` declaration for matrix axes | plain contracts / field-set shapes plus compiler-facing matrix facts | axis signatures do not need a new source construct; contracts already name fields and units |
| source `basis` / mode declaration for kernel approximations (`basis Fourier<N> on Domain`) | ordinary feature / mode relations plus workflow artifacts and compiler facts | feature maps, modes, eigenfunctions, inducing points, and random features are not a special source construct |
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
| backend runtime AD results as symbolic derivative facts | runtime AD values/provenance unless compiler-derived or certified by an audited backend capability | keeps opaque gradients from silently authorizing rewrites, envelopes, or conditioning facts |
| Tier C per-factor PPL handoff | whole unresolved stochastic SCC serialized as one `InferenceTask` after Tier A/B exhaustion | PPL backends need the joint posterior geometry, shared latents, observations, and constraints |
| backend-returned PPL samples as new parametric envelope facts | opaque draws / empirical summaries with provenance | posterior samples do not prove a closed-form distribution family |
| silent gradient stop at an opaque callable | workflow-composition error unless the workflow explicitly marks a gradient stop | accidental black boxes should not quietly sever training gradients |
| general SMT / theorem-prover guarantee for all shape arithmetic | represented shape-expression AST with staged solver support | hard shape cases must be expressible, but v2.1 only guarantees a conservative automatic solver subset |

## Retired architectural framing

| retired | replacement | why |
|---|---|---|
| JAX-as-primary emitter | backend trait (burn-style) with capability advertising | no primary backend; trait-based |
| PyTorch-as-primary emitter | same | same |
| NumPy / CPU reference as privileged language backend | semantics-complete CPU reference as first conformance implementation target | first implementation target is for correctness and debugging; backend trait remains symmetric |
| fat backend trait requiring every scientific operation | `CoreBackend` plus advertised capabilities / capability profiles | keeps backend portability honest without making every backend implement Cholesky, SVD, PPL, sparse kernels, dynamic axes, and runtime AD modes |
| residual graph as core semantic object | e-graph three-layer split (equational core / envelope metadata / adjacent keyed state); residual = user-facing projection | chunk 04 recommitment |
| one unified e-graph where types are terms | separate type graph + expression e-graph with explicit live guard-discharge bridge | equality, implication, contract satisfaction, and conversion legality are different relations; merging them into one equational substrate blurs Myco's fact discipline |
| type graph erased at elaboration as semantic model | live monotone guard discharge, with precompiled / cached guards allowed only as optimization | facts discovered during saturation can unlock later rewrites; caching must not be a semantic limit |
| refinement casts or source-level proof witnesses | refinements as evidence-backed facts attached to e-classes | `positive_definite(A)` is a fact about `A`, not a new value or user-plumbed witness argument |
| ambient generic parameter variance | generic parameters invariant by default; parameter relationships are explicit facts, conversions, obligations, or dispatch rules | prevents silent substitution across units, shapes, runtime sizing, and contract heterogeneity |
| conversion legality as execution-cost choice | type graph owns semantic conversion edges; extraction / lowering owns realization cost | cheap or convenient does not make an illegal conversion legal; legal conversions may still be expensive or unsupported on a backend |
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
| `gram(k, xs, ys)` for cross-domain finite kernel assembly | `kernel_matrix(k, xs, ys)` | `gram` is reserved for same-domain covariance assembly; cross-domain / rectangular kernel evaluation should not imply covariance facts |
| symmetry / PSD / covariance inferred from any kernel-shaped relation | facts derived from kernel contracts and construction provenance (`SymmetricKernel`, `PositiveDefinite`, `StrictPositiveDefinite`, etc.) | kernel-shaped syntax alone does not prove same-domain covariance structure or authorize downstream linear-algebra rewrites |
| compiler-inserted jitter / pivoting / opaque handoff to make Gram matrices positive definite | prove `StrictPositiveDefinite` plus `distinct(points)`, explicitly model jitter, or select a PSD-compatible primitive / workflow policy | numerical stabilization changes the model or the algorithmic contract; Myco must surface that choice instead of making the covariance "work" silently |
| hidden noncanonical measure / density / empirical weight inside a kernel integral | canonical domain measure only; all model-specific densities and weights appear as ordinary graph factors | geometry can supply length / area / volume / counting measure, but biological, empirical, quadrature, and normalization weights are model claims |
| user-marked kernel-operator annotations | compiler recognition of ordinary `integrate` / `sum` expressions plus emitted operator facts | users write the math; the compiler recognizes linear kernel actions when evidence supports them |
| silent quadrature or finite approximation of continuous kernel integrals | exact symbolic closure when provable, otherwise workflow-selected approximation policy or explicit `.myco` `approximate` claim with provenance / error ledger | continuous-world claims must not become finite backend computations invisibly |
| unchecked user-declared compact support (`property k is CompactSupport(r)`, source-level support assertion) | support facts from visible relation bodies, audited stdlib facts, or provider-validated finite artifacts | source assertions would reintroduce property annotations / proof escape hatches |
| deriving sparse zero patterns from `support` alone | derive exact sparse patterns from `zero_when` over concrete finite axes or provider validation | closed support, nonzero region, and exact-zero predicates are distinct; boundary points can be in support and still evaluate to zero |
| treating kernel tail bounds as exact zeros | tail bounds produce approximation opportunities / envelopes only | infinite-tail kernels remain nonzero unless explicitly modeled as truncated or approximated by workflow policy |
| silent infinite-tail kernel truncation for speed | explicit `.myco` truncated model or workflow-selected approximation with `truncation_of`, error, and relaxation ledger facts | small is not zero; truncation changes either the model or the execution approximation |
| assuming all support boundaries are smooth or all are discontinuous | boundary-specific `boundary_smoothness` facts gate gradient / event behavior | Wendland-style compact support and hard cutoffs have different differentiability obligations |
| exact sparse/index lowering without complete coverage | `complete_for(index, support_predicate, axes)` for exact lowering; approximation policy when coverage may miss nonzero pairs | false positives are fine with predicate checks, but false negatives change the computation |
| approximate nearest-neighbor / thresholded sparse indexes as exact plans | approximate-index lowering with error / relaxation ledger facts | performance indexes that may omit nonzero pairs are approximation, not storage |
| provider-validated pattern as relation-level kernel fact | artifact-level facts (`zero_pattern`, `complete_for`, `pattern_phase`, provenance) scoped to the concrete run artifact | provider evidence can satisfy run obligations but cannot prove unchecked `support(k)` globally |
| reusing sparse patterns after their dependencies change | `pattern_phase`, `depends_on`, and `invalidates_on` facts with rebuild / query / replan policy | geometry-dependent locality can change during growth, movement, or events |
| sparse kernel support as matrix-only machinery | operator-general sparse/index lowering, including matrix-free kernel actions and neighbor sums | many kernel actions should never materialize a matrix |
| workflow storage policy authorizing approximation | separate exact lowering policy vs approximation policy | choosing CSR vs dense is exact execution policy; dropping pairs or truncating tails changes semantics |
| approximate low-rank / feature transform as an exact rewrite | exact separability / exact feature facts, or explicit approximation provenance with scope and envelope | Nyström, RFF, truncated spectral / HSGP, and SVD truncation usually change the object unless exactness is proven |
| finite matrix low-rank factorization as relation-level kernel fact | scoped `approx_matrix` / artifact facts unless a relation-level expansion is proven | a factorization of one Gram matrix says nothing global about the kernel relation |
| approximation envelope scope inflation | explicit `approx_relation`, `approx_matrix`, or `approx_operator` scope with named propagation rules | matrix error, relation error, and operator error are different claims |
| low-rank PSD covariance upgraded to PD | PD only from full-rank evidence or explicit positive diagonal / noise component | low-rank `Phi Lambda Phi^T` is generally PSD and rank-bounded, not automatically Cholesky-ready |
| random-feature workflow draws as source-model stochastic roots | workflow artifact provenance (`feature_draw`, seed, probabilistic error) unless the user explicitly writes model stochasticity | randomized algorithms are execution approximations, not world randomness |
| HSGP as a GP-only special language mechanism | spectral truncation approximation over explicit domain / boundary / mode facts | HSGP is a consumer-facing workflow pattern over general feature-expansion semantics |
| silent jitter / stabilization inside Nyström or low-rank solves | ordinary solver obligations plus explicit stabilization / PSD-compatible primitive selection | inducing-point blocks still obey matrix fact requirements |
| treating dynamic topology dimensions as silently static tensor shapes | `ShapePhase` facts (`provider_validated`, `runtime_bounded`, `dynamic_unknown`) | runtime topology counts must carry evidence phase; static-specialized code cannot assume them without proof or provider validation |
| automatic cross-view envelope implication (`entrywise -> PSD`, `PSD -> entrywise`, `norm -> symmetric`, etc.) | named stdlib/compiler implication rules only | envelope views are parallel; no view silently proves another just because both are attached to the same e-class |
| matrix structures as a closed enum / fixed structural-subtype tree | open-world matrix fact-entailment lattice | `PositiveDefinite`, `Diagonal`, `Orthogonal`, etc. are facts/refinements with evidence and implication rules, not a finite list of cases |
| tensor `convert` as precision/layout/device tuning | `approximate` for value-changing precision; backend/provider facts for layout and device placement | `convert` is meaning-preserving type-layer machinery, not execution tuning |
| reshape by element count alone | reshape with a named index bijection that preserves axes, entry-unit laws, patterns, and provenance | same cardinality does not prove the same tensor meaning |
| dense-to-sparse `convert` by threshold or over-approximate pattern | dense-to-sparse only with proven/provider-validated `zero_pattern`; thresholding routes through `approximate` | sparsification can change values and must carry an envelope |
| scalar-only `Distribution<U>` contract | `Distribution<S>` over a sample type; scalar distributions use `Scalar<U>` as S | multivariate, simplex, discrete, and structured joint samples need the same contract story without pretending every sample is one scalar unit |
| required `sample` / `pdf` methods on `.myco` distributions | visible `log_density` relation; `density` / `pdf` default convenience; sampling as backend/runtime capability | sampling is execution behavior, not an ordinary graph relation; `pdf` adds no core obligation beyond `log_density` |
| user-authored opaque stochastic density in `.myco` | visible user-authored `log_density`; opaque stochastic families only as curated stdlib/backend capabilities | opaque user densities are an annotation escape hatch and cannot grant compiler facts safely |
| free-floating `correlate(x, y, rho)` or covariance patches after independent draws | dependence lives inside a structured joint family, visible shared-latent construction, or same e-class identity | correlation is not a post-hoc relation between already-independent stochastic roots |
| tuple / positional destructuring of joint stochastic samples | record-`~` sugar over named fields, desugared to one structured joint root plus `.at()` projections | joint dependence needs stable field names and one root; positional fields are brittle and hide coupling provenance |
| independence inferred from distinct field names within one joint root | same-root fields are dependent unless the joint envelope proves independent partitions or a dependency graph | names are labels, not probabilistic evidence |
| implicit field projection from enum-typed values (`stage.height`) | explicit narrowing (`match`, or event `where ... is Variant` guard) before field access | common field names are not structural evidence; narrowing keeps shape changes visible |
| enum wildcard/default match arm in the core surface | explicit exhaustive arms for every declared variant | adding a variant should surface a type-check diagnostic, not disappear into a catch-all |
| expression-position `match` returning a value | body-form `match` whose arms contain ordinary equations / relation calls | aligned with the user-`fn` ban and parameterized-relation lock; branch bodies add graph obligations visibly |
| enum variant transition via `=` assignment | event-only `becomes` with explicit next-variant construction | `=` is equality / relation claim machinery; variant change is a regime-boundary event effect |
| implicit same-name carryover in enum variant transitions | every next-variant field supplied explicitly | same field name does not prove same meaning, unit, or provenance |
| enum variant transition outside `event` bodies | event-boundary `becomes` only | shape/discriminant changes are regime-boundary crossings, not mid-solve mutation |
| tombstoned access to removed enum-variant fields | removed fields leave scope unless explicitly copied into the next variant or an event/history record | history is a model claim, not implicit storage |
| generated Python mirror classes for `.myco` enums | catalog-validated dumb-data tagged records, plus optional thin helpers | Python remains catalog-driven and does not mirror source type definitions |
| lifted arithmetic or `materialize(prior, out)` sugar for `Prior<S>` in v2.1 | explicit exhaustive `match` on `Prior<S>` | keeps `~` visible for the first compiler; sugar waits for real model pressure |
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
