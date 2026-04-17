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

## Controllers (formerly "Slots")

### ~~Shared-controller portability~~ — RESOLVED (data contracts)
**Decision:** Data contracts (output-only contracts) are the interface a
controller consumes. Any concrete model that satisfies the contract — via
direct fields or via internal relations computing the interface fields —
can be bound to the same controller. `N_SOIL`, VC choice, species sets can
all vary across studies as long as each concrete model exposes the contract's
interface. Shared controller weights thread across studies through the
common contract.

Example: a `Tree` data contract enumerates `interface_leaf_water_potential`,
`interface_vpd`, `interface_soil_water_at_rooting_depth`, etc. SperryTree
with 3 soil layers and Weibull VC satisfies it; SperryTree with 5 soil
layers and van Genuchten VC also satisfies it; both bind to the same
controller at workflow time.

Applies no new language primitive — contracts already do this if we accept
that a contract may have only outputs. See v2.1_in_progress "Data contracts"
section. Multi-study training threads one controller weight tensor across
all bound models — a workflow-layer concern, not language.

### ~~`[*]` wildcard resolves to ~everything~~ — RESOLVED (via slot collapse)
**Decision:** The `slot` keyword has been removed. With slots collapsed
into "unowned node + workflow binding", there is no wildcard syntax on the
`.myco` side. The controller's visibility is its input contract, enumerated
at workflow bind time. No undirected-walk resolution, no exposure-explosion
problem, no invalidation of trained controllers when the mechanistic model
grows new parts (unless the controller's data contract itself changes).

### ~~Transparent controller ABI~~ — RESOLVED (unified interface)
**Decision:** Transparent controllers (heuristic `.myco` modules) and
learned callables (NN/GP/ensemble) share the same binding interface at the
workflow layer — both are owners for otherwise-unowned nodes, both take
the same `input_contract` argument. No separate ABI design needed.

### ~~Slot declaration syntax~~ — RESOLVED (syntax removed)
**Decision:** No slot syntax. The `slot` keyword is gone. Nodes declared
without a relation in the `.myco` are unowned and must receive an owner
at workflow bind time (any of `assume_*`, `learn_*`, `bind_controller`,
`bind_topology`). See v2.1_in_progress "Controllers" section.

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

### Cross-container events
Can events span multiple container types? Currently: events live on the
container that owns the `some`-sized collection. Revisit if cross-container
events prove necessary.

### Within-event conflict tiebreaking
Index order is the default. Should the user be able to specify a tiebreak
function? Related: ship 2-3 canonical event scheduling policies as stdlib
Python (declaration-order, shuffled, priority-by-scalar). Custom callable
stays as power-user escape hatch. Flagged by external review.

---

## Compiler / Runtime

### Solver convergence during early training
An untrained controller outputs garbage → SCC solver diverges → NaN. Every
PINN/hybrid-model project hits this. Need a concrete plan before
implementation. Options:
- Homotopy continuation (gradually increase controller authority)
- Warm-starting from previous timestep's solution
- Pre-training the controller to mimic a hand-coded baseline (Ball-Berry,
  gain-risk optimization) before end-to-end training
- Differentiable projection for box constraints (cheap, reduces penalty burden)

This is a compilation/runtime concern, not language syntax, but it must be
on paper before the training story works. Flagged by external review.

### Constraint enforcement strategy during training
Soft penalty methods are brittle — weight too low, controller ignores
constraint; weight too high, optimization stalls. For sparse multi-study
training, this is exactly the regime where penalties misbehave. Consider:
- Augmented Lagrangian for equality-residual consistency losses
- Differentiable projection for all box constraints, not just admissibility
- Surface `consistency_loss_weight` prominently in diagnostics as a tuning
  knob, not a sensible default
Runtime/compilation concern, not language design. Flagged by external review.

### Closure policy semantic interface
What a policy receives (candidates, enumeration). Spec mentions closure
policies for overdetermined-but-consistent systems but doesn't specify the
API.

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
- **Spec `slot` keyword:** Removed in v2.1. Collapsed into "unowned node +
  workflow binding." Strike §7.1 slot syntax; controllers now bind via
  `bind_controller(target, fn, input_contract)` at the workflow layer.
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
- Can kernels be learned (neural slots)? Concept says yes, syntax undesigned.
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
- `where` on runtime values (e.g., `where mass > threshold`) is piecewise
  function behavior. How does this interact with differentiability? Does the
  compiler need to know about discontinuities?
- **Named-type rules for equality and comparison.** Spec section 4.7 defines
  named-type rules for arithmetic but not for `=`, `<`, `<=`, `>`, `>=`.
  `CarbonPool = WaterPool` should be a compile error (same dimension, different
  semantic type). Extend named-type rules to cover relations and comparisons.

---

## Probabilistic Inference

The residual graph is structurally a factor graph. The scientific bet —
identifying regulatory strategies from sparse data across studies — is
fundamentally a Bayesian inference problem with hierarchical structure (shared
controller, per-study latent trajectories). Point-estimate MLE with MSE losses
leaves uncertainty quantification on the table. A reviewer of the *scientific*
work will demand UQ on the recovered controller.

- Can the residual graph map cleanly to a NumPyro/Pyro program?
- Does the factor graph structure support hierarchical priors (per-study
  random effects, shared controller prior)?
- What changes in the compilation story to support posterior sampling vs
  point optimization?

Not blocking v2.1 language design, but the compilation target architecture
should not preclude it. Flagged by external review.

---

## Compiler Internals (Tier 2)

- `deriv` primitive needs to handle matrix/tensor expressions for non-Euclidean
  spatial operators.
