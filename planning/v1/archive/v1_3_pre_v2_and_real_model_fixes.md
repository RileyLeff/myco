# Pre-`v2` And Pre-Real-Plant-Model Fixes

Status: `v1` is real, and the pre-`v2` correctness/runtime-contract hardening pass is now complete.

Audience: project authors

## Purpose

This note captures the work that should land after the successful `v1.2` state and before:

- broadening the frontend language
- building a real plant-model family
- opening a `v2` spec

The goal here is not to add more capability for capability's sake. The goal is to tighten the places where the current implementation can still mislead users or encode a weaker runtime contract than the spec suggests.

## Current Assessment

### What Is Already Proven

- The compiler architecture is real end to end.
- The equality substrate is genuinely `egg`-backed.
- The Python/JAX emission path supports synthetic generation and learned recovery.
- Sparse observations, dense rollout, differentiable projections, and consistency regularization all work together.
- Introspection and provenance are useful enough to debug the current TinyTree workflow.

### What Was Tightened

The remaining work is mostly about correctness and honesty, not missing architecture.

The highest-value items are:

1. candidate-local extraction and provenance isolation
2. dense-provider semantics for `DataSeries`
3. runtime contract validation for emitted artifacts
4. stronger constraint/runtime policy for mechanistic/state outputs
5. small parser/runtime cleanup items that will matter immediately on larger models

All five of these have now landed in the implementation.

## Recommended Sequence

1. Fix candidate-local extraction/provenance leakage. (Done)
2. Tighten `DataSeries` semantics so `v1` means dense per-step forcing over the horizon. (Done)
3. Add runtime validation on emitted artifacts or the Python wrapper for keys, shapes, masks, and horizon agreement. (Done)
4. Decide and implement a narrow runtime policy for mechanistic/state-output constraint handling. (Done)
5. Land small parser/runtime quality fixes that unblock the first real model family. (Done)

This order matters because the first three items are about correctness and trust. They should land before any `v2` language or model-scope work.

## Work Remaining

## 1. Candidate-Local Extraction And Provenance Isolation (Done)

### Current Problem

Equation candidates currently resolve through the global output e-class for a quantity. That means a candidate sourced from relation A can extract an expression that actually originated in relation B if both relations define the same output and B happens to be cheaper or more available under the current bindings.

This can corrupt:

- source labels
- explanation text
- alternative-path semantics
- path costs
- consistency interpretation

### What Landed

Keep the global `egg` equality core, but make candidate resolution local to the candidate’s own directional seed expression.

The implementation now:

- uses candidate-local `egg` extraction over the candidate seed expression plus local rewrites
- keeps the global equality core for equivalence bookkeeping and global visibility
- prevents one candidate from silently resolving through another candidate’s seed expression
- includes a regression test with two relations targeting the same output where only one path is actually available

### Acceptance Criteria

- a candidate can no longer “steal” another relation’s expression through a shared output e-class
- chosen-path explanations remain aligned with the relation or slot that actually produced the plan step
- there is a regression test covering two different relations targeting the same quantity where one candidate is unavailable and must not resolve through the other

## 2. Dense `DataSeries` Semantics (Done)

### Current Problem

The compile surface accepts `DataSeries { steps: Vec<usize> }`, but the emitted runtime assumes dense per-step forcing is available for every step in the compiled horizon.

That means the current implementation supports:

- sparse observations

but not actually:

- sparse direct providers

### What Landed

For `v1`, tighten the contract instead of pretending sparse providers exist.

`DataSeries` should mean:

- dense per-step forcing across the entire compiled horizon

If sparse providers are wanted later, that should be a separate feature with explicit semantics.

The implementation now rejects direct `DataSeries` bindings unless they cover every step in the compile horizon.

### Acceptance Criteria

- compile-time validation requires direct `DataSeries` bindings to cover exactly the full horizon
- the Python surface and docs no longer imply sparse forcing support in `v1`
- examples and tests use the dense-forcing interpretation consistently

## 3. Runtime Input Validation (Done)

### Current Problem

Compile-time validation is structural, but emitted artifacts still accept runtime payloads with only thin checking.

The current runtime contract is under-specified for:

- required forcing keys
- required constant keys
- initial-state keys
- observation keys
- observation value/mask shape agreement
- horizon agreement
- slot-provider presence

### What Landed

Add a narrow but explicit runtime validation surface.

This can live either:

- in emitted modules as `validate_inputs(...)`
- in the Python wrapper before module execution

For `v1`, either is acceptable as long as the checks are consistent and easy to call.

The implementation now emits:

- `validate_rollout_inputs(...)`
- `validate_observations(...)`

on both Python and JAX artifacts, and rollout/loss paths call them before execution.

### Acceptance Criteria

- missing required forcing or constant keys are reported before rollout
- observation masks and values are checked for matching horizon length
- rollout inputs and compile horizon must agree
- missing learned slot providers are reported cleanly

## 4. Constraint Runtime Policy For Mechanistic And State Outputs (Done)

### Current Problem

Hard constraints are currently operational on:

- learned slot outputs
- learned initial states

But they are not yet handled explicitly for:

- mechanistic equation outputs
- temporal state updates
- direct runtime inputs

### What Landed

Do not add a full inequality solver. Just make the runtime policy explicit and narrow.

Reasonable `v1.3` options include:

- validate-only
- project selected classes of outputs
- assert on violation

### What Landed

The implementation now makes the policy explicit and backend-visible:

- learned slot outputs and learned initial states are projected
- direct runtime inputs are validated against simple bounds
- Python artifacts raise on constrained mechanistic/current or temporal/state-output violations
- JAX artifacts accumulate explicit `constraint_violation_loss`
- compiled artifact metadata now exposes the backend-specific runtime policy

### Acceptance Criteria

- the compiler/runtime behavior for bound-constrained mechanistic and state quantities is explicit
- violations do not silently disappear without any signal

## 5. Small Parser And Runtime Fixes Before A Real Model (Done)

These were not architectural blockers, but they were likely to matter quickly:

- scientific notation in expressions
- better missing-brace / malformed inline-constraint diagnostics
- safe division warnings or guards for automatically extracted inverted paths

### What Landed

The implementation now includes:

- scientific notation support in the expression tokenizer
- explicit diagnostics for unterminated multiline constraint blocks
- explicit diagnostics for malformed inline-constraint trailing content
- emitted safe-division helpers in both Python and JAX artifacts

This is enough to stop treating these as open pre-`v2` risks.

## What This Note Is Not

This is **not** the `v2` plan.

It does **not** include:

- parameter roles
- function registries
- local implicit solve blocks
- demand-driven planning
- richer observation operators
- production-grade JAX backend structures

Those belong in the next planning document after the current implementation is trustworthy enough to serve as a base for the first real plant-model family.

## Conclusion

At this point, the pre-`v2` hardening pass is complete.

The next planning step should not be another general cleanup note. It should be a focused `v2` plan for one real plant-model family, with the compiler changes driven by that concrete target rather than by abstract language expansion.
