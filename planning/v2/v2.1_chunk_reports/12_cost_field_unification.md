# 12 — Cost / Objective Vocabulary Resolution

Status: resolved 2026-04-23.

## Decision

Myco keeps two separate surfaces:

- `cost_of(expr)` is compiler/planner economics. It is consumed by
  extraction (§19.1), diagnostics, and approximate-block reporting.
- `objective_terms(residual)` is workflow-facing training-objective
  decomposition over a residual site. It is consumed by training
  emission (§25) and workflow objective helpers.

The retired spelling is `loss_of(residual)`. The word "loss" was doing
too much: likelihood contribution, training objective, planner cost,
and approximation cost. The new vocabulary keeps those concepts apart.

## `cost_of(expr)`

Canonical fields:

- `compute`
- `memory`
- `approximation`
- `condition`
- `truncation`
- `discretization`

`compute` and `memory` are resource economics. The remaining four are
faithfulness / numerical-quality economics inherited from the chunk 04
O2.4 shape, with `memory` added because extraction needs peak
allocation as a first-class axis.

Extraction consumes the full record and returns a Pareto front unless
workflow config selects a policy:

- `compute_first`
- `memory_first`
- `faithfulness_first`
- weighted policy over the named fields

The config surface is `run.config.extraction_policy`.

## `objective_terms(residual)`

Canonical fields:

- `data_fit`
- `constraint_violation`
- `regularization`

The return is not a scalar. Workflow code chooses which terms to
aggregate, and helpers such as `soft_penalty(weights)` and
`augmented_lagrangian(weights, mu, lambda_init, mu_schedule)` consume
these fields. The config surface for scalarization policy is
`run.config.objective_policy`.

## Consequences

- §14.2 owns both intrinsic definitions and their separation.
- §19.1 no longer carries a private extraction-axis vocabulary.
- §25 no longer depends on `loss_of` fields.
- §35's "Memory as a `cost_of` field" open is closed: memory is a
  first-class field.
- anti_spec.md retires `loss_of(residual)`.

## Remaining Adjacent Opens

This decision does not close the matrix/backend questions around
conditioning shape for multi-output operations. `condition` remains a
single `cost_of` field for now; whether its value is scalar, structured,
or tolerance-class-indexed belongs with the matrix/backend work.
