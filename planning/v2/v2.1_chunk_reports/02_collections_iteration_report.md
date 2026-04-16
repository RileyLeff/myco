# Myco v2.1 — Collections, Iteration & Dynamic Sizing Design Report

**Date:** 2026-04-16
**Authors:** Riley Leff, Claude (Opus 4.6)
**Status:** Settled decisions. Ready for integration into the v2.1 spec.

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

**The closed set of concrete types is all in-scope implementations of the
contract.** No explicit listing needed. The compiler does whole-program
analysis on the model module and can enumerate all implementations. If a
user wants to restrict which implementations are allowed in a specific
collection, that's a future refinement (constraint on the type set), not
the default.

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
knows the maximum degree from `assume_topology` data and pads/masks per
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
- **Lowering:** Standard GPU reduction operation.
- **Empty collection:** Undefined. Compiler requires a `count > 0` guard
  or emits a compile-time warning.

For smooth selection when the discontinuity hurts training, the user writes
a softmax manually:

```myco
// Hard selection (subgradient):
let tallest = argmax(t.height for t in trees)

// Soft selection (smooth, user-written):
let weights = softmax(t.height / temperature for t in trees)
smooth_crown = sum(weights[t] * t.crown_area for t in trees)
```

### 4.4 Empty collection behavior

| Primitive | Empty collection result |
|---|---|
| `sum(...)` | `0` (additive identity) |
| `product(...)` | `1` (multiplicative identity) |
| `any(...)` | `false` |
| `all(...)` | `true` (vacuous truth) |
| `count(...)` | `0` |
| `min(...)` / `max(...)` | undefined — requires `count > 0` guard |
| `argmin(...)` / `argmax(...)` | undefined — requires `count > 0` guard |

---

## 5. Workflow-Layer Naming

`assume_*` renamed to `bind_*` across the workflow API:

| Old | New | Purpose |
|---|---|---|
| `assume_constant` | `bind_constant` | Fix a quantity to a scalar value |
| `assume_series` | `bind_series` | Fix a quantity to a time series |
| `assume_topology` | `bind_topology` | Provide graph/mesh data for a domain |

The `bind` terminology better communicates the operation: the workflow layer
*binds* data to model quantities. `assume` sounded informal; `bind` is
precise.

Schema validation, unit checking, and tag coverage checking are unchanged —
only the method names change.

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

### 7.1 Event type specification for heterogeneous dynamic collections

When an event creates a new entity in a `[Tree<impl Photosynthesis>; some]`
collection, how is the concrete type specified? Three options discussed,
not yet settled:

1. **Concrete type in event output:**
   `event oak_recruit: -> Tree<FarquharC3> { ... }`
2. **Generic event parameterized over type set:**
   `event recruit<S: Photosynthesis>: -> Tree<S> { ... }`
3. **Workflow-layer selection:**
   Event creates generic `Tree`, species binding from Python.

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
