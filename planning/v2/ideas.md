# Myco V2 Ideas

This note is intentionally about the next real compiler milestone, not the full long-term platform vision.

If you want the bigger-picture material, see:

- [v2_do_this_first.md](./v2_do_this_first.md)
- [charter.md](./charter.md)
- [node_first_ownership_and_relationships.md](./node_first_ownership_and_relationships.md)
- [sparse_multi_context_training_notes.md](./sparse_multi_context_training_notes.md)
- [../v_long_term/README.md](../v_long_term/README.md)

## V2 In One Sentence

`v2` should prove that Myco can support one genuinely plant-relevant model family by adding only the language and runtime features that family truly requires.

## What V2 Is Trying To Prove

The right `v2` proof is not:

- more syntax
- more abstract generality
- a whole scientific platform

It is:

- one real plant workflow
- over the same world/workflow compiler boundary established in `v1`
- with one level up in scientific realism from TinyTree

The likely target is:

- mixed mechanistic structure
- sparse and dense observations
- explicit parameters
- one learned or fitted component where useful
- at least one new capability such as a minimal function registry or a local within-step solve

## The Most Important V2 Design Rule

The most important `v2` principle is still the one in [v2_do_this_first.md](./v2_do_this_first.md):

> the world model should describe structure, while the binding/config layer should describe workflow

This matters more than any individual syntax choice.

Everything else in `v2` should be built on top of that boundary rather than weakening it.

## What V2 Should Probably Be About

These feel like the right near-term directions.

### 1. One Real Plant Model Family

The first real target should pull the language forward.

Good candidates:

- hydraulic + stomatal control
- water + carbon + allocation
- Farquhar-lite + stomatal coupling

The default recommendation is still:

- start with hydraulic + stomatal control unless the literature survey proves a different family is the better first proof

Why:

- closest to TinyTree
- likely the smallest meaningful expansion
- likely valuable quickly for real data

### 2. Explicit Parameters

`v2` likely wants a clearer notion of rollout-stable scientific quantities.

This does not need to become a huge ontology.

The key idea is that some quantities are:

- not external forcing
- not temporal updates
- not merely derived at each step

and yet may still be:

- fixed
- learned
- assumed

for a given workflow.

Examples:

- conductivity terms
- vulnerability parameters
- photosynthesis coefficients
- respiration coefficients

### 3. Minimal Function Registry

The first real model will probably need at least a tiny function registry.

This should stay narrow.

The goal is not:

- a giant built-in scientific library

The goal is:

- a small standard library for common operations
- a clean hook for named scientific functions needed by the target model family

This should support things like:

- vulnerability curves
- temperature-response functions
- observation operators

without forcing every user to decompose them into primitive arithmetic by hand.

### 4. Observation Operators

The first real model will almost certainly need at least minimal observation operators.

Examples:

- sap flux vs transpiration
- water potential sampling
- simple aggregations or protocol-specific projections

These should stay tightly scoped to the chosen model family.

### 5. Data And Observation Indexing

The first real model will also likely need a clearer indexing story than TinyTree.

The useful near-term target is probably:

- explicit timestep index arrays
- dense-per-step support where appropriate
- sparse observation schedules that are easy to bind from ordinary data structures

No need to solve every future asynchronous or interpolation problem in `v2`.

### 6. Local Solve Blocks, If The Chosen Model Truly Needs Them

If the chosen model family requires same-step coupling that cannot be represented cleanly as an acyclic step plan, then `v2` will likely need a narrow local-solve feature.

But this should be earned by the first real model, not introduced speculatively.

That means:

- if the model can be expressed cleanly without a local solve, defer it
- if the model genuinely requires it, add the smallest useful version

## Candidate First Model Family

Current recommendation:

- **Option A: hydraulic + stomatal control**

Why:

- closest to the working TinyTree proof
- likely to require only a small set of new features
- likely to connect quickly to real data and real scientific questions

The more ambitious alternatives are still interesting:

- water + carbon + allocation
- Farquhar-lite + stomatal coupling

But they should be chosen because the literature survey demands them, not because they are architecturally exciting.

## Backend Direction For V2

`v2` should keep the current stance:

- backend-agnostic core
- JAX-first product quality

That means:

- core planning and type/constraint work should not become JAX-specific
- JAX can remain the first backend to support the chosen real model well
- other backends are explicitly not the `v2` proof target

## What Probably Does Not Belong In V2

These are good ideas, but not the next compiler milestone:

- semantic schemas/components as a production feature
- full package/registry ecosystem
- shared Myco/OzzyDB type-system convergence
- uncertainty-rich modeling as a first-class core feature
- broad demand-driven planning refactors unless the first model truly forces them
- broad continuous-time infrastructure
- a large CAS or theorem-proving system

Those are north-star topics, not immediate obligations.

## What Success Looks Like

A good `v2` would mean:

- one real plant-relevant model family implemented cleanly
- the world/workflow split strengthened rather than weakened
- one or two carefully chosen compiler/runtime additions added because the science required them
- a credible path from TinyTree to a paper-relevant workflow

## Short Version

`v2` should stay narrow. It should prove one real plant-model family over the same workflow-neutral structural compiler architecture established in `v1`, most likely by adding explicit parameter semantics, a minimal function registry or local solve mechanism if required, richer observation/indexing support, and just enough runtime ergonomics to support real scientific data.
