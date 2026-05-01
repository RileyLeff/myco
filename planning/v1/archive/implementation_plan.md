# Myco `v1` Implementation Plan

Status: active

This plan tracks the current implementation sequence for bringing the codebase to honest `v1` spec parity.

## Completed

1. Core vertical slice
- `.myco` parsing
- semantic lowering
- equality-oriented lowering
- compile-spec binding
- provider-aware single-step planning
- Python and JAX emission

2. Python package surface
- `uv`-managed mixed Rust/Python package
- experiment builder API
- compile/write helpers
- runnable examples

3. End-to-end training proof
- TinyTree synthetic generation and learned recovery
- Optax training loop against emitted JAX artifact
- held-out behavioral recovery

4. `v1` parity slices completed
- explainability / introspection surface
- hard constraint lowering and soft penalties
- compile-time dimensions
- minimal unit validation and emission-time conversion
- compile-mode validation and metadata

## Current Phase

Phase: `egg` equality-core rollout completed

What changed:
- the lowered equality model now owns an `egg`-backed equality core directly
- equation registrations live inside that core with e-class metadata
- compile binding and single-step planning consume the shared equality core instead of a parallel flat equation list
- the former spike module is now the actual equality substrate rather than sidecar infrastructure

## Current Rollout Steps

1. Make `egg` the primary equality substrate in the lowered model. Complete.
2. Move existing equation registration and equivalence tracking into that core. Complete.
3. Rewire single-step planning to consume the `egg`-backed core instead of a parallel equation-only layer. Complete.
4. Remove redundant spike-only equality code once the planner is running through the new core. Complete for the current arithmetic MVP.
5. Re-run Rust tests, Python tests, and the TinyTree training demo. Complete.
6. Run another review round after the refactor settles. Pending.

## Explicit Non-Goals During The Egg Rollout

- no new frontend language features
- no parser rewrite
- no new learned provider templates
- no new scientific model scope
- no attempt at general symbolic solving beyond the current arithmetic MVP

## Acceptance Criteria For The Egg Rollout

- the lowered model owns an `egg` equality core rather than a sidecar spike module. Done.
- the planner consumes that equality core directly. Done.
- the emitted artifacts are unchanged in observable behavior on the current examples. Done.
- the TinyTree training recovery demo still converges. Done.
- all Rust and Python tests pass. Done.
