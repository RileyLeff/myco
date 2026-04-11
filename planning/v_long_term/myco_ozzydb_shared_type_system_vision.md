# Myco, OzzyDB, And A Shared Type System

This note is a standalone record of a design conversation about where Myco may be headed after `v1`, especially in relation to OzzyDB and the possibility of a broader scientific platform.

It is not an implementation plan.

It is not a final spec.

It is a longer-term architectural and philosophical note about:

- what Myco should own
- what OzzyDB should own
- what a shared type-and-constraint system might make possible
- why agents may change the adoption story for formal scientific systems

This note is intentionally expansive. The point is to capture the north star without forcing everything into `v2`.

## Short Version

The core intuition is:

- Myco is about **proof by construction**
- OzzyDB is about **proof by observation**
- the bridge between them is likely a **shared type-and-constraint system**

In this framing:

- Myco represents executable structural claims about the world
- OzzyDB represents typed evidence, transforms, artifacts, and provenance
- a shared type system lets models, claims, data, transforms, and evidence speak a common language

This could eventually support a much richer scientific workflow than today's PDF-plus-CSV publication model.

## What Myco Is Best At

Myco currently provides:

- a structural world model
- explicit relations and constraints
- compile-time binding for workflow-specific roles
- compilation into ordinary executable artifacts
- support for simulation, fitting, and learned control over mechanistic structure

So Myco is best understood as a language for expressing scientific construction:

- what quantities exist
- what relationships hold
- what constraints hold
- what follows when a particular workflow binds the system in a particular way

In that sense, it is about proof by construction.

## What OzzyDB Is Best At

OzzyDB, at least in its current shape, is best understood as a provenance system over typed computation.

It provides:

- typed artifacts
- typed transforms
- reproducible fetch against pinned revisions and environments
- conformance states and evidence
- provenance-bearing execution and caching

So OzzyDB is best understood as a system for proof by observation and typed evidence:

- what data exists
- what transform produced it
- what environment it ran in
- what type it conforms to
- what other artifacts and transforms it depends on

That is a very different but complementary role.

## Why They Seem Related

They are related because both systems are really about typed scientific objects.

Myco wants to say:

- this model has these quantities
- these constraints hold
- this relation is executable
- this compiled path is valid under these assumptions

OzzyDB wants to say:

- this artifact has this type
- this transform consumes and produces these types
- this output is reproducible from these typed inputs and versions
- this evidence conforms to this contract

Underneath both is a similar question:

> how do we represent scientific structure explicitly enough that it can be checked, composed, reused, and reasoned about by both people and machines?

That is why the type system matters so much.

## The Core Vision

The deeper opportunity is not only:

- a better model language

or only:

- a better data/provenance registry

It is something more like:

- a typed graph of scientific objects
- where evidence, transforms, models, assumptions, claims, and constraints can all be represented explicitly
- where executable model structure and empirical evidence can interact directly
- where claims are backed by both construction and observation

That is a much higher-bandwidth way for scientific meaning to move than prose alone.

## Proof By Construction And Proof By Observation

One of the most useful framings from this conversation is:

- **proof by construction**
- **proof by observation**

Proof by construction means things like:

- mechanistic models
- executable relationships
- typed constraints
- derivations
- structured assumptions

Proof by observation means things like:

- raw and derived datasets
- typed transforms
- conformance records
- provenance
- empirical evidence supporting or falsifying claims

Myco is naturally oriented toward the first.

OzzyDB is naturally oriented toward the second.

The long-term scientific payoff comes from connecting them.

## Why This Matters For Science

Much of modern science is still communicated in a very low-bandwidth way:

1. collect expensive or difficult data
2. run a small set of conventional analyses
3. write prose interpretations
4. publish a PDF
5. maybe upload raw files somewhere

This tends to produce:

- weak connections between evidence and claim
- poor machine readability
- poor reuse
- limited reproducibility
- a lot of scientific meaning trapped in text

Even when data is "open," it is often open as a bucket of files with weak, inconsistent, or overly local metadata.

The result is that a huge amount of scientific structure is lost.

## Why Agents Change The Adoption Story

Historically, scientists have been unwilling to adopt highly formal tools for open-ended research.

That makes sense.

The main reasons are:

- the learning cost is too high
- the payoff is too delayed
- the benefit is weak unless many others also adopt the system

In other words, a formal scientific system used to ask people to do extra work before they got real value.

Agents may change this.

With good agents, the workflow can become:

- think and work in natural language
- work from ordinary files, tables, and scripts
- ask the agent to formalize incrementally
- inspect and correct the formalization
- get immediate benefits from typed structure, validation, reuse, and composition

That means the barrier to entry can become lower than today's brittle publication-and-supplement model rather than higher.

This is a very important reason the larger vision may be timely now in a way it was not before.

## The World Model Versus The Workflow

One important conclusion from this conversation is that Myco should keep strengthening the separation between:

- the structural world model
- the workflow binding/configuration

The `.myco` world should primarily describe:

- quantities
- relationships
- constraints
- temporal structure
- semantic schemas later

The binding/config layer should describe:

- what is provided
- what is observed
- what is fixed
- what is learned
- what is rollout-persistent for this workflow
- what outputs are required

This matters because it makes structural models more reusable and sharable.

The same world should support many workflows without being rewritten to reflect one use case's provisioning assumptions.

This same principle likely generalizes to the broader platform:

- typed scientific objects should not bake in local workflow assumptions when they can instead expose clean contracts

## Types, Schemas, And Semantic Layers

Another important thread is that "type" is probably doing several different jobs.

There is likely a future distinction between:

- low-level quantity typing and dimensional metadata
- grouped or semantic schemas
- relationships/functions over those schemas

Examples:

- `Leaf`
- `Tree`
- `CanopyLayer`
- `DietaryComposition`
- `Sperry_2017_Tree`

These are not just scalar labels.

They may bundle:

- quantities
- grouped constraints
- parameters
- defaults
- functions
- observation operators
- relations

The important design instinct is:

- keep the executable core simple
- allow richer semantic layers above it
- preserve those semantics for interpretability, sharing, and reasoning
- erase them only as far as execution actually requires

That means Myco may eventually want something like semantic schemas or components that compile down into a simpler core graph.

## Grouped Constraints And "Types" Over Multiple Nodes

The conversation surfaced an important example:

- dietary composition with `protein`, `carbs`, and `fat`
- each component individually bounded
- all components together constrained to sum to 1

That is not naturally captured by independent scalar node labels.

It implies something like:

- a grouped semantic object
- with member quantities
- plus structural constraints over the group

That means some future "types" or schemas may need to behave more like structured contracts than simple categories.

This is one reason the type system likely needs to be richer than a flat set of labels.

## Declarative Core, Optional Scripting Front-Ends

A key question is whether future types and schemas should be authorable directly in a general scripting language like Python or Lua.

The most likely healthy answer is:

- **declarative core**
- **optional scripting helpers**

In other words:

- the canonical meaning should remain declarative, inspectable, and shareable
- but host-language tooling can help generate or compose those declarations

This matters because:

- arbitrary scripting is convenient
- but canonicalization, proving, registry publication, and machine reasoning need something more stable than opaque scripts

So the likely future is:

- a real type/schema/constraint language
- plus helper APIs/macros/generators in host languages

## The Function Registry

The conversation also sharpened what "function registry" should mean.

The healthiest shape is probably:

- a very small Myco standard library for common operations
- richer scientific function families living in versioned packages

So plant physiology content should not be hard-coded into Myco core.

It should probably live in packages such as:

- `rileyleff/plant_ecophys`

That package ecosystem would ideally support:

- reusable function families
- reusable schemas/components
- reusable model bundles
- reproducible, pinned imports

That is much more compatible with shareability than requiring people to rebuild your model from scratch every time.

## A Shared Type System Does Not Mean A Shared Everything System

One thing that became clearer in this conversation is that convergence does not require monolithic unification.

The likely shape is layered.

For example:

- Myco owns executable structural modeling
- OzzyDB owns typed artifacts, evidence, transforms, and provenance
- a shared type-and-constraint layer sits between them
- package registries sit above them

That means the systems can stay separately useful while still converging over time.

This is probably much healthier than trying to merge them prematurely into one giant platform.

## Logical Relations Versus Operational Conversion

One of the deepest technical ideas to come out of this conversation is the difference between:

- logical/refinement relations
- operational transform/conversion relations

This matters a lot.

### Logical Relations

Examples:

- equivalence
- refinement
- conformance
- structural compatibility
- proof that a constraint implies another constraint

These are about meaning.

### Operational Conversion

Examples:

- Polars DataFrame -> Arrow
- Arrow -> R Tibble
- Parquet -> CSV
- Myco graph -> JAX artifact
- model output -> observation operator output

These are about actual transforms in the world.

They may be:

- lossless
- lossy
- exact
- approximate
- reversible
- conditionally reversible
- one-way
- preferred
- expensive
- verified

The polars/arrow/tibble example is especially useful because it shows that the system should be able to reason about available transform paths without collapsing convertibility into simple equivalence.

That is very similar in spirit to Myco's question of not just:

- "is this invertible?"

but:

- "when is it invertible?"
- "how reversible is it?"
- "what assumptions make it valid?"
- "how much information is lost?"

This seems like one of the strongest conceptual bridges between Myco and OzzyDB.

## Lossiness, Reversibility, And Typed Morphisms

Parquet -> CSV is not merely "a conversion."

It may preserve some things and lose others:

- column values may survive
- metadata may not
- richer schema information may not
- nested structure may flatten

That suggests that transform edges should eventually carry properties like:

- exact vs approximate
- lossless vs lossy
- reversible vs one-way
- assumptions required
- validation strategy
- preference/cost

This is very close to the richer directional metadata Myco may eventually want for equations and function inverses.

In both cases, the deeper idea is:

- transforms are typed morphisms with properties

not just edges that happen to exist.

## Conformance As Evidence

Another important connection is the idea that conformance should not just be boolean metadata.

Ideally, conformance should produce something more like:

- a witness
- a verification record
- a structured explanation of what was checked and what passed

That matters on the OzzyDB side for artifacts.

It may also matter on the Myco side later for:

- constraint satisfaction
- invertibility assumptions
- branch validity
- compatibility between models and data

This is part of why a richer type system could become much more powerful than ordinary schema validation.

## Uncertainty As Explicit Structure

The conversation also touched on uncertainty.

A tempting path would be to make uncertainty a magical built-in property of every node.

A more plausible near- to medium-term direction is:

- keep uncertainty explicit in the model
- represent it through ordinary structure

Examples:

- mean and standard deviation nodes
- distribution-parameter bundles
- semantic wrappers that expand into underlying parameter nodes

This preserves clarity and inspectability and avoids forcing a probabilistic ontology into the core too early.

Later, if the workflows demand deeper uncertainty semantics, that can be revisited.

## The Backend Question

The conversation also clarified backend strategy.

Myco should probably be:

- backend-agnostic in core architecture
- JAX-first in current product quality

That means:

- parsing, planning, equality reasoning, and types should not depend on JAX assumptions
- backend emitters can differ
- JAX can remain the best-supported backend for now
- other backends like PyTorch or Burn can be added if concrete use cases justify them

This aligns with the OzzyDB lesson too:

- semantic meaning should stay distinct from runtime realization

## The Biggest Long-Term Opportunity

The biggest long-term opportunity may be a truly excellent type-and-constraint system for science.

Not just:

- data schemas
- dimensional units
- simple node labels

But a richer system that can carry:

- grouped constraints
- dimensional facts
- assumptions
- branch conditions
- invertibility conditions
- observation contracts
- evidence links
- provenance-bearing claims

That could support:

- safer composition of models and data
- stronger machine reasoning
- richer agent assistance
- theorem-prover-like questions about scientific structure

This is where the long-term Lean / CAS / SMT inspiration starts to become relevant.

Not because Myco should immediately become a proof assistant, but because the core scientific questions begin to look like:

- what do we know?
- what follows from it?
- under what assumptions?
- what transformations preserve meaning?
- what transforms are lossy?
- what claims are actually supported by evidence?

## Why This Might Be Better Than Today's Publication System

The current publication system has serious limitations:

- low machine readability
- weakly formalized evidence chains
- poor reuse
- detached datasets
- claims mostly embedded in prose

A typed, agent-mediated scientific platform could instead encourage:

- reusable executable models
- typed evidence artifacts
- explicit transforms
- explicit claims and assumptions
- reproducible composition
- discussion around formal objects, not only text

The point is not to eliminate informal science.

It is to make formal structure easier to build and easier to share.

## A Plausible Long-Term Shape

One plausible long-term architecture looks like:

### Layer 1: Shared Type-And-Constraint Core

Owns:

- logical/refinement relations
- grouped constraints
- semantic schemas/components
- conformance and witness concepts
- maybe stronger symbolic reasoning later

### Layer 2: Operational Transform Graph

Owns:

- adapters
- conversions
- lossiness/reversibility metadata
- backend realizations
- runtime capabilities

### Layer 3: Myco

Owns:

- executable structural models
- planning over typed relations
- proof by construction

### Layer 4: OzzyDB

Owns:

- typed artifacts
- typed transforms
- provenance/evidence
- proof by observation

### Layer 5: Package And Registry Ecosystem

Owns:

- versioned scientific packages
- reusable functions
- reusable schemas
- reusable model bundles
- lockfiles and reproducible resolution

That shape preserves flexibility while still allowing the systems to converge meaningfully.

## What Not To Do Too Early

There are several obvious failure modes:

- prematurely building a giant ontology
- prematurely building a universal CAS/proof system
- over-baking domain content into Myco core
- merging Myco and OzzyDB before their shared layer is ready
- requiring scientists to formalize everything manually before they get value

The healthier path is probably:

- simple core primitives
- good type contracts
- explicit provenance
- typed sharing
- agents as the human-formalization adapter
- bring-your-own tools wherever possible

## The Main Conclusion

The main conclusion of this conversation is that the larger opportunity is not only a better model language.

It is a broader substrate for scientific knowledge exchange:

- executable construction
- observational evidence
- shared typed meaning
- transform graphs with explicit lossiness and reversibility
- package-based reuse
- agent-mediated formalization

That vision is ambitious, but it is internally coherent.

And importantly, it can be pursued incrementally:

- build Myco first
- preserve the right abstractions
- let OzzyDB and the shared type layer converge when ready
- keep the long-term north star visible without forcing premature unification
