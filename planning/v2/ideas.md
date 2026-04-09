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

### 3. Function Registry

This is probably the single biggest unlock after `v1`.

The long-term system should not require every scientific relation to be decomposed manually into primitive arithmetic in surface syntax.

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

### 4. Local Solve Blocks

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

### 5. Observation Operators

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

### 6. Demand-Driven Planning

`v1` plans the single-step graph in a mostly whole-graph way.

That was the right tradeoff for getting the compiler real quickly.

But `v2` should probably move toward demand-driven compilation:

- identify required quantities
- backward-slice the relevant subgraph
- choose paths only for that slice
- schedule the minimal required plan

That will matter once a model has many variables and multiple optional substructures.

### 7. Better Runtime Contracts

The current generated artifacts are already usable, but `v2` will probably want stronger contracts around:

- runtime payload validation
- slot provider signatures
- parameter trees
- save-point selection
- batched evaluation

This is especially important if Myco starts being used for repeated scientific training workflows rather than only one-off demos.

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
- one or more domain libraries like a `myco-plant`

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
- a credible path from TinyTree to a paper-relevant workflow

A good long-term outcome would mean:

- Myco becomes the layer where structural models are declared once
- workflows are bound explicitly
- emitted backend artifacts remain ordinary tools
- domain libraries carry real scientific content
- learned components sit on top of, not instead of, mechanistic structure

## Short Version

If this whole document had to collapse into one paragraph, it would be this:

`v2` should not try to make Myco universally expressive. It should make Myco capable of one real plant-model family by adding only the features that family forces into existence: probably explicit parameters, a function registry, local solve blocks, richer observation operators, and more demand-driven planning. After that, later versions can broaden into domain libraries, hierarchical fitting, stronger backend runtimes, richer constraints, and eventually larger-scale plant and ecosystem workflows.
