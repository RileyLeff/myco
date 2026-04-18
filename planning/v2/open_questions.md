# Myco v2.1 — Open Questions

Extracted from v2.1_in_progress.md, chunk reports, design sessions, and
external reviews (Gemini 2.5 Pro, GPT 5.4 Thinking, GPT 5.4 Pro). Organized
by topic. Split into two tiers:

- **Tier 1 — blocks v2.1 correctness.** Must resolve before implementation.
- **Tier 2 — elegance / future work.** Important but not blocking.

---
---

# Tier 1 — Must Resolve Before Implementation

---

## Temporal Semantics

### ~~`d(x)` vs `step(x)` — two surface forms needed~~ — RESOLVED
**Decision:** Two surface forms with distinct semantics.
- `d(x) = expr` — continuous ODE. Compiler owns the integration scheme.
- `step(x) = expr` — discrete update. Compiler applies RHS verbatim at each
  tick. RHS reads prior-tick values; LHS writes current-tick values.

Applied to spec §6.3 and mock_sperry cavitation block. The `rate()` future-
direction note and the old `[t+1]/[t]` explicit syntax have been removed.

### ~~`dt` as workflow-layer concern~~ — RESOLVED (with nuance)
**Decision:** For `d(·)`, `dt` is not referenced in the model — the compiler
owns the integration step size (possibly adaptive, possibly finer than the
user-visible output cadence). For `step(·)`, the tick cadence is set at the
workflow layer via `assume_constant("config.dt", …)` or `assume_series(…)`
for variable time-stepping. `dt` remains a normal quantity in the world
model when it appears as a physical quantity (e.g., a flux accumulated over
an interval) — it participates in dimensional analysis and may have
constraints. It is not a reserved name. See spec §6.3.

---

## Variables, Relations, and Workflow Binding

The key framing principle: the `.myco` layer contains variables and
relations (symmetric constraints) only. Whether a variable's value comes
from the relations, from data, from a callable, or from training is
entirely a workflow-layer question. "Controller" is workflow vocabulary
for the callable attached by `bind_controller`, not a `.myco` kind. See
the canonical glossary in v2.1_in_progress for authoritative definitions.

### ~~Callable reuse across studies (shared-controller portability)~~ — RESOLVED (data contracts)
**Decision:** Data contracts (output-only contracts) are the interface a
callable consumes. Any concrete model that satisfies the contract — via
direct fields or via internal relations computing the interface fields —
can bind to the same callable via `bind_controller(path, fn, Tree)`.
`N_SOIL`, VC choice, species sets can all vary across studies as long as
each concrete model exposes the contract's interface. A single callable's
weights thread across studies through the common contract — a workflow-
layer pattern, not a language primitive.

Example: a `Tree` data contract enumerates
`interface_leaf_water_potential`, `interface_vpd`,
`interface_soil_water_at_rooting_depth`, etc. SperryTree with 3 soil
layers and Weibull VC satisfies it; SperryTree with 5 soil layers and van
Genuchten VC also satisfies it; both bind to the same callable at workflow
time. See v2.1_in_progress "Data contracts."

### ~~`[*]` wildcard resolves to ~everything~~ — RESOLVED (via slot removal)
**Decision:** The `slot` keyword has been removed. There is no wildcard
syntax on the `.myco` side. The callable's visibility is its
`input_contract` argument to `bind_controller`, resolved at workflow bind
time. No undirected-walk resolution, no exposure-explosion problem, no
invalidation of trained callables when the mechanistic model grows new
parts (unless the `input_contract` itself changes).

### ~~Transparent-heuristic ABI~~ — RESOLVED (unified interface)
**Decision:** Analytic heuristics (`.myco` modules or Python functions)
and learned callables share the same `bind_controller` verb. The
`input_contract` argument determines visibility identically for both. No
separate ABI.

### ~~Slot declaration syntax~~ — RESOLVED (syntax removed)
**Decision:** No slot syntax. The `slot` keyword is gone. Variables are
just variables; the workflow supplies sources via binding verbs
(`assume_*`, `learn_*`, `bind_controller`, `bind_topology`, `observe`).
See v2.1_in_progress "Variables, Relations, and Workflow Binding."

---

## Iteration — Compile-Time Structural Introspection

### ~~Structural introspection iteration~~ — RESOLVED (not needed)
**Decision:** Structural introspection is not supported as a language
feature. It was a symptom of a scoping limitation that shouldn't have
existed. The motivating use case (per-XylemSegment cavitation tracking in
mock_sperry) is solved cleanly by declaring `initial` and `temporal`
blocks inside type bodies — state evolution that is intrinsic to an
entity lives on that entity, not as module-scope glue reaching in.

See v2.1_in_progress "Scoping for `initial` and `temporal` blocks."

The Sperry mock needs rewriting: the module-scope `initial cavitation_init
for seg in pathway where seg is XylemSegment:` and corresponding
`temporal cavitation ...` blocks move into `XylemSegment`'s body.

---

## Type System

### ~~Named generic argument sugar~~ — RESOLVED
**Decision:** Positional allowed for exactly-one-parameter types; named
required for 2+ parameters.

- `Scalar<kg>` — positional, single-parameter, idiomatic
- `SperryTree<V = WeibullVC, P = FarquharC3, N_SOIL = 4, N_CANOPY = 2>` —
  named, required for 2+ parameters
- `fn arrhenius<U: Unit>(x: Scalar<U>) -> Scalar<U>` — passthrough falls
  out as positional binding to the local generic

No partial-positional form. Adding a second parameter to a previously
single-parameter type breaks existing positional uses at compile time —
intended, since adding a generic parameter is already an API break.

See v2.1_in_progress "Named generic arguments."

---

## Events (Dynamic Topology)

### ~~Generic events — commit to first-class sugar~~ — RESOLVED
**Decision:** Generic events are first-class. The compiler monomorphizes at
model expansion, emitting one concrete event per type (or const value) in
scope satisfying the bound. Multi-parameter generic events expand over the
cartesian product of their bounds. Const generic events are permitted on the
same footing as type generics. The bound is the sole expansion filter —
context-dependent group membership is out of scope. Users push such
distinctions into the type hierarchy or handle them with `when`-guards.

Named/positional generic argument rules follow the type rule: positional for
a single parameter, named for 2+.

Workflow-layer addressing exposes both the group name (`"recruit"`,
tiebreak by type declaration order) and per-instantiation names
(`"recruit<Loblolly>"`) for fine ordering control.

See v2.1_in_progress "Generic events — first-class" under the Events section.

### ~~Cross-container events~~ — RESOLVED
**Decision:** An event is declared on the smallest container type whose
scope contains all its input/output participant types. For single-container
events this is the container itself (the existing rule). For cross-container
events this is the nearest common ancestor container. When a participant
type appears as a pool in multiple sibling children, the signature uses
dotted paths (`pond.fish -> sky.birds`); otherwise the bare type is
unambiguous. No new syntax — the rule generalizes what was already
implicit.

See v2.1_in_progress "Container scoping rule" under the Events section.

### ~~Within-event conflict tiebreaking~~ — RESOLVED (no language decision)
**Decision:** Scheduling and tiebreaking are entirely workflow-side (Python
layer), same status as random seed. The `.myco` file commits to nothing
about ordering, priority, or randomness. Three canonical policies
(`declaration_order`, `shuffle(seed)`, `priority_by_scalar(quantity_path)`)
will ship as Python library helpers; a unified policy API
`policy(pending_firings, state) -> List[Firing]` covers both between-event
ordering and within-event tiebreaks. Custom callables remain the escape
hatch. None of this affects language design. Priority hints on event
declarations (`event predation priority: predator.mass`) were explicitly
rejected as a workflow-into-model leak.

---

## Compiler / Runtime

### ~~Solver convergence during early training~~ — RESOLVED
**Decision:** The compiler emits warm-started solvers by default. Iterative
solvers (Newton, fixed-point) accept the previous tick's solution as their
initial guess; the workflow can disable warm-start for cold-start
evaluation or regime changes. This is a compilation detail that does not
change emitted residuals or forward values — backend-agnostic, every
solver library accepts an initial guess.

Homotopy continuation (blending controller output with a baseline via
annealing `α`) and pre-training against a hand-coded heuristic (Ball-Berry,
gain-risk) are workflow-layer patterns that the controller-binding API
already supports. No language features needed — shipped as documented
Python recipes, not spec additions.

Differentiable box-constraint projection is covered under the constraint
enforcement decision below.

See v2.1_in_progress "Training emission and constraint enforcement."

### ~~Constraint enforcement strategy during training~~ — RESOLVED
**Decision:** Compiler surfaces refinement-type bounds on unknowns
(`0 <= self <= 1`, `self >= 0`, etc.) as workflow-visible metadata. The
compiler does **not** auto-emit projection — projection flavor is a
training-dynamics choice. Stdlib ships three projection helpers
(`hard_clip`, `sigmoid`, `soft_clip`) the workflow selects per-unknown
or globally.

For equality-residual consistency losses, overdetermined components
expose residuals individually by name (per §14.6 conventions). v2.1
stdlib ships two loss helpers:
- `soft_penalty(weights)` — default. `consistency_loss_weight` surfaced
  prominently with no "sensible" default; user must tune.
- `augmented_lagrangian(weights, mu, lambda_init, mu_schedule)` — opt-in
  for brittle-penalty regimes. Adds `λ_i · r_i` dual term with standard
  multiplier update; two API shapes (PyTorch mutable state, JAX pure
  update). Both helpers read the same `model.residuals` list.

Shipping both helpers at v2.1 gives users an immediate escape hatch from
penalty brittleness and commits us to the per-residual exposure API
needed for AL. Purely additive library code — no compiler or spec
changes required.

See v2.1_in_progress "Training emission and constraint enforcement."

### ~~Closure policy semantic interface~~ — RESOLVED
**Decision:** Spec §14.6 is the authoritative interface. A policy receives
the target quantity and candidate paths (N = M + 1 common case: two
candidates; general case: all C(N, M) maximal square subsystems).
Candidates are named by the producing relation and ordered lexicographically
for cross-version stability. Policies are ordinary `.myco` functions whose
arguments are candidate values and user hyperparameters supplied at
`closure_config` time. Custom policies only operate on values — they do
not access compiler-internal metadata.

**v2.1 stdlib ships three policies:** `weighted_average`, `soft_select`,
`hard_select`. All are ordinary `.myco` functions users could replicate.

**`condition_weighted` deferred** beyond v2.1. Conditioning-aware weighting
requires either a `condition_of(expr)` compiler intrinsic (parallel to
`deriv`) or a compiler-provided black box — both have real cost. Most
v2.1 workflows reconcile overdetermined systems via controller plus
consistency loss rather than via conditioning-aware closure. Revisit
post-v2.1 if demand emerges.

**Spec §8.5 needs patching:** the phrase "via structural introspection on
the competing paths" (line ~2073) is stale — structural introspection was
killed in v2.1. Replace with "custom policies receive only values and
hyperparameters; compiler-intrinsic policies are a future extension."

See v2.1_in_progress "Closure policies."

---

## Spec Maintenance

### Reconcile v2.0 spec to v2.1
The v2.0 spec uses `dyn`, `[t+1]/[t]`, `param`, `assume_*` vocabulary. V2.1
has renamed/replaced all of these. The combined doc forces reviewers to
internalize old syntax then unlearn it. Need either: (a) a clean v2.1 spec
rewrite, or (b) a "what changed from v2.0" delta document separate from the
spec. Flagged by external review as actively confusing.

Specific items:
- ~~**Spec section 6.3 temporal relations:** Currently shows explicit Forward
  Euler. Should use `d(x) = expr` / `step(x) = expr`.~~ — DONE
- **Spec `dyn` keyword:** Update to `impl` / `some` throughout.
- **Spec `assume_*` methods:** `assume_constant`/`assume_series` stay.
  `bind_topology` and `bind_controller` are standalone additions.
- **Spec `param` keyword:** Removed in v2.1. Update references.
- **Spec `slot` keyword:** Removed in v2.1. There is no `.myco`-layer
  syntax that distinguishes a variable intended to be supplied externally
  from any other variable. Strike §7.1 slot syntax. Variables with no
  workflow source remain unknowns in the residual system; variables with
  a `bind_controller(target, fn, input_contract)` workflow binding get
  their values from the callable. "Controller" is a workflow-only term.
- **Spec contracts section:** Add data contracts (output-only), multi-contract
  satisfaction (`: A + B + C`), supertraits (`contract B : A`).
- **Spec §2 module-scope declarations:** Retire the "implicit top-level type"
  mechanism for module-scope `temporal`/`relation`/`initial`. State evolution
  belongs on types; `initial`/`temporal` are legal inside type bodies. Strike
  module-scope forms.
- **Spec §6.3 "at most one initial" rule:** Rephrase as post-expansion,
  per-fully-qualified-path. Type-body `initial` blocks expand into per-
  instance equations; the rule catches duplicates across all expansion
  sources.
- **Spec locus-scoped declarations:** Extend `on locus:` clause to
  `temporal` by symmetry with `relation`.
- **Spec generic events section:** Add a "Generic events" subsection under
  §7 (or wherever events live in v2.1). Cover: declaration syntax over type
  and const generic parameters, monomorphization at model expansion,
  cartesian expansion for multi-parameter events, named/positional argument
  rules matching the type rule, and workflow-layer addressing by group
  name and per-instantiation name.
- **Spec cross-container events:** Clarify that an event is declared on the
  smallest container type whose scope contains all its participants; cross-
  container events live on the nearest common ancestor. Signature uses
  dotted paths when a participant type is ambiguous across sibling children.
- **Spec §8.5 structural introspection:** Strike the phrase "via structural
  introspection on the competing paths" (line ~2073). Replace with language
  that custom closure policies receive only candidate values and user
  hyperparameters; metadata-aware policies (e.g., `condition_weighted`) are
  deferred compiler intrinsics, not `.myco`-level features.
- **Spec §14.6 stdlib list:** Drop `condition_weighted` from the v2.1
  stdlib policy list. Note it as a deferred compiler intrinsic. Keep
  `weighted_average`, `soft_select`, `hard_select`.
- Add lib/bin analogy framing to the spec prose.

---
---

# Tier 2 — Elegance / Future Work

---

## Domain Geometry

The core geometry subsystem is settled (see
`v2.1_chunk_reports/01_geometry_design_report.md`): `geometry` keyword,
`Domain<G>`, `chart`, `topology`, `metric`, `locus`, `requires`, `trace()`,
locus-scoped relations with `replaces` obligation keys, `normal_grad()`,
`identify`, `bind_topology`. Implementation should start with 1D intervals +
1D graphs (sufficient for Sperry/Potkay). What remains:

### Manifold boundary conditions for 2D/3D
The `boundary coord = value:` selector and `normal_grad(field)` work for
axis-aligned boundaries in any dimension. Open:

- **Non-axis-aligned boundaries:** Circular domains, irregular coastlines,
  complex 3D surfaces, microtopographic depressions (in field-ecologist-speak:
  sunken spots in the mud). Need a boundary naming/selection mechanism beyond
  `coord = value`.
- **Additional boundary primitives:** `normal()` (vector itself), `jump()`
  (discontinuity across interface), `mean()` (average across interface)?
- **Periodic boundaries beyond `identify`:** Full design for seam handling in
  2D/3D manifolds. Vector/tensor seam transforms deferred beyond v2.1.
- **Internal interfaces between subdomains/materials.**
- **Tangential/slip conditions for vector fields.**

**Deprioritized:** For ecosystem modeling, terrain-as-field on a flat domain
covers all practical use cases. Irregular boundaries are an elegance/efficiency
concern, not a correctness concern.

### Compiler internals for custom metrics
- **Basis-aware tensor IR:** Custom coordinate-dependent metrics require the
  compiler to derive `g^{-1}`, `det(g)`, Christoffel symbols, and co-normals
  symbolically. Needs a tensor calculus subsystem.
- **Heterogeneous metric units:** Polar's metric `[[1, 0], [0, r^2]]` mixes
  length and angle units across elements. Dimension checker must handle
  per-element unit analysis.
- **Pole / singularity handling:** `locus pole` names the problem. Compiler
  backend must emit L'Hopital limits at poles rather than naive `1/sin(theta)`
  formulas. The `locus` declaration provides the structural information needed.

### Cross-domain coupling at embedding (ambient locus problem)
A 1D root network embedded in 3D soil interacts along its physical extent, not
at an intrinsic locus. This is a kernel coupling problem, not a geometry
problem. The geometry system stays strictly intrinsic. Deferred to kernel
coupling design.

### Plant hydraulics features needing geometry support
- **Embolism-driven edge deactivation** (topology masking — connects to dynamic
  topology / events)
- **Mixed-dimensional coupling** between 1D network and 0D/3D compartments
  (partially addressed by kernel coupling)
- **`rooted_graph` topology class** for cyclic-but-rooted structures
  (anastomosing roots, leaf vein reticulation, fungal mycelium)
- **Edge-level scientific data binding** — per-edge diameter, conductivity,
  vulnerability parameters as model quantities (separate from topology tags)

### Spatial operator catalogue
Full set of spatial operators and their dimensional signatures needed. Currently
settled: `grad`, `diverg`, `laplacian`, `curl`, `normal_grad`. May need more
for specific PDE classes.

---

## Collections & Iteration

### Restricting the type set
Default is all in-scope implementations (at model-module level). If a user
wants to restrict which implementations appear in a specific collection, needs
a constraint mechanism. Deferred until there's demand.

### `softmax` as a primitive
Appeared in the `argmax` smooth-selection example. Stdlib candidate — standard
mathematical operation, compiler could optimize (numerically stable
log-sum-exp). Connected to the smooth-selection syntax TBD (chunk report 02).
Low priority but needed before `argmax` over learned-system populations
becomes common.

---

## Coupling & Kernels

- Is a kernel just a function used inside an `integrate` call, or does it need
  its own declaration? (Leaning toward: just a function — Approach 1.)
- **Coupling topology:** Entity-entity, entity-field, and field-to-field
  coupling should all be expressible. Field-to-field (nonlocal diffusion,
  lateral signaling) falls out naturally if kernels are just functions used
  in `integrate()` over spatial domains. No special syntax per coupling type.
- Can kernels be learned (i.e., bound via a `bind_controller`-like verb)? Concept says yes, syntax undesigned.
- How does kernel sparsity (characteristic length scale) get communicated to
  the compiler for spatial indexing optimization? (Leaning toward: workflow
  layer for opaque/learned kernels, compiler analysis for transparent ones.)
- Is `coupling` a keyword, or just a pattern the compiler detects in
  kernel-weighted integrals?

---

## Conservation

- Scoped conservation? `{ conserved within Pond }` for open systems where
  quantities can leave via declared boundary fluxes.
- How do boundary fluxes interact with conservation? (Birds flying away with
  mass — is that a boundary condition on the container?)
- Does `{ conserved }` work for fields (continuous) as well as scalars?

---

## Type System

- Clarify that "atomic" means leaf of the containment tree (holds a numerical
  value), not "single-field."
- ~~`where` on runtime values~~ — RESOLVED. Runtime `where` is legal
  everywhere and stays piecewise-hard in emitted code. Smoothing is a
  **model claim**: users write the smooth form (`smooth_threshold`,
  `smooth_max`, etc., shipped in stdlib) when they want smooth transitions;
  workflow-configurable sharpness goes in a `universal` parameter. The
  compiler never silently rewrites semantics based on compile mode.
  Training diagnostics for sharp predicates depending on learned parameters
  live at workflow composition (capability layer), not as `.myco`
  annotations — consistent with the no-`.myco`-annotations principle.
  See v2.1_in_progress "`if/else`, `where` conditionals."
- **Named-type rules for equality and comparison.** Spec section 4.7 defines
  named-type rules for arithmetic but not for `=`, `<`, `<=`, `>`, `>=`.
  `CarbonPool = WaterPool` should be a compile error (same dimension, different
  semantic type). Extend named-type rules to cover relations and comparisons.

---

## Probabilistic Inference

### ~~Probabilistic inference integration~~ — RESOLVED
**Decision:** Probabilistic programming is a first-class v2.1 language
feature, not a post-hoc addition.

- **`~` as first-class stochastic relational operator.** Used for
  observation likelihoods, noise models, and stochastic dynamics.
- **Aleatoric/epistemic split.** Noise models live in `.myco` (world
  claim). Parameter priors live workflow-side (experimenter claim) via
  a future `assume_prior` verb.
- **Stdlib distributions** (unit-parameterized where applicable):
  `Normal<U>`, `LogNormal<U>`, `Uniform<U>`, `StudentT<U>`, `HalfNormal<U>`,
  `Exponential<U>`, `Cauchy<U>`, `Beta`, `Dirichlet`, `Gamma<U>`, and
  discrete distributions (`Bernoulli`, `Categorical`, `Poisson`, etc.).
- **User-extensibility via `Distribution<U>` contract.** Required:
  `log_pdf`. Optional: `sample`, `reparameterized_sample`.
- **Automatic marginalization** of finite-support latent discrete
  variables. SCC-aware. Zero `.myco` syntax — the compiler does the work.
- **Truncation via refinement types.** `count: Scalar<dimensionless> where { value <= 100 }`
  + `count ~ Poisson(rate)` composes automatically. No truncation syntax.
- **Stochastic dynamics (SDEs)** expressed via `~` on a time derivative.
  Itô vs Stratonovich interpretation via generics on `~` (`~<Ito>`,
  `~<Stratonovich>`); required only for multiplicative noise.
- **Compilation** targets both probabilistic backends (NumPyro: direct
  HMC/NUTS) and deterministic backends (negative log-likelihood loss
  terms; log-sum-exp for marginalized discretes). Capability errors at
  workflow composition, not `.myco` annotations.
- **No `.myco` annotations.** All tolerance/override decisions are
  workflow-layer (accept-large-enumeration, backend choice, etc.).

See v2.1_in_progress "Probabilistic Programming" and "Refinement types."

---

## Compiler Internals (Tier 2)

- `deriv` primitive needs to handle matrix/tensor expressions for non-Euclidean
  spatial operators.

---

## Workflow Verb Taxonomy

The v2.1 workflow ships eight binding verbs: `assume_constant`,
`assume_series`, `learn_constant`, `learn_initial`, `learn_trajectory`,
`bind_controller`, `bind_topology`, `observe`. Earlier drafts grouped them
into a "four-way workflow vocabulary" (`assume` / `observe` / `learn` /
`bind`), with `bind_controller` and `bind_topology` as siblings under
`bind`.

That grouping may or may not survive the clarified framing now that
"controller" is not a `.myco` kind and `bind_topology` supplies
structure (geometry/connectivity) rather than values. Specifically:

- Is `bind_controller` actually a `bind` verb in the same sense as
  `bind_topology`, or is it more naturally grouped with `learn_*` as
  "supply a trainable source" with the difference being only whether the
  training loop lives inside Myco's learnable parameters or in the
  callable's own autodiff?
- Does the four-verb partition clarify or obscure the fact that each verb
  simply supplies a source of a particular flavor?

Not blocking — verbs themselves are load-bearing on the Python side and
stay. Revisit the taxonomy once callable-binding, topology-binding, and
prior-binding (deferred) have all been designed.

---

## Literal constants in `.myco`

Current spec permits literal scalar constants and universals at module
scope in `.myco` (e.g., `universal R: Scalar<J_mol_K> = 8.314`). Question:
should `.myco` have any literal values at all, or should every value
enter via a workflow binding verb?

Motivation for removing them: `.myco` would become purely structural —
types, contracts, variables, relations, with zero concrete numerical
content. Every value the experimenter brings to the model (physical
constants, parameter guesses, starting conditions, measured data) would
arrive through the same mechanism (a binding verb). This is a stronger
form of "the `.myco` describes what is true, the workflow describes what
you assume" — physical constants like `R` or `stefan_boltzmann` are also
measured and assumed, just at humanity's scale rather than the
experimenter's.

Motivation for keeping them: certain values (dimensional constants, unit
conversions, exact rationals) are intrinsic to the mathematical structure
and awkward to require binding for. Also clarity: a formula like
`value_25 * exp(E / (R * T))` reads naturally with `R` as a universal; it
becomes noisier if `R` must be bound per experiment.

Revisit after more Tier 2 locking. Touches the workflow API design and
the question of whether "what the experimenter supplies" and "what humans
have collectively measured over centuries" are meaningfully different
categories.

---
---

# Deferred — Revisit After More v2.1 Design Locking

Items explicitly postponed with intent to re-address. These are not
blocking v2.1 language design but require their own design passes.

---

## Sequential inference for time-varying discrete latents (HMMs)

A latent discrete variable with Markov transitions over time (mode state
per timestep, phenological stage, regime-switching in SDEs) requires
forward-backward, Viterbi, or particle filter inference — not compile-
time marginalization. The v2.1 compiler detects the pattern and errors
with guidance. Full design covers:

- Syntactic recognition of Markov-structured latent discrete chains.
- Which inference algorithms to generate (forward-backward for likelihood;
  Viterbi for MAP sequence; particle filters for continuous-state
  extensions).
- Integration with the continuous-parameter inference loop.
- Whether PPL machinery (Pyro's `markov`, NumPyro's `contrib.funsor`)
  covers enough to lean on, or whether Myco emits its own forward-backward.

---

## MultivariateNormal and multi-dimensional distributions

Deferred pending vector/matrix/container story lock. The distribution's
log-pdf is standard; the typing question is how mean vectors and
covariance matrices are declared in Myco's type system (container types,
matrix units, positive-definiteness constraints). Revisit once the
container and collection design is settled.

---

## Workflow-side API for epistemic priors

Parameter priors (Bayesian beliefs about unknown values) live workflow-
side. The verb name and signature (`assume_prior(path, Distribution)` or
similar), composition with other `assume_*`/`learn_*` verbs, per-parameter
vs vectorized priors, and hierarchical-prior construction are workflow
design questions. Not blocking v2.1 `.myco` language spec.

---

## Workflow-side capability overrides

The no-`.myco`-annotations principle pushes all tolerance / approximation
/ override decisions to the workflow layer. Concrete verbs still to
design:

- Accept large enumeration states (override the default compile-time
  capability error).
- Choose inference backend (deterministic MLE/MAP, HMC/NUTS, future VI)
  and surface capability mismatches as errors at workflow composition.
- Approximate-inference switches when exact methods (full marginalization,
  HMC on a large model) are infeasible.
- Per-residual projection flavor (`hard_clip`, `sigmoid`, `soft_clip`)
  selection — already partly designed under constraint enforcement.

---

## Variational inference backends and reparameterization machinery

ELBO-maximizing VI via pathwise gradients is a future backend, not on the
v2.1 critical path (HMC/NUTS covers the initial inference target). The
`Distribution<U>` contract reserves the optional `reparameterized_sample`
hook for when VI arrives.

---

## Stdlib distributions beyond v2.1

Additional distributions (truncated continuous with runtime bounds,
mixture distributions, copulas, etc.) may be added over time. The
`Distribution<U>` contract is the extensibility surface; new
distributions should not require language changes.
