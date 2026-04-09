# Myco Spec v1

Status: Draft for external review

Audience: project authors and technical reviewers

## Introduction

Myco is a model-description and compilation framework for partially known dynamical systems. A user should be able to describe a scientific system once, keep that description close to the relationships it is meant to represent, and then compile it into different executable programs for simulation, fitting, or training.

The core design bet is that a scientific model should not have to be rewritten every time the user wants a different causal direction, a different observation bundle, or a different mix of mechanistic and learned components. The `.myco` file describes the world and the relationships within it. A separate binding and compile layer describes what is observed, what is supplied as data, what is fixed, what is deferred, and what backend artifact should be emitted.

Myco is not intended to be a runtime manager or a replacement for JAX, PyTorch, or differential equation libraries. It should parse, validate, reason about the model, and emit ordinary executable artifacts that users can run directly in their own environment. The long-term goal is a general framework for acausal scientific modeling with constraints, multiple value sources, and reusable compilation paths. The short-term goal is much narrower: build a small but real end-to-end vertical slice that proves the abstraction is sound.

This document is meant to do two things at once:

- Make the long-term design intent explicit so short-term decisions do not paint the project into a corner.
- Set strict near-term scope so `v1` remains buildable and testable within a short sprint.

## Section Overview

### 1. Problem Statement and Design Intent

Defines the problem Myco is trying to solve, the user pain it addresses, and the architectural principles that distinguish it from a normal code generator, a pure symbolic algebra tool, or a pure ML framework.

### 2. Project Scope, Success Criteria, and Non-Goals

States what counts as success for `v1`, what is intentionally out of scope, and what should not be attempted in the first implementation.

### 3. User Model and Primary Workflows

Describes the expected end-user experience: who writes `.myco` files, who binds data from Python, who compiles models, and how Myco fits into a normal scientific workflow.

### 4. Surface Language Overview

Defines the intended shape of the `.myco` language at a high level: how models declare quantities, relations, constraints, temporal structure, slots, and reusable structure.

### 5. Python API and Compile Specification

Describes the primary user-facing Python interface, including loading models, checking them, binding providers, attaching observations, and compiling to a backend.

### 6. Core Semantic Model and E-Graph Representation

Describes the semantic building blocks of Myco and the representation they live in. Explains why Myco uses an e-graph-backed symbolic core, what equivalence means in the system, and how native equivalence support shapes the rest of the compiler.

### 7. Directional Extraction, Invertibility, and Path Cost

Describes how Myco moves from acausal relationships to a directional execution plan for a specific compile configuration. Covers directional implementations, branch ambiguity, exact versus approximate inversion, and cost-weighted extraction.

### 8. Overdetermination, Underdetermination, and Consistency

Defines how the compiler detects multiple paths to the same quantity, missing paths, and ambiguous paths. This section builds on the extraction strategy from Section 7.

### 9. Type-Level Constraints, Dimensions, and Units

Describes the intended treatment of dimensions, units, bounds, and related constraints.

### 10. Providers, Slots, and Value Sources

Defines how values enter the model: data, constants, mechanistic rules, learned functions, latent states, or inferred quantities.

### 11. Temporal Semantics

Describes how Myco represents time in the model and how temporal structure is lowered during compilation.

### 12. Observations, Missingness, and Loss Wiring

Defines how observed data constrain the model, how missing values are represented, and how observation operators connect measurements to internal state.

### 13. Compilation Pipeline, Rust/Python Boundary, and Backend Emission

Specifies the pipeline from parse to emitted artifact and defines the physical architecture of the implementation.

### 14. Diagnostics, Introspection, and Explainability

Defines the minimum debugging and explanation surface for `v1`.

### 15. MVP v1 Scope

Defines the exact features that must exist in the first implementation, the exact demo target, and the minimum proof that the architecture works.

### 16. Deferred Features and Open Questions

Explicitly lists what is deferred beyond `v1`, why it is deferred, and what questions reviewers should pressure-test.

### 17. Milestones and Review Questions

Translates the spec into a near-term build plan and lists the highest-value review questions.

## 1. Problem Statement and Design Intent

### 1.1 Problem Statement

Scientific models of partially known systems are frequently rewritten for adjacent tasks that should have shared the same underlying representation.

Common failure modes include:

- A mechanistic model is implemented once for forward simulation, then reimplemented for inverse fitting, then reimplemented again for a learned augmentation.
- Acausal scientific relationships are forced into one causal ordering too early, making later reuse awkward or error-prone.
- Model structure, data wiring, and ML runtime concerns get entangled in a single codebase, making the resulting system difficult to inspect, validate, or repurpose.
- Sparse and heterogeneous observations are added ad hoc rather than through an explicit observation layer.
- Constraints, physical bounds, and sign conventions are reimplemented ad hoc in each new codebase rather than declared once and enforced centrally.
- Mechanistic and learned components are treated as different kinds of systems rather than different value sources within one system.

The motivating claim for Myco is that these are not separate problems. They are all symptoms of the same underlying issue: the user lacks a single representation of the world that survives across modeling tasks.

### 1.2 Design Intent

Myco is intended to provide that representation.

The design intent is:

- Describe the world once in a form that remains close to the scientific relationships being modeled.
- Defer value-source decisions until compile time rather than hard-coding them into the model description.
- Treat compilation as the place where causal direction, provider binding, and backend emission are chosen.
- Emit ordinary backend code that users can run, inspect, and integrate into their own workflow.

In concrete terms, the `.myco` file should say what relationships exist. It should not say where the data lives, which optimizer to use, or how a downstream training loop should be orchestrated. Those are compile or execution concerns, not world-description concerns.

### 1.3 Architectural Principles

The following principles shape the design.

#### World Description And Compile Binding Are Separate

The model definition describes quantities, relationships, constraints, and temporal structure. The compile layer describes what is bound to data, what is fixed, what is deferred, what is observed, and what artifact should be emitted.

#### The Surface Language Is Acausal

Users should describe relationships, not hand-author execution order. The compiler is responsible for extracting a directional plan for a specific configuration.

#### Value Sources Are Pluggable

The same quantity may be supplied by data in one configuration, computed by a mechanistic rule in another, or populated by a learned function in a third. The model should not have to change when this changes.

#### Equivalence Support Is Foundational

Myco is not a one-pass graph builder that later learns about equivalence. Native support for equality and alternative paths is foundational to the architecture and informs the symbolic core from day one.

#### The Compiler Emits Artifacts And Gets Out Of The Way

Myco should not manage a hidden JAX or PyTorch runtime. It should emit ordinary backend code or callable artifacts that users can run directly.

#### Diagnostics Are Part Of The Architecture

Rich diagnostics are not polish. Source-aware, structured errors and minimal introspection shape the data structures required in the Rust core and must be considered early.

#### `v1` Favors A Narrow Vertical Slice Over Feature Breadth

The first implementation should prove the architecture on a deliberately small example. It should not attempt to deliver the full long-term language or backend vision in one shot.

## 2. Project Scope, Success Criteria, and Non-Goals

### 2.1 Long-Term Goals

The long-term project goal is a general framework for acausal scientific modeling in which:

- a model is written once and reused across multiple causal directions
- mechanistic, observed, inferred, and learned quantities coexist in one representation
- sparse observations can constrain internal state through explicit observation operators
- reusable bundles and libraries can encode domain structure without copying imperative code
- different backend targets can be emitted from the same underlying model

This is the architectural direction. It is not the `v1` deliverable.

### 2.2 `v1` Goal

The `v1` goal is to prove that the architecture works on a small but real vertical slice.

`v1` should demonstrate:

- a `.myco` file can be parsed into a native symbolic representation
- the symbolic representation supports equivalence as a first-class concern
- a Python user can bind data, slots, and observations explicitly
- the compiler can extract a directional plan for a concrete configuration
- the compiler can emit ordinary Python/JAX code
- the emitted code can be used to recover either an identifiable parameterization or a behaviorally equivalent controller from synthetic data with sparse observations

If `v1` proves those points cleanly, the architecture is worth extending. If it cannot, the project should be narrowed or revised before additional features are added.

### 2.3 `v1` Success Criteria

`v1` is successful if all of the following are true:

- A user can write one small `.myco` model that is reused across at least two adjacent workflows.
- The Rust core can parse, validate, and compile that model without requiring hand-authored backend code.
- The Python API can bind explicit data and observations rather than relying on hidden file conventions.
- The emitted backend artifact is ordinary code that the user can run directly.
- The demo is strong enough to expose real compiler behavior, not just parsing behavior.
- Diagnostics are good enough that the project authors can debug the first real example without treating the compiler as a black box.

### 2.4 `v1` Non-Goals

The following are explicitly out of scope for the first implementation:

- a full scientific modeling language
- a rich module or package ecosystem
- multiple backend targets
- a complete algebra system
- general-purpose symbolic simplification or computer algebra
- a broad rewrite library
- generalized numerical root-finding throughout the system
- advanced numerical conditioning analysis
- a managed ML runtime
- a polished visualization or IDE experience

These may become important later. They are not required to prove the core architecture.

### 2.5 Current `v1` Deliverable

The current intended `v1` deliverable is:

- a Rust core that parses `.myco`, builds an e-graph-backed symbolic representation, performs basic validation, and emits Python/JAX code
- a Python interface that binds providers and observations explicitly
- a minimal example that exercises constraints, temporal structure, observed versus internal quantities, and a learned component
- a training demo that recovers either an identifiable parameterization or a behaviorally equivalent controller from synthetic data with sparse observations

This is deliberately a small target. The point is not to prove everything. The point is to prove the right thing.

## 3. User Model and Primary Workflows

### 3.1 Intended Users

The initial user is the project author. The design should still assume eventual use by other technical users who are comfortable with model structure and Python, but not necessarily interested in editing the compiler internals.

The primary user roles are:

- the model author, who writes `.myco` descriptions of systems
- the experiment author, who binds concrete data, provider choices, and observations
- the execution author, who runs emitted backend artifacts in an existing scientific or ML workflow

In `v1`, these may all be the same person. The architecture should still keep the roles conceptually separate.

### 3.2 Primary Workflow

The intended base workflow is:

1. Write a `.myco` file describing the system.
2. Load and validate that model from Python.
3. Bind concrete data sources, value-source choices, and observations.
4. Build an explicit compile specification.
5. Compile to an ordinary backend artifact.
6. Run that artifact in user-owned code.

This workflow is meant to make the compiler boundary obvious. Myco owns model description, reasoning, and emission. The user owns experiment wiring and downstream execution.

### 3.3 Core Workflows To Support In `v1`

`v1` only needs to support a small set of workflows, but it needs to support them cleanly.

#### Forward Simulation

The user fixes all required providers and compiles a program that predicts a trajectory from known initial state and forcing data.

#### Partial-Observation Fitting

The user binds forcing data and sparse observations, fixes some quantities, and asks the compiler to emit an artifact that can evaluate loss terms against those observations.

#### Learned-Slot Training

The user leaves one or more slots deferred to learned implementations, binds observations, and compiles backend code that can be optimized in an external training loop.

These workflows are adjacent enough that they should share one `.myco` model. If they do not, the architecture is failing its main purpose.

### 3.4 Future Workflow That Should Fall Out Naturally

Counterfactual simulation is a likely important use case, but does not require special language machinery in `v1`. The same architecture should allow the user to bind different forcing data, ablate a slot implementation, or replace a mechanistic component and recompile without rewriting the world description.

### 3.5 What The User Should Not Have To Do

Users should not have to:

- manually author execution order for every new workflow
- reimplement the same scientific relation in multiple directions
- rewrite the model description when swapping a data-bound quantity for a learned one
- hide experimental logic in unnamed conventions
- depend on Myco to run a private training or solver process

## 4. Surface Language Overview

### 4.1 Purpose Of The `.myco` Language

The `.myco` language exists to describe the world. It should be declarative, explicit, and close to the scientific relationships being modeled.

The `.myco` file should carry:

- quantities and states
- relationships among quantities
- constraints
- temporal structure
- deferred value-source declarations

The `.myco` file should not carry:

- dataset paths
- optimizer choices
- hardware choices
- hidden execution logic
- backend-specific training code

### 4.2 Surface Language Design Goals

The surface language should aim for:

- readability over terseness
- explicit naming over clever shorthand
- acausal relation description over imperative execution steps
- extensibility without making the `v1` grammar too large

Time should not need a separate language sub-world. Temporal structure should be expressible using the same general language concepts as other relationships, even if the compiler later treats temporal edges as structural boundaries during planning.

### 4.3 Intended Language Concepts

The first draft of the surface language is expected to support the following concepts:

- `external` quantities for exogenous forcing inputs
- `state` quantities for internal values that evolve through time
- ordinary `node` declarations for derived or constrained values
- `relation` declarations for scientific relationships built from supported primitives
- `constraint` declarations for hard or soft restrictions
- `slot` declarations for deferred value sources
- `temporal` relations that connect quantities across timesteps

This list may still change at the grammar level. The concepts themselves are the important part.

### 4.4 Primitive Operations In `v1`

`v1` should keep the primitive operation set small and explicit. The point is not to become a general symbolic math language. The point is to support a minimal set of operations for which the compiler can provide clear directional behavior.

The required primitive set for `v1` is:

- symbol reference
- constants
- addition and subtraction
- multiplication and division
- explicit grouping into named relations

Optional unary inverse pairs such as `exp` and `log` may be added if the MVP genuinely needs them, but they are not required for the architectural proof. Arbitrary symbolic exponentiation is not part of the `v1` commitment.

### 4.5 Illustrative Example

The following example is illustrative rather than normative. It is meant to show the intended shape of the language, not lock final syntax.

```text
model TinyTree

external vpd_scale : scalar
external soil_water : potential
external hydraulic_cond : conductance

state water : potential { self <= 0 }
state carbon : carbon_mass { self >= 0 }

node stomata : conductance {
  self >= 0
  self <= g_max
}

node transpiration : water_flux { self >= 0 }
node g_max : scalar { self >= 0 }

relation demand_transpiration:
  transpiration = stomata * vpd_scale

relation supply_transpiration:
  transpiration = hydraulic_cond * (soil_water - water)

slot controller provides [stomata]:
  inputs = [water, carbon, vpd_scale, soil_water, hydraulic_cond, g_max]

temporal water_step:
  water[t+1] = water[t] - dt * transpiration[t]
```

What this example is trying to show:

- the language describes quantities and relationships, not an execution loop
- relations are built from supported primitives rather than opaque built-in science functions
- slots explicitly state which quantities they provide
- the same quantity can be reached through multiple relations
- temporal relationships look like ordinary relationships with time structure attached

In particular, the two `transpiration` relations are intentionally redundant. They are the smallest illustrative example of an overdetermined quantity in the language and exist to demonstrate why native equivalence support matters.

### 4.6 What Is Deferred In The Surface Language

The language should anticipate future support for reusable bundles and imports, but `v1` does not need a rich module system. It is enough for the grammar and reserved words to avoid painting the project into a corner.

Similarly, `v1` does not need surface-language support for every possible constraint shape. It needs a clean enough core that later syntactic sugar can desugar into the same semantic machinery.

## 5. Python API and Compile Specification

### 5.1 Python Is The Primary User Interface

The primary user-facing interface for `v1` should be Python, not a static config file.

This is the right default because:

- scientific users already have data in Python
- downstream ML and simulation workflows already live in Python
- explicit binding from Python avoids hidden file-loading conventions
- agent tooling can generate and inspect Python more easily than it can manage a bespoke config layer in isolation

This does not eliminate the need for a compile specification. It means the compile specification should be constructed from Python rather than forced through a standalone static file in `v1`.

### 5.2 Explicit Compile Specification

Even though the Python API is primary, Myco still needs an explicit compile-spec object.

That object exists for:

- reproducibility
- caching
- diffability
- diagnostics
- possible future CLI support

The Python API should be understood as a convenient frontend for building an explicit compile specification, not as a replacement for one.

### 5.3 Intended Python Workflow

The intended Python-side workflow should look roughly like this:

```python
import myco

model = myco.load("tiny_tree.myco")
model.check()

model.bind_data("vpd_scale", values=vpd_values, times=vpd_times)
model.bind_data("soil_water", values=soil_values, times=soil_times)
model.bind_data("hydraulic_cond", values=cond_values, times=cond_times)
model.bind_constant("g_max", 0.5)

model.bind_slot("controller", kind="learned", impl="mlp", hidden=[64, 64])

model.observe(
    "water",
    values=water_obs,
    times=water_times,
    loss="huber",
    mask_missing=True,
)

model.observe(
    "transpiration",
    values=transp_values,
    times=transp_times,
    loss="mse",
)

spec = myco.CompileSpec(
    mode="train",
    backend="jax",
    dt="1h",
    horizon=720,
)

compiled = model.compile(spec)
```

This API sketch is illustrative. The main design point is that provider binding, observation binding, and compile choice are explicit and inspectable.

### 5.4 Validation Behavior

Binding should validate eagerly.

Examples of errors that should be caught at bind or compile time:

- binding a quantity with incompatible dimensions
- binding a time series that does not cover the requested horizon
- leaving a required slot unresolved in a workflow that needs it
- declaring observations against a quantity the compiled plan cannot reach
- binding data to a symbol whose role is not compatible with the current compile mode

`v1` can surface these as Python exceptions backed by Rust errors. The important thing is that failures are descriptive and early.

### 5.5 Error Model

Myco errors should carry structured context rather than raw strings.

The minimum diagnostic payload in `v1` should aim to include:

- error category
- source location when relevant
- symbol or slot identity
- relevant dimensional or type information
- a short explanation of why the attempted action failed
- a suggested fix when one is obvious

In Python, these should surface as typed exceptions rather than opaque generic failures.

### 5.6 What Compilation Returns

Compilation should return ordinary backend artifacts, not a managed runtime.

In `v1`, that means something like:

- a generated Python source artifact or module object
- pure backend functions with documented signatures
- metadata about constraints, observations, and extracted plan structure
- metadata about emitted projections, slot shapes, and other compile-time interface decisions

The user should then run those artifacts directly in their own code. Myco should not own the optimizer, the hardware, or the long-lived execution process.

### 5.7 Rust/Python Boundary At The User Level

The user should experience the Python API as thin orchestration over a Rust core.

The intended split is:

- Rust owns parsing, symbolic representation, validation, planning, and code emission.
- Python owns experiment wiring and downstream execution.
- The PyO3 layer should stay thin and explicit.

This split is specified more concretely in Section 13.

## 6. Core Semantic Model and E-Graph Representation

### 6.1 Semantic Building Blocks

At the semantic level, Myco models are built from:

- declared quantities
- relationships among quantities
- constraints on quantities or relationships
- temporal links between quantities across steps
- provider interfaces that can populate selected quantities at compile time
- observation operators that compare model quantities to data

The semantics are acausal. A relation states that quantities are related. It does not commit the model to one privileged computation order.

### 6.2 Why An E-Graph-Backed Core

Myco needs native support for equivalence. The same quantity may be reachable through multiple symbolic paths. Alternative paths are not an edge case in this system; they are part of the reason the system exists.

An e-graph-backed symbolic core is the right fit because it gives the compiler:

- a first-class representation of equivalence classes
- a natural place to merge multiple expressions for the same quantity
- a substrate for extraction based on a cost model
- a clean foundation for later reasoning about overdetermination and ambiguity

In practical terms, the e-graph is the foundational symbolic representation in `v1`. The semantics are not reduced philosophically to implementation machinery, but the compiler should be designed from the start as an e-graph-based system rather than a simpler graph that later learns about equality.

The important scoping point is that the e-graph is the equality core of Myco, not the entire semantic system. Temporal links, observations, provider bindings, provenance, and non-equational constraint metadata live in adjacent compiler structures keyed to that equality core.

### 6.3 Conceptual `v1` IR

The `v1` compiler should lower syntax into a small semantic core.

Conceptually, that core contains:

- symbols for declared quantities
- primitive expressions over symbols
- e-classes representing equivalence sets of expressions
- relations that assert equivalence among expressions
- constraints attached to e-classes or relation results
- slots and provider bindings keyed to selected e-classes
- temporal links keyed to step-indexed e-classes
- observation bindings, penalty bindings, and provenance links keyed to the lowered structure

This is intentionally much smaller than the surface language. The surface language may grow. The semantic core should stay narrow.

### 6.4 E-Class Intuition

Suppose a model contains:

- `transpiration = stomata * vpd_scale`
- `transpiration = hydraulic_cond * (soil_water - water)`

In Myco, `transpiration` is not simply a node with two incoming edges. It participates in an equivalence set that includes alternative symbolic expressions that claim to denote the same quantity. The compiler can later ask:

- what is the cheapest way to compute this quantity from what is currently bound?
- what additional paths exist, and how independent are they?
- are there conflicting or ambiguous ways to reach it?

That is the core role of the e-graph representation.

### 6.5 Named Relations Are Structured Subgraphs

Named relations in the surface language are not opaque function calls. They are labels over subgraphs built from supported primitives. The point of naming them is readability, reuse, and diagnostics, not hidden evaluation semantics.

This matters because Myco should be able to inspect, merge, and extract through the internals of those relations. Opaque function calls can exist later as a deliberate escape hatch, but they are not the primary semantic story of `v1`.

### 6.6 Hard And Soft Constraints

Myco distinguishes between:

- hard constraints, which restrict the valid model space
- soft constraints, which contribute penalty terms or regularization signals

The long-term design intent is that both should live naturally in the graph-based semantic system rather than becoming disconnected ad hoc runtime features. In `v1`, some surface-language sugar may exist for convenience, but the lowered representation should still treat soft penalties as first-class constraint-like objects rather than special one-off features.

In `v1`, the runtime semantics should be explicit:

- compile-time checks validate dimensions, unit compatibility, and simple static inconsistencies where possible
- hard constraints on learned slot outputs are lowered to differentiable projection functions when possible
- soft constraints are lowered to named penalty terms in emitted loss code
- general inequality solving is deferred

## 7. Directional Extraction, Invertibility, and Path Cost

### 7.1 From Acausal Model To Directional Plan

The model is acausal. Compilation for a concrete workflow is not.

Given a compile specification, the compiler must determine:

- which quantities are already bound
- which quantities are requested outputs or observed intermediates
- which symbolic paths can compute those quantities from the currently bound set
- which path should be treated as canonical

This plan extraction step is where Myco moves from world description to executable artifact.

In `v1`, this should be understood as provider-aware planning over directional candidates derived from the equality core. It is not merely ordinary e-graph extraction of the cheapest expression in an e-class.

### 7.2 Relations Are Unconditional, Directional Use Is Not

A relation in the model states that quantities are related. It does not stop being true because a particular inversion is ambiguous or numerically ugly.

However, using a relation in a specific direction is conditional on the compile context. The compiler therefore needs a notion of directional implementation rather than pretending invertibility is a single Boolean property.

Conceptually, a directional implementation includes:

- the target quantity to solve for
- the symbolic form to use in that direction
- whether the implementation is exact or approximate
- whether branch selection is required
- any analytical preconditions that must be satisfied
- extraction cost hints

The relation is unconditional. The directional extraction choice is not.

In practice, `v1` should derive directional candidates from supported primitive and named relations, then choose among those candidates using the current compile-time availability set, dependency structure, and extraction cost hints.

### 7.3 `v1` Invertibility Scope

`v1` should support directional extraction only for a narrow class of expressions.

Required for `v1`:

- closed-form directional behavior for the primitive arithmetic operations
- extraction through named relations built from those primitives
- rejection of ambiguous or unsupported directions with clear diagnostics

Not required for `v1`:

- general symbolic solving
- general numerical root-finding
- broad branch analysis across arbitrary math libraries

### 7.4 Branch Ambiguity

Some inversions are analytically valid only up to branch choice. In those cases, the compiler should prefer to resolve the branch using active constraints and bound information from the reachable graph, not merely local constraints on the target quantity. If that is not possible, the direction should be treated as ambiguous rather than silently chosen.

### 7.5 Path Cost

When multiple valid paths exist, the compiler should not treat them as equal. It should extract a canonical plan using a cost model.

The `v1` cost model should account for:

- computational cost
- exact versus approximate implementation
- branch ambiguity or branch resolution burden
- information loss

Future versions may incorporate explicit numerical conditioning estimates, but `v1` does not need full conditioning analysis.

### 7.6 What Extraction Cost Is For

The extraction cost model serves two roles:

- selecting the canonical computation path used in emitted code
- ranking non-canonical paths that may later be turned into consistency checks or diagnostics

This is why extraction cost is not just an optimization detail. It shapes how the compiler interprets alternative paths throughout the system.

## 8. Overdetermination, Underdetermination, and Consistency

### 8.1 Why This Needs An Explicit Pass

The e-graph makes alternative paths representable, but it does not by itself decide what those paths mean in a compile configuration. Myco therefore needs an explicit analysis pass after binding and extraction planning.

### 8.2 Overdetermination

An overdetermined quantity is one for which multiple valid paths exist from the currently bound set.

The compiler should:

- select one canonical path using the extraction cost model
- record other valid paths
- determine whether those paths are appropriate for diagnostics, consistency checks, or both

The existence of multiple paths is not automatically an error. It is often the point.

This includes mixed cases where one path is mechanistic and another path is provided through a slot binding. A slot-provided quantity that is also reachable through mechanistic relations is still an overdetermined case; the only difference is that one path may be opaque while another is symbolically inspectable.

### 8.3 Consistency Terms

In `v1`, non-canonical paths may contribute consistency signals when they provide meaningful alternative predictions for the same quantity.

The minimum `v1` behavior should support:

- reporting multiple valid paths
- generating an optional consistency term between canonical and non-canonical predictions
- allowing that term to be emitted as part of the loss wiring when requested

The compiler should not assume that every alternative path should always become a consistency penalty. That remains a compile-time decision.

### 8.4 Path Independence

Not all multiple paths are equally informative. Two paths that share most of the same upstream structure are weaker consistency witnesses than genuinely distinct paths.

`v1` does not need a sophisticated formal independence metric, but the architecture should not preclude later weighting by path overlap or other dependence measures.

### 8.5 Underdetermination

An underdetermined quantity is one for which no valid path exists from the currently bound set, or for which only ambiguous or unsupported paths exist.

The `v1` compiler should:

- refuse to silently invent values
- surface the unresolved quantity explicitly
- explain which upstream bindings or directional implementations are missing

Simple bound propagation may still narrow feasible values, but `v1` does not need a full interval-solver story.

### 8.6 Ambiguity Is Not The Same As Missingness

A quantity may be underdetermined because nothing reaches it, or because multiple ambiguous branches reach it but none can be chosen safely. Those should be reported differently in diagnostics.

## 9. Type-Level Constraints, Dimensions, and Units

### 9.1 Constraint Kinds

Myco needs to represent at least three kinds of quantity-level restrictions:

- semantic constraints that are part of the model
- compile-time dimension and unit checks
- soft penalties that prefer but do not require specific behavior

These are related but not identical.

### 9.2 Dimensions And Units

`v1` should perform lightweight compile-time dimensional analysis.

The intended approach is:

- each declared quantity carries a dimension and unit annotation
- primitive operations compose dimensions according to simple algebra
- unit conversion factors are inserted during emission when dimensions match but units differ
- emitted backend code uses plain numeric values after validation and conversion

This is a compile-time validation feature, not a runtime quantity-wrapper system.

### 9.3 `v1` Implementation Approach

The `v1` implementation should use a lightweight internal dimension representation rather than wrapping runtime values in unit-aware Rust types. A small custom dimension struct is sufficient for the compiler core.

### 9.4 Bounds And Signs

Bounds and sign restrictions should be representable at the model level. Examples include:

- non-negativity
- non-positivity
- interval bounds

The surface language may eventually expose richer constraint syntax. The lowered representation in `v1` only needs a small set of constraint forms plus the ability to attach them to e-classes.

### 9.5 What `v1` Defers

`v1` does not need:

- a rich unit library
- generalized interval arithmetic across the whole compiler
- sophisticated symbolic propagation of all bound forms

It does need enough dimension and bound handling to catch obvious model-wiring errors early.

### 9.6 Runtime Constraint Lowering In `v1`

For `v1`, the runtime behavior of constraints should be explicit:

- compile-time checks validate dimensions, unit compatibility, and simple static inconsistencies when possible
- hard interval, sign, or simplex constraints on learned slot outputs should be lowered to differentiable projection functions when possible
- soft constraints should be lowered to named penalty terms in emitted loss code
- general inequality solving is deferred

## 10. Providers, Slots, and Value Sources

### 10.1 Value Sources

The same quantity can be populated through different mechanisms in different compile configurations.

The full design space includes:

- direct data
- constants
- mechanistic relations in the model
- learned implementations
- latent state updates
- inferred quantities reached through extraction

The key design point is that these are all ways of supplying values into one semantic system, not separate kinds of models.

### 10.2 Slot Semantics

A slot is a deferred value-source interface. It is not a synonym for a neural network placeholder.

A slot should declare:

- which quantities it provides
- which quantities it can read as inputs
- optionally, shape or grouping information relevant to binding

The compile configuration then decides how that slot is satisfied.

For `v1`, slots should be stateless pure per-step providers. Hidden recurrent state inside a slot implementation is deferred. If a model needs memory, that memory should be represented explicitly as state in the model rather than hidden inside the provider object.

### 10.3 `v1` Provider Kinds

The minimum provider kinds needed in `v1` are:

- data-bound
- constant-bound
- learned-bound
- initial-state-bound

Mechanistic relations are already part of the model definition and do not need to be rebound through slot machinery.

### 10.4 Provider Precedence And Role Exclusivity

Provider behavior in `v1` should follow these rules:

- direct providers satisfy a quantity explicitly
- observations never satisfy a quantity directly; they only constrain via loss or diagnostics
- mechanistic paths compute unresolved quantities when possible
- a quantity may be both provided and observed, in which case the observation acts as a consistency signal rather than a provider
- a quantity may be both mechanistically reachable and observed, which is the normal case for supervision
- a quantity should not silently have multiple direct providers unless the compile specification explicitly asks for reconciliation

### 10.5 Learned Implementations

Myco should not hard-code one learned method. A compile specification should be able to state that a slot is learned and provide enough metadata for the emitter to generate the appropriate interface.

In `v1`, this can be kept deliberately narrow. It is enough for the backend emitter to support a single learned implementation template, such as a small MLP, while keeping the slot abstraction backend-agnostic.

The compiler should also surface slot interface metadata, including aggregate input and output shapes, so the emitted backend artifact and the Python caller agree on the shape contract without manual counting.

## 11. Temporal Semantics

### 11.1 Time In The Surface Language Versus The Compiler

Time need not be special in the surface language. A temporal relation can still look like a relation with indexed quantities.

Time is special in the compiler, because temporal links determine:

- what belongs to the within-step computation graph
- what crosses timestep boundaries
- how unrolling or stepping is planned

### 11.2 `v1` Time Model

`v1` should use a discrete-time fixed-step model.

That is enough to prove:

- temporal state propagation
- external forcing sequences
- sparse observations over time
- compile-time planning across timesteps

Continuous-time and ODE-oriented lowering are explicitly deferred.

For `v1`, forcing and observation timestamps should align to the compile grid after explicit preprocessing. Myco should not perform interpolation or asynchronous stepping internally in the first implementation.

### 11.3 External, Internal, And Observed Time Series

Myco should distinguish among:

- external forcing series, which are exogenous inputs
- internal state trajectories, which are part of the model state
- observations, which constrain model quantities at specific times but are not themselves the state

This distinction is central to sparse-data workflows.

### 11.4 Initial Conditions

State quantities require an initial condition story. In `v1`, an initial condition should be bindable in the same explicit spirit as other compile-time inputs.

The minimum supported initial-condition sources are:

- constant initial state
- data-provided initial state
- learned initial state when the compile mode explicitly allows it

If a state quantity required by the extracted plan lacks an initial condition, compilation should fail with a clear diagnostic.

### 11.5 Intra-Step Algebraic Loops

`v1` does not support general intra-step algebraic loop solving. If the compiler cannot extract an acyclic within-step plan because a set of same-step relations forms a loop, it should detect that condition and reject compilation with a clear diagnostic.

This is a deliberate `v1` limitation. The architectural proof does not require generalized same-step root-finding.

### 11.6 Events

`v1` does not need a dedicated event ontology. Sharp changes in exogenous conditions can be represented as forcing data. More sophisticated event semantics can be revisited later if the compiler genuinely needs them.

## 12. Observations, Missingness, and Loss Wiring

### 12.1 Observations Are Not Providers

An observation is not the same thing as binding a quantity to a value source. An observation constrains the model. It does not necessarily populate the model state directly.

### 12.2 Observation Operators

The compiler needs an explicit notion of how model quantities are compared to data.

The minimum `v1` observation story should support:

- direct identity observations on reachable quantities
- sparse timestamps
- explicit missingness masks
- simple loss kinds such as MSE and Huber

More complex observation operators can be added later.

### 12.3 Missingness Semantics

Missingness must not collapse to zero or to any other implicit numeric value.

`v1` should support:

- masked loss evaluation
- explicit sparse timestamps
- a clear distinction between "zero observed" and "not observed"

For the initial JAX backend, the preferred emitted form is dense observation arrays paired with dense boolean mask arrays over the compiled horizon. This keeps the emitted loss functions compatible with static-shape tracing while preserving explicit missingness semantics.

### 12.4 Loss Composition

The emitted backend artifact should be able to compose:

- observation losses
- optional consistency terms
- optional soft penalties

Myco does not need to own the optimizer. It does need to wire the relevant loss components into emitted functions cleanly.

When possible, the emitted artifact should expose named loss components such as:

- `obs_loss`
- `consistency_loss`
- `soft_penalty_loss`
- `total_loss`

### 12.5 Observation Shape In `v1`

The first backend should prefer simple, explicit shapes over clever sparse data structures. In particular, sparse observations should be lowered to:

- a dense array over the compiled horizon
- a boolean mask array of the same shape

This is sufficient for the MVP and keeps the emission model simple.

## 13. Compilation Pipeline, Rust/Python Boundary, and Backend Emission

### 13.1 Pipeline

The intended compile pipeline is:

1. Parse `.myco` into an AST.
2. Lower the AST into the core semantic representation.
3. Populate the e-graph-backed symbolic core for the single-step template.
4. Validate dimensions, constraints, and basic model coherence.
5. Bind the explicit compile specification.
6. Run directional extraction and path analysis on the single-step template.
7. Analyze overdetermined and underdetermined quantities on that extracted template.
8. Elaborate the temporal execution form for the requested horizon from the extracted single-step plan.
9. Build observation and loss wiring.
10. Emit backend code and companion metadata.

The important `v1` rule is that equality reasoning and extraction happen on the single-step symbolic template, not on a fully unrolled multi-horizon graph.

All lowered semantic objects and extracted plan nodes should retain stable provenance links back to surface-language declarations and relation spans so diagnostics remain meaningful after lowering and extraction.

### 13.2 Rust/Python Boundary

The implementation boundary should be:

- Rust owns parse-through-emit.
- Python owns binding, experiment orchestration, and downstream execution.
- PyO3 is used only to expose a thin, explicit interface.

This keeps the compiler logic in one place while still fitting into normal scientific Python workflows.

### 13.3 Suggested `v1` Module Split

The exact crate boundaries may change, but the responsibilities should be roughly:

- syntax layer: parsing, spans, AST
- core layer: lowered semantic objects, e-graph population, constraints, extraction, diagnostics
- emitter layer: backend-specific code generation
- Python binding layer: thin PyO3 interface over the Rust core

### 13.4 Generated Artifact

For the initial backend, compilation should emit a Python/JAX artifact containing ordinary functions. A likely minimal set is:

- parameter initialization
- step function
- rollout function
- loss function
- exported metadata describing projections, constraint enforcement, slot shapes, and extracted plan structure

The exact interface can evolve, but the emitted result should be ordinary code the user can call directly.

### 13.5 What Myco Does Not Own

Myco should not own:

- the optimizer
- the training loop
- hardware placement
- dataset loading conventions outside the explicit API

Those remain user responsibilities.

## 14. Diagnostics, Introspection, and Explainability

### 14.1 Diagnostics Are Core Requirements

If the first real model cannot be debugged without reading compiler internals, `v1` has failed even if it technically emits code.

### 14.2 Minimum `v1` Diagnostics

The compiler should provide:

- structured validation errors with source spans
- symbol-aware messages
- dimensional mismatch explanations
- unresolved-slot errors
- unresolved-path errors
- ambiguous-direction errors

### 14.3 Minimum `v1` Introspection

The user should be able to ask at least:

- what constrains this quantity?
- what path did the compiler choose?
- what alternative paths existed?
- why was this quantity not solved?

Text output is sufficient in `v1`.

### 14.4 Plan Explanation

For extracted plans, the compiler should be able to explain:

- the chosen canonical path
- the cost basis for that choice
- any non-canonical paths that were discarded or repurposed

This is especially important for reviewer trust and for debugging extraction behavior.

## 15. MVP v1 Scope

### 15.1 Required Features

`v1` should include:

- `.myco` parsing
- lowering into a narrow semantic core
- an e-graph-backed symbolic representation
- a minimal rewrite and extraction system over primitive arithmetic
- explicit slots and compile-spec binding
- discrete-time temporal unrolling
- explicit initial-condition binding for state quantities
- basic dimensional validation
- sparse observation binding with masking
- Python/JAX emission
- minimal diagnostics and introspection

### 15.2 Minimum Rewrite Set

The initial directional support and rewrite machinery should be kept small.

Required:

- directional support for addition and subtraction
- directional support for multiplication and division
- commutativity for addition
- commutativity for multiplication

Additional rewrites should only be added when demanded by the MVP example.

### 15.3 Demo Target

The MVP demo should use one small model that supports at least two adjacent workflows:

1. synthetic data generation from a known provider
2. learned-slot recovery from observations using the same `.myco` model

The model should include:

- at least one state quantity
- at least one slot-provided quantity
- at least one overdetermined quantity with multiple symbolic paths
- sparse observations
- dense per-step observations

The target success case is recovery of either:

- the known parameterization, when the provider class makes that identifiable
- a behaviorally equivalent controller under held-out rollout or input-output tests

### 15.4 Acceptance Checklist

The MVP should not be considered complete unless all of the following hold:

- the same `.myco` model supports both simulation and training
- overdetermined structure is visible in diagnostics
- a canonical path is chosen and explainable
- same-step algebraic loops are either absent in the MVP or rejected cleanly by the compiler
- sparse observations are handled without missing-as-zero bugs
- emitted backend code is ordinary runnable Python/JAX code

## 16. Deferred Features and Open Questions

### 16.1 Deferred Beyond `v1`

Deferred features include:

- rich module and import system
- reusable domain bundles
- multiple backend targets
- continuous-time and ODE-oriented lowering
- generalized root-finding
- advanced numerical conditioning diagnostics
- sophisticated path-independence weighting
- rich interval reasoning
- visualization and IDE tooling

### 16.2 Open Questions

Questions that should be pressure-tested early include:

- Is a standard e-graph representation sufficient for the directional extraction strategy, or are custom extensions required?
- How much branch reasoning is needed before extraction becomes useful on real models?
- What is the smallest primitive set that still supports a compelling MVP?
- How narrow can the learned-slot interface stay while still serving the first real use case?
- How much bound and dimension propagation is actually needed in `v1` to catch meaningful mistakes without bloating the implementation?

## 17. Milestones and Review Questions

### 17.1 Suggested Near-Term Milestones

1. Finalize the semantic core and `v1` scope.
2. Implement parser, lowering, and source-aware errors.
3. Implement the e-graph-backed core with the minimum rewrite set.
4. Implement compile-spec binding and extraction.
5. Emit Python/JAX code for the MVP model.
6. Run the synthetic recovery demo.
7. Iterate on diagnostics and introspection until the first real model is debuggable.

### 17.2 Current Review Questions

External review should focus on:

- whether the semantic core is narrow enough
- whether the e-graph commitment is justified by the extraction story
- whether the directional extraction model is coherent and sufficiently scoped
- whether `v1` promises too much or too little
- whether the MVP demo is strong enough to actually validate the architecture
