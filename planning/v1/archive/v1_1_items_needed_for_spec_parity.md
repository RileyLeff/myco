# Items Needed For True `v1` Spec Parity

Status: working implementation exists, but `v1` spec parity is not complete yet

Audience: project authors

## Purpose

This document records what is still missing between the current Myco implementation and the `v1` spec in [spec.md](/Users/rileyleff/Documents/dev/myco/planning/v1/spec.md), and proposes an execution order for closing those gaps.

The current implementation is already a real vertical slice:

- parse
- semantic lowering
- equality-oriented lowering
- compile-spec binding
- single-step planning with simple inversion
- Python and JAX emission
- Python package surface
- structured diagnostics

That is enough to justify moving into end-to-end testing. It is not yet enough to claim full `v1` spec parity.

## Current Assessment

### What Is Already Proven

- The world-description / compile-binding split is real.
- The Python API can bind providers and observations explicitly.
- The compiler can change causal direction based on what is bound.
- The emitted JAX artifact is usable and structurally correct.
- Overdetermined paths can be tracked and surfaced as consistency alternatives.
- Same-step algebraic loops are detected and rejected.
- The Python package is real enough to support examples and tests.

### Where The Current Implementation Diverges From The Spec

The biggest remaining gaps are:

- The symbolic core is not yet a true e-graph-backed equality core.
- Dimensional analysis and unit compatibility are not implemented.
- Constraint handling is still narrow and emitter-driven rather than a fuller compiler feature.
- Introspection is thinner than the spec calls for.
- Compile modes exist structurally but do not yet change backend behavior.
- Soft penalties are not implemented.
- The end-to-end training proof has not yet been run.

The implementation is therefore best described as:

- architecturally correct
- pragmatically narrowed
- not yet fully spec-complete

## Recommended Next Sequence

I do **not** recommend immediately jumping into e-graphs, dimensions, and broader compiler machinery in parallel.

The better order is:

1. Prove the emitted JAX artifact can actually train end to end.
2. Fill in the remaining `v1` semantics that are independent of the equality-core replacement.
3. Decide whether true e-graph replacement is required for `v1`, or whether it should become the first major `v2` refactor after the training proof.

This order matters because the training demo is the shortest path to learning whether the current architecture is scientifically useful. If gradients do not flow or the emitted artifact is awkward to train, that feedback is more valuable right now than a more sophisticated symbolic core.

## Work Remaining

## 1. End-To-End Synthetic Training Demo

### Why It Comes First

This is the most important missing proof.

The spec says `v1` should demonstrate that the same model can be reused across adjacent workflows and that emitted backend code can recover either:

- an identifiable parameterization, or
- a behaviorally equivalent controller

That has not been demonstrated yet.

### What Needs To Exist

- A known controller implementation in Python/JAX that can generate synthetic trajectories.
- A compiled Myco artifact for the same model with the controller left learned.
- A small Optax training script that fits the learned controller to synthetic observations.
- A held-out rollout comparison showing behavioral recovery.

### Acceptance Criteria

- Training converges without hand-editing emitted code.
- Gradients flow through the emitted artifact and any slot-output projection path.
- Recovery is defined behaviorally, not by exact parameter equality.
- The same `.myco` model is used for both synthetic generation and learned recovery.

### Planned Implementation

- Add a small Python training example under `examples/` or `python/` that imports the emitted JAX artifact.
- Start with the existing TinyTree fixture rather than enlarging the language.
- Use one known analytic controller first, not a second learned model.
- Measure held-out rollout error rather than parameter matching.

## 2. Introspection And Explainability Surface

### Why It Matters

The data structures already contain most of what is needed, but the user cannot yet ask the compiler enough questions.

The spec explicitly calls for minimal explainability in `v1`.

### Missing Surface

- `inspect(quantity)`-style explanation of what constrains a quantity
- explanation of which path was chosen and which alternatives existed
- explanation of why a quantity is unresolved
- plan-oriented diagnostics beyond count summaries

### Planned Implementation

- Add a lightweight introspection module in the Rust core that converts planner state into structured explanation objects.
- Expose this first in CLI form, then through the Python package.
- Keep it text-first; no visualization work is needed for `v1`.

### Acceptance Criteria

- A user can ask why `stomata` was solved through inversion rather than direct provision.
- A user can ask why a quantity remains unresolved.
- Alternative paths can be listed with their costs.

## 3. Hard Constraints And Soft Penalties

### Current State

Current hard constraints are narrow:

- simple lower/upper bound parsing from surface constraints
- runtime projection in the emitter

Current soft penalties are not implemented:

- `soft_penalty_loss(...)` is still effectively a stub

### What `v1` Needs

- A compiler-level notion of simple hard output constraints for learned slots.
- Named penalty terms for soft constraints.
- Loss outputs separated by component:
  - observation loss
  - consistency loss
  - soft penalty loss

### Planned Implementation

- Keep `v1` narrow:
  - interval/sign bounds for learned slot outputs
  - one or two penalty forms only
- Represent penalties explicitly in the lowered model rather than only in the emitter.
- Consider smooth projections for learned-slot outputs if training shows clipped gradients are a practical issue.

### Acceptance Criteria

- At least one soft penalty path exists end to end.
- Penalty contributions appear in emitted loss components.
- Hard learned-slot bounds are enforced consistently across Python and JAX emission.

## 4. Dimensions And Units

### Current State

- Quantity type strings exist.
- No dimension algebra or unit compatibility exists.

### Why This Still Belongs In `v1`

The spec treats basic dimensions and units as part of the intended `v1` safety surface, and this is a high-value compiler check for relatively low implementation cost.

### Planned Implementation

- Start with a small custom dimension representation on quantities and expressions.
- Implement compatibility checking for:
  - equality
  - addition/subtraction
  - multiplication/division composition
- Do not build a broad unit ecosystem yet.
- Keep conversions minimal or defer them if they complicate the first pass.

### Acceptance Criteria

- Obvious mismatches fail at compile time.
- Correct arithmetic composition produces expected derived dimensions.
- Diagnostics mention both quantities and dimensions involved.

## 5. Real Equality Core / E-Graph Decision

### Current State

The current implementation does **not** use a true e-graph. It uses:

- equality-oriented lowering
- directional candidate generation
- greedy provider-aware planning

This is a deliberate pragmatic shortcut that works for the current arithmetic MVP.

### Why This Is Still The Biggest Architectural Gap

The spec says `v1` should use an e-graph-backed equality core.

The current planner will eventually struggle with:

- equivalence that requires algebraic rewrites
- indirect constraint discovery
- richer extraction over multiple equivalent forms

### Recommendation

Do **not** replace the planner with an e-graph before the training demo.

Instead:

- run the training proof first
- then do a focused spike on `egg`
- decide whether e-graph replacement is required for honest `v1` completion, or whether this should be recorded as the first major `v2` refactor

### Decision Gate

After the training demo, answer:

- Does the current planner block the first real plant model?
- Are there concrete algebraic cases we need right now that the planner cannot express?
- Can a small `egg`-backed equality core slot into the current pipeline without destabilizing the rest?

If the answer is yes, implement the e-graph before calling `v1` complete.

If the answer is no, document the divergence explicitly and treat the e-graph as the first major post-`v1` internal rewrite.

## 6. Compile Modes

### Current State

`Simulate`, `Fit`, and `Train` exist in the compile spec, but currently do not change emitted behavior meaningfully.

### Planned Implementation

- Keep the semantics lightweight:
  - `simulate`: no observation losses required
  - `fit`: observations expected, learned slots optional
  - `train`: learned slots allowed and loss wiring enabled
- This should mostly affect validation and emitted convenience surfaces, not symbolic planning.

### Acceptance Criteria

- Mode-specific validation exists.
- Emitted artifacts reflect the intended mode clearly enough for users.

## 7. Parser And Constraint Representation Debt

### Current State

- Parsing is hand-rolled.
- Constraint parsing for bounds is currently string-driven in places.

### Recommendation

This is real technical debt, but not the next thing to fix.

For true `v1` completion:

- remove brittle string-based constraint parsing from emission paths
- ensure constraints are represented semantically before emission

Do **not** broaden the language substantially before that cleanup.

## 8. Python UX Beyond The Current Builder API

### Current State

The package is already usable, but still minimal.

### `v1`-Relevant Improvements

- stable file-backed compile spec flow
- richer typed summaries
- easier artifact write/load helpers
- clearer diagnostics in Python

### Not Needed Yet

- broader documentation site
- publishing workflow
- notebook integrations
- packaging polish beyond what supports iteration

## Proposed Milestones

### Milestone A: Training Proof

- Add synthetic controller
- Add training script
- Prove behavioral recovery

### Milestone B: Compiler Semantics Parity

- Add simple soft penalties
- Add dimension checks
- Add introspection APIs
- Add mode-aware validation

### Milestone C: Equality-Core Decision

- Run focused `egg` spike
- Decide whether to retrofit true e-graph support before calling `v1` complete

### Milestone D: Close Remaining Spec Gaps

- Replace brittle bound parsing
- Finish any minimal missing diagnostics/introspection
- Reconcile implementation notes against `spec.md`

## What I Would Not Do Yet

- broaden the surface language
- add more mathematical primitives
- add more backends
- build a visualization system
- spend time on publication packaging or external docs

The current implementation is finally good enough that the bottleneck is no longer missing scaffolding. The bottleneck is proving the scientific workflow and then closing the remaining narrow semantic gaps deliberately.

## Practical Recommendation

If choosing only one next step, choose this:

- build the end-to-end synthetic training demo first

If that demo succeeds, continue immediately with:

- simple soft penalties
- dimension checks
- introspection surface

Then make the real decision about e-graph replacement using concrete evidence from the first real model, not speculation.
