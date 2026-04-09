# Myco End-to-End Walkthrough

This document is a user-facing walkthrough of the current Myco `v1` experience.

It does three things:

1. shows what Myco looks like from the outside
2. traces what happens internally when you compile and run a model
3. comes back out to what the user receives as results

The walkthrough uses the current TinyTree demo as the concrete example:

- model file: [tiny_tree.myco](/Users/rileyleff/Documents/dev/myco/crates/myco-core/tests/fixtures/tiny_tree.myco)
- demo entrypoint: [train_tiny_tree_controller.py](/Users/rileyleff/Documents/dev/myco/examples/train_tiny_tree_controller.py)
- demo implementation: [demos.py](/Users/rileyleff/Documents/dev/myco/python/myco/demos.py)

## What Myco Feels Like As A User

From the user’s point of view, Myco has three layers:

1. a `.myco` file that describes the world
2. a Python-side experiment binding that says what is data, what is fixed, what is learned, and what is observed
3. a compiled artifact that behaves like ordinary Python or JAX code

The important idea is that the `.myco` file is not a training script and not a simulator script. It is the shared structural model.

The binding layer decides how to use that model in a particular workflow.

## The TinyTree Model

The TinyTree model is intentionally small, but it already exercises the core ideas:

- external forcing
- internal state
- a learned slot
- overdetermination
- temporal update
- constraints

The actual file is:

```myco
model TinyTree

external vpd_scale : potential
external soil_water : potential
external hydraulic_cond : conductance

state water : potential { self <= 0 }
state carbon : carbon_mass { self >= 0 }

node stomata : conductance {
  self >= 0
  self <= g_max
}

node transpiration : water_flux { self >= 0 }
node g_max : conductance { self >= 0 }

relation demand_transpiration:
  transpiration = stomata * vpd_scale

relation supply_transpiration:
  transpiration = hydraulic_cond * (soil_water - water)

slot controller provides [stomata]:
  inputs = [water, carbon, vpd_scale, soil_water, hydraulic_cond, g_max]

temporal water_step:
  water[t+1] = water[t] - dt * transpiration[t]
```

Even in this toy model, one quantity is intentionally overdetermined:

- `transpiration` can be computed from stomata demand
- `transpiration` can also be computed from hydraulic supply

That is one of the central reasons Myco exists.

## The User Workflow

The current user experience is best seen through the Python package:

```python
import myco

model = myco.load("crates/myco-core/tests/fixtures/tiny_tree.myco")
experiment = model.experiment(mode="train", horizon_steps=64)

experiment.bind_data_series("vpd_scale", range(64))
experiment.bind_data_series("soil_water", range(64))
experiment.bind_constant("hydraulic_cond")
experiment.bind_constant("g_max")
experiment.bind_initial_state("water")
experiment.bind_initial_state("carbon")
experiment.bind_slot("controller", kind="learned")

experiment.observe_dense("transpiration")
experiment.observe_sparse("water", range(0, 64, 8))

artifact = experiment.compile(backend="jax")
module = artifact.to_module("tiny_tree_training_artifact")
```

The user is doing four conceptual things here:

1. load the structural model
2. declare the workflow
3. compile it to an executable backend artifact
4. use that artifact directly in ordinary Python/JAX code

That is the intended Myco ergonomics: compile-time binding, then ordinary runtime execution.

## What The User Sees In The Artifact

The compiled JAX artifact is a real Python module. It includes:

- metadata constants
- input validation helpers
- `init_params()`
- `resolve_initial_state(...)`
- `step(...)`
- `rollout(...)`
- `obs_loss(...)`
- `consistency_loss(...)`
- `constraint_violation_loss(...)`
- `soft_penalty_loss(...)`
- `loss_components(...)`
- `total_loss(...)`

The artifact is not a hidden runtime process. It is generated source code.

That means the user can:

- inspect it
- write it to disk
- import it
- call it directly
- plug it into JAX/Optax however they want

## The Demo Workflow

The TinyTree training demo does two passes through the same model.

### Pass 1: Generate Synthetic Data

The demo first uses a known controller to create synthetic trajectories:

- forcing series are generated for `vpd_scale` and `soil_water`
- constants are provided for `hydraulic_cond` and `g_max`
- initial state is provided for `water` and `carbon`
- a known controller produces `stomata`
- the compiled artifact rolls the system forward

That produces:

- dense transpiration history
- sparse water observations via masking

### Pass 2: Recover The Controller

The demo then recompiles the same structural model with a learned slot:

- the same `.myco` model is used
- the `controller` slot is now a learned provider
- the observations are bound as losses
- a JAX training loop optimizes the learned controller

The objective currently includes:

- observation loss
- a small consistency loss term
- soft penalties if present

The demo then evaluates the learned controller on held-out forcing.

## What Happens Internally

Here is the current internal pipeline.

### 1. Syntax Parsing

File:

- [syntax.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/syntax.rs)

The parser reads the `.myco` source into a structured syntax model:

- quantities
- relations
- slots
- temporal blocks
- source spans

At this stage Myco is still close to the text format. It knows names, declarations, and block structure, but not yet symbolic meaning.

### 2. Semantic Lowering

File:

- [semantic.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/semantic.rs)

The semantic layer parses equation strings into expression trees:

- symbols
- numbers
- binary arithmetic

This is where Myco turns:

```text
transpiration = stomata * vpd_scale
```

into an internal expression structure that can be reasoned about.

At this stage:

- precedence is resolved
- scientific notation is supported
- malformed expressions are diagnosed

### 3. Equality Lowering

File:

- [equality.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/equality.rs)

The equality layer assigns stable internal identities:

- `QuantityId`
- `EquationId`

It also attaches:

- dimensions and quantity type information
- parsed constraints
- provenance and source spans

This is where the symbolic model becomes a graph of named quantities and equations rather than just parsed syntax.

### 4. E-Graph Construction

File:

- [egraph.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/egraph.rs)

Myco then builds an `egg`-backed equality core.

This is the equality substrate of the compiler, not the entire compiler.

The e-graph stores equivalences among expressions and seeds directional registrations for:

- forward equations
- inverted arithmetic forms where supported

For TinyTree, this means Myco can reason about both:

- `transpiration = stomata * vpd_scale`
- `stomata = transpiration / vpd_scale`

without the user writing two separate model implementations.

One important detail in the current implementation:

- candidate extraction is now local to each candidate seed expression
- it no longer resolves through the full shared output e-class

That preserves provenance and prevents one relation from silently “stealing” another relation’s expression.

### 5. Compile Binding

File:

- [compile.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/compile.rs)

This is where Myco answers:

- which quantities are dense forcing?
- which are constants?
- which are initial state?
- which slots are learned?
- which quantities are observed?

For TinyTree training, the binding says:

- `vpd_scale` and `soil_water` are dense per-step forcing
- `hydraulic_cond` and `g_max` are constants
- `water` and `carbon` are initial state
- `controller` is learned
- `transpiration` is densely observed
- `water` is sparsely observed

This step also performs structural validation such as:

- every state needs an initial-state binding
- direct data-series bindings must cover the full horizon in `v1`
- compile mode requirements must be satisfied

### 6. Single-Step Planning

File:

- [plan.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/plan.rs)

Planning happens on a single step, not on an unrolled full horizon.

This is one of the most important implementation decisions in Myco `v1`.

The planner:

- starts from what is directly available at the current step
- asks which slots and equations are buildable
- uses the equality core to extract candidate-local expressions
- chooses a canonical path
- records alternatives
- rejects same-step algebraic cycles
- separately handles temporal equations for `t -> t+1`

For TinyTree, the planner decides things like:

- `stomata` comes from the `controller` slot
- canonical `transpiration` comes from `demand_transpiration`
- `supply_transpiration` is an alternative path
- `water[t+1]` comes from the temporal update

If the binding were different, the plan would change.

For example, if `transpiration` were directly provided and `stomata` were not, Myco could invert the demand relation and recover `stomata`.

That is the acausal-to-causal part of the system.

### 7. Emission

File:

- [emit.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/emit.rs)

Once a plan exists, Myco emits backend code.

For the JAX backend, that code includes:

- a JAX-friendly `step(...)`
- `rollout(...)` using `lax.scan`
- masked observation losses
- consistency loss from alternative paths
- constraint violation accounting
- smooth learned-surface projections
- runtime input validation

The emitted code is shaped by compile mode and policy.

For example:

- `simulate` omits loss helpers
- `train` includes them
- learned surfaces are projected
- Python artifacts raise on derived-output bound violations
- JAX artifacts expose `constraint_violation_loss`

### 8. Python Bridge

Files:

- [crates/myco-py/src/lib.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-py/src/lib.rs)
- [python/myco/api.py](/Users/rileyleff/Documents/dev/myco/python/myco/api.py)
- [python/myco/types.py](/Users/rileyleff/Documents/dev/myco/python/myco/types.py)

The Python package is intentionally thin.

Rust owns:

- parse
- lower
- equality reasoning
- planning
- emission

Python owns:

- user ergonomics
- experiment construction
- packaging
- actual runtime orchestration

This keeps Myco as a compiler, not a hidden runtime system.

## What The User Gets Back

After compilation, the user gets an `Artifact` object with:

- backend
- generated source
- suggested filename
- typed metadata

The metadata includes things like:

- compile mode
- consistency policy
- constraint runtime policy
- learned slot names
- learned initial-state names
- slot interfaces

So the user can inspect not just the generated source, but the contract of the compiled artifact.

## What Happens During Training

In the TinyTree demo, the JAX training loop uses the compiled artifact directly.

Conceptually:

1. make a learned controller parameterization
2. wrap it as a `slot_providers["controller"]` callable
3. call `rollout(...)`
4. compute `loss_components(...)`
5. differentiate through the whole system with `jax.value_and_grad`
6. update parameters with Optax

The compiler is not running the optimizer. It emitted the functions that the optimizer uses.

That separation is important:

- Myco compiles model structure
- the user owns the runtime loop

## What Results Look Like

The current TinyTree demo prints:

```text
initial loss: 0.023409
final loss: 0.012462
initial obs loss: 0.006026
final obs loss: 0.000565
holdout transpiration MSE: 0.000373
holdout water MSE: 0.000234
```

Those numbers matter because they show that the whole abstraction is real:

- the same `.myco` model supports generation and recovery
- the emitted artifact is differentiable enough to train
- sparse and dense observations can coexist
- alternative mechanistic structure can contribute regularization

## The Mental Model To Keep

If you want the shortest accurate mental model of Myco right now, it is this:

1. write the world once in `.myco`
2. decide how to bind that world for a particular workflow
3. let Myco compile the workflow-specific causal plan
4. receive ordinary backend code
5. run that code with normal Python/JAX tooling

In other words:

- Myco is not a simulator format
- Myco is not an optimizer framework
- Myco is not a hidden runtime
- Myco is a compiler for acausal scientific models with explicit binding

## Where To Look Next

If you want to trace the current implementation in code, this is the best order:

1. [tiny_tree.myco](/Users/rileyleff/Documents/dev/myco/crates/myco-core/tests/fixtures/tiny_tree.myco)
2. [demos.py](/Users/rileyleff/Documents/dev/myco/python/myco/demos.py)
3. [api.py](/Users/rileyleff/Documents/dev/myco/python/myco/api.py)
4. [pipeline.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/pipeline.rs)
5. [compile.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/compile.rs)
6. [plan.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/plan.rs)
7. [emit.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/emit.rs)

That path follows almost exactly the same journey the model takes through the system.
