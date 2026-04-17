# Myco v2.1 — Collections, Iteration & Dynamic Sizing Design Report

**Date:** 2026-04-16
**Authors:** Riley Leff, Claude (Opus 4.6)
**Reviewers:** Gemini 2.5 Pro (one review round), GPT 5.4 Thinking (one review round)
**Status:** Settled with noted caveats. Ready for integration into the v2.1 spec.

---

## 1. Context

The v2.0 spec defines only fixed-size arrays (`[Type; N]` where `N` is a
compile-time `val` constant) with index-based iteration (`for i in 0..N`).
Variable-length collections are explicitly out of scope (spec line 371).

The v2.1 design sessions introduced dynamic topology (entities born/die via
events) and graph-based domains (edge neighborhoods of variable degree),
both requiring iteration over collections whose size isn't compile-time-known.
Additionally, the v2.0 `dyn` keyword was overloaded to mean two unrelated
concepts.

This report covers: renaming the `dyn` overload, dynamic collection sizing,
heterogeneous dynamic collections, iteration syntax, aggregation primitives,
and workflow-layer naming.

---

## 2. Keyword Renaming: Resolving the `dyn` Overload

The v2.0 spec uses `dyn` for heterogeneous contract implementations per
element (compile-time polymorphism). The v2.1 design added `dyn` for
runtime variable sizing. These are orthogonal concepts sharing a keyword.

### 2.1 `impl` — heterogeneous element types (was `dyn Contract`)

`impl` means "each element implements this contract; concrete type resolved
at compile time." Semantics are unchanged from the v2.0 `dyn` — the compiler
monomorphizes, no runtime dispatch. Just renamed.

```myco
// v2.0 syntax (retired):
pfts: [PFT<dyn Photosynthesis>; N]

// v2.1 syntax:
pfts: [PFT<impl Photosynthesis>; N]
```

Per-element type ascription in model modules works identically:

```myco
node my_site: MySite {
    eco.pfts[0]: PFT<FarquharC3>
    eco.pfts[1]: PFT<C4Photo>
    eco.pfts[2]: PFT<FarquharC3>
}
```

### 2.2 `some` — runtime variable sizing (was `dyn` sizing)

`some` means "the collection's size changes at runtime via events."

```myco
fish: [Fish; some]
trees: [Tree; some]
```

Backend lowering is a compiler concern, not a language concern:
- **PyTorch:** packed array, actual resize via `torch.compile(dynamic=True)`
  symbolic shapes
- **JAX:** fixed-size `[Type; MAX_CAPACITY]` with validity mask,
  `MAX_CAPACITY` supplied by workflow layer

The `.myco` file describes semantics ("this can grow/shrink"). The compiler
emits the right lowering per backend. No `#feature` flags or backend
annotations in the language.

**Bind-time vs event-time dynamism:** `some` covers two cases:
- **Bind-time variable, rollout-static:** Size unknown at compile time but
  fixed once the workflow layer provides data (e.g., "I have 47 fish, here
  they are"). No events target this collection — it never resizes.
- **Event-time variable, rollout-dynamic:** Size changes during rollout as
  events fire (birth/death). Requires validity mask or packed-array resize.

The language uses `some` for both. The compiler distinguishes them
mechanically: if no events target a `some`-sized collection, it's
bind-static. The compiler can optimize accordingly (no validity mask
updates, static Jacobian after bind). No user annotation needed — this is
compiler-inferred from the absence of events. Models with truly dynamic
collections (events present) route through the dynamic topology lowering
path; bind-static `some` collections do not.

### 2.3 `impl` + `some` compose

A collection can be both heterogeneous and dynamically sized:

```myco
trees: [Tree<impl Photosynthesis>; some]
```

This means "a dynamic-sized collection of Trees where each element can be
any in-scope implementation of Photosynthesis, determined at event time."

**Frontend desugaring:** The compiler enumerates all in-scope types that
implement the contract and generates one homogeneous pool per concrete type:

```
// Frontend expands to (conceptually):
_trees_FarquharC3: [Tree<FarquharC3>; some]
_trees_C4Photo: [Tree<C4Photo>; some]
_trees_NeedlePhoto: [Tree<NeedlePhoto>; some]
```

`trees` becomes a virtual handle over all pools. Iteration merges pools.
Spatial queries merge results. Events route to the correct pool by type.

**The closed set of concrete types is all implementations in scope at the
model module level, after whole-program assembly.** No explicit listing
needed. The compiler does whole-program analysis on the model module and
can enumerate all implementations. "In-scope" means the model module's
import set — adding an implementation to a library doesn't change anything
until a model module imports it. If a user wants to restrict which
implementations are allowed in a specific collection, that's a future
refinement (constraint on the type set), not the default.

**GPU efficiency:** Within each pool, all elements are structurally identical
(same equations, same Jacobian structure), enabling vmap. No warp divergence.
The per-type-pool pattern is how the GPU would execute it regardless of
syntax — the desugaring just makes the user's life easier.

---

## 3. Iteration Syntax

### 3.1 Design Principle

Myco's existing `for i in 0..N` is a declarative quantifier ("for each i,
this relation holds"), not an imperative loop. The compiler decides execution
strategy. Dynamic iteration extends the same quantifier to runtime-sized
domains.

### 3.2 Iterator-style: `for x in collection`

```myco
// Fixed-size, index-style (existing, unchanged):
relation local_climate for i in 0..N_HORSE:
    horses[i].ambient_temp = air_temp.at(horses[i].x, horses[i].y)

// Dynamic collection, iterator-style (new):
relation local_climate for t in trees:
    t.ambient_temp = air_temp.at(t.x, t.y)
```

Both coexist. Index-style gives `i` for arithmetic (`layers[i+1]`).
Iterator-style is cleaner when the index isn't needed. They're not redundant.

### 3.3 Graph neighborhoods

Same syntax. Graph neighborhoods are per-vertex structural iterables exposed
by the topology class:

```myco
// rooted_tree — parent/children labels:
relation pit_drop on junction:
    for c in children:
        trace(pressure, edge = parent) - trace(pressure, edge = c)
            = pit_resistance * trace(axial_flux, edge = c)

// General incident-edge form:
relation flux_balance on junction:
    0 = sum(orientation(e) * trace(axial_flux, edge = e)
            for e in incident_edges)
```

`children` and `incident_edges` are per-vertex iterables. Their size varies
per vertex (different junctions have different branch counts). The compiler
knows the maximum degree from `bind_topology` data and pads/masks per
backend.

### 3.4 Relation-level and nested iteration

`for` works both as a relation-level quantifier and nested within bodies:

```myco
// Relation-level — one equation per tree:
relation climate for t in trees:
    t.ambient_temp = air_temp.at(t.x, t.y)

// Nested — at each junction, one equation per child edge:
relation pit_drop on junction:
    for c in children:
        trace(pressure, edge = parent) - trace(pressure, edge = c)
            = pit_resistance * trace(axial_flux, edge = c)

// Mixed nesting — dynamic outer, fixed inner:
relation root_uptake for t in trees:
    for j in 0..N_SOIL:
        t.water_supply[j] = K_root(t, soil.layers[j]) * soil.layers[j].water
```

### 3.5 Filtering with `where`

Value-based and type-based filtering, extending the spec's existing
`where` clause + mask lowering pattern:

```myco
// Value-based:
stressed_lai = sum(t.lai for t in trees where t.water < threshold)

// Type-based (extends spec section 5.4):
c3_lai = sum(t.lai for t in trees where t.photo is FarquharC3)

// Combined:
stressed_c3 = count(trees where t.photo is FarquharC3 and t.water < threshold)
```

**Type narrowing:** `where t.photo is FarquharC3` narrows the type within
that scope. Fields specific to `FarquharC3` (not in the `Photosynthesis`
contract) become accessible:

```myco
// Works — vcmax_25 is FarquharC3-specific, but type is narrowed:
avg_vcmax = mean(t.photo.vcmax_25
    for t in trees where t.photo is FarquharC3)

// COMPILE ERROR — vcmax_25 not in Photosynthesis contract, no type filter:
// bad = sum(t.photo.vcmax_25 for t in trees)
```

Since the frontend desugars heterogeneous dynamic collections to per-type
pools, type filtering simply selects the corresponding pool. Zero runtime
cost.

### 3.6 Lowering

| Context | Size known | Lowering |
|---|---|---|
| `for i in 0..N` | compile time | unrolled or vmapped, static Jacobian |
| `for e in incident_edges` | after topology bind | padded to max degree, masked |
| `for t in trees` (JAX) | runtime | `[T; MAX_CAPACITY]` with validity mask |
| `for t in trees` (PyTorch) | runtime | packed array, symbolic shapes |

Syntax is uniform. Lowering varies by backend. User never sees the difference.

---

## 4. Aggregation Primitives

### 4.1 Existing (unchanged)

```myco
sum(expr for i in 0..N)
product(expr for i in 0..N)
forall i in 0..N: predicate
```

### 4.2 New primitives

All work with both index-style and iterator-style:

```myco
// Reductions:
total_lai = sum(t.lai for t in trees)
max_height = max(t.height for t in trees)
min_water = min(t.water for t in trees)

// Predicates:
any_stressed = any(t.water < threshold for t in trees)
all_healthy = all(t.water > threshold for t in trees)

// Count:
n_alive = count(trees)
n_stressed = count(trees where t.water < threshold)

// Element selection (subgradient differentiability):
let tallest = argmax(t.height for t in trees)
dominant_crown = tallest.crown_area

let driest = argmin(t.water for t in trees)
priority_species = driest.photo
```

### 4.3 `argmin` / `argmax` semantics

`argmax(expr for x in collection)` returns the element that maximizes `expr`.

- **Return type:** The element type. For heterogeneous collections, the
  contract type (can narrow with `is` check).
- **Ties:** Broken by index order (deterministic).
- **Differentiability:** `subgradient`. Gradient flows through the
  currently-selected element. Discontinuous at switchover points (same
  class as `max(a, b)` at `a = b`). The compiler's existing
  differentiability annotation system handles this — no new machinery.
- **Empty collection:** Undefined. Must be guarded (see section 4.4).

**Lowering — homogeneous collections:** Standard GPU reduction. `argmax`
returns an index into the (possibly masked) array.

**Lowering — heterogeneous collections:** When `argmax` operates across a
heterogeneous `[T<impl Contract>; some]` collection (desugared to per-type
pools), the result is a **tagged handle** — internally a `(pool_id, index)`
pair. Field access on the result emits a multiplexed gather across pools:

```python
# What the emitter generates for: tallest.crown_area
# where tallest came from argmax over a heterogeneous collection:
dominant_crown = jax.lax.switch(
    tallest.pool_id, [
        lambda: _trees_FarquharC3.crown_area[tallest.index],
        lambda: _trees_C4Photo.crown_area[tallest.index],
    ]
)
```

This is invisible to the user — `tallest.crown_area` looks like a normal
field access. But the compiler team must emit multiplexed lookups for any
field dereference on an element reference from a heterogeneous collection.
The reduction itself (finding the max) is still a standard GPU reduction
across all pools; the multiplexing only applies to subsequent field access.

**Important: this introduces a new IR concept.** The v2.0 semantic story is
"no runtime dispatch — everything monomorphizes before flattening." The
tagged handle from heterogeneous `argmax` is a genuine IR-level tagged
union — a runtime sum type that the emitter must dispatch on. This is NOT
the same as the pre-monomorphization `impl` concept. It's a lightweight
runtime concept that only arises from element-selecting aggregations over
heterogeneous collections. The user never sees `pool_id` — the abstraction
is sound — but the IR must represent it, and the compiler team must know
it exists. Field access on a tagged handle is multiplexed dispatch, not
free monomorphization.

For smooth selection when the discontinuity hurts training, the user can
write a weighted sum instead of hard selection. The exact syntax for this
depends on aligned-vector / zip semantics for comprehensions (not yet
designed). Conceptually:

```myco
// Hard selection (subgradient):
let tallest = argmax(t.height for t in trees)

// Soft selection — conceptual, syntax TBD:
// smooth_crown = weighted_sum(softmax(t.height / temp), t.crown_area
//                             for t in trees)
```

See open question 7.4 (`softmax` as a primitive) for status.

### 4.4 Empty collection behavior

| Primitive | Empty collection result |
|---|---|
| `sum(...)` | `0` (additive identity) |
| `product(...)` | `1` (multiplicative identity) |
| `any(...)` | `false` |
| `all(...)` | `true` (vacuous truth) |
| `count(...)` | `0` |
| `min(...)` / `max(...)` | undefined — requires guard |
| `argmin(...)` / `argmax(...)` | undefined — requires guard |

**Guard syntax:** Use inline `if/else` (already settled in the language):

```myco
let max_h = if count(trees) > 0
    then max(t.height for t in trees)
    else 0.0 m

let tallest_crown = if count(trees) > 0
    then argmax(t.height for t in trees).crown_area
    else 0.0 m2
```

**Dependency note:** Runtime guards and `where` filtering on `some`-sized
collections extend the v2.0 spec's existing mask-lowering mechanism (spec
section 5.4, `where` on fixed index ranges). This design assumes that
mechanism generalizes to `some` collections — the mask is the same validity
mask used for dynamic sizing, so the extension is natural but should be
explicitly confirmed during spec integration.

**Lowering note:** On JAX/PyTorch, `jax.numpy.where` / `torch.where`
evaluates both branches regardless of the condition. The `max()` or
`argmax()` still executes on the padded array even when the collection is
logically empty. The backend emitter must inject safe sentinels into invalid
mask slots: `-inf` for `max`/`argmax`, `+inf` for `min`/`argmin`. This
ensures the reduction produces a valid (but unused) result rather than NaN.

### 4.5 `count` semantics

`count(collection)` for a `some`-sized collection means the number of
**valid** (alive) elements, not the backing array length. On JAX (masked
lowering): `count(trees)` = `sum(validity_mask)`. On PyTorch (packed
lowering): `count(trees)` = `len(packed_array)`. The distinction is
invisible to the user but matters for backend implementors.

---

## 5. Workflow-Layer Naming

### 5.1 `assume_constant`, `assume_series` — KEPT (rename reverted)

The original proposal renamed `assume_*` to `bind_*` across the board.
**This rename is reverted.** The existing workflow vocabulary has a clean
four-way split:

| Verb | Meaning |
|---|---|
| `assume` | Provide data values for model quantities |
| `observe` | Provide evidence for inference |
| `learn` | Make a quantity trainable |
| `bind` | Provide a slot implementation (controller) |

Collapsing `assume` and `bind` into one word loses the distinction between
"fix this quantity to a value" and "attach this controller implementation."
`assume` in the mathematical sense ("assume this quantity takes this value")
is precise — the informality concern doesn't justify overloading `bind`.

### 5.2 `bind_topology` — NEW (standalone addition)

`bind_topology` is kept as a standalone new API. Topology binding is
structurally different from assuming a quantity's value — it provides
graph/mesh structure to a domain, not a scalar or time series. It doesn't
conflict with slot binding either (topologies aren't controllers).

| Method | Purpose |
|---|---|
| `assume_constant` | Fix a quantity to a scalar value |
| `assume_series` | Fix a quantity to a time series |
| `bind_topology` | Provide graph/mesh data for a domain |

Schema validation, unit checking, and tag coverage checking are unchanged.

---

## 6. Decisions on Non-Issues

### 6.1 No `#feature` flags for backend-specific behavior

The `.myco` file describes the world. Backend-specific lowering (masked
arrays on JAX, packed arrays on PyTorch) is the compiler's job. If the
semantics genuinely differ between backends, the language abstraction is
wrong — not something to patch with feature flags.

### 6.2 No new collection types needed

Fixed-size arrays (`[T; N]`) and dynamic arrays (`[T; some]`) cover the
ecosystem modeling use cases. Maps, sets, linked lists, and graphs-as-
collections are handled by other language features (type system, geometry
system, containment tree).

The one collection pattern that was previously awkward — heterogeneous
dynamic collections — is now addressed by `[T<impl Contract>; some]`
desugaring to per-type pools.

### 6.3 Terrain as field on flat domain is sufficient

For ecosystem modeling, representing topography as a data-bound field on
a flat Euclidean domain (with slope-driven flow terms in the physics) covers
all practical use cases. Irregular domain boundaries (non-axis-aligned
surfaces, microtopographic depressions) are an elegance/efficiency concern,
not a correctness concern. The 2D/3D manifold boundary open question remains
but is deprioritized.

### 6.4 `for i in 0..N` and `for x in collection` coexist

Both syntaxes are kept. Index-style gives `i` for arithmetic. Iterator-style
is cleaner when the index isn't needed. Different situations call for
different forms.

---

## 7. Open Questions (carried forward)

### 7.1 Event type specification for heterogeneous dynamic collections — RESOLVED

**Decision: Option 1 (concrete type in event output).** Option 2 (generic
event) is acceptable as syntactic sugar. Option 3 (workflow-layer selection)
is rejected.

**Why Option 3 fails:** If the event creates a generic `Tree` and relies on
Python to assign the species at runtime, the compiler cannot determine which
type pool the entity belongs to. This destroys the static pool desugaring —
the compiler would need a generic buffer with dynamic dispatch, ruining GPU
performance.

**Why Option 1 is correct:** The output type is concrete at compile time, so
the compiler knows exactly which pool to append to. The validity mask for
that specific pool is incremented. The entire operation remains static and
vmappable.

```myco
event oak_recruit: -> Tree<FarquharC3>
    when oak_seed_bank > threshold
{
    new_tree.mass = seed_mass
}

event grass_recruit: -> Tree<C4Photo>
    when grass_seed_bank > threshold
{
    new_tree.mass = grass_seed_mass
}
```

For 50 species, a declarative macro (spec section 18.1) generates the 50
concrete recruitment events at compile time.

**Option 2 as sugar:** `event recruit<S: Photosynthesis>: -> Tree<S>` is
syntactic sugar — the compiler monomorphizes it to N concrete events, one
per in-scope implementation. Same as how generic types are monomorphized.

### 7.2 Restricting the type set

Default is all in-scope implementations. If a user wants to restrict which
implementations can appear in a specific collection (`only these 5 of 50
species`), that would need a constraint mechanism. Not needed for v2.1 —
deferred until there's demand.

### 7.3 Jacobian structure with dynamic sizing

Fixed-size `for i in 0..N` lets the compiler size the Jacobian statically.
`some`-sized iteration means Jacobian dimensions change at runtime. On
PyTorch this is handled by symbolic shapes. On JAX by the validity mask
(Jacobian is always MAX_CAPACITY-sized, masked entries zeroed). Needs
implementation validation but not language design work.

### 7.4 `softmax` as a primitive

`softmax` appeared in the argmax smooth-selection example. Should it be a
builtin or a user-defined function? Probably stdlib — it's a standard
mathematical operation and the compiler could optimize it (numerically
stable log-sum-exp form). Low priority.
