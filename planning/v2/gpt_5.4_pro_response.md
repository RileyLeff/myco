After reading the four docs, my answer is: add a real `geometry` declaration, but keep it narrow. Geometry should own the chart, metric, topology class, named singular loci, and structural hints; everything that states physics at a boundary, junction, pole, or seam should still be written as Myco equations. That fits the language’s own philosophy: the model states scientific structure, the workflow carries experiment data, and discretization and solver choice stay compiler-owned. The current v2.1 domain work already points that way: domains are type annotations with coordinate units, extents, fields, `.at()`, boundary syntax, and spatial operators derived from metric, while geometry authoring and topology injection are still the missing pieces.    

My recommended split is:

* `geometry` = charted geometry descriptor
* domain type = world entity that lives on that geometry
* topology instance = workflow-bound data, for cases like xylem graphs
* discretization = workflow/compiler concern
* singularity physics = locus-scoped relations using `trace()`

That gives you a clean long-term story without turning geometry into a physics mini-language.  

## 1. Does the `geometry` keyword earn its keep?

Yes.

A Myco `contract` already has a specific meaning: inputs, outputs, properties, and function-like invocation. A Myco `type` also already has a specific meaning: structural schema that can instantiate into the containment tree. Geometry is neither. It is compile-time metadata the spatial operator lowerer needs before planning; it is not a runtime entity and not a behavior interface. Reusing `contract` would mean smuggling charts, metric tensors, locus declarations, topology schemas, and operator capabilities into a construct whose current semantics are functional. Reusing `type` would make geometry look instantiable and bindable, which is exactly the wrong mental model.   

There is a third option: make `Geometry` a sealed builtin kind implemented with `type Euclidean : Geometry`. I would reject that too. It looks smaller on paper, but it still needs special parser and checker rules, so it is “keyword avoidance” rather than real simplification. You still need geometry-specific syntax for charts, metrics, topology, loci, and geometry inputs. At that point a dedicated declaration form is cleaner.

The decisive point is this: the thing Myco needs is not “geometry” in the pure mathematician’s sense. It is a **charted geometry descriptor**. `Euclidean<Dim = 2>` and `Polar` describe the same flat plane differently, but the compiler must treat them differently because the operator formulas, singular sets, and admissible discretizations are different. The current docs already recognize that coordinate transforms and singularities are part of the unresolved design space. A dedicated `geometry` declaration makes that explicit. 

So I would spec `geometry` as:

* importable like `type` or `fn`
* non-instantiable
* legal only in `Domain<G = ...>` annotations
* allowed to have compile-time generics and runtime geometry inputs
* forbidden from containing relations, temporals, slots, or workflow bindings

That is a small, sharp surface area.

## 2. Is selector + interface the right abstraction?

Keep the idea of named structural subsets. Drop the `interface` keyword.

The simpler and more general mechanism is:

* geometry declarations expose named **loci**
* ordinary `relation` and `constraint` declarations can be scoped to a locus
* a builtin `trace()` gives one-sided values on a locus

That covers boundaries, poles, seams, graph junctions, confluences, fractures, and subdomain interfaces with one equation language instead of two. It is also much more Myco-like, because the existing language already treats equalities as solver-eligible equations and already has overdetermination machinery. A separate `interface` block would mostly duplicate semantics you already have.  

So the core abstraction should be:

```myco
relation name on junction:
    conservation(axial_flux)
```

not:

```myco
interface junction:
    conservation(axial_flux)
```

`boundary x = 0 m: ...` should stay as sugar for the common case, because common cases should be trivial. But poles and junctions are not “interfaces” in the OO sense; they are loci where traces live. The open questions already hint at exactly this need when they ask about named edges, selectors, and what `grad` means on a network at a junction.  

This is especially natural for graph domains. In metric-graph mathematics, functions are differentiated along edges; the special behavior sits at vertices, where one imposes vertex conditions. Standard Kirchhoff conditions combine continuity at the vertex with zero sum of incident derivatives, and graph divergence is naturally expressed with an oriented incidence matrix. That is exactly the shape Myco needs for xylem, rivers, airway trees, vein networks, and fracture networks. ([Mathematics Home][1])

One naming change I would make: the core standard-library primitive should really be `MetricGraph`. `BranchingManifold` is a very good plant-facing alias, but rivers and fractures are not always trees. The math and the compiler IR are metric graphs.

My synthesis here is:

**Geometry owns where singularities are and how traces are taken there.
Physics owns what equations hold there.**

That is the clean split.

## 3. How should the metric be declared?

The primary declaration should be the **covariant metric tensor**.

Not:

```myco
metric(r, theta) = [[1, 0], [0, r**2]]
```

but:

```myco
metric covariant = [[1, 0],
                    [0, r^2]]
```

Coordinates are already in scope from the geometry’s chart, so repeating them is noise. For orthogonal systems, `diagonal(...)` is good sugar; for general metrics, keep full matrix syntax. I would allow a textbook-style `line_element = dr^2 + r^2 dtheta^2` later as sugar, but not as the core form, because the compiler ultimately needs `g_ij`, `g^{ij}`, and `|g|`. The current v2.1 work already takes the metric tensor as the fundamental primitive behind spatial operators; this just makes the authoring surface explicit.   

For geometries whose metric depends on per-instance quantities, geometry declarations should be allowed to take **runtime geometry inputs** bound at the domain site. That solves `SphereSurface(radius = crown_radius)` cleanly without abusing compile-time generics.

The unit story is the subtle part. Do **not** try to type a metric as `Matrix<Scalar<U>>`. That fails immediately in polar coordinates. You need basis-aware tensor typing. If coordinate `q^i` has unit `U_i`, then the checker should validate each covariant component `g_ij` through the contraction `g_ij dq^i dq^j`. In polar coordinates, `g_rr = 1` is dimensionless while `g_θθ = r^2` carries `m^2`; both are correct. The checker should require the **contracted line-element terms** to agree, not the raw components to share one scalar unit. That is the right fix for the heterogeneous-units problem.

Internally, this means Myco needs a small hidden tensor IR with:

* variance tags
* basis-unit metadata
* tensor literals for geometry definitions
* differentiation of tensor components with respect to scalar coordinates

It does **not** need to become a general user-facing tensor language in v2.1. It only needs to be powerful enough to hold `g_ij`, `g^{ij}`, `det g`, and Christoffel expressions so the compiler can derive operators. The load-bearing formula is the Laplace-Beltrami operator,
[
\Delta f = |g|^{-1/2},\partial_i!\left(|g|^{1/2} g^{ij}\partial_j f\right),
]
so once the compiler can represent and differentiate metric components, `grad`, `diverg`, and `laplacian` follow. The docs already flag tensor support in `deriv` as the missing compiler piece.  ([OpenMETU][2])

One more important rule: coordinate singularities should be explicit loci, not “places where the formula blows up and maybe the stencil handles it.” `Polar` should say it has a `pole`; `SphereSurface` should say it has poles and a periodic seam handled by the stdlib geometry. That lets the compiler reject naive pointwise evaluation there and either regularize or require locus-scoped equations. 

## 4. How should automatic vs explicit junction conditions work?

Reviewer A is directionally right, but the trigger is too coarse.

Automatic junction balance should **not** fire merely because a field’s type has a `{ conserved }` ancestor. In spatial models, what is conserved is usually a **total amount**, while the field is a density and the vertex law applies to an associated flux. A graph field might be `mol_per_m`, while the conservation group is `WaterAmount : Scalar<mol> { conserved }`. The automatic law needs to attach to the **canonical flux in a divergence-form equation**, not to the field annotation alone. That is the only design that survives real PDE use. The current docs already note that conservation groups live at the type level and that whether conserved fields over domains need special treatment is still open.   

So my rule would be:

1. Parse explicit locus-scoped relations first.
2. Infer default obligation classes from geometry and equations:

   * continuity of a top-dimensional graph field
   * balance of a canonical flux appearing in `diverg(...)`
3. Synthesize defaults only for unmatched `(locus, quantity, obligation-class)` triples.
4. Run the ordinary redundancy/inconsistency pass on the union.

That is much better than a bespoke override system. Generated defaults are just generated equations, so they fit the language architecture and the existing overdetermination classifier. 

Shadowing should be **obligation-class specific**. If I write a pressure-drop law at a junction, that should suppress the default **pressure continuity** obligation on that locus, but it should **not** silently suppress default flux balance. If I write an explicit balance law `sum_flux(J) = leak`, that shadows the zero-balance default. Unmatched defaults stay. That gives the right ergonomics for real hydraulics, because a pit-membrane law usually changes continuity but not conservation.

Terminals should get **no automatic closure at all**. They need explicit source, sink, or boundary relations. Open systems are common; a language that auto-closes terminals will be wrong in practice.

One more implementation point: generated defaults should appear in plan inspection. If Myco is going to synthesize Kirchhoff conditions, the user should be able to see them exactly the same way they can inspect any other planning decision. That is consistent with the current plan-inspection direction. 

## 5. What about embedding?

Intrinsic geometry and extrinsic embedding must be separate.

A xylem network is intrinsically 1D for `grad`, `diverg`, and `laplacian`, but it may carry a 3D embedding for kernels, shading, spatial competition, and root-soil distances. Geometry declarations should define the **intrinsic differential structure only**. The embedding belongs on the domain type instance, because different measured trees share the same intrinsic geometry but different spatial coordinates. That also matches the rest of Myco: children do not “see up,” there is no special `physical` keyword, and coupling already happens through ordinary fields and `.at()` rather than hidden runtime magic.   

So today I would model embedding as ordinary fields:

```myco
field world_x: Scalar<m> over s
field world_y: Scalar<m> over s
field world_z: Scalar<m> over s
```

That is enough for kernels and cross-scale coupling. Later, if you want sugar, add something like `embedding world = (world_x, world_y, world_z)`, but I would not make that part of the core language yet.

Also: changing embedding coordinates on fixed topology is **not** the same as changing topology. A moving or growing geometry coefficient can still be a fixed-topology problem. Remeshing, branch birth, and rewiring are different problems and should not be collapsed into one “geometry changed” bucket.

## 6. What does the compiler actually need?

The compiler needs a **geometry IR**, not just a parsed geometry declaration.

At minimum that IR should contain:

* geometry class id
* coordinate axes and units
* runtime geometry inputs
* topology kind and required data schema
* metric expressions on regular strata
* named loci
* explicit and synthesized locus equations
* workflow-supplied topology instance or discretization config

Without the last item, Myco can type-check operators but it cannot emit a kernel. The planner and emitter already expect a closed, explicit intermediate representation; geometry has to lower into the same style of inspectable artifact.   

For a **finite-difference Laplacian on a 2D Euclidean rectangle**, the compiler needed:

* intrinsic dimension 2
* coordinate names and units
* extents
* identity metric
* structured tensor-product topology
* boundary loci and their equations
* grid resolution or mesh from workflow
* interpolation rules for `.at()`

From that, it can emit centered stencils, boundary kernels or ghost-cell updates, and then let the planner choose time integration. Euclidean geometry alone is not enough; the structured grid instance is also needed. The docs already separate “domain structure” from “discretization chosen by the compiler or workflow,” which is exactly right.   

For a **finite-volume conservation law on a metric graph**, the compiler needed:

* oriented incidence or CSR adjacency
* edge lengths
* optional root/direction metadata
* vertex classes such as terminal vs junction
* any tags used by locus-scoped relations
* the edge-local metric, usually 1
* canonical fluxes from divergence-form equations
* explicit or default trace laws at vertices

That lowers naturally to an edge-parallel flux kernel plus a vertex gather/scatter kernel driven by the incidence structure. This is why graph topology must enter as **data** through something like `assume_topology`, while computational refinement per edge belongs in `compile(spatial_config=...)`. The measured xylem branching pattern is science; “one cell per segment” vs “eight cells per segment” is computation. ([Motion][3]) 

A very useful optimization falls out of this too: degree-2 vertices with standard Kirchhoff conditions are often removable. The compiler should collapse them unless they carry tags, observations, or explicit locus equations. ([Arizona Repository][4])

So: does the proposed geometry definition provide enough? **Yes, but only if you explicitly separate geometry class, topology instance, and discretization config.** Geometry syntax by itself is necessary, not sufficient.

## 7. My best syntax proposal

I would actually ship `MetricGraph` in the stdlib and maybe alias `BranchingManifold` to it. I’m using your name below for continuity.

This keeps domains as type annotations, preserves `boundary` and `.at()`, and adds only the minimum new core needed for geometry authoring and locus-scoped equations. It also matches v2.1’s “named generics” direction.   

```myco
pub geometry Euclidean<Dim: val> {
    topology manifold<Dim = Dim>
    metric covariant = identity(Dim)

    hint flat
    hint orthogonal
    hint tensor_product
}

pub geometry Polar {
    chart (r: Scalar<m>, theta: Scalar<rad>)
    topology manifold<Dim = 2>
    metric covariant = [[1, 0],
                        [0, r^2]]

    locus pole where r = 0 m

    hint orthogonal
}

pub geometry SphereSurface {
    input radius: Scalar<m>

    chart (theta: Scalar<rad>, phi: Scalar<rad>)
    topology manifold<Dim = 2>
    metric covariant = [[radius^2, 0],
                        [0, radius^2 * sin(theta)^2]]

    locus north_pole where theta = 0
    locus south_pole where theta = pi

    // standard-library geometry: phi is periodic
    hint closed
    hint orthogonal
}

pub geometry BranchingManifold {
    topology metric_graph {
        oriented_incidence
    }

    metric on edge = [[1]]

    locus junction where degree >= 3
    locus terminal where degree = 1

    hint rooted
    hint acyclic
}
```

```myco
type Horse {
    x: Scalar<m>
    y: Scalar<m>
    ambient_temp: Temperature
}

type Pasture<N_HORSE: val> : Domain<G = Euclidean<Dim = 2>>
    as (x: Scalar<m> in 0 m .. width,
        y: Scalar<m> in 0 m .. height)
{
    width: Scalar<m>
    height: Scalar<m>

    field air_temp: Temperature
    horses: [Horse; N_HORSE]

    relation local_climate for i in 0..N_HORSE:
        horses[i].ambient_temp = air_temp.at(horses[i].x, horses[i].y)
}

type RootDisc : Domain<G = Polar>
    as (r: Scalar<m> in 0 m .. radius,
        theta: Scalar<rad> in 0 .. 2 * pi)
{
    radius: Scalar<m>
    bulk_soil_water: WaterPotential

    field soil_water: WaterPotential

    boundary r = radius:
        soil_water = bulk_soil_water
}

type CrownShell : Domain<G = SphereSurface(radius = crown_radius)>
    as (theta: Scalar<rad> in 0 .. pi,
        phi: Scalar<rad> in 0 .. 2 * pi)
{
    crown_radius: Scalar<m>
    lateral_diffusivity: Scalar<m2_per_s>

    field leaf_temp: Temperature

    temporal lateral_mixing:
        d(leaf_temp) = lateral_diffusivity * laplacian(leaf_temp)
}

type HydraulicTree : Domain<G = BranchingManifold>
    as (s: Scalar<m>)
{
    axial_conductivity: Scalar<mol_per_s_MPa_m>
    pit_resistance: Scalar<MPa_s_per_mol>
    transpiration_sink: Scalar<mol_per_s>

    field pressure: WaterPotential
    field water_density: Scalar<mol_per_m>
    field axial_flux: Scalar<mol_per_s>

    // optional 3D embedding for kernels and competition
    field world_x: Scalar<m> over s
    field world_y: Scalar<m> over s
    field world_z: Scalar<m> over s

    relation darcy:
        axial_flux = -axial_conductivity * grad(pressure)

    temporal water_balance:
        d(water_density) = -diverg(axial_flux)

    // Unmatched junctions get generated defaults:
    //   continuity(pressure)
    //   conservation(axial_flux)

    // Example override: pit membranes at branch points.
    // This shadows default pressure continuity, but leaves flux balance intact.
    relation pit_drop_child0 on junction:
        trace(pressure, edge = parent)
            - trace(pressure, edge = child_0)
            = pit_resistance * trace(axial_flux, edge = child_0)

    relation pit_drop_child1 on junction:
        trace(pressure, edge = parent)
            - trace(pressure, edge = child_1)
            = pit_resistance * trace(axial_flux, edge = child_1)

    relation leaf_outflow on terminal:
        trace(axial_flux) = transpiration_sink
}
```

And on the workflow side, I would make the topology/data split explicit:

```python
experiment.assume_topology(
    "tree.hydraulics",
    edges=edge_index,
    edge_length=edge_length,
    root=0,
    vertex_xyz=vertex_xyz,
    vertex_tags=vertex_tags,
)

experiment.compile(
    spatial_config={
        "pasture": {"resolution": [256, 256]},
        "tree.hydraulics": {"cells_per_edge": 1},
    }
)
```

That is consistent with the docs’ separation between world model, workflow, and compiler-owned discretization. 

## 8. What are you still not thinking about?

A few things look niche now but will become blocking very quickly.

* **Topology schema and validation.** `assume_topology` needs a real schema: orientation, lengths, tags, connected components, optional root, optional embedding coordinates, and validation rules. Measured plant graphs are messy. You will get duplicate vertices, dangling segments, unit mismatches, and “branch points” that are really degree-2 segmentation artifacts. The compiler should canonicalize degree-2 Kirchhoff vertices away unless they matter scientifically.  ([Arizona Repository][4])

* **Continuity class of fields.** “Field on a graph” does not always mean continuous at vertices. Pressure often is; concentration under mixing/advection may not be. You will eventually need a continuity story more refined than “all fields continuous unless proven otherwise.” Otherwise your default junction logic will overconstrain transport models. The metric-graph literature already distinguishes edgewise regularity from vertex conditions very sharply. ([Mathematics Home][5])

* **Identified seams and periodic domains.** `SphereSurface` is the first warning sign. A sphere in latitude-longitude coordinates is a single singular chart plus a seam identification, not a true global smooth chart. You can ship that in the stdlib, but general user-authored periodic seams and atlas composition are a later feature, not a detail. 

* **Operator catalogue and Hodge semantics.** “`curl` exists when dimension > 1” is not good enough. Surface curl, 3D curl, and graph analogues are not the same object. For v2.1 I would actually restrict `curl` to `Euclidean<Dim = 3>` and leave manifold Hodge-operator semantics for later. The docs already flag the operator catalogue as unfinished; this is one reason why. 

* **Open-system conservation bookkeeping.** Once a conserved quantity can leave across a terminal or boundary, you need a principled notion of “conserved within container except through declared boundary fluxes.” That is more than event checking. It touches domain boundaries, junction laws, and nested domains all at once. The open questions already point straight at this.  

* **Interpolation and adjoints.** `.at()` and kernel-weighted coupling are not just convenience features; they define hidden approximation operators with gradients. If interpolation order, edge extrapolation, or sparse-neighbor truncation changes, your learned controller gradients change. The docs already know interpolation and kernel optimization are open; they should be visible in plan inspection, not hidden.   

* **Growth vs remeshing.** Time-varying extents are already in scope. Time-varying topology and remeshing are not the same thing. Real plant growth eventually needs both. If you do not separate “metric coefficient changed on fixed topology” from “topology changed” early, the implementation will get tangled fast. 

My bottom line is this:

**Adopt `geometry`.
Do not adopt `interface`.
Adopt loci plus `trace()` plus locus-scoped relations.
Make the metric tensor primary.
Infer default graph junction laws from conservative form, not just `{ conserved }` tags.
Keep embedding on the entity, not in the geometry.
Separate geometry class, topology instance, and discretization config.**

That gives Myco a geometry system that matches its philosophy instead of fighting it.

[1]: https://www.math.ucdavis.edu/~saito/data/LapEig/kuchment-quantum-graphs.pdf?utm_source=chatgpt.com "arXiv:0802.3442v1  [math-ph]  23 Feb 2008"
[2]: https://open.metu.edu.tr/bitstream/handle/11511/98769/thesis.pdf?utm_source=chatgpt.com "https://open.metu.edu.tr/bitstream/handle/11511/98769/thesis.pdf"
[3]: https://motion.me.ucsb.edu/pdf/2020n-skjbss.pdf "https://motion.me.ucsb.edu/pdf/2020n-skjbss.pdf"
[4]: https://repository.arizona.edu/bitstream/handle/10150/665683/azu_etd_19884_sip1_m.pdf?isAllowed=y&sequence=1&utm_source=chatgpt.com "Metric Graphs: Numerical methods, localization, and the ..."
[5]: https://www.math.ucdavis.edu/~saito/data/LapEig/kuchment-quantum-graphs.pdf "https://www.math.ucdavis.edu/~saito/data/LapEig/kuchment-quantum-graphs.pdf"
