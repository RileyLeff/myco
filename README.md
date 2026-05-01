<p align="center">
  <img src="assets/brand/myco-logo-blue-on-white.png" alt="Myco logo" width="420">
</p>

# Myco

Myco is a specification-first programming language project for scientific and mathematical models.

Myco is declarative and acausal by default. The short version: a `.myco` file describes a reusable model of a world, and a workflow is an ordinary Python script using the Myco API to ask a particular question of that model.

The current product is the v2 language specification:

- [planning/v2/spec.md](planning/v2/spec.md)
- [planning/v2/anti_spec.md](planning/v2/anti_spec.md)
- [planning/soul.md](planning/soul.md)

I think the spec is around 85-90% complete, with several open questions left for me to resolve. The implementation has been intentionally reset to bare workspace shells while the v2 spec becomes canonical. The goal is to build from the spec that survives real examples, review, and pressure from hard models.

The surface syntax is heavily Rust-inspired. The design also draws from acausal modeling systems like ModelingToolkit.jl and Modelica, probabilistic programming systems like Stan, proof assistants like Lean, e-graph rewriting, and scientific workflow tooling.

Internally, Myco pairs a type graph with a layered e-graph. The type graph preserves scientific meaning. The e-graph explores what follows from the model, the evidence, and the approximations a workflow explicitly authorized. That matters because real workflows are often underdetermined, overdetermined, or lossy in one direction. Myco needs those cases to remain explainable. More detail is in [Under The Hood](#under-the-hood).

## Why This Exists

Myco began from a frustration that shows up all over scientific programming.

In 2019, I was working as a research technician on several plant hydraulics modeling projects. The strange thing was that different projects kept reaching for the same underlying model with slightly different assumptions, inputs, outputs, fitted parameters, and data sources. Each use case needed its implementation, or required me to write a messy switchboard into someone else's implementation. Each version could drift. The model's math was reusable in principle, but the implementation made reuse brittle.

That felt like a language problem.

I wanted a tool that let me describe the relationships once, compose them out of reusable pieces, and reuse the same model in many workflows. Scientific models often share most of their components, but ordinary implementations make those components hard to recombine. Myco is an attempt to make the reusable structure explicit enough that different models, assumptions, data sources, and workflows can cooperate without turning every new analysis into a fresh rewrite.

Plant systems are early stress tests because they are messy in exactly the useful ways: units matter, data are partial, models mix continuous and discrete structure, assumptions change between projects, and inference often runs through the same equations in different directions. The long-term target is general modeling across domains.

## Tiny Sketch

These examples are design sketches from the current spec direction. The implementation API is still being built.

```myco
type LeafWaterPotential = Scalar<MPa>
type SoilWaterPotential = Scalar<MPa>
type HydraulicConductance = Scalar<mmol_m2_s_MPa>
type Transpiration = Scalar<mmol_m2_s>

relation flow_law(
    soil: SoilWaterPotential,
    leaf: LeafWaterPotential,
    k: HydraulicConductance,
    e: Transpiration,
) {
    e = k * (soil - leaf)
}
```

The relation has no permanent input side. A workflow can use the same relation to predict transpiration from potentials, infer a missing conductance, diagnose incompatible observations, or compile part of a larger plant-atmosphere model.

```python
import myco

model = myco.load("models/plant_water.myco")

run = model.workflow()
run.bind("soil.water_potential", Series(data.soil_psi, unit="MPa"))
run.observe("leaf.water_potential", data.leaf_psi, noise=StudentT())
run.bind("hydraulic_conductance", Trainable(prior=LogNormal()))

fit = run.compile(backend="cpu").sample()
```

## The Core Split

Myco separates a model's description of the world from the workflow that asks a question about that world.

A `.myco` file contains things like:

- Types, units, refinements, domains, and geometry.
- Relations among quantities.
- Conservation laws and other obligations.
- Events, regimes, and discontinuities.
- Probabilistic structure, including distributions, kernels, covariance, and process-valued unknowns.
- Hooks where external evidence, observations, controllers, and workflow choices can bind in.

A workflow is a Python script using the Myco API. It contains things like:

- Which data are being used.
- Which quantities are observed, fixed, inferred, controlled, or predicted.
- Which backend to compile to.
- Which approximations are acceptable.
- Which solver policies, smoothing policies, priors, and diagnostics are active.

That separation is the heart of the project. The same `.myco` model should be able to support simulation, inference, optimization, sensitivity analysis, controller evaluation, and model checking without rewriting the equations for each case.

The product of compilation is generated code plus an inspectable plan. The plan records what the compiler proved, what the workflow supplied, what the backend can support, and which approximations or relaxations were authorized.

## Types, Claims, And Evidence

A Myco type is part of the scientific record. It says what a value can mean downstream, what transformations preserve that meaning, and which claims it can support. Named quantities distinguish values that share units. Units and affine conversions keep measurements honest at the workflow boundary. Refinements, contracts, and geometry/domain types carry bounds, behavior, and location.

That matters when observations are sparse, redundant, or uneven. If a workflow cannot bind every value, the compiler can still push exact consequences through the graph from types, refinements, units, geometry, conservation obligations, matrix facts, and process facts. Unknowns can remain symbolic and inspectable. If a workflow binds several claims about the same quantity, those claims remain typed and attributed so the plan can compare, score, relax, or reject them according to an explicit workflow strategy.

The longer arc is a system where scientific claims stay connected to the observations, assumptions, transformations, approximations, and models that support them. Myco starts with the part that needs precision first: how a model states claims, how those claims interact, and how a workflow turns them into computation while preserving their meaning.

In ordinary code, `x = y` can hide several scientific meanings: an exact law, a measurement, a calibration target, a soft constraint, a lossy inverse, a residual, or an approximation chosen for one run. Myco keeps those meanings separate enough that the compiler can explain what happened. Some claims become exact facts. Some become evidence. Some become residuals or costs. Some require workflow authorization. Some should fail.

## Working With Things Outside Myco

Scientific work spans many systems: instruments, field notes, spreadsheets, databases, papers, simulations, learned controllers, and old code.

Myco should be able to touch those systems without pretending they all speak Myco. The workflow side is where external data, opaque callables, providers, controllers, and backend choices enter. The source model gives those boundaries types, units, contracts, provenance hooks, and obligations so the compiler can inspect what crosses them.

The broader external-integration story needs more design work. For now, the priority is to make Myco's internal semantics consistent, then support a narrow set of workflow boundaries well: dataframes and arrays that bind observed values, ML-framework callables, backend providers, and other explicitly typed sources.

## Relationship To OzzyDB

[OzzyDB](https://github.com/RileyLeff/ozzydb) and Myco are sibling projects.

OzzyDB is about scientific memory: where data came from, how it changed, what context it carried, and how to keep that history queryable.

Myco is about scientific model meaning: what a model claims, which assumptions it depends on, how those claims interact, and how different workflows can ask different questions of the same description.

The shared concern is that scientific work leaks meaning as it moves between notebooks, scripts, datasets, papers, and models. Eventually, I see Myco and OzzyDB unifying around that problem. I am building them separately, with smaller scopes at first, so each project can teach me from real use before I try to combine them.

See [Science Is Leaking](https://rileyleff.com/blog/science-is-leaking) for the longer version of the problem these projects are growing toward.

## Status

This repository is early and intentionally spec-heavy.

What exists now:

- A large canonical v2 spec at [planning/v2/spec.md](planning/v2/spec.md).
- A supporting anti-spec at [planning/v2/anti_spec.md](planning/v2/anti_spec.md).
- A philosophy document at [planning/soul.md](planning/soul.md).
- Evolving model sketches in [planning/v2/mocks](planning/v2/mocks).
- Bare Cargo and uv workspaces for the future implementation.
- Spec navigation and verification helpers in [scripts](scripts).

What is happening next:

- Finish canonicalizing the v2 spec.
- Rewrite and review examples against real modeling papers.
- Turn the spec into a staged compiler and workflow implementation.
- Keep the language honest by testing it against models from plant physiology, probabilistic programming, spatial modeling, kernels, PDEs, and machine learning.

The planned first implementation should support more than one backend from the start. A correctness-first CPU path is useful for debugging and dynamic models. Array and autodiff backends such as JAX or PyTorch are useful when the compiled plan fits their capabilities. Native or Rust lowering may make sense later for performance-critical kernels. Backend capability is explicit because dynamic topology, stochastic inference, differentiability, and sparse spatial operators are supported unevenly across runtimes.

Myco is also being designed for humans and agents. `hypha`, the Myco CLI, is intended to follow the spirit of tools like `cargo` and `uv`: tight verification loops, structured diagnostics, useful compiler explanations, reproducible generated artifacts, and agent-friendly documentation. The aim is for a person or an LLM agent to ask the compiler focused questions and get answers that make the model easier to understand.

The long-term ambition is broad, but the path is deliberately incremental: write the language down, stress-test the abstractions, build the smallest useful compiler, and let real use reshape the next layer.

## Reading The Spec

The canonical spec is long, around 100k tokens. If you have specific questions about Myco, try dropping [planning/v2/spec.md](planning/v2/spec.md) into your favorite LLM and asking it to read the whole thing before answering.

Useful local commands:

```bash
just spec-index
just spec-section 8
just spec-summary 28
just spec-verify
```

`just spec-verify` currently reports a known spec style backlog while the canonicalization pass is underway.

## What Myco Is Trying To Make Natural

Myco is being designed around models that are hard to keep honest in ordinary code.

### Acausal Relations

Many scientific equations describe relationships with no permanent input side or output side.

```myco
relation ideal_gas(p: Pressure, v: Volume, n: Amount, t: Temperature) {
    p * v = n * R * t
}
```

A workflow can bind any compatible subset and ask for the rest, subject to identifiability and solver support.

### Overdetermined And Underdetermined Workflows

Real science often works with sparse, uneven, and partly incompatible observations. This is especially true in open-ended fields like ecology, where a model may be the best available language for a system even when the data cannot ground every quantity. Today, people often make extra assumptions or force a question into an existing framework just to get a solver to run.

Myco treats underdetermination as a useful state. If the workflow supplies only part of the evidence, the compiler should still propagate every consequence it can through the graph: exact equalities, units, refinements, bounds, geometry, conservation obligations, probabilistic envelopes, and contract facts. This is constraint propagation over the scientific structure of the model. The type system and relation graph carry information even when values are missing. Values that remain unknown should remain symbolic and inspectable, with an explanation of which additional evidence would ground them.

For example, an underdetermined plant-atmosphere relation can still have exact consequences if type-level information is strong enough:

```myco
type VaporPressureDeficit = Scalar<kPa> where { self >= zero_like(self) }
type StomatalConductance = Scalar<mmol_m2_s / kPa>
type ClosedStomatalConductance = StomatalConductance where {
    self = zero_like(self)
}
type Transpiration = Scalar<mmol_m2_s>

relation vapor_flow(
    g: ClosedStomatalConductance,
    d: VaporPressureDeficit,
    e: Transpiration,
) {
    e = g * d
}
```

Even if the workflow has no value for `d`, the compiler can derive that `e` is exactly zero because the refinement on `g` is enough. In weaker cases, Myco may derive bounds, units, signs, shape facts, or remaining free variables rather than a ground value.

Overdetermination is also a feature. Real workflows often want to tie together pieces from different model families, compare partly redundant measurements, evaluate agreement between assumptions, or stay robust when some data are missing. In Myco, multiple relations can make claims about the same quantities. The compiler classifies the resulting residual blocks, preserves the source of each claim, and lets the workflow choose an explicit strategy where a choice is legitimate.

This is the detailed version of the claim/evidence story above. A model says what is true. A workflow says whether a redundant block is averaged, selected, scored, relaxed, or rejected. The generated plan preserves the difference between a world claim, a measurement, a residual, and an authorized execution strategy.

### Units, Refinements, And Meaning

Myco wants physical meaning to live in the model itself, where comments, column names, and downstream code can all refer back to it.

```myco
type Temperature = Scalar<K>
type LeafArea = Scalar<m2>
type StomatalConductance = Scalar<mmol_m2_s>

type PositiveConductance = StomatalConductance where { self > 0 }
```

The spec is still refining the best ergonomics for wrapper-like unit patterns, dimensionless phase variables, and domain-specific unit conventions. The design goal is simple: users should be able to express scientific meaning without turning every model into type-system paperwork.

### Geometry And Space

Models often live on domains: leaves, stems, watersheds, ocean grids, tissue surfaces, fault planes, soil columns, and many stranger things.

```myco
type StemAxis : Domain<G = Interval> as (z: Scalar<m>)
type CarbonGrowthDensity = Scalar<mol_C_s / m>
type WholeStemGrowth = Scalar<mol_C_s>

relation total_axis_growth(
    growth_at_z: CarbonGrowthDensity,
    axis: StemAxis,
    total: WholeStemGrowth,
) {
    total = integrate(growth_at_z, z, axis)
}
```

The geometry system is meant to let models talk about where quantities live, how fields move between spaces, and which operations are valid over a domain.

### Probabilistic Models

A model can describe uncertainty as part of the world it is representing.

```myco
type GrowthRate = Scalar<cm / day>
type ObservedGrowth = Scalar<cm / day>
type MeasurementNoise = Scalar<cm / day>

relation growth_observation(
    latent: GrowthRate,
    observed: ObservedGrowth,
    sigma: MeasurementNoise,
) {
    observed ~ Normal(latent, sigma)
}
```

The workflow decides what is observed, what is inferred, and which inference backend is used.

### Kernels And Process-Valued Unknowns

Some unknowns are best represented as structured processes: curves, fields, correlated latent functions, spatial effects, temporal effects, or low-rank approximations.

```myco
type LeafSurface : Domain<G = Euclidean<Dim = 2>> as (
    x: Scalar<m>,
    y: Scalar<m>,
)
type PhotosyntheticCapacity = Scalar<umol_m2_s>
type PhotosyntheticCapacityVariance = Scalar<umol_m2_s * umol_m2_s>

universal leaf_capacity_variance: PhotosyntheticCapacityVariance
universal leaf_capacity_length_scale: Scalar<m>

relation smooth_leaf_variation(
    x: Point<LeafSurface>,
    y: Point<LeafSurface>,
    out: PhotosyntheticCapacityVariance,
) {
    out = leaf_capacity_variance
        * matern52(distance(x, y) / leaf_capacity_length_scale)
}
```

A kernel in `.myco` is an ordinary relation with two typed inputs and a scalar output. Contracts and compiler facts determine whether that relation is a valid covariance kernel. A workflow can bind that structure to a Gaussian process, an HSGP approximation, a low-rank basis, or another compatible process representation.

```python
run.bind(
    "leaf.photosynthetic_capacity",
    ProcessPrior(
        index=["position"],
        value="capacity",
        law=GaussianProcess(
            mean=Zero(),
            kernel="smooth_leaf_variation",
            representation=HSGP(basis_size=48),
        ),
    ),
)
```

### Machine Learning Inside Models

Myco should be able to describe models that include learned components without letting those components erase the surrounding scientific structure.

```python
run.bind(
    "stomata.conductance_policy",
    Controller(
        callable=policy_net,
        reads=["light", "vpd", "leaf_water_potential"],
        writes=["stomatal_conductance"],
        trainable=True,
    ),
)
```

The long-term compiler story is to preserve what can be checked: units, domains, obligations, provenance, approximation choices, differentiability boundaries, and backend capabilities.

## Under The Hood

Myco's source language is meant to feel familiar to people who like Rust's explicitness and compositional style. The project also learns from several neighboring systems:

- ModelingToolkit.jl and Modelica for acausal modeling and symbolic transformation.
- Stan and probabilistic programming systems for model-based inference.
- Lean and proof assistants for the idea that claims should have structure the system can inspect.
- E-graph tools for equality saturation, rewrite search, and explainable compiler decisions.
- Cargo and uv for the feeling of a toolchain that helps continuously instead of appearing only at the end.

Under the hood, the type graph and layered e-graph stay separate. That split is deliberate: facts about meaning should inform computation without collapsing into ordinary value equality.

- Type graph: units, named quantities, contracts, refinements, conversions, generics, domains, and capability facts.
- Exact layer: terms, equalities, rewrites, and algebraic structure.
- Evidence layer: observations, probabilistic envelopes, residuals, and uncertainty.
- Plan layer: backend capabilities, approximation ledgers, lowering choices, provenance, and generated artifacts.

That structure is load-bearing because scientific meaning is not a single relation kind. Exact laws, noisy measurements, residuals, soft constraints, conservation obligations, lossy inverses, and regime-limited claims need different handling. Keeping them separate lets the compiler reject false certainty, explain partial certainty, and preserve workflow choices.

When a workflow supplies too much, too little, or directionally lossy evidence, Myco can surface inconsistency, leave quantities symbolic, or require an explicit cost / relaxation policy. Backend code is generated only after those choices are in the plan.

The backend story follows from that plan. Myco should compile to generated code for the selected backend, and the backend should advertise what it can really do. A CPU backend can be the most permissive and inspectable path for early correctness. JAX-like backends may be excellent for static array and autodiff-heavy plans. PyTorch-like backends may fit some dynamic and learned-component workflows better. The compiler's job is to match the model and workflow to a backend honestly, then generate code whose assumptions can be inspected.
