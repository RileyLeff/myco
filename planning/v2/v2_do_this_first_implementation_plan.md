# V2 Do This First: Implementation Plan

This note turns [v2_do_this_first.md](./v2_do_this_first.md) into a concrete implementation plan.

It is still pre-model-family work.

The goal is to fix the abstraction boundary before more `v2` features land on top of the current source-language assumptions.

## Goal

Shift Myco from:

- source-level workflow roles in `.myco` such as `external` and `state`

toward:

- workflow-neutral structural models in `.myco`
- workflow roles declared in binding/config
- structural persistence inferred from temporal relations and/or explicit binding-time annotations

## Non-Goal

This plan does **not** try to solve:

- the full parameter design
- the full function registry
- local solve blocks
- package registries
- long-term semantic schemas/components

Those come after the boundary is cleaned up.

## Desired End State

At the end of this refactor, the project should read conceptually like this:

### In `.myco`

The model declares:

- quantities
- relations
- temporal equations
- constraints
- dimensions/units

But not:

- whether a quantity is supplied from data
- whether a quantity is learned
- whether a quantity is observed
- whether a quantity is fixed in this workflow

### In binding/config

The experiment declares:

- what is assumed
- what is observed
- what is learned
- what is used as initial values
- what is rollout-persistent if that is not fully inferable
- what outputs are required

## Guiding Design Choice

Keep the user-facing workflow vocabulary small.

The likely role surface is:

- `assume`
- `observe`
- `learn`

Then let each role have more specific modes.

Examples:

- assumed constant
- assumed indexed series
- assumed initial value
- learned slot
- learned rollout-static quantity later
- observed dense
- observed sparse

Avoid growing a large surface ontology like:

- external
- latent
- persistent
- fixed
- state
- driver
- parameter

unless those distinctions are really earning their keep.

## Recommended Technical Strategy

Do this as an internal refactor first, then simplify the surface syntax.

The implementation should move the center of gravity before it removes all of the old words.

That means:

1. decouple planning/runtime semantics from `QuantityKind::{External,State,Node}`
2. add binding-time ways to express the needed workflow facts
3. infer persistence from temporal structure where possible
4. only then simplify or deprecate the source-level role keywords

## Phase 1: Introduce Workflow-Neutral Internal Semantics

### Objective

Stop using source-level quantity kind as the primary source of execution semantics.

### Main code areas

- [syntax.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/syntax.rs)
- [semantic.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/semantic.rs)
- [equality.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/equality.rs)
- [compile.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/compile.rs)
- [plan.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/plan.rs)
- [pipeline.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/pipeline.rs)

### Changes

- Introduce an internal distinction between:
  - source declaration form
  - inferred structural persistence
  - workflow binding role
- Add a compiler pass that scans temporal equations and marks quantities that participate in temporal carry/update structure.
- Stop treating `QuantityKind::State` as the only way a quantity can require initialization.

### Result

The compiler should be able to say:

- this quantity behaves like rollout state because of temporal structure

without depending on a `state` keyword in the source model.

### Current Status

The first slice of this phase is now implemented:

- equality lowering infers persistent quantities from temporal update left-hand sides
- compile validation uses inferred persistence when deciding which quantities require initial-state bindings
- emitted artifacts and typed artifact metadata now expose persistent quantities explicitly

For compatibility during the migration:

- legacy source-level `state` declarations still contribute to persistence

That is deliberate.

It means the compiler's internal source of truth is no longer only the source keyword, while existing models continue to compile until binding-time persistence annotations are ready.

## Phase 2: Expand Binding-Time Role Annotations

### Objective

Make config/binding capable of carrying the workflow facts the compiler actually needs.

### Main code areas

- [compile.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/compile.rs)
- [python/myco/api.py](/Users/rileyleff/Documents/dev/myco/python/myco/api.py)
- [python/myco/types.py](/Users/rileyleff/Documents/dev/myco/python/myco/types.py)
- [crates/myco-py/src/lib.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-py/src/lib.rs)

### Changes

Add or refactor binding/config concepts around:

- assumed values
- observed values
- learned values
- initial values
- optional explicit persistence annotations if inference is not enough

The exact public API does not need to be final immediately, but the semantics should become:

- workflow roles live here
- not in `.myco`

### Likely API direction

Something closer to:

- `assume_constant(...)`
- `assume_series(...)`
- `assume_initial(...)`
- `observe_dense(...)`
- `observe_sparse(...)`
- `bind_slot(..., kind="learned")`

or a grouped equivalent that still keeps the underlying semantics clear.

### Current Status

The first slice of this phase is now implemented:

- `initial_state` bindings now make a quantity rollout-persistent in the compiled workflow, even if the source model did not already mark or imply persistence
- the Python API now exposes `assume_series(...)`, `assume_constant(...)`, and `assume_initial(...)` aliases alongside the older `bind_*` names

This matters because it gives config a real way to carry persistence intent.

That means the remaining legacy dependence on source-level `state` is now much smaller:

- source-level `state` is still accepted and still contributes persistence during migration
- but config can now express persistence for non-`state` quantities directly

## Phase 3: Make Persistence A Compiler Property, Not A Source-Level Role

### Objective

Move initialization and rollout-state requirements onto inferred/compiler properties plus config, rather than onto the `state` keyword.

### Main code areas

- [compile.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/compile.rs)
- [plan.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/plan.rs)
- [emit.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/emit.rs)

### Changes

- Determine which quantities need initial values by temporal analysis and/or explicit workflow annotation.
- Ensure runtime validation, artifact metadata, and emitted module contracts refer to those inferred persistent quantities rather than to source-level `state`.
- Keep the planner and emitter aligned with the new persistence source of truth.

### Result

The system should be able to reject:

- missing initialization for a persistent quantity

even if the source model never used a `state` keyword.

## Phase 4: Simplify The Source Language

### Objective

Once Phases 1–3 are solid, simplify the `.myco` surface so it stops implying that workflow roles belong in the model.

### Main code areas

- [syntax.rs](/Users/rileyleff/Documents/dev/myco/crates/myco-core/src/syntax.rs)
- fixtures, examples, docs

### Options

Two reasonable options:

1. keep old keywords temporarily as sugar or compatibility syntax while lowering them into the new world-neutral semantics
2. replace them more directly with a flatter quantity declaration surface

Because this project has no public compatibility burden, the codebase can be fairly aggressive here once the internal semantics are ready.

Still, the refactor should be staged enough that tests continue to protect the planner and emitter behavior.

## Phase 5: Clean Up Docs, Examples, And Tests

### Objective

Make the new boundary obvious everywhere.

### Main code areas

- [docs/end_to_end_walkthrough.md](/Users/rileyleff/Documents/dev/myco/docs/end_to_end_walkthrough.md)
- [README.md](/Users/rileyleff/Documents/dev/myco/README.md)
- fixtures and examples
- `planning/v2/` notes if needed

### Test focus

Add or update tests that prove:

- a structurally valid world model can be compiled under multiple different workflows
- the same quantity can be assumed in one workflow and observed in another
- persistence/initialization requirements arise from temporal structure plus config, not only from source keywords
- invalid model/config combinations fail cleanly

## Suggested Execution Order

The best sequencing is probably:

1. internal persistence inference
2. binding/config role expansion
3. planner/emitter/runtime contract switch-over
4. source-language simplification
5. docs/examples cleanup

This order keeps the risk concentrated in the compiler core before user-facing syntax is rewritten.

## Acceptance Criteria

This refactor is successful when all of the following are true:

1. `.myco` no longer needs to encode workflow assumptions to support the current compiler behavior
2. the compiler can infer or accept binding-time declarations for persistence/initialization requirements
3. the same structural model can support multiple workflows without source edits that only change role assignment
4. runtime artifacts and metadata reflect the new source of truth cleanly
5. tests and docs describe the system in terms of world structure plus workflow binding, not source-level role keywords

## Recommended Immediate Next Step

Start with a narrow internal pass:

- infer persistent quantities from temporal structure
- thread that through compile validation and artifact metadata

That is the smallest change that begins to detach the execution semantics from `QuantityKind::State`, and it will make the rest of the refactor much easier to reason about.
