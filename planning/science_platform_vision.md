# Science Platform Vision

This note is broader than Myco `v2`.

It is not an implementation plan.

It is a statement of the larger direction that Myco may be part of over time, especially in relation to OzzyDB and the broader problem of scientific knowledge representation.

## The Problem

A lot of science is still communicated in a very low-bandwidth way.

The common workflow looks something like:

1. collect difficult, expensive, often irreplaceable data
2. run a small number of conventional analyses
3. make a set of interpretive claims in prose
4. publish a PDF that is hard for humans to access and even harder for machines to reason about
5. maybe upload a pile of CSVs somewhere with inconsistent metadata

This creates several problems:

- the reasoning chain from evidence to claim is often weak or implicit
- data and models are hard to reuse directly
- formal structure is mostly lost in prose
- reproducibility is poor
- scientific communication remains low bandwidth

Even many "open science" efforts are still mostly buckets of files with weak machine-usable meaning.

## Why This Might Be Changing Now

Historically, scientists have been very unwilling to adopt highly formal systems for open-ended research.

That is understandable.

The traditional reasons are:

- the learning cost is too high
- the payoff is unclear unless a large community buys in
- formalization feels like extra work on top of an already difficult workflow

In other words, a more formal scientific system used to require people to change how they think and work before they received any benefit.

That is a hard sell.

Agents may change that.

With good agents, a scientist may be able to:

- work in natural language
- work from familiar data structures and files
- ask the agent to formalize ideas incrementally
- inspect and correct the formalization interactively
- benefit from formal structure without hand-authoring everything from scratch

If that works, then formalization stops being a heavy prerequisite and starts becoming an interactive assistive layer.

That changes the adoption equation dramatically.

## A Useful Framing

One framing that seems especially useful is:

- proof by construction
- proof by observation

In this picture:

- **proof by construction** means executable structural models, typed assumptions, constraints, and derivations
- **proof by observation** means evidence, artifacts, transforms, datasets, and empirical support

Myco is closer to the first side.

OzzyDB is closer to the second side.

The long-term opportunity may be to connect them through a shared type-and-constraint system.

## Myco's Role

Myco can provide:

- structural scientific models
- explicit assumptions
- executable relationships
- workflow-specific compilation
- generated artifacts for simulation, fitting, control learning, and inference

In that sense, Myco is a language for making scientific construction more explicit and machine-usable.

## OzzyDB's Role

OzzyDB can provide:

- typed artifacts
- typed transforms
- reproducible fetch and version pinning
- provenance
- explicit conformance and evidence chains

In that sense, OzzyDB is a system for making scientific observation and computation more explicit and auditable.

## The Combined Direction

The deeper opportunity is not just "a model language" or "a data registry."

It is something more like:

- a typed graph of scientific objects
- where data, transforms, models, constraints, claims, and evidence can all be represented explicitly
- where claims can be backed by both executable construction and observational support
- where assumptions are inspectable
- where reproducibility is native rather than bolted on

That could eventually look more like:

- a GitHub for scientific objects
- a forum for discussing and refining those objects
- a package registry for reusable model and evidence components
- an agent-mediated interface for formalizing work incrementally

rather than:

- a traditional publication system
- or a pile of disconnected CSV files and PDFs

## Why A Type System Matters So Much

The central enabling idea may be a strong type-and-constraint system for science.

Not just data column types.

Something richer that can carry:

- dimensions and units
- structural constraints
- grouped constraints
- observation contracts
- model assumptions
- evidence relationships
- provenance-bearing claims

If that type system is good enough, then many important things become possible:

- better scientific reuse
- machine-checkable compatibility between ideas and evidence
- safer composition of models and datasets
- more formal reasoning about what follows from what
- richer agent assistance

This is likely a higher-bandwidth interface for scientific meaning than plain publication text.

## Why Agents Matter

Without agents, a system like this risks becoming too hard to use.

With agents, the experience could be very different:

- a scientist works mostly in normal language and familiar tools
- the agent translates ideas into typed structures
- the scientist corrects the translation iteratively
- formal objects accumulate gradually
- the payoff appears early, not only after total formalization

That means the system does not require a scientist to become a theorem prover engineer before getting value.

The agent acts as an adapter between:

- informal scientific thinking
- formal typed scientific structure

This may be the biggest reason the idea is timely now when it would have been much harder to adopt before.

## What This Should Not Become Too Quickly

There are obvious failure modes:

- an overengineered proof system nobody uses
- a giant ontology project
- a rigid metadata bureaucracy
- a system that requires all participants to buy in before any one participant sees benefit

The healthier path is probably:

- simple core primitives
- strong reproducibility
- incremental formalization
- typed sharing
- agent-assisted onboarding
- bring-your-own tools wherever possible

In other words:

- formal where it pays off
- flexible where open-ended research still needs room

## Why This Could Be Better Than The Current Publication System

The current publication system has several pathologies:

- high effort for low downstream reuse
- claims weakly linked to evidence
- poor machine readability
- poor reproducibility
- strong incentives toward narrative polish over formal clarity

A typed, agent-mediated platform could improve on that by making it easier to:

- upload and version raw and derived artifacts
- publish executable models
- attach claims to evidence and assumptions
- reuse prior work directly instead of paraphrasing it
- discuss interpretations around formal objects instead of only around prose

The point is not to eliminate informal discussion.

It is to make formal structure available when it matters, and easier to build than it used to be.

## A Plausible Long-Term Shape

One plausible long-term shape is:

- Myco handles executable structural modeling
- OzzyDB handles typed artifacts, transforms, and evidence provenance
- a shared or aligned type system connects the two
- package registries allow reusable scientific objects to be published and imported
- agents help convert informal work into formal structure incrementally

In that world, science becomes more like:

- structured, versioned, inspectable objects
- with human discussion layered around them

and less like:

- isolated PDF claims with detached supplementary files

## Short Version

The larger opportunity is not only a better modeling language. It is a better substrate for scientific knowledge exchange: proof by construction through executable models, proof by observation through typed evidence and provenance, and a shared type-and-constraint system that lets agents help scientists formalize their work without imposing the old cost of adopting a rigid formal system upfront.
