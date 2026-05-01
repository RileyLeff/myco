# Sparse Multi-Context Training Notes

This note captures one of the strongest `v2` use cases discussed after the
`v2_do_this_first` boundary cleanup:

- one structural world
- many experiment bindings
- sparse and heterogeneous observations
- a shared learned controller
- progressive recovery under partial observability

It is intentionally about the next scientific/compiler proof, not the longer
Myco/OzzyDB platform vision.

## Why This Matters

If Myco is valuable only when the user has one clean, dense, fully observed
dataset, then it is not solving the real problem.

The more important case is:

- the world model is shared
- data availability differs across runs
- some quantities are observed densely
- some are observed sparsely
- some are not observed at all
- the user still wants to train one controller against all of those contexts

That is much closer to the motivating ecophysiology workflow.

## Proposed V2 Proof

The clean test is:

1. implement a Sperry-style or similarly plant-relevant world in `.myco`
2. generate synthetic trajectories from a known controller over many contexts
3. recover the controller from perfect or nearly complete data
4. progressively erase observations across those contexts
5. measure when controller recovery remains possible and when it breaks down

This is stronger than another single-run TinyTree-style demo because it tests:

- one world, many experiment bindings
- partial observability
- shared-controller learning
- identifiability under diverse data contexts

## The Intended Learning Pattern

The right mental model is:

- shared controller parameters across runs
- structured per-run latent variables where needed
- hidden trajectories induced by rollout where possible
- observations only on the quantities actually measured

The key distinction is between:

- **partial observability**
- **free latent trajectory inference**

Those are not the same thing.

### Preferred Case

The preferred case is:

- some nodes are unobserved
- but the rollout still determines them once assumptions, initial values, and
  learned controller outputs are fixed

Then training is straightforward:

- compile the world for each workflow binding
- run rollout
- compute losses only on observed quantities
- backpropagate through the whole system

In that setup, unobserved nodes are not free optimization variables. They are
implied by the model.

### Harder Case

If the system is genuinely underdetermined, then training becomes joint
inference over:

- shared controller parameters
- plus structured per-run latent variables

The structured latent variables should be things like:

- learned initial states
- rollout-stable parameters
- low-dimensional latent context

Avoid treating every hidden node at every timestep as an independent free
variable unless absolutely necessary. That makes the problem too unconstrained
and much harder to interpret scientifically.

## What Actually Creates Identifiability

Type information and constraints help, but they are not enough by themselves.

The real identifying signal comes from:

- dynamical structure
- coupling between quantities
- alternative mechanistic paths
- sparse observations of downstream effects
- variation across many runs and environments
- shared controller structure across all runs
- learned initial states or fixed parameters where needed

So the expected story is:

- constraints narrow the feasible set
- structure ties the hidden quantities together
- diversity across runs identifies the shared controller

## What Myco Can Already Support

The current architecture already supports the basic pattern in principle:

- one `.myco` world
- many separate experiment bindings
- sparse observations
- learned slots
- learned initial states
- ordinary JAX loss composition outside the compiler

That means the user can already write a joint objective in Python/JAX that sums
losses across multiple compiled experiments sharing one controller parameter set.

Conceptually:

- compile one artifact per experiment binding
- share one controller parameterization across them
- sum the per-experiment losses
- optimize one joint objective

This does not require one monolithic compiled artifact.

## What V2 Should Probably Add

### 1. A Study-Level Training Pattern

Not necessarily a big new Rust-core abstraction.

But `v2` likely wants a standard pattern or helper layer for:

- one world
- many experiments
- one shared learned controller
- optional per-run latent initial states or parameters
- one combined objective

That can start in Python/JAX.

### 2. Better Data Binding Adapters

It should become easy to bind observations and assumptions from:

- `pandas`
- `polars`
- `xarray`

by quantity name and explicit index or schedule information.

This should remain a workflow-side adapter layer. The compiler should still
receive validated assumptions and observations aligned to the rollout horizon.

### 3. Stronger Practical Type/Unit Validation

The next type/unit step should help validate:

- binding compatibility
- observation compatibility
- index/schedule alignment
- array shape expectations

This should remain practical and narrow rather than becoming a giant ontology.

## Performance Direction

For training-heavy workflows, the first optimization target should be the JAX
backend, not a Rust-native training backend.

The main likely wins are:

- make emitted JAX artifacts more static
- reduce dict and string-key overhead in the hot path
- move toward fixed field ordering and pytree-friendly payloads
- avoid retaining full histories unless requested
- separate boundary validation from rollout-time execution

This suggests a future split between:

- **debug mode**: more checks and richer diagnostics
- **fast/train mode**: minimal repeated checks inside the hot loop

Generating Rust may become interesting later for simulation or deployment, but
it should not be the first answer to training performance.

## Constraint/Prover Motivation

This training story is also one reason a stronger symbolic/constraint layer may
matter later.

If the compiler can prove more about:

- branch validity
- bound satisfaction
- shape compatibility
- safe omission of runtime checks

then the training runtime can get simpler and faster.

That is a good long-term motivation for better constraint reasoning, but it is
not the first `v2` deliverable.

## What Not To Do Yet

To keep this proof clean, avoid turning it into:

- generic latent trajectory inference everywhere
- a Rust-native training backend
- a giant new syntax expansion
- a broad ontology of scientific roles

The goal is narrower:

- one world
- many sparse contexts
- one shared learned controller
- a credible recovery story under progressive data erasure

## Short Version

One of the best `v2` proofs is:

- implement one real plant world
- train one controller across many sparse and heterogeneous experiment
  bindings
- show that recovery survives progressive data erasure because the world model,
  dynamics, constraints, and cross-context variation provide enough structure

That is exactly the sort of workflow where Myco should beat repeated
hand-written model/training implementations.
