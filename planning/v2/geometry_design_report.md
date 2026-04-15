# Myco v2.1 — Domain Geometry Design Report

**Date:** 2026-04-15
**Authors:** Riley Leff, Claude (Opus 4.6)
**Reviewers:** Gemini 2.5 Pro, GPT 5.4, GPT 5.4 Pro (three review rounds)
**Status:** Settled decisions + acknowledged open questions. Ready for
integration into the v2.1 spec.

---

## 1. Design Goals

Myco needs to support spatially explicit simulation across fundamentally
different kinds of space:

- **Flat 2D/3D regions** (pastures, soil surfaces, canopy layers)
- **1D intervals** (soil columns, horse body axes, pipe segments)
- **Branching networks** (xylem/phloem vasculature, river networks, root systems)
- **Curved surfaces** (fruit, bark, non-planar terrain)
- **User-defined exotic geometries** (the language must not hardcode a closed set)

**Explicit non-goal:** Time-varying geometries (moving meshes, ALE methods,
metrics that depend on fields or evolve in time). If the metric changes, that
is a different problem requiring its own design. Geometry coefficients are
constrained to be scalar, spatially constant, and time-invariant (see section
3.4).

The spatial system must:
1. Let the compiler derive correct spatial operators (`grad`, `diverg`,
   `laplacian`, `curl`) from the geometry — users should never hand-write
   coordinate corrections.
2. Support automatic flux balance at graph junctions inferred from
   conservation equations, with user-provided conditions for everything else.
3. Separate compile-time geometric structure from runtime topology data
   (measured tree architectures, mesh data).
4. Compose cleanly with the existing type system, containment tree, and
   cross-scale coupling via kernels.

---

## 2. Core Abstractions

### 2.1 `geometry` — compile-time structural descriptor

A geometry is a first-class language construct declared with the `geometry`
keyword. It describes the *kind* of space (metric structure, topology class,
named singularities) but not any *specific* space.

Geometries are reusable across many domain types. `Euclidean<Dim = 2>` is
used by pastures, image domains, etc. `BranchingManifold` is used by xylem
networks, river networks, bronchial trees, etc.

A geometry contains up to four kinds of declaration:

**`chart`** — local coordinate binders for the geometry body. These are
geometry-private names used to write readable metric expressions and locus
predicates. They carry no units and are not visible outside the geometry. The
domain type's `as` clause provides the public coordinate names and units; the
compiler maps them positionally (`as` first name -> first chart binder, etc.).
Lowered to positional slots (`c0`, `c1`, ...) in the IR.

**`topology`** — the global connectivity class.
- `manifold<Dim = N>` for smooth N-dimensional regions
- `rooted_tree` for branching networks with a designated root and
  parent/child relationships (subclass of `metric_graph`)
- `metric_graph` for general 1D networks without inherent direction
  (undirected pipe networks, electrical circuits)
- Extensible to other classes (simplicial complex, cell complex, etc.)

**`metric`** — the distance measure, expressed as a covariant metric tensor.
This is the fundamental primitive from which the compiler derives all spatial
operators. For manifolds, it's a DxD matrix (possibly coordinate-dependent).
For metric graphs, it's declared per-edge (each edge is a 1D Riemannian
manifold).

**`locus`** — named structural singularities (see section 2.4).

Additionally, a geometry may declare:

**`requires`** — geometry coefficients. Scalar, spatially constant,
time-invariant values that the metric depends on (e.g., a sphere's radius).
Bound at the domain type's instantiation site using parentheses:
`Domain<G = Sphere(radius = my_radius)>`. These are NOT compile-time
generics — they are per-instance values resolved at planning time from the
domain type's fields. See section 3.4.

**`identify`** — periodic seam declarations. See section 2.8.

```myco
pub geometry Euclidean<Dim: val> {
    topology manifold<Dim = Dim>
    metric = identity(Dim)
}

pub geometry Polar {
    chart (r, theta)
    topology manifold<Dim = 2>
    metric = [[1, 0], [0, r^2]]
    locus pole where r = 0
}

pub geometry Sphere {
    chart (theta, phi)
    requires radius: Scalar<m>
    topology manifold<Dim = 2>
    metric = [[radius^2, 0], [0, radius^2 * sin(theta)^2]]
    locus pole where theta in {0, pi}
    identify phi = 0 <-> phi = 2 * pi
}

pub geometry BranchingManifold {
    topology rooted_tree
    metric on edge = [[1]]
    locus junction where degree >= 3
    locus terminal where degree = 1
}

pub geometry MetricGraph {
    topology metric_graph
    metric on edge = [[1]]
    locus junction where degree >= 3
    locus terminal where degree = 1
}
```

**What a geometry does NOT contain:**
- Coordinate names or units (those belong on the domain type — see section 3.1)
- Embedding in ambient space (that belongs on the entity — see section 3.3)
- Runtime topology data (that comes from the workflow layer — see section 4)
- Discretization choices (those are workflow-layer config — see section 5)
- Solver hints (deferred — see section 3.5)
- Time-varying or field-dependent quantities

### 2.2 `Domain<G>` — type-level annotation

A domain type is an ordinary Myco composite type annotated with
`Domain<G = SomeGeometry>`. This annotation tells the compiler that instances
of this type are regions of space with continuous fields.

The domain type provides:
- **Coordinate names and units** via the `as` clause
- **Coordinate extents** via `in low .. high` syntax
- **Geometry coefficient bindings** via parentheses on the geometry
- **Fields** that vary continuously over the domain
- **Relations** including spatial operators
- **Locus-scoped relations** that provide or override junction/boundary
  conditions

```myco
type FruitSurface : Domain<G = Sphere(radius = radius)>
    as (lat: Scalar<rad>, lon: Scalar<rad>)
{
    radius: Scalar<m>       // bound from data, feeds geometry coefficient

    field temperature: Temperature
    // ...
}

type HydraulicTree : Domain<G = BranchingManifold>
    as (s: Scalar<m>)
{
    field pressure: WaterPotential
    field axial_flux: Scalar<mol_per_s>

    relation darcy:
        axial_flux = -k * grad(pressure)
}
```

The `as` clause is required on every domain type. It names the coordinates
and specifies their units. The number of coordinate names must match the
geometry's chart dimensionality. The mapping is positional: the first name
in `as` binds to the first chart binder in the geometry, the second to the
second, etc.

### 2.3 `field` — continuous quantity over a domain

Inside a domain type, `field name: Type` declares a quantity that varies
continuously over the domain's spatial extent. Fields are implicitly over the
containing domain — no `over` keyword needed unless the field varies over
fewer dimensions than the domain (subdimensional fields):

```myco
// In a 2D domain with as (px: Scalar<m>, py: Scalar<m>):
field nitrogen: Scalar<g_m2> over py    // 1D field varying only along py
```

The `over` coordinate must reference a name from the domain type's `as`
clause.

After discretization, fields lower to arrays on a mesh. Spatial operators on
fields derive their dimensional signatures from the domain's coordinate units.

**Graph field semantics:** On graph topologies, fields live on **edge
interiors**. Vertices are loci where traces are taken. This distinction
matters for `grad` (defined on edge interiors), `trace` (defined at
vertices), and finite-volume lowering.

**Vertex-local 0D state** (storage, capacitance, reaction terms at graph
vertices) is declared using `field name on locus_name: Type`. This scopes
the field to the named locus class rather than the domain's edge interiors:

```myco
type HydraulicTree : Domain<G = BranchingManifold>
    as (s: Scalar<m>)
{
    // Edge-interior fields (default)
    field pressure: WaterPotential
    field axial_flux: Scalar<mol_per_s>

    // Vertex-local 0D state — only exists at junctions
    field junction_volume on junction: Scalar<m3>
    field junction_pressure on junction: WaterPotential

    // Dynamics of the vertex storage tank:
    relation tank_dynamics on junction replaces balance(axial_flux):
        d(junction_volume) = sum(e in incident_edges,
            orientation(e) * trace(axial_flux, edge = e))
        for e in incident_edges:
            trace(pressure, edge = e) = junction_pressure
}
```

The compiler allocates locus-scoped fields as flat arrays indexed by locus
instances (size = `N_junctions`), not as mesh arrays. Locus-scoped fields
compose naturally with `trace()` — `trace` brings edge-interior fields to
the vertex boundary, where they interact with locus-scoped fields.

### 2.4 `locus` — named structural singularity

A locus is a named class of topologically special points in a geometry.
Declared inside a `geometry` block with a structural predicate:

```myco
locus junction where degree >= 3
locus terminal where degree = 1
```

Loci serve two purposes:
1. **Default condition generation:** The compiler generates default flux
   balance at graph junctions when triggered by `diverg()` usage (see
   section 3.2).
2. **User condition and override targets:** The user writes locus-scoped
   relations for continuity, boundary conditions, and overrides of generated
   defaults (see section 2.6).

Not all geometries have loci. `Euclidean<Dim = 2>` has none — nothing
structurally special happens anywhere in a flat rectangle. `BranchingManifold`
has two (junctions and terminals). `Polar` and `Sphere` have pole loci at
coordinate singularities.

Locus-scoped `where` predicates can reference both topology metadata tags
(from `assume_topology`) and built-in structural facts exposed by the
topology class:

| Structural fact | Source | Example |
|---|---|---|
| `degree` | topology class | `on junction where degree = 3` |
| `role`, custom tags | `assume_topology` metadata | `on terminal where role = "leaf"` |
| `depth` (distance from root) | `rooted_tree` | `on junction where depth > 5` |
| `strahler_order` | `rooted_tree` | `on junction where strahler_order >= 3` |

Structural facts are computed from the graph topology after `assume_topology`
and are available without Python preprocessing. This keeps scientific
structure in the `.myco` file where it belongs.

### 2.5 `trace()` — directional limit primitive (graph topologies)

`trace(field, edge = label)` evaluates a field's limiting value as you
approach a locus from a specific direction. This is the mechanism for
expressing physics at graph junctions where field values may differ depending
on which edge you approach from.

`trace()` is specific to graph topologies (`rooted_tree`, `metric_graph`).
For manifold boundaries, the `boundary` keyword is used instead (see section
2.7).

**Edge labels depend on the topology class:**

For `rooted_tree` (used by `BranchingManifold`):
- `parent` — the edge toward the root
- `children[i]` — the ith child edge (away from root)
- These are **anatomical labels** describing the tree's branching structure,
  not flow direction constraints. Flow direction is determined by the physics
  (pressure gradients). Water can flow rootward (e.g., during saltwater
  floods that reverse osmotic gradients) — the equations are acausal.

Both `rooted_tree` and `metric_graph` also expose a unified half-edge /
incident-edge API. `rooted_tree`'s `parent`/`children` labels are sugar over
this common base:
- `incident_edges` — all edges at a vertex
- `orientation(e)` — sign convention for each edge

```myco
// On a rooted_tree — parent/children labels (sugar):
relation pit_drop on junction:
    for c in children:                                       // (*)
        trace(pressure, edge = parent) - trace(pressure, edge = c)
            = pit_resistance * trace(axial_flux, edge = c)

// On a metric_graph — general incident-edge form:
relation flux_balance on junction:
    sum(e in incident_edges,                                 // (*)
        orientation(e) * trace(axial_flux, edge = e)) = 0
```

**(\*) Note on iteration syntax:** `for c in children` and
`sum(e in incident_edges, ...)` are graph-neighborhood iteration constructs.
The settled v2.1 language only has `for i in 0..N` over fixed-size arrays.
Graph-neighborhood iteration is shown here as the intended semantics; the
exact syntax will be settled alongside the general dynamic iteration design
(see section 8.2).

At a terminal (degree 1), `trace(field)` has only one direction, so the
`edge` argument is optional.

`trace` is only meaningful at graph loci. Using it in a bulk relation (away
from any locus) is a compile error.

### 2.6 `relation name on locus_name:` — locus-scoped relation

A relation can be scoped to a specific locus using `on locus_name:`. This
either provides a condition where no default exists, or overrides a
compiler-generated default.

**Overrides use explicit `replaces` targeting with obligation keys.** When a
locus-scoped relation replaces a compiler-generated default, it names the
obligation it replaces using a stable semantic key — not a user-chosen
relation name:

```myco
// Override the auto-generated balance(axial_flux) with a leaky junction:
relation leaky_junction on junction replaces balance(axial_flux):
    sum(e in incident_edges,
        orientation(e) * trace(axial_flux, edge = e)) = leak_rate

// Provide a new condition — not replacing anything:
relation pressure_continuity on junction:
    continuous(pressure)
```

Obligation keys are the stable identifiers that the compiler uses for
generated defaults (e.g., `balance(axial_flux)`) and recognized patterns
(e.g., `continuous(pressure)`). Using obligation keys rather than relation
names ensures that `replaces` targets are unambiguous and that plan
inspection can clearly show which default was replaced by which user relation.

Locus-scoped relations can include `where` predicates that reference topology
metadata and structural facts:

```myco
// Different conditions for leaf-tips vs root-tips:
relation leaf_outflow on terminal where role = "leaf":
    trace(axial_flux) = transpiration_sink

relation root_boundary on terminal where role = "root":
    trace(pressure) = soil_water_potential
```

**Terminal coverage is a global well-posedness check.** The compiler does not
enforce "exactly one condition per field per terminal." Instead, the planner
checks that the overall system is well-posed after topology injection,
equation analysis, and discretization. A tree can be correctly specified with
pressure at root terminals and flux at leaf terminals — neither terminal type
needs both. Well-posedness is planner territory, not topology-loader
territory.

**Terminals are always explicit.** The compiler does not generate default
conditions at terminal loci — the user must provide boundary conditions.
This prevents silent zero-flux or silent Dirichlet assumptions, which are a
common source of scientific bugs in PDE solvers.

---

## 2.7 Manifold boundary conditions

For manifold geometries (`Euclidean`, `Polar`, `Sphere`, `Interval`),
boundaries are handled with the `boundary` keyword, not `trace()`.

**1D boundaries (settled):**

```myco
type Horse : Domain<G = Interval>
    as (b: Scalar<m> in 0 m .. body_length)
{
    field body_temp: Temperature

    boundary b = 0 m:
        body_temp = nose_temp                              // Dirichlet

    boundary b = body_length:
        -k * normal_grad(body_temp) = h * (body_temp - ambient)  // Robin
}
```

`normal_grad(field)` is the primitive for the component of the gradient in
the **intrinsic outward co-normal** direction at a boundary. On a curved
surface-with-boundary, this is the co-normal within the manifold, NOT the
ambient 3D surface normal. The distinction matters for surfaces embedded in
3D space.

In 1D, `normal_grad` is the spatial derivative with a sign determined by
which endpoint you're at. In 2D/3D, it's `grad(field) . outward_co_normal`.
This primitive unifies Neumann and Robin conditions across all manifold
dimensions.

**2D/3D boundaries and periodic/seam identification** are open questions (see
section 8).

### 2.8 `identify` — periodic seam declaration

Geometries with periodic coordinates (e.g., longitude on a sphere) declare
seams with `identify`:

```myco
pub geometry Sphere {
    chart (theta, phi)
    requires radius: Scalar<m>
    topology manifold<Dim = 2>
    metric = [[radius^2, 0], [0, radius^2 * sin(theta)^2]]
    locus pole where theta in {0, pi}
    identify phi = 0 <-> phi = 2 * pi        // longitude wraps
}
```

Without `identify`, the compiler would see fake boundaries at `phi = 0` and
`phi = 2*pi` and demand boundary conditions there. `identify` tells the
compiler these are the same edge — fields are continuous across the seam and
no boundary conditions are needed.

**Scope for v2.1:** `identify` is guaranteed only for **scalar fields**. For
vector or tensor fields, seam identification may require component remapping
or orientation flips (e.g., tangent vectors on a Mobius strip). Vector/tensor
seam transforms are deferred beyond v2.1.

The full design for periodic/seam handling (especially in 2D/3D and for
internal subdomain interfaces) is an open question (see section 8).

---

## 3. Design Decisions and Rationale

### 3.1 Coordinates live on the domain type, not the geometry

**Decision:** The `as` clause (coordinate names, units, extents) is declared
on the domain type, not inside the `geometry` block. The geometry declares
local `chart` binders for internal readability; these carry no units and are
not visible outside the geometry.

**Rationale:** Geometries are reusable structural constraints. Different
domain types using the same geometry need different coordinate names and
units:

| Domain type | Geometry | Coordinates |
|---|---|---|
| `Pasture` | `Euclidean<Dim = 2>` | `(px: Scalar<m>, py: Scalar<m>)` |
| `ImageDomain` | `Euclidean<Dim = 2>` | `(u: Scalar<pixel>, v: Scalar<pixel>)` |
| `SoilColumn` | `Interval` | `(z: Scalar<m> in 0 m .. depth)` |
| `RootSegment` | `Interval` | `(s: Scalar<m> in 0 m .. length)` |
| `HydraulicTree` | `BranchingManifold` | `(s: Scalar<m>)` |
| `RiverNetwork` | `BranchingManifold` | `(d: Scalar<km>)` |

The geometry's `chart` binders let geometry authors write `r^2` instead of
`c0^2` — pure ergonomics. The domain type's `as` clause does the real work:
naming, units, extents.

**Analogy to contracts:** A contract declares "you must have an output called
`conductance`." The implementing type provides it. Similarly, a geometry
declares "you need N coordinates with this metric structure." The domain type
provides them via `as`.

### 3.2 Default junction conditions: balance only, from `diverg()`

**Decision:** The compiler generates **flux balance only** at graph junctions
when it sees `diverg(flux_field)` in a temporal or conservation equation.
**Continuity is not auto-generated.** Users opt in to continuity explicitly.

**Rationale:** Continuity at junctions is common but not universal. Pit
membranes, valves, leaks, advection/mixing junctions, and vertex storage are
all legitimate counterexamples. The quantum graph literature is explicit that
vertex conditions can mix field traces and derivative traces in arbitrary
ways. Auto-generating continuity would silently produce wrong physics in
many real models.

Furthermore, inferring "the associated potential field" from a constitutive
relation (e.g., inferring pressure continuity because Darcy's law mentions
`grad(pressure)`) is constitutive inference — fragile and implicit.

**The inference rule:**

When the compiler sees `diverg(flux_field)` in any conservation-form
equation (temporal `d(x) = diverg(f) + ...` OR steady-state
`0 = diverg(f) + ...`), it synthesizes at unhandled graph junctions:

- **`balance(flux_field)`** — sum of oriented traces = 0 (Kirchhoff)

That's it. No continuity inference. If the user wants pressure continuity,
they write it:

```myco
// Opt-in continuity — explicit, one line:
relation pressure_continuous on junction:
    continuous(pressure)

// Or the stdlib helper for the standard Kirchhoff pair:
relation standard_junction on junction:
    kirchhoff(pressure, axial_flux)     // continuity + balance as a bundle
```

`continuous(field)` and `kirchhoff(potential, flux)` are stdlib convenience
functions, not compiler magic. The user can always write the full trace
equations instead.

**Generated defaults are inspectable** via the plan inspection artifact
(same mechanism as SCC decomposition and solver strategy inspection). The
plan shows `GENERATED: balance(axial_flux) at junctions` and lists all
user-provided locus relations.

### 3.3 Embedding is separate from intrinsic geometry

**Decision:** A geometry describes intrinsic structure only. How a domain is
embedded in ambient 3D space (position, orientation) lives on the entity as
regular scalar fields.

**Example:** A `HydraulicTree` with `BranchingManifold` geometry has 1D
intrinsic structure (arc length along edges). But the tree exists in a 3D
forest. Its 3D position is a property of the tree entity:

```myco
type Tree {
    x: Scalar<m>      // position in forest
    y: Scalar<m>
    hydraulics: HydraulicTree   // contains the 1D vascular network
}
```

The `BranchingManifold` geometry doesn't know about 3D space. It only knows
about edges, vertices, and arc length. This is the standard differential
geometry distinction between intrinsic and extrinsic properties.

**Cross-domain coupling at the embedding** (e.g., where a 1D root network
meets 3D soil) is handled by kernel coupling, not by the geometry system.
See section 8.

### 3.4 Geometry coefficients use `requires`, not generics

**Decision:** Geometry bodies use `chart` binders for coordinates and
`requires` for scalar parameters the metric depends on. Geometry coefficients
are NOT compile-time generics.

**Rationale:** Myco generics are compile-time type parameters and `val`s,
resolved before planning/flattening. A sphere's radius that comes from data
binding (`assume_constant`) is a runtime value — it doesn't exist at compile
time. Putting it in angle brackets (`Sphere<R>`) would mean the planner sees
unresolved generics, which breaks the current compilation model.

The correct split:
- **Generics (angle brackets)** for discrete compile-time structure:
  dimension count, topology class, feature flags. `Euclidean<Dim: val>`.
- **Geometry coefficients (`requires`, parentheses)** for scalar per-instance
  values that appear in the metric. `Sphere(radius = my_radius)`.

```myco
pub geometry Sphere {
    chart (theta, phi)
    requires radius: Scalar<m>
    topology manifold<Dim = 2>
    metric = [[radius^2, 0], [0, radius^2 * sin(theta)^2]]
    locus pole where theta in {0, pi}
    identify phi = 0 <-> phi = 2 * pi
}

type FruitSurface : Domain<G = Sphere(radius = radius)>
    as (lat: Scalar<rad>, lon: Scalar<rad>)
{
    radius: Scalar<m>    // regular field, bound from data
}
```

**Constraints on geometry coefficients:**
- Scalar (not fields, not tensors)
- Spatially constant over the domain instance
- Time-invariant

**The compiler enforces these constraints as hard errors.** If a geometry
coefficient is bound to a field that receives `assume_series` (time-varying)
or is itself a spatial field, the compiler rejects the program at planning
time. This prevents users from accidentally creating a time-varying geometry
when that is an explicit non-goal.

If the metric needs to depend on a spatially varying field or on time, that
is moving-geometry / ALE territory — an explicit non-goal for v2.1.

### 3.5 No `hint` keyword (deferred)

**Decision:** Structural hints (`hint flat`, `hint acyclic`, `hint
orthogonal`) are deferred until there is demonstrated demand.

**Rationale:** The compiler can derive most structural properties from the
metric and topology declarations. Flatness is detectable from a constant
metric. Acyclicity is a property of the topology class. Adding hints now
would be speculative language surface area. If exotic user-defined geometries
later need to assert properties the compiler can't infer, `hint` can be
added with real use cases to guide the design.

**User control philosophy:** Myco provides reasonable defaults (compiler
infers discretization, solver strategy) with explicit override from the
Python workflow layer. The `.myco` file describes the world; how to solve it
is a workflow concern.

---

## 4. Runtime Topology: `assume_topology`

For geometries with data-driven topology (branching networks, irregular
meshes), the actual graph/mesh structure comes from measurements, not from
the `.myco` file. The `.myco` file declares the geometry class and the
physics. The Python workflow layer provides the specific topology:

```python
model = myco.load("forest.myco")

# Load measured tree architecture
tree_data = load_xylem_network("tree_42.graphml")
model.assume_topology("tree_42.hydraulics", tree_data)
```

`assume_topology` validates the provided data against the geometry's topology
class **at the schema level**: structural correctness, data completeness,
and unit consistency. Whether the resulting PDE/DAE is well-posed (enough
boundary conditions, correct constraint counts) is a separate planner-stage
check that depends on equations, generated defaults, user-provided
conditions, and discretization. Validation failure is a runtime error with a
clear diagnostic.

### 4.1 Topology data schema

Each topology class defines what `assume_topology` expects. This is a
Python-side data format specification.

**For `rooted_tree`:**

```python
tree_data = {
    "vertices": [0, 1, 2, 3, 4, 5],
    "edges": [(0,1), (1,2), (1,3), (3,4), (3,5)],
    "root": 0,
    "edge_lengths": {
        (0,1): 0.5,  # meters — must match coordinate units in `as`
        (1,2): 1.2,
        (1,3): 1.0,
        (3,4): 0.8,
        (3,5): 0.3,
    },
    "vertex_tags": {
        2: {"role": "leaf"},
        4: {"role": "leaf"},
        5: {"role": "leaf"},
        0: {"role": "root"},
    },
    "edge_tags": {},          # optional metadata on edges
    "edge_orientation": {},   # canonical sign convention per edge
}
```

**Additional schema requirements:**

- **Stable IDs** and a **canonical edge-orientation convention** for
  deterministic sign-sensitive flux laws and plan inspection.
- **Optional embedding geometry**: vertex positions and edge polylines for
  ambient coupling and visualization. Not required for intrinsic operators.
- **Per-edge and per-vertex scientific data** (diameter, conductivity,
  capacitance, membrane properties) should use a **separate binding channel**
  from tags. Tags are metadata for locus predicates; scientific data are
  model quantities bound via `assume_constant`/`assume_series`. Overloading
  tags with numeric data blurs the model/workflow boundary.

**Schema validation rules for `rooted_tree`:**
- Graph must be connected with no cycles (it's a tree)
- Exactly one root vertex
- Edge lengths must have units matching the domain type's coordinate
- Vertex tags must provide all metadata keys referenced by `where` predicates
  in the `.myco` file (e.g., if the model says `on terminal where role = ...`,
  every terminal needs a `role` tag)
- Validation failure reports which vertex/edge failed and why

**For `metric_graph`:**
Same as `rooted_tree` but without the `root` field. No parent/child
semantics. Cycles are permitted.

**For manifold topologies:**
Manifold topology is determined by the geometry class itself — no
`assume_topology` needed. Mesh resolution is configured via
`experiment.compile(spatial_config=...)`.

---

## 5. Discretization

The compiler owns spatial discretization. The user declares continuous
equations; the compiler chooses a discrete approximation.

**Defaults:** Compiler-inferred from the geometry class, equation structure,
and field properties. Finite differences for simple domains, finite volume
for conservation-law-dominated systems, etc.

**Override:** Python-side configuration, not language syntax:

```python
experiment.compile(
    spatial_config={
        "HydraulicTree": {"method": "finite_volume", "resolution": 0.01},
        "Pasture": {"method": "finite_difference", "nx": 50, "ny": 50}
    }
)
```

This is consistent with the design principle that `.myco` describes the world
and the workflow layer controls how to solve it.

---

## 6. Standard Library Geometries

The following geometries ship in the standard library. All are defined in
Myco from the primitives described above — they are not compiler built-ins.
Users can define custom geometries using the same primitives.

| Name | Topology | Chart | Metric | Loci | Typical use |
|---|---|---|---|---|---|
| `Interval` | `manifold<1>` | `(s)` | `[[1]]` | none | Soil columns, body axes |
| `Euclidean<Dim>` | `manifold<Dim>` | positional | `identity(Dim)` | none | Pastures, canopy layers |
| `Polar` | `manifold<2>` | `(r, theta)` | `[[1,0],[0,r^2]]` | `pole where r=0` | Radial symmetry |
| `Sphere` | `manifold<2>` | `(theta, phi)` | see below | `pole`, `identify` | Fruit, globes |
| `BranchingManifold` | `rooted_tree` | `(s)` | `[[1]]`/edge | `junction`, `terminal` | Xylem, phloem |
| `MetricGraph` | `metric_graph` | `(s)` | `[[1]]`/edge | `junction`, `terminal` | Pipe networks |

`Sphere` requires `radius: Scalar<m>`. Metric:
`[[radius^2, 0], [0, radius^2 * sin(theta)^2]]`. Longitude identified
periodically.

**Future roadmap:** `rooted_graph` for cyclic-but-rooted structures
(anastomosing roots, leaf vein networks, fungal mycelium). Not needed for
v2.1 but the architecture should not prevent it.

---

## 7. Worked Example: Hydraulic Tree with Pit Membranes

This example exercises the full geometry subsystem: geometry declaration,
domain type with edge-interior and locus-scoped fields, bulk relations with
spatial operators, balance-only default inference via `diverg()`, opt-in
continuity, pit membrane override, and `where`-predicated terminal
conditions.

```myco
use std::geometry::BranchingManifold

type HydraulicTree : Domain<G = BranchingManifold>
    as (s: Scalar<m>)
{
    // Edge-interior fields
    field pressure: WaterPotential
    field axial_flux: Scalar<mol_per_s>
    field water_content: Scalar<mol_per_m>

    // Parameters (bound from data or learned)
    axial_conductivity: Scalar<mol_per_m_s_MPa>
    pit_resistance: Scalar<MPa_s_per_mol>
    transpiration_sink: Scalar<mol_per_s>
    soil_water_potential: WaterPotential

    // Bulk physics: Darcy's law along each edge (constitutive relation)
    relation darcy:
        axial_flux = -axial_conductivity * grad(pressure)

    // Conservation equation — diverg() triggers balance default at junctions
    temporal water_balance:
        d(water_content) = diverg(axial_flux)

    // Compiler generates at junctions:
    //   GENERATED: balance(axial_flux)
    // No continuity generated. User provides all other junction conditions.

    // Pit membrane pressure drop at junctions.
    // This is NOT replacing anything — continuity was never declared.
    // The pit drop IS the junction condition for pressure.
    relation pit_drop on junction:
        for c in children:                                   // (*)
            trace(pressure, edge = parent) - trace(pressure, edge = c)
                = pit_resistance * trace(axial_flux, edge = c)

    // Terminal conditions discriminated by topology metadata tags.
    // "role" tags are provided via assume_topology in the workflow layer.

    // Leaf terminals: transpiration demand
    relation leaf_outflow on terminal where role = "leaf":
        trace(axial_flux) = transpiration_sink

    // Root terminals: soil water potential
    relation root_boundary on terminal where role = "root":
        trace(pressure) = soil_water_potential
}
```

**(\*) Note:** `for c in children` is graph-neighborhood iteration, shown as
intended semantics. Exact syntax pending the general dynamic iteration
design (see section 8.2).

**What the compiler does with this:**
1. Recognizes `BranchingManifold` with `rooted_tree` topology — emits
   per-edge discretization, exposes `parent`/`children`/`incident_edges`
2. Derives `grad(pressure)` as `dp/ds` along each edge (from the `[[1]]`
   metric)
3. Sees `diverg(axial_flux)` in `water_balance` — generates
   `balance(axial_flux)` at junctions
4. Sees `pit_drop on junction` — registers as user-provided junction
   condition for pressure (no default to replace, since continuity is not
   auto-generated)
5. At junctions: `balance(axial_flux)` (generated) + `pit_drop` (user) =
   two junction conditions. Plan inspection shows both.
6. Sees `leaf_outflow` and `root_boundary` at terminals — no auto-generation
   at terminals
7. After `assume_topology` provides the actual graph with vertex tags,
   validates schema (connectivity, root, tags, units)
8. Planner checks global well-posedness: enough boundary conditions for a
   unique solution given the topology, discretization, and equation structure
9. Sizes the system from the graph and emits the solver

**Replacing a generated default:** If the user needs to override the
auto-generated balance law (e.g., for a leaky junction), they use `replaces`
with the obligation key:

```myco
relation leaky_junction on junction replaces balance(axial_flux):
    sum(e in incident_edges,                                 // (*)
        orientation(e) * trace(axial_flux, edge = e)) = leak_rate
```

**Note on flow direction:** `parent`/`children` are anatomical labels (which
segment branched from which), not flow direction constraints. During a
saltwater flood event that reverses soil osmotic gradients, the pressure
gradient can reverse and water flows rootward through the xylem. Darcy's law
handles this naturally — `axial_flux` comes out negative relative to the
root-to-leaf orientation. The equations are acausal.

---

## 8. Open Questions

The following are acknowledged open questions organized by priority.

### 8.1 Manifold boundary conditions for 2D/3D

The `boundary coord = value:` selector and `normal_grad(field)` primitive
work for axis-aligned boundaries in any dimension. Open questions:

- **Non-axis-aligned boundaries:** Circular domains, irregular coastlines,
  complex 3D surfaces. Need a boundary naming/selection mechanism beyond
  `coord = value`.
- **Does `normal_grad()` cover all cases?** Proposed as the single primitive
  for Neumann/Robin conditions across 1D, 2D, 3D. Clarified as intrinsic
  co-normal. Needs validation against real PDE examples.
- **Additional boundary primitives:** `normal()` (vector itself), `jump()`
  (discontinuity across interface), `mean()` (average across interface)?
- **Periodic boundaries beyond `identify`:** Full design for seam handling
  in 2D/3D manifolds. Vector/tensor seam transforms deferred beyond v2.1.
- **Internal interfaces between subdomains/materials.**
- **Tangential/slip conditions for vector fields.**

### 8.2 Graph neighborhood iteration

Locus-scoped relations use constructs like `for c in children` and
`sum(e in incident_edges, ...)`. The settled v2.1 language only has
`for i in 0..N` over fixed-size arrays. Either the geometry system needs a
dedicated neighborhood-reduction construct, or the language needs to settle
general dynamic iteration first.

This is the same design question as `for i in fish` when `fish` is
`dyn`-sized. Resolving dynamic iteration for events/collections would likely
resolve graph-neighborhood iteration simultaneously.

### 8.3 Ambient locus problem — cross-domain coupling

A 1D root network embedded in 3D soil interacts along its physical extent,
not at an intrinsic locus. This is a kernel coupling problem, not a geometry
problem. The geometry system stays strictly intrinsic. Deferred to the
kernel coupling design.

### 8.4 Compiler internals for custom metrics

- **Basis-aware tensor IR:** Custom coordinate-dependent metrics require the
  compiler to derive `g^{-1}`, `det(g)`, Christoffel symbols, and
  co-normals symbolically. Needs a tensor calculus subsystem in the compiler.
- **Heterogeneous metric units:** Polar's metric `[[1, 0], [0, r^2]]` mixes
  length and angle units across elements. The dimension checker must handle
  per-element unit analysis of the metric tensor.
- **Pole / singularity handling:** `locus pole` names the problem but
  doesn't yet specify how `laplacian` avoids coordinate blow-ups at `r = 0`
  or `theta = 0, pi`. **Implementation note:** When lowering
  `laplacian(field)` on a `Sphere` or `Polar` geometry, the naive algebraic
  form contains `1/sin(theta)` or `1/r` terms that produce NaN at poles.
  The compiler backend must recognize when a mesh node coincides with a
  declared `locus pole` and emit a branch that evaluates the L'Hopital limit
  of the Laplacian at that point (which is finite and well-defined) rather
  than the naive formula. The `locus` declaration gives the compiler the
  structural information it needs to do this.

### 8.5 Plant hydraulics features that need geometry support

- **Embolism-driven edge deactivation** (topology masking — connects to
  dynamic topology / events)
- **Mixed-dimensional coupling** between 1D network and 0D/3D compartments
  (partially addressed by kernel coupling)
- **`rooted_graph` topology class** for cyclic-but-rooted structures
  (anastomosing roots, leaf vein reticulation, fungal mycelium)

### 8.6 Previously identified open questions

- **Full spatial operator catalogue** with dimensional signatures
- **Kernel coupling syntax** — cross-geometry integration, learned kernels,
  compiler optimization of kernel sparsity
- **Edge-level scientific data binding** — per-edge diameter, conductivity,
  vulnerability parameters as model quantities (separate from topology tags)
