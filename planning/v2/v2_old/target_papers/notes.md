# V2 Target Model And Benchmark Notes

This file is the bridge between:

- the `v2` compiler/language design
- the literature and reference implementations
- the eventual first real-model validation benchmark

Right now it is intentionally a structured stub rather than a finished brief.

## Why This File Exists

`v2` should not be driven only by architectural taste.

It should be driven by:

- one real plant model family
- one real benchmark story
- one concrete validation loop

This note is where those choices should become concrete.

## Candidate Target Families

Current leading candidates:

- hydraulic + stomatal control
- a Sperry-style hydraulic/control model
- water + carbon + allocation
- Farquhar-lite + stomatal coupling

Current default recommendation in the other `v2` notes is still:

- hydraulic + stomatal control unless the literature/code survey clearly
  points elsewhere

## Validation Story To Support

The current strongest validation story is:

1. implement one real plant world in `.myco`
2. generate synthetic trajectories from a known controller across many contexts
3. recover the controller from full data
4. progressively erase observations across those contexts
5. measure when recovery remains possible and when it breaks down

This should be the standard used when deciding whether a candidate target model
is a good `v2` proof.

## What This Benchmark Needs To Specify

Before a full implementation plan is written, this note should be expanded to
answer:

- what the target world actually is
- which quantities it contains
- which rollout-stable parameters it needs
- which temporal states it needs
- which named functions it needs
- whether same-step implicit solves are required
- which observations are available
- which observation operators are needed
- which units/dimensions are required
- what kinds of sparse or heterogeneous bindings the benchmark should exercise

## Literature And Code Pointers

Known placeholders to fill in:

- Sperry code is located at:
- Potkay code is located at:

This section should eventually include:

- paper citations
- repository links
- notes about what each candidate model forces into the language/runtime

## The Main Open Decision

The main question this file needs to settle is:

- what is the first real model family that is both scientifically valuable and
  architecturally honest for `v2`?

Until that is answered concretely, a lot of other `v2` design discussion will
remain underconstrained.
