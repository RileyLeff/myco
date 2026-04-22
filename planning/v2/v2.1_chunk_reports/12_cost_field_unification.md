# 12 — Cost-Field Struct Unification

Three sections of spec_new.md carry divergent cost-field inventories
with no cross-reference. This chunk tracks the unification. Status:
open, no option chosen. Flagged O/ACK in
`planning/v2/audit/adjudication.md` under the batch-3/batch-4
cross-cutting item.

## The divergence

- §14.2 `loss_of(residual)` returns a struct of
  `{data_fit, constraint_violation, regularization}`. Loss-function
  terms the workflow aggregates at training emission (§25).
- §19.1 extraction cost vector is
  `{precision, latency, memory, approximation class}`. Pareto axes
  for e-graph residual extraction; workflow picks a point.
- Chunk 04 O2.4 locks `cost_of(expr)` as
  `{compute, approximation, condition, truncation, discretization}`.
  Auto-derived per-expression lossiness. Primary consumer: the
  extractor cost-vector computation and approximate-block
  diagnostics.

Same word "cost" across three surfaces, three different field sets,
no cross-reference in spec prose today.

## Options

### (a) One unified struct with O2.4's five fields

`loss_of` and the §19.1 cost vector both reshape onto
`{compute, approximation, condition, truncation, discretization}`.
One concept, one struct, three consumer sites.

Implications:
- §14.2 stops being a loss-function-terms surface. Training-loss
  aggregation in §25 rewrites to compose directly from observe
  likelihoods plus prior log-densities plus unsatisfied
  `constraint` residuals, rather than reading named fields off a
  `loss_of` return.
- §19.1 Pareto axes collapse. `latency` and `memory` fold into
  `compute` or become separate fields on the unified struct (so the
  struct grows to six or seven fields). Either choice breaks the
  O2.4 lock.
- Extractor and training-emission consume the same inventory.
  Cleanest for cross-referencing.

### (b) Two intrinsics, distinct fields

Keep `loss_of` as the loss-function-terms surface (training
aggregation). Introduce `cost_of` per O2.4 as the
per-expression-lossiness surface (extractor input). §19.1 extraction
cost vector references `cost_of` fields directly and drops its
private inventory.

Implications:
- Clean separation by consumer. `loss_of` is workflow-facing,
  `cost_of` is compiler-facing.
- §19.1 still needs a resolution for `latency` and `memory`, which
  are not O2.4 concepts. Candidates: separate extractor-policy
  struct; additional fields on `cost_of`; fold into a generic
  `resource_of` intrinsic.
- `loss_of` keeps its name despite overlap; collision between
  `loss_of.regularization` and `cost_of.approximation` continues
  as a naming concern only.

### (c) Rename `loss_of` and keep §19.1 separately

Rename §14.2 `loss_of` → `objective_terms` or similar. Acknowledge
that three concepts with a naming collision exist, break the
collision, add explicit cross-references for the reader.

Implications:
- Smallest surface change. §14.2 and §19.1 both retain their
  current field inventories.
- `approximation` appears in both §19.1 and O2.4 `cost_of`; the
  overlap stays unreconciled. Readers consulting either section
  must keep both in mind.
- Defers the real unification question. May be the right call if
  the three surfaces are genuinely separate concerns that share
  vocabulary by accident.

## Load-bearing questions

1. Is `loss_of` a loss-function surface (training aggregation) or a
   per-expression extractor-cost surface? Current prose reads as the
   former. Chunk 04 O2.4's `cost_of` covers the latter. If both
   exist as distinct intrinsics, option (b); if they collapse,
   option (a).
2. Do §19.1 `latency` and `memory` live inside a per-expression
   `cost_of` struct (making it a seven-field struct) or in a
   separate extractor-policy struct? O2.4 does not cover them.
3. Does the extractor consume O2.4 `cost_of` fields directly, or
   compose them through a §19.1 policy layer that adds latency /
   memory weighting?
4. Peak allocation as a first-class `cost_of` field is already open
   in §35 ("Memory as a `cost_of` field"). This chunk subsumes that
   open.
5. Training-emission aggregation in §25 presumes `loss_of` named
   fields today. A change to option (a) requires rewriting §25's
   aggregation contract.

## Cross-refs

- §14 preamble (compiler intrinsics)
- §14.2 `loss_of` named-field return
- §19.1 extraction cost model
- §25 training emission (workflow aggregation)
- §35 "Memory as a `cost_of` field" open (subsumed here)
- chunk 04 O2.4 `cost_of` named-field struct lock

## Status

Open. No option chosen. Resolution unblocks the ACCEPT items in
batches 3 and 4 of `planning/v2/audit/adjudication.md`: §14 C1
(`loss_of` / `cost_of` field inventory reconciliation), §15 H4
(multi-dimensional cost vector cross-link to named-field
`loss_of`), §15 H5 (same struct citation in approximate-block
ordering), §19 H2 (§19.1 cost-dimension divergence from O2.4),
§19 C2 (same).
