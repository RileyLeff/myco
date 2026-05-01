# Myco v2.1 — Open Questions

Extracted from v2.1_in_progress.md, chunk reports, design sessions, and
external reviews (Gemini 2.5 Pro, GPT 5.4 Thinking, GPT 5.4 Pro). Organized
by topic. Split into three tiers:

- **Tier 0 — substrate commitments.** Foundational decisions that shape
  the entire rest of the design. These are prerequisites, not features.
  Everything downstream leans on these being answered.
- **Tier 1 — blocks v2.1 correctness.** Must resolve before implementation.
- **Tier 2 — elegance / future work.** Important but not blocking.

---
---

# Tier 0 — E-Graph Foundations (HIGHEST PRIORITY)

## Context

The v1 spec (§6 "Core Semantic Model and E-Graph Representation", §6.2 "Why
An E-Graph-Backed Core") committed unambiguously to an e-graph as Myco's
internal equality substrate, backed by the `egg` Rust crate. During the v2
rewrite, the defining section was rewritten and the remaining e-graph
references became orphans; the fix chosen was to remove the references
rather than redefine the section. The result: the current v2 spec and
v2.1_in_progress.md describe behavior that only makes sense with an e-graph
underneath (residual graph, closure policies, three-way overdetermination
classification, multi-relation quantities, controller "merging," function
inverses, named-type converts, `identify` seams, `replaces` obligations),
without ever naming the substrate.

This is a regression, not a design pivot. The e-graph commitment needs to
be restored before the rest of v2.1 can claim a coherent story. Placing
it in Tier 0 (ahead of Tier 1) reflects that downstream correctness
depends on it.

**Primary reference documents:**
- `planning/v1/spec.md` §6 — the v1 commitment that should be restored
- `/tmp/egraph_foundation_audit.md` — the full 15-page audit dispatched
  2026-04-19 (ephemeral; re-run if needed)
- `planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md` §7 — the only
  v2.1 document that currently owns up to the regression

---

## Author's pre-audit list of topics that need e-graph treatment

These are the topics Riley flagged before the audit was run. Each is an
open design question in its own right:

- **Units and the e-graph.** If two expressions are equal in the e-graph,
  must they have compatible units? Can unit inference use e-class merging?
- **Types and the e-graph, especially `From`/`Into` conversions.** If
  `Celsius: From<Kelvin>` exists, are `c` and `k` in the same e-class, or
  different classes connected by conversion edges? What about named types
  that share a representation?
- **Overdetermined systems and the e-graph.** Existing closure-policy
  machinery (`weighted_average`, `soft_select`, `hard_select`) is
  conceptually extraction over an e-class with multiple evaluators.
  Needs to be made explicit.
- **Underdetermined systems and the e-graph.** What does an unknown-but-
  related quantity look like in the e-graph? Can symbolic math (inversion,
  substitution) help narrow an underdetermined system via the e-graph?
- **Symbolic math and the e-graph.** `deriv`, `integrate`, algebraic
  rewrites — are these eagerly normalized, lazily materialized, or kept
  as opaque wrappers? Each has different e-graph consequences.
- **Collections (vectors, arrays, heterogeneous collections) and the
  e-graph.** Is an array of N quantities N separate e-classes, or one
  e-class of an array? How does indexing interact with e-class membership?
- **Continuous vs discrete representations in the e-graph.** A continuous
  domain (`Real` interval) vs a discrete enumeration (canopy layers). How
  do these live in the same e-graph? What happens when a relation couples
  them?
- **Functions and contracts in the e-graph.** A function call `f(x)` —
  distinct node, or participates in e-class merging when `f` is inverted?
  Contracts (bounded generics) — do they constrain which classes a
  quantity can belong to?

---

## Claude's pre-audit additions

- **Temporal layer.** `[t-1]` references, `initial` blocks, events — is
  it one e-graph spanning all timesteps, a per-timestep e-graph, or a
  template e-graph that's instantiated? This has big implications for
  rewrite scope.
- **Stochastic equality `~` vs strict `=`.** Do they produce the same
  e-class merges, or is `~` a different edge type with probabilistic
  semantics the extractor must respect?
- **Inequalities / domain constraints.** Not equalities, so not e-class
  merges — but they still need to travel with the e-class for interval
  propagation. Where do they live relative to the substrate?
- **Opaque callables (`bind_controller`, neural nets, external Python).**
  Black-box nodes — no internal structure the e-graph can rewrite. How
  does extraction handle "this subtree is not rewritable"?
- **Events / topology change.** An event that births or kills entities:
  does this invalidate parts of the e-graph? Require reconstruction?
  Or is topology-change represented *in* the e-graph?
- **SCCs / algebraic loops.** E-graph handles rearrangement; but once
  a loop is genuinely a loop, the compiler hands off to a solver.
  Where's the boundary?
- **Learning targets / free variables.** `learn_constant` means a
  quantity's value is free to be picked by gradient. What is a free
  variable in the e-graph — a class with no canonical extraction?

---

## Audit findings (summary)

The full 15-page audit catalogues findings per-document with severity
tags (`must-resolve` / `should-resolve` / `note`). Key consolidated
results follow.

### The single biggest unstated semantic

**When do two expressions get merged into one e-class?** Pure syntactic
identity? After unit/convert normalization? Only after user-declared
equivalence via `=`? This decision drives spec §6.4 (multi-relation
quantities), §12.3 (three-way classification), §4.7 (named-type convert),
§9.2 (function inverse), §7.2 (controller merging), §14.6 (closure
policies). The spec today relies on "merging happened somehow" without
saying how. (Audit A.3.3, A.3.8.)

### Orphaned e-graph / equality-substrate references in current text

Phrases that imply an equality substrate but no longer have a defined
referent:

- spec §8.5 "structural introspection on the competing paths"
- spec §12.3 "computational redundancy: algebraically equivalent
  evaluators of the same solved component"
- spec §12.3 "canonical evaluator"
- spec §12.6 `realization`, `free_variables`, `bounds`, `obligations`,
  `resolver_sets`, `provenance` (all e-class-level metadata in
  substance, not named as such)
- spec §14.6 closure policies "blend alternative evaluators"
- spec §7.2 "their relations merge into the model graph"
- v2.1_in_progress `replaces balance(axial_flux)` via obligation keys
- v2.1_in_progress `identify ... <-> ...` (a literal e-class merge)
- `03_kernels_in_progress.md` §7 (the only doc that owns up)

### Design surfaces that assume an e-graph without saying so

Ordered roughly by how much of the surface stops working without one:

1. Multi-relation quantities with three-way classification (spec §6.4,
   §12.3)
2. Closure policies (spec §14.6)
3. Operation algebra + function inverse (spec §8, §9.2, §9.3)
4. Convert / named-type / unit equalities (spec §4, §4.7)
5. Controller merging (bind) (spec §7.2)
6. Spatial operator lowering (geometry report §2, §3)
7. Seam identification `identify` (geometry report §2.8)
8. `replaces` obligation retraction (geometry report §2.6,
   v2.1_in_progress)
9. Pit-pole L'Hopital rewrites (geometry report §8.4)
10. Temporal / initial / step isolation (spec §6.3) — must live outside
    the e-graph per v1 §6.2; must be stated
11. Stochastic `~` and marginalisation (v2.1 in progress)
12. Events / topology mutation (v2.1 in progress) — biggest departure
    from v1's static-e-graph assumption
13. Type narrowing `where x is T` (collections report §3.5) — contextual
    e-graph
14. Heterogeneous `argmax` tagged handles (collections report §4.3) —
    runtime-sum e-node kind
15. Kernel optimisation surfaces (kernels report §7)

### Novel frontiers beyond v1

Places where v1's static-world commitment doesn't cover us:

1. **Events / dynamic topology.** v1 assumed static structure. Events
   create/kill entities at runtime. Rebuild the e-graph per event?
   Version it? Template + per-entity instantiation? Biggest structural
   question downstream.
2. **Stochastic `~`.** Is it an e-class merge (samples tied to e-class
   instances, rewrites respect noise model) or a different edge type
   (separate analysis pass)? Not even gestured at in v2.1 yet.
3. **Heterogeneous `argmax` tagged handles `(pool_id, index)`.**
   Runtime sum type. Can't be compile-time merged across branches;
   needs a new kind of e-node.

### Top-10 priority list (from audit Part B.3)

Ordered by how much they block the v2.1 spec from having a coherent
substrate story:

1. **Restore the v1 §6.2 commitment in the v2.1 spec.** State that an
   e-graph is Myco's equality core, scope what lives in it and what
   lives in adjacent keyed structures (temporal, observation,
   provenance, non-equational constraint metadata).
2. **Relate "residual graph" to the e-graph.** Is the residual graph
   the user-visible diagnostic projection of the saturated e-graph
   after extraction? If so, say so in spec §12. Otherwise, rename one
   of them to avoid collision.
3. **Define when two expressions produce a merge.** Syntactic? After
   unit/convert normalisation? Only after user-declared equivalence?
   This is the biggest single unresolved semantic.
4. **Define knowledge envelope ownership.** Attached to e-class,
   e-node, or symbol — determines merge/rewrite preservation rules.
5. **State the temporal invariant.** Time is outside the e-graph;
   `initial` is a t=0-guarded merge; events rebuild or re-version the
   e-graph. Pick a discipline.
6. **State the stochastic invariant.** Is `~` a merge, a different
   edge type, or a rewrite source? Marginalisation rewrites — in-graph
   or adjacent?
7. **Specify inverse semantics.** Registered inverse ⇒ rewrite rule?
   Alternative evaluator in same e-class? Independent named relation?
8. **Specify closure policies over extraction.** Run before or after
   saturation; do policies see the full e-class or only the canonical
   evaluator?
9. **Specify spatial operator lowering.** Rewrite rules in the e-graph,
   or pre-e-graph codegen? Especially for `identify` and pole L'Hopital.
10. **Specify event-driven topology mutation.** E-graph is rebuilt
    per-event? Versioned? Kept as a template with per-entity
    instantiations? This affects the whole compilation pipeline and is
    the single largest structural unknown downstream of restoring the
    substrate commitment.

---

## Trajectory observations from the audit

- **The v1 scoping sentence is the single most reusable sentence from
  v1 §6.2:** *"the e-graph is the equality core of Myco, not the
  entire semantic system. Temporal links, observations, provider
  bindings, provenance, and non-equational constraint metadata live
  in adjacent compiler structures keyed to that equality core."* If
  placed as the opening of a new "Semantic Substrate" section in the
  v2.1 spec, a substantial fraction of the audit collapses into
  checklist items ("this lives in the e-graph; that lives in an
  adjacent keyed structure").
- **The residual graph as diagnostic:** users should probably never
  see the e-graph directly. They see the residual graph — one
  canonical path chosen per class after extraction — as the factor
  graph surfaced via `explain_plan()`. The e-graph is the compiler's
  internal substrate; the residual graph is the user-visible
  projection. Stating this relationship sharply removes apparent
  tension in the current text.
- **Dynamic topology is the novel frontier.** v1 assumed static
  topology, which makes its e-graph story simple. v2's events change
  that. The event × e-graph interaction probably needs its own design
  pass.

---

## Proposed structure for the commitment section

The following is a *proposal* for how a new top-level section in
`v2.1_in_progress.md` (eventually `spec.md`) should be organized. Not
yet locked — see open questions below.

Section name: **"Part 0 — Semantic Substrate"** or **"§5.5 Equality
Substrate"**, placed *early* in the v2.1 spec, before user-facing
surface sections.

Subsections:

0. **What the e-graph is.** One paragraph.
1. **What the e-graph does and does not hold.** The v1 scoping
   sentence, expanded into a table: equalities-yes, temporal-no-but-
   keyed, inequalities-no-but-keyed, provenance-no-but-keyed, etc.
2. **When merges happen.** The big unstated semantic. Proposed answer:
   syntactic identity + compiler-internal rewrites (constant folding,
   associativity, convert normalization) + user-declared equalities
   via `=`. Never silent inference beyond compiler-known rewrites.
3. **Extraction and residual graph.** The residual graph is defined
   here as the extraction projection.
4. **Cost model.** Named as first-class, with the detailed shape
   deferred to a downstream design pass.
5. **Scoping: what lives outside the e-graph but keyed to it.**
   Checklist: temporal, observations, provenance, inequalities,
   refinements, properties, events.

Then three forward-reference subsections acknowledging the v2.1-
specific frontiers without solving them:

6. **Temporal layer.** States the invariant (time lives outside) and
   references the existing §6.3 for detail.
7. **Dynamic topology and events.** States that events may re-version
   the e-graph; marks the detailed story as open.
8. **Stochastic layer.** States that `~` is [TBD] relative to the
   e-graph; marks as open.

Remaining must-resolve items from the audit get filed here in Tier 0
as separate questions below; the commitment section addresses the
shape, not every answer.

---

## Open questions on the shape of the commitment

1. **Placement.** Early in the spec, before user-facing surface? Or
   later, in the compiler-internals section where it sits structurally
   but risks being read as "implementation detail"?
2. **"When do merges happen" answer.** The proposed answer (syntactic
   + compiler-known rewrites + user-declared `=`, no silent inference
   beyond that) — is this the right discipline?
3. **Frontier items placement.** Should the three frontier items
   (events, stochastic, tagged handles) be addressed in this
   commitment section or deferred entirely to their existing design
   docs with forward-references?

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
- **Dynamic matrix shapes.** Fixed-shape `Matrix<N, M>` is well-defined in
  the §3.9 structural subtype lattice. Dynamic-shape `Matrix<?, ?>` (shape
  unknown at compile time, bound by the workflow) needs a worked-out story:
  how the shape-refinement system interacts with the lattice, how shape-
  dependent dispatch resolves at workflow composition vs runtime, and what
  the error surface looks like when a runtime shape violates a structural
  constraint (e.g. bound matrix turns out non-square when caller expected
  `Symmetric`). Fixed-shape cases cover most v2.1 use; dynamic-shape is
  the remaining extent question.

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
- **Cross-backend callable interop.** §31.6 locks that Myco commits to no
  primary backend; §23.3 locks that trained callables reuse across workflows
  via plain contracts. What's unresolved: if workflow A trains a callable
  on backend X (e.g., PyTorch), can workflow B bind the same callable
  when running on backend Y (e.g., JAX)? Weight-format translation,
  gradient-plumbing compatibility, and advertised-capability reconciliation
  all need to be specified. The single-backend-per-run policy (§32.1) caps
  intra-run scope; cross-run interop is the open question.

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

RESOLVED. CC1 (spec §4, anti_spec "Dropped features") bans literal
numerics in value position. Physical constants and mathematical
constants (π, e, `R`, `stefan_boltzmann`, etc.) are ordinary
stdlib-declared universals whose values enter at compile time via
the workflow binding verbs. The stdlib ships default bindings for
mathematical constants so users do not write them by hand. See
spec_new.md §4 and the CC1 cluster in anti_spec.md.

---

## Module packaging and distribution

The §2 surface commits to file-as-module with path-based imports
(`use path::to::symbol`) and a DAG import graph. Still open:

- Filesystem-to-module-path mapping. How does `use a::b::c` resolve
  to a file on disk? Fixed convention (`./a/b/c.myco`), configurable
  project root, or registry lookup?
- Distribution / registry. Is there a first-class package registry
  (a "spore" system) for sharing Myco modules, or does distribution
  piggyback on Python packaging?
- Versioning and dependency resolution rules for transitive imports.

None of these block the language semantics of §2 (which are
path-name resolution rules only). Revisit once the workflow
packaging story is settled.

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
