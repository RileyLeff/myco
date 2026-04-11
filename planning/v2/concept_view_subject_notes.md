# Concept, View, Subject, And Data

This note captures a narrower design thread within the broader `v2` and shared-type-system discussion:

- how to think about scientific concepts
- how to think about different views of the same concept
- how data and subject tracking relate to those views
- how this might shape the future type system for Myco and its eventual relationship to OzzyDB

This is not an implementation plan.

It is a conceptual note to keep the project from collapsing too early into a flat notion of "type."

## Why This Matters

One of the big risks in a scientific type system is treating every meaningful thing as:

- a flat label
- or a flat schema
- or a runtime object like a dataframe

That is too coarse.

Science does not usually work with one canonical representation of an object.

A `Tree` is a good example.

To one study, a tree may be:

- a hydraulic system
- a control problem
- a carbon allocation system
- a point in trait space
- a member of a community
- an individual at a GPS point with a measurable history

Those are not all the same formalization.

But they are not unrelated either.

So the type system needs to help different scientific compressions of the same thing meet each other rather than forcing everything into one rigid schema.

## A Working Stack

The cleanest layered model from this conversation looks something like:

1. **Concept**
2. **Subject**
3. **View**
4. **World / context**
5. **Measurements / evidence**
6. **Models / claims**
7. **Representations / carriers**

These are not all the same kind of object.

## 1. Concept

A concept is the thing in the world we think we are talking about.

Examples:

- `Tree`
- `Leaf`
- `SoilColumn`
- `DietaryComposition`
- `SapFluxObservation`

A concept is not yet the full formalization of the thing.

It is closer to:

- the scientific object of interest
- the entity or phenomenon around which different views may be built

One reason this matters is that many different studies can talk about "tree" while meaning different typed projections of it.

## 2. Subject

A subject is a particular instance of a concept in a particular study or dataset.

Examples:

- tree 17 in plot 3
- a specific loblolly pine
- this plant sampled at these GPS coordinates
- this participant in a dietary study

This is where "subject tracking" starts to matter.

A subject may carry things like:

- species
- age
- location
- treatment
- DBH
- plot membership
- identity across time

This is not the same as the abstract concept of `Tree`, and it is not the same as a particular scientific view of a tree either.

It is the thing that data and evidence often attach to.

## 3. View

A view is a typed scientific framing or projection of a concept.

Examples for `Tree`:

- hydraulic view
- carbon-allocation view
- trait-space view
- community/ecosystem view

Each view chooses some features, relationships, and constraints as relevant while ignoring others.

For example:

- in one study, a tree may have canopy water potential and NSC composition as key features
- in another, the tree may be represented mainly by trait coordinates
- in another, the tree may be represented through neighborhood and environment class

The important idea is:

- a view is not the concept itself
- it is a structured scientific compression of the concept for some purpose

This feels like one of the most important abstractions for the long-term system.

## 4. World / Context

A world is the modeled setting in which a view is meaningful.

Examples:

- a world with categorical environments
- a world with certain ecological processes active
- a world with a specific temporal scale
- a world where only some variables are tracked explicitly

This matters because a view does not float in the void.

For example:

- in one model world, a tree may have NSC composition and canopy water potential
- in another, nitrogen processes may be central instead
- in another, community environment class may be essential

So a world/context defines part of what it even means for a view to be appropriate.

## 5. Measurements / Evidence

Measurements are how subjects become connected to evidence.

Examples:

- sap flux measurements
- water potential measurements
- trait measurements
- environmental covariates
- repeated observations over time

These are not automatically the same thing as model variables.

They often need:

- indexing
- observation operators
- uncertainty or conformance
- subject identity
- timing context

This is also where OzzyDB-like ideas become very relevant:

- typed artifacts
- typed transforms
- provenance
- conformance

## 6. Models / Claims

Models and claims are where proof by construction happens.

A model might say:

- these relationships hold
- these constraints hold
- these predictions follow from this setup

A claim might say:

- given this evidence and this model, a certain interpretation is supported or falsified

This is where Myco is strongest.

## 7. Representations / Carriers

Representations are how things are encoded or carried operationally.

Examples:

- Arrow
- Parquet
- CSV
- pandas
- Polars
- JAX arrays
- PyTorch tensors

This layer is important, but it should not be confused with scientific meaning.

For example:

- a dataframe is probably not a primitive scientific type
- it is more like a carrier around a row schema and associated semantics

This is one reason composition and generics matter so much.

## Why Generics Matter

The conversation strongly suggested that a flat categorical approach will not be enough.

Compositional, parametric constructors are likely much healthier.

For example:

- `Table<Row>`
- `Collection<T>`
- `Distribution<Params>`
- `Measurement<T, Schedule>`
- `Composition<Parts, Constraint>`
- `View<Concept, Schema>`

This matters because many things that look atomic are really wrappers over more basic structure.

A dataframe is a good example.

It is better thought of as something like:

- a table over a row type
- with a representation/carrier layered on top

rather than as one giant primitive `DataFrame` type.

This same compositionality likely matters for model-side types too:

- `Leaf`
- `Tree`
- `CanopyLayer`
- grouped compositions
- uncertainty-bearing wrappers

## Grouped Constraints

The dietary composition example is a good illustration.

You might have:

- protein
- carbs
- fat

Each one individually bounded, but together constrained to sum to 1.

That suggests that some "types" or views are really structured objects over multiple quantities, not labels on single nodes.

So the type system likely needs to support:

- grouped structure
- grouped constraints
- semantic names over groups

This is one reason to avoid thinking only in terms of scalar node categories.

## A Tree Example

A good example from the conversation is:

- a loblolly pine
- 100 years old
- eastern shore of Virginia
- known GPS coordinates
- certain size/diameter and other subject-level features

That sounds like subject tracking.

But then there is another layer:

- in this study, a tree has NSC composition
- it has canopy water potential
- it belongs to a world with categorical environments
- those are the relevant features in the world where claims are being made

That sounds like a view embedded in a world.

In another study:

- the same broad concept `Tree` might instead be represented through nitrogen variables

So the system should not force one canonical "tree type."

It should help align multiple partial representations of trees.

## A Better Target Than One True Tree Schema

The goal should probably not be:

- discover the one correct schema for `Tree`

It should be closer to:

- represent many partial views of `Tree`
- relate them explicitly
- state where they overlap and where they do not
- make their mappings or morphisms visible

Some mappings may be:

- exact
- partial
- lossy
- approximate
- empirical rather than structural

That is fine, as long as the system can say so.

## Workflow Roles Should Stay Small

Another useful conclusion from the conversation is that user-facing workflow roles should probably stay minimal.

Rather than a large ontology of roles, something like this may be enough:

- `assume`
- `observe`
- `learn`

Then:

- `fixed` can be a mode of `assume`
- `constant`, `series`, `initial value`, etc. can be binding modes
- "latent" may be mostly an internal status rather than a user-facing role

This keeps the world model and workflow layer cleaner.

## World Model Versus Workflow, Again

This note reinforces the point made elsewhere:

- the world model should describe the world
- the workflow config should describe how a run uses that world

The world model should not need to know:

- where the data came from
- whether a quantity is learned in this experiment
- whether a value is assumed versus observed in this run

Those are workflow questions.

This matters because it lets:

- one concept support many views
- one world support many workflows
- one published package be reused in many different scientific contexts

## Relationship To OzzyDB

This discussion also clarifies where OzzyDB may fit later.

OzzyDB is especially relevant for:

- subject-linked evidence
- typed artifacts
- transforms
- provenance
- conformance

Myco is especially relevant for:

- structural models
- executable relations
- constraints
- proof by construction

The shared layer is likely:

- concepts
- views
- grouped constraints
- type relations
- mappings between views
- subject/evidence linkage

That shared layer is probably more important than immediate implementation unification.

## What This Suggests We Should Do

A useful next step would be to test these abstractions against real literature and datasets.

In particular:

- take several different scientific representations of trees
- try to fit them into a concept / subject / view / world / measurement structure
- see where the abstraction breaks down
- refine the type system accordingly

That feels like a much better way to design the system than inventing a perfect ontology from first principles.

## Working Summary

The best current working picture is something like:

- a **concept** is the scientific thing
- a **subject** is a concrete instance of that thing in a study
- a **view** is a typed scientific projection of the concept
- a **world** is the modeled context that gives the view meaning
- **measurements** attach evidence to subjects under views
- **models** and **claims** operate over those views
- **representations** are operational carriers, not the scientific meaning itself

And the type system likely needs to support:

- composition
- parametric/generic structure
- grouped constraints
- semantic alignment across partial views

rather than only flat categories or flat schemas.
