# Myco V2 Ideas

This document is a vision note, not an implementation plan.

The goal is to capture where Myco could go after the current `v1` milestone:

- what `v2` should probably focus on
- what should likely wait until later
- what the longer-term shape of the system might be

It is intentionally coarse. The point is to preserve direction without freezing design too early.

## Where We Are Coming From

`v1` proved the core compiler story:

- write one structural model in `.myco`
- bind it differently per workflow
- compile an acausal model into a workflow-specific causal plan
- emit ordinary Python or JAX artifacts
- train a learned controller end to end through the emitted artifact

That is already enough to show that Myco is not vapor.

What `v1` does **not** yet try to do is support a serious plant model family. It proves the architecture on a deliberately small dynamical system.

So `v2` should not be "more language for its own sake."

It should be:

- one real model family
- one level up in scientific realism
- only the compiler/runtime additions needed to support that family cleanly

## V2 North Star

The most useful framing for `v2` is:

> Myco should be able to express and train one genuinely plant-relevant model family with mixed mechanistic structure, trainable parameters, sparse observations, and at least one local implicit or registry-backed scientific function block.

That would move Myco from:

- "compiler prototype with a toy demo"

to:

- "credible research tool for a first real ecophysiology workflow"

## What V2 Should Probably Be About

These feel like the right `v2` directions.

### 1. One Real Plant Model Family

`v2` should target a concrete family rather than abstract generality.

Plausible candidates:

- a discrete-time hydraulic + stomatal control model
- a simplified water + carbon + allocation model
- a Farquhar-lite photosynthesis + stomatal coupling model

The best `v2` target is probably the one that:

- still fits the discrete-time compiler shape
- creates real scientific value quickly
- forces only a few carefully chosen new compiler features

The key idea is to let the model family pull the language and runtime forward, rather than inventing features speculatively.

### 2. Explicit Parameters

`v1` treats static values mostly as constants, but real scientific models want a more explicit parameter role.

`v2` likely needs a first-class idea of:

- fixed parameters
- learned parameters
- maybe grouped or contextual parameters later

Examples:

- hydraulic conductivity
- vulnerability curve parameters
- photosynthesis coefficients
- respiration coefficients
- allometric constants

The important distinction is that parameters are:

- not external forcing
- not temporal state
- not merely derived quantities

They are stable within a rollout, but often trainable across experiments.

### 3. Semantic Schemas And Component Types

One thing that likely wants to get clearer after `v1` is the separation between:

- base quantity types and shapes
- semantic wrappers or component schemas
- relationships and functions over those quantities

Right now Myco has a useful but still relatively flat notion of quantities and relations.

Longer-term, it may be valuable to support semantic component definitions that expand into more basal pieces and are mostly erased by the compiler.

Examples:

- `Leaf`
- `RootSegment`
- `CanopyLayer`
- `Tree`

These would not primarily be "types" in the physical-dimension sense. They would be semantic schemas or bundles that could contain:

- quantities
- parameters
- defaults
- constraints
- observation operators
- functions
- relations

This is what could eventually make things like the following feel natural:

- import a package that exports only types or component schemas
- import a package that exports only functions
- import a package that exports a full model bundle
- instantiate something like `Sperry_2017_Tree` with traits and parameters rather than rebuilding it manually

The important design instinct here is:

- the user-facing semantic structure can be rich
- the compiler core can still erase it into a flatter IR for planning and emission

That feels like a powerful UX direction, but it should likely be layered above the current core rather than baked directly into the lowest-level compiler representation.

### 4. Function Registry

This is probably the single biggest unlock after `v1`.

The long-term system should not require every scientific relation to be decomposed manually into primitive arithmetic in surface syntax.

The healthiest shape is probably:

- a small standard library for very common operations
- a package-backed registry for domain-specific functions

So the goal is likely not "put all scientific functions in Myco core."

It is closer to:

- ship a tiny standard library for common reusable pieces
- let real scientific content live in versioned packages
- allow models to depend on those packages explicitly

That would support workflows like:

- core language and common runtime helpers shipped by Myco
- domain packages such as `rileyleff/plant_ecophys`
- published reusable function families and model pieces imported rather than recopied

Myco will likely want a registry of named scientific functions carrying:

- signature
- dimension and kind contracts
- backend emission logic
- optional inverse or monotonicity information
- exact vs approximate status
- differentiability notes

This is how Myco could represent things like:

- vulnerability curves
- temperature response functions
- saturating uptake curves
- smooth thresholds
- observation operators

The registry matters because it lets the compiler stay small while the domain library grows richer.

### 5. Local Solve Blocks

Real plant models often contain same-step coupling that should not be rejected forever as an algebraic loop.

Examples:

- assimilation / intercellular CO2 / stomata coupling
- hydraulic supply-demand balance
- leaf water potential equilibrium within a timestep

`v2` likely needs a way to say:

- these equations form a local implicit subproblem
- these variables are the unknowns
- solve them each step with a declared strategy

That is much more realistic than expecting symbolic inversion alone to handle everything.

This would still fit a discrete-time global model:

- temporal rollout remains explicit
- local implicit solves happen inside a step

### 6. Observation Operators

`v1` observations are close to identity-on-node.

That is fine for TinyTree, but real research workflows need observation models.

Examples:

- sap flux as an operator on transpiration and sapwood area
- water potential sampling operator
- gas exchange operator
- canopy or plot aggregation
- irregular sampling windows

So `v2` likely needs:

- explicit observation operators
- operator-specific metadata
- a slightly richer loss interface

This is important because many real datasets do not observe state variables directly.

### 7. Data And Observation Indexing

Another place that likely wants a cleaner long-term story is assignment of sparse or irregular data into a rollout grid.

`v1` made the right simplification:

- dense forcing
- masked observations

But real workflows will often want:

- multiple sparse vectors
- observation series shorter than the compile horizon
- explicit assignment of values to timestep indices
- eventually timestamp-based alignment

So `v2` or soon after likely needs a clearer indexing model for bindings and observations.

Potential directions:

- explicit timestep index arrays
- named schedules reusable across bindings
- timestamp arrays aligned to a compile grid
- observation operators with attached index metadata

This matters because otherwise "here are four short vectors" becomes ambiguous very quickly.

### 8. Demand-Driven Planning

`v1` plans the single-step graph in a mostly whole-graph way.

That was the right tradeoff for getting the compiler real quickly.

But `v2` should probably move toward demand-driven compilation:

- identify required quantities
- backward-slice the relevant subgraph
- choose paths only for that slice
- schedule the minimal required plan

That will matter once a model has many variables and multiple optional substructures.

### 9. Better Runtime Contracts

The current generated artifacts are already usable, but `v2` will probably want stronger contracts around:

- runtime payload validation
- slot provider signatures
- parameter trees
- save-point selection
- batched evaluation

This is especially important if Myco starts being used for repeated scientific training workflows rather than only one-off demos.

### 10. Better Binding Ergonomics

The current Python binding API is explicit and semantically honest, which was the right choice for `v1`.

But it is also fairly verbose.

That is not necessarily a crisis, especially in an agent-assisted world, but it is probably not the final UX.

Later layers may want:

- bulk binding from mappings
- grouped forcing binding helpers
- grouped observation binding helpers
- adapters from `pandas`, `xarray`, or other structured containers
- more declarative experiment constructors

This should be approached carefully. The current explicitness is valuable. The goal should be to reduce repetition without making the binding semantics fuzzy again.

### 11. Richer Constraint Design Space

Constraints are another area where `v1` intentionally stayed narrow.

Longer-term, it would be useful to think much more broadly about what kinds of constraints Myco might want to express, even before all of them are implemented.

Possible classes include:

- simple bounds
- simplex and sum-to-one constraints
- monotonicity constraints
- smoothness and total-variation penalties
- rate constraints
- ordering constraints
- conservation constraints
- temporal coupling constraints
- structural parameter-family constraints
- custom registry-backed constraints

The important thing is that these do not all have the same runtime meaning.

Different constraints may lower to:

- compile-time checks
- projections
- penalties
- assertions
- solver-side conditions

So the future direction is probably not "one giant generic constraint system." It is a richer catalog of constraint kinds with explicit lowering semantics.

### 12. Teaching Demos For Constraint Structure

There is also value in building better demo and teaching examples, not just richer features.

Especially useful would be examples showing:

- overdetermined quantities
- underdetermined quantities
- partially constrained latent quantities
- difficult or non-invertible functions
- what Myco can infer automatically
- what it cannot infer without additional assumptions or solve machinery

These examples would help shape both the language and the user mental model.

### 13. Explicit Uncertainty Patterns

Another long-term direction worth keeping in mind is uncertainty-aware modeling.

One possible temptation would be to add special built-in concepts like:

- probabilistic nodes
- nodes with value plus variance
- first-class distributions everywhere

But a simpler and more composable direction may be:

- keep the core graph semantics simple
- represent uncertainty explicitly in the world model using ordinary structure

Examples:

- a Gaussian-valued quantity represented by mean and standard deviation nodes
- a distribution wrapper schema that expands into parameter nodes
- a "value plus uncertainty" semantic component layered above ordinary quantities

That would let uncertainty remain visible and inspectable rather than becoming hidden compiler magic.

So the likely direction is:

- no rush toward a special probabilistic core
- prefer explicit model patterns first
- only add deeper uncertainty semantics if real workflows demand more than those patterns can express

## Candidate V2 Model Shapes

These are not commitments. They are candidate scientific anchors for `v2`.

### Option A: Hydraulic + Stomatal Control

Likely ingredients:

- soil water forcing
- hydraulic state
- stomatal controller
- transpiration
- sparse water potential observations
- dense sap flux or transpiration-like observations

Why it is attractive:

- close to the current TinyTree shape
- strong continuity with `v1`
- likely to matter quickly for real field data

### Option B: Water + Carbon + Allocation

Likely ingredients:

- water state
- NSC / carbon pool state
- allocation fractions
- growth or respiration terms
- simplex or bound constraints
- a learned controller replacing hand-written allocation heuristics

Why it is attractive:

- clean fit to the "learned controller over mechanistic substrate" story
- very compatible with discrete time
- could support interesting ecological questions without full gas-exchange complexity

### Option C: Farquhar-Lite + Stomata

Likely ingredients:

- a small parameter set
- function registry support
- a local solve block for assimilation and stomatal coupling

Why it is attractive:

- scientifically important
- pushes Myco toward one of its most ambitious eventual use cases

Why it is risky:

- likely the fastest way to expand both the language and runtime simultaneously

If momentum matters most, a hydraulic or water-carbon model is probably the better first `v2` target.

## Backend Direction

One important strategic question is whether Myco should become deeply JAX-specific or remain fundamentally backend-agnostic.

My current view is:

- Myco core should remain backend-agnostic
- JAX should remain the first-class and best-supported backend for the near term

That means:

- the compiler should target an internal representation and plan that are not inherently JAX-only
- backend emitters should remain replaceable
- scientific function registries may carry backend-specific implementations

This matters because there are plausible future backends beyond JAX:

- PyTorch
- a Rust-native backend such as Burn
- maybe other ML or simulation runtimes later

At the same time, it would be a mistake to pretend all backends are equally important right now.

JAX is currently the strongest fit for the project because it already supports:

- functional rollout style
- `scan`
- autodiff through compiled dynamics
- a clean scientific-ML workflow

So the healthy stance is probably:

- backend-agnostic architecture
- JAX-first product quality
- other backends only when a concrete use case demands them

In practice that likely means:

- keep core planning and equality reasoning independent of backend concerns
- treat emitted runtime contracts as a backend-neutral interface
- let each backend own its own artifact shape, runtime helpers, and performance strategy

## Registry And Package Ecosystem

Once Myco grows a function registry or semantic component layer, it will likely also want a package and dependency story.

Longer-term, useful directions might include:

- versioned package dependencies declared by models
- packages that export only schemas or semantic component types
- packages that export only scientific functions
- packages that export complete model bundles
- registry resolution pinned to explicit versions
- a tiny standard library shipped with Myco plus richer package registries above it

This is what could make workflows like these feel natural:

- import a named plant model family and fill in parameters
- reuse a published package across projects
- depend on a domain package such as `rileyleff/plant_ecophys`
- depend on a domain library without copying all of its internals into one repo

This should likely be designed as a layer above the core compiler, not as a requirement for the core itself to understand package management deeply.

## What Probably Belongs After V2

These are things that feel important, but not necessarily immediate.

### 1. Richer Scientific Function Libraries

Once the function registry exists, Myco could grow a real domain library.

Possible families:

- Farquhar/FvCB pieces
- Medlyn / Ball-Berry style pieces
- Sperry-style hydraulic functions
- vulnerability curve families
- temperature correction functions
- growth and respiration modules

This likely belongs in a domain package rather than the core compiler.

### 2. Multi-Experiment / Hierarchical Fitting

Later versions may want workflows like:

- shared global structure
- species-level parameters
- site-level parameters
- experiment-specific forcing
- shared learned controllers with contextual inputs

That would matter for moving from one plant or one plot toward comparative biology.

### 3. Better Backend IR And Runtime Shapes

The current JAX emitter is intentionally readable and flexible.

Later systems may want:

- more static structs and less dict-based runtime access
- better batching
- selected-history saving
- less Python overhead in traced code
- cleaner integration with production training loops

This feels more like an optimization and scaling phase than a language-design phase.

### 4. More Expressive Constraints

Potential later directions:

- simplex constraints
- rate constraints
- structural inequalities
- piecewise policies
- richer penalty families
- richer semantic type or schema-level constraints

This should be driven by real model needs rather than by abstract type-system ambition.

### 5. Broader Time Semantics

Potential later directions:

- adaptive stepping
- event-like structures
- asynchronous observations and forcing
- partial interpolation policies

This is likely useful eventually, but not necessary to justify the current compiler approach.

## V3 And Longer-Term Directions

The longer-term system could become significantly more ambitious, but it helps to separate "possible future" from "next step."

### 1. Full Plant Modeling Substrate

In a strong long-term version, Myco could be the substrate for expressing:

- hydraulic systems
- photosynthesis and gas exchange
- carbon allocation
- growth and respiration
- seasonal strategy and phenology
- observation models
- learned control policies

The compiler would remain responsible for:

- structural reasoning
- workflow-specific compilation
- code emission

but not for scientific content itself.

### 2. Domain Libraries On Top Of Myco

The healthiest long-term shape may be layered:

- `myco-core`
- `myco-jax`
- maybe `myco-torch`
- maybe `myco-burn`
- a tiny Myco standard library
- one or more domain libraries like a `myco-plant`
- package namespaces for published scientific work

That would keep the compiler general while letting real scientific building blocks accumulate in a domain-specific way.

### 3. Mechanistic + Learned Hybrid Inference At Scale

A mature Myco could support workflows like:

- partially observed state inference
- learned policies embedded in mechanistic dynamics
- shared parameters across many experiments
- contextual conditioning on species, site, or treatment
- counterfactual interventions on forcing or control structure

That is likely where the project becomes most scientifically distinctive.

### 4. Ecosystem And Stand-Level Extensions

Later still, Myco could potentially express:

- multiple individuals
- canopy aggregation
- resource competition
- stand-level fluxes
- ecosystem-scale observation operators

This is likely well beyond `v2`, but it is one plausible destination.

### 5. Better Interactive Tooling

Longer-term UX ideas:

- richer explainability tools
- graph visualization
- compile-plan visual inspection
- model debugging notebooks
- auto-generated docs from `.myco` models
- registry browsing and schema exploration tools

These are not core to scientific correctness, but they could make the system much more usable.

## Things To Be Careful About

There are a few failure modes that are easy to slip into after a successful `v1`.

### 1. Turning V2 Into "More Syntax"

The language should grow only when a concrete model family requires it.

New syntax without a target scientific use case is likely to create drag.

### 2. Turning The Core Into A Domain Library

The compiler core should stay focused on:

- representation
- planning
- validation
- emission

It should not absorb every scientific equation family directly.

The same caution applies to schemas, registries, and package ecosystems: they are important, but they should sit on top of a small core rather than collapse into it.

### 3. Overcommitting To Continuous-Time Too Early

A lot of scientifically meaningful work is still possible with:

- discrete time
- local within-step solves
- explicit state updates
- sparse observations

Jumping straight to fully general continuous-time differentiable modeling may slow down the most important scientific milestones.

### 4. Letting V2 Become A General CAS Project

Myco is at its best when it uses symbolic reasoning in service of scientific compilation.

It should keep resisting the temptation to become general-purpose algebra software.

## What Success Could Look Like

A good `v2` would probably mean something like:

- one real plant-relevant model family implemented cleanly
- one or two carefully chosen new compiler capabilities added
- stronger scientific data interfaces
- a clearer story for semantic schemas, function packages, or both
- a credible path from TinyTree to a paper-relevant workflow

A good long-term outcome would mean:

- Myco becomes the layer where structural models are declared once
- workflows are bound explicitly
- emitted backend artifacts remain ordinary tools
- a small standard library exists, while real scientific content lives in versioned packages
- learned components sit on top of, not instead of, mechanistic structure

## Short Version

If this whole document had to collapse into one paragraph, it would be this:

`v2` should not try to make Myco universally expressive. It should make Myco capable of one real plant-model family by adding only the features that family forces into existence: probably explicit parameters, a function registry, local solve blocks, richer observation operators, and more demand-driven planning. After that, later versions can broaden into domain libraries, hierarchical fitting, stronger backend runtimes, richer constraints, and eventually larger-scale plant and ecosystem workflows.
