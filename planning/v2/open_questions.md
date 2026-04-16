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

## Slots & Controllers

### Shared-controller portability — named-port interface
The spec (§7.1) requires identical model instantiation for shared controllers.
This directly conflicts with the multi-study training thesis: if study 24 has
NSC data and study 41 has canopy dieback, you want to share the controller
across them — but different `N_SOIL` or species sets break structural identity.

**Options to consider:**
- **Named-port system:** Slot declares semantic ports ("leaf water potential",
  "VPD", "soil water at canopy depth"). Model binds ports to paths. Controllers
  are compatible iff port signatures match. Decouples controller identity from
  model detail.
- **Explicit `inputs = [...]` as default** with `[*]` as sugar. Forces the
  scientist to reason about the controller's input interface once and have it
  be stable across model revisions.
- **Semantic interface layer:** Separate the controller's abstract interface
  (named physical quantities it consumes) from the model instantiation that
  provides them. Two trees with different `N_SOIL` can both provide "soil water
  at canopy depth" via a derived quantity.

This is the critical path for the science. Prioritize over most other v2.1
work. Flagged by external review as potentially structural flaw, not just
conservative constraint.

### `[*]` wildcard resolves to ~everything
The undirected equality walk from `[*]` will reach nearly every quantity in a
connected model (atm.temperature → leaf.temperature → photo.assimilation →
nsc.C → allocation → root_depth...). Structurally correct (NN picks its
inputs) but practically means enormous input dim, hurting sample efficiency
on sparse data. Also: adding a soil microbial carbon pool to the mechanistic
model invalidates every previously trained controller's manifest.

Connected to the named-port redesign above. If ports are explicit, `[*]`
becomes "resolve the default port set" rather than "the controller sees
everything."

### Transparent controller ABI for wildcard/metadata
Learned wildcard slots have a precise ABI (element-local/global partition,
vmap, metadata). Transparent controllers (`.myco` files imported as relations)
just "get path-rebased." Need to define how wildcard partitioning and metadata
work for transparent controllers.

### Slot declaration syntax
v2.0 uses `slot stomatal_control provides [...]: inputs = [*]`. v2.1 summary
mentions `slot name(inputs) -> outputs` but gives no detailed design. These
are different syntaxes for the same feature. Needs confirmation of which form
is canonical, or a proper design pass. Flagged by mock_sperry update.

---

## Iteration — Compile-Time Structural Introspection

### Structural introspection iteration
`for seg in pathway where seg is XylemSegment` — iterating over a type's
*fields* filtered by contract implementation. This is compile-time structural
reflection, not runtime collection iteration. The v2.0 spec (§5.5) supports
this pattern, but the v2.1 iteration design (chunk report 02) only addresses
runtime collections (`some`-sized, graph neighborhoods). Needs explicit
confirmation that compile-time field introspection is a supported iteration
form, or the Sperry mock needs a different way to express "apply this to every
xylem segment in my hydraulic pathway." Flagged by mock_sperry update.

---

## Type System

### Named generic argument sugar
The v2.1 rule requires all generic args to be named (`Scalar<U = kg>`), but
single-parameter types make this verbose. Options: (a) positional is fine when
there's exactly one type parameter, (b) always named, update all examples.
Also: passthrough case `Scalar<U = U>` inside `fn arrhenius<U: Unit>` needs
sugar (just `Scalar<U>`). Flagged by mock updates — every worked example in
the design docs uses positional form. Blocks writing correct examples.

---

## Events (Dynamic Topology)

### Generic events — commit to first-class sugar
Generic events (`event recruit<S: Photosynthesis>: -> Tree<S>`) are currently
described as sugar over macros. Should be committed to as first-class syntax
that the compiler monomorphizes. The macro workaround is a fallback, not the
primary mechanism. For 50 species, source-level generic events with automatic
monomorphization is the right UX. Flagged by external review.

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
  `bind_topology` is a standalone addition. Update accordingly.
- **Spec `param` keyword:** Removed in v2.1. Update references.
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
