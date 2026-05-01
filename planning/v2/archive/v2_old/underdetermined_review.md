# Design Review: Underdetermined System Handling

**Date**: 2026-04-13
**Models**: Codex (GPT 5.4, xhigh), Gemini, Claude Opus 4.6
**Scope**: Full v2 spec + proposed changes for partial plans, multi-binding
compilation, interactive constraint propagation, and proof-like semantics.
**Context**: ~40k tokens (spec + soul + mocks + project note + proposed changes)

Two rounds of cross-model deliberation followed the initial parallel reviews.
Gemini and Claude findings were synthesized and sent to Codex for multi-turn
discussion. This document captures the merged consensus.

---

## The Big Conceptual Shift

All three models converge on a single core claim:

> In train and prove modes, Myco cannot remain only a forward execution planner.
> It has to become a compiler for **residual graphs** over explicit latent
> variables and horizon-wide temporal factors.

The current spec's mental model is: planner -> execution plan -> emit code.
The right mental model is: planner -> **residual graph** (which contains
executable subgraphs + residual equations + latent variables) -> emit code that
includes forward computation + residual losses + solver blocks.

The residual graph IS the partial plan, but with proper formal structure. It is
also the "knowledge envelope" — the complete picture of what the compiler knows
about the system given the current bindings.

---

## Major Findings

### 1. [CONSENSUS] Residual graph as the core semantic object

**All three models agree.** The planner should produce a deterministic factor
graph (residual graph), not just an execution plan.

Components of the residual graph:
- **Variable nodes**: quantities that remain free after bindings, including
  explicit latents and time-indexed state variables
- **Derived nodes**: quantities that can be eliminated explicitly from the
  variables (these become forward computation)
- **Residual factors**: equalities from relations and temporals, inequality
  and domain constraints, observation terms, explicit discrepancy relations
- **Slot nodes**: explicit numeric functions from inputs and parameters to
  provided outputs
- **Metadata**: bounds, monotonicity facts, differentiability class,
  provenance

The planner produces it by: flattening as usual -> applying bindings and
marking latent-owned variables -> decomposing the graph into components ->
eliminating what is acyclic or square-solvable -> leaving the rest as residual
blocks.

The emitter consumes it by generating:
- A forward evaluator for eliminated subgraphs
- A residual function `r(theta, z, data)` for the remaining blocks
- Observation losses
- Admissibility projections or barrier/penalty terms where required
- `custom_root` blocks for square implicit subcomponents

**Key insight**: if every free variable has an owner (slot, learned trajectory,
assumed value), the emitted JAX graph is fully closed even though the semantics
came from a residual graph. The residual graph is the semantic source object;
emitted JAX is what you get after eliminating everything that's square-solvable.


### 2. [CONSENSUS] Four types of coupled components

The current spec conflates four distinct situations under "overdetermination."
All three reviewers agree the spec should distinguish:

- **Computational redundancy**: the same underlying system admits multiple
  algebraic evaluators. These are different computation paths to the same
  answer. Pick a canonical evaluator. Users usually shouldn't care.

- **Square implicit component** (n_eq = n_unknown): like Farquhar A-Ci, where
  assimilation and c_i are jointly determined by two equations. This is an SCC,
  not overdetermination. The planner should count equations vs. unknowns, not
  paths.

- **Underdetermined residual** (n_eq < n_unknown): needs explicit latent
  owners (learned trajectories, learned constants, slot outputs) or additional
  bindings.

- **Overconstrained residual** (n_eq > n_unknown): simultaneous world-claims
  that should remain residual constraints unless the user explicitly introduces
  a closure or discrepancy policy.

**What this means for the current overdetermination design**: The
`myco::resolution` strategies (weighted_average, soft_select, etc.) are
legitimate but MISFRAMED. They are not "path selection strategies" — they are
**closure policies** that explicitly relax an overconstrained system into a
single forward value. This changes the science of the executed artifact and
must be opt-in, surfaced in the plan, and clearly named.

Codex recommends renaming `myco::resolution` to `myco::closure` or
`myco::discrepancy` to make the semantics honest. The distinction:
- If the blend is part of the world claim (should itself be learned/shared),
  model it in `.myco` as discrepancy/fusion structure
- If it's an experiment-side approximation for forward computation,
  workflow/compiler config is fine

Either way, the plan/report should state that the original equations were
relaxed.


### 3. [CONSENSUS] Every free variable needs an explicit owner

Partial plans are the right analysis/diagnostic object. But the emitted code
must be fully closed — JAX requires a concrete computation graph. The rule is
not "no partial plans in JAX" but "no unowned degrees of freedom in emitted
code."

The compiler should:
1. Build the residual graph (the partial plan)
2. Check that every free variable has an explicit owner
3. If unowned variables remain, error with the Resolution Frontier: list the
   unresolved quantities, what they depend on, and what bindings or latent
   declarations would close the system
4. If all variables are owned, emit closed JAX code

This follows the no-trust principle: the compiler won't silently invent latent
owners. The user must explicitly declare what is learned, assumed, or provided.

**Gemini adds**: frame partial plans strictly as an interactive/diagnostic
concept — what you interrogate to figure out what data or learned trajectories
your experiment still needs. The compiler is a co-pilot, not an adversary.

**Claude adds**: require explicit opt-in for intentionally partial plans
(e.g., `mode="train"` with `learn_slot` implies the user expects unknowns).
Guard against "accidentally partial" plans violating no-trust.


### 4. [CONSENSUS] Knowledge envelope as orthogonal fields

The CONCRETE/SYMBOLIC/BOUNDED/CONDITIONAL taxonomy conflates two axes. All
three reviewers converge on a richer per-quantity representation:

**Codex's formulation** (most complete):
- `realization`: explicit(expr) | implicit(residual_block) | opaque(provider)
- `free_variables`: the latent or still-unbound symbols it depends on
- `bounds`: current abstract value, initially intervals but not limited to
  intervals
- `obligations`: residual equations and inequality/domain constraints still
  to be satisfied
- `resolver_sets`: minimal additional bindings or latent-owner declarations
  that would make the quantity explicit
- `provenance`: which assumptions, equations, properties, and analyzers
  justified the envelope

The familiar labels become derived views:
- `concrete` = explicit realization with no free variables
- `symbolic` = explicit with free variables, or implicit with a residual block
- `bounded` = bounds are informative (tighter than type bounds)
- `conditional` = resolver_sets is non-empty


### 5. [CONSENSUS] Two-level slot interface for shared controllers

Claude and Codex agree (Gemini didn't address this directly):

- **Structural interface**: derived from model structure alone, invariant
  across studies. This is what a shared learned controller architecture needs —
  it always receives the same named inputs.
- **Numeric feed**: per-study, the actual values. Some structural inputs will
  be concretely available, some supplied by explicit latents, some may require
  masking.

`inputs = [*]` should resolve based on the model graph (what quantities COULD
be computable under some set of bindings), not from any individual study's
bindings. This preserves a fixed architecture while allowing per-study
variation in what is known.

Controllers should NOT silently consume symbolic intervals as ordinary scalars.
If Myco ever wants controllers that consume uncertainty envelopes, that should
be a different slot kind.


### 6. [CONSENSUS] Interval propagation is the floor, not the ceiling

All three flag this. Naive interval arithmetic through nonlinear SCCs (like
the hydraulic flow-pressure-conductance cycle) rapidly widens to type bounds
and provides no useful signal.

**Codex elaborates on what "better" looks like:**
- **Monotonicity-aware propagation**: track per-argument monotone direction.
  Bounds propagate by endpoint evaluation instead of naive interval arithmetic.
  Example: vc(psi).plc is monotone increasing in psi, so a bound on psi gives
  a tight bound on conductance. Invertibility on the monotone segment then
  contracts psi from a conductance bound.
- **Contractor passes**: local domain-pruning from interval/constraint
  programming. Forward and backward passes that shrink participating domains.
  For SCCs, use block operators (interval Newton, Krawczyk-like contraction).
- **E-graph as rewrite substrate**: not the abstract domain itself, but
  exposes algebraically equivalent forms better for monotone analysis.
- **Stacked analyzers**: intervals mandatory, then monotonicity, then symbolic
  fragments, then stronger contraction, on demand.

**But**: compile-time bounds are for initialization, stability, and simple
proofs — not the primary training signal. The real training signal comes from
runtime constraint enforcement: the SCC solver must find a solution, and that
solution must satisfy all losses.


### 7. [CONSENSUS] Temporal: semantics vs. execution strategy

**Refined consensus after deliberation:**

- For **train mode**: forward rollout + BPTT is sufficient AFTER the user has
  closed all free variables. Learned trajectories as splines allow later
  observations to constrain earlier knots through ordinary backprop.
- For **prove/analyze modes**: horizon-wide factor semantics are
  indispensable. "What do we know at t=7 given observations at t=0 and t=20?"
  requires propagation through temporal equations across the full horizon.
- The **residual graph** is the right semantic model for both: temporal
  equations lower to horizon-wide factors, and rollout + BPTT is the execution
  strategy when the graph is closable.

Gemini and Claude initially said "no special temporal handling needed — BPTT
handles it." Codex refined this: BPTT is an execution strategy, not a semantic
model. The semantic model must support horizon-wide reasoning even if the
execution strategy is forward rollout.


### 8. [CONSENSUS] Multi-binding is workflow-layer

All three agree. The core compilation unit is one model + one binding context.
Multi-study coordination lives in the workflow layer. The existing
Study/Experiment terminology (section 17) should be preserved — Study is the
outer container, Experiment is one binding set.

The only genuinely new compiler concept is the study-invariant slot interface
(finding 5). Everything else — per-study planning, shared parameters, gradient
accumulation, study weighting — is workflow-layer orchestration.


### 9. [GEMINI, CODEX AGREE] Domain-guarding projections, not just penalties

Soft penalties alone let the controller produce domain-violating values
(negative stomatal conductance) that flow into log/sqrt and produce NaN,
crashing training before the penalty can correct.

The right distinction:
- **Admissibility constraints** (guard definedness of downstream operations):
  enforce via differentiable projection or reparameterization at slot
  boundaries (e.g., softplus for positivity)
- **Scientific feasibility constraints** (don't guard definedness): may remain
  residuals or penalties depending on mode

The operation algebra metadata should dictate which constraints are
admissibility guards.


---

## Minor Findings

### 10. Learned trajectories + temporal equations = PINN pattern
[Claude, Codex agree] If a user declares `learn_trajectory("soil_water")` for
a quantity with a temporal equation, the trajectory provides values and the
temporal equation becomes a physics residual loss. This is the right PINN-like
semantics and falls out naturally from the residual graph view.

### 11. Adaptive quadrature breaks gradients
[Gemini, Codex agree] Adaptive quadrature with discrete point-count changes
creates discontinuous loss. Train-mode differentiable integrals should use
fixed-shape quadrature. Adaptive belongs in simulate/analyze only.

### 12. Study weighting must be specifiable
[Claude, Codex agree] Different studies have different loss magnitudes. Without
configurable study weighting, one data-rich study dominates gradients and
prevents the shared controller from generalizing. This belongs in the workflow
layer: per-study reducers, per-loss-family weighting, normalization rules.

### 13. deriv through SCCs needs implicit function theorem
[Claude, Codex agree] `deriv(assimilation, g_s)` goes through the A-Ci SCC.
The spec's "symbolic chain rule on the expression graph" is incomplete for
implicit systems. Spec should state: deriv through square implicit SCCs is
supported via the implicit function theorem. Through underdetermined residual
blocks, deriv is undefined unless the block is closed.

### 14. Resolution frontier is not identifiability
[Codex] "Binding X unlocks the most computation" is not the same as "binding
X constrains shared controller parameters the most." The resolution frontier
is a planning heuristic, not an information-gain claim.

### 15. Long rollout stability
[Claude] Training through long rollouts (growing seasons, multi-year) via
BPTT is notoriously unstable. The JAX emitter should support gradient
checkpointing (jax.checkpoint on the scan function) and the spec should
acknowledge truncated BPTT as an option.

### 16. prove() should have explicit provenance and conditionality
[Codex] Results should be `proven | disproven | indeterminate`, plus the
exact assumptions used. Proofs stop at slot boundaries except for declared
output constraints. Proof-like statements will realistically be limited to
type constraints, interval arithmetic through linear chains, and dimensional
consistency for nonlinear systems.

### 17. integrate inside SCCs needs specification
[Claude] If an integral's value feeds back into an SCC, the numerical
quadrature is nested inside Newton-Raphson. This should be specified.

### 18. Mock unit issues validate the spec's type system
[Claude] Multiple mock issues (PositiveScalar using ratio for dimensional
quantities, bare numeric literals, peaked Arrhenius sv units) demonstrate the
unit system would catch real errors. Not spec issues, but validates section 4.


---

## Concrete End-to-End Trace (from Codex deliberation)

**Study A** (observes transpiration + soil moisture, not NSC/growth):
- Planner builds residual graph, eliminates acyclic + square-solvable
- Per timestep: hydraulic/photosynthesis block is **square implicit** (given
  state + forcings + controller output, solve for transpiration, water
  potentials, leaf temp, internal CO2)
- Temporal equations for soil water, NSC, cavitation memory, growth are
  explicit rollout updates (free initials/constants in per-study latents z_A)
- Residual factors: observation residuals on transpiration + soil moisture,
  admissibility/domain factors. No observation factors on NSC or growth.

**Study B** (observes NSC + diameter growth, not transpiration/soil moisture):
- Same model, same shared controller theta, different latents z_B
- Same per-step square hydraulic/photosynthesis solve
- Same temporal rollout structure
- Residual factors: observation residuals on NSC + diameter growth,
  admissibility factors. No observation factors on transpiration.

**Emitted JAX**: each study gets a concrete artifact (all free variables have
owners: shared theta + per-study z). Contains rollout function, embedded
solver calls for square per-step blocks, observation losses, constraint terms.

**Gradient flow**: Study A constrains the controller through water-use
behavior (transpiration losses -> hydraulic solve -> controller outputs).
Study B constrains through carbon-growth consequences (NSC/growth losses ->
carbon balance -> photosynthesis/hydraulics coupling -> controller outputs).
Gradients add on shared theta. This is the identifiability story.

---

## Summary

The direction is right. The proposed changes for underdetermined systems are
directionally correct but need one conceptual pivot to be sound:

**The planner should produce a residual graph, not just an execution plan.**

The residual graph cleanly separates:
- What can be eliminated into forward computation
- What requires a numerical solver (square implicit blocks)
- What requires latent owners (underdetermined residuals)
- What requires closure policies (overconstrained residuals)
- What the constraint system can prove (bounds, monotonicity, provenance)

From this single semantic object, everything else follows: interactive
knowledge queries, proof-like reasoning, training loss emission, and the
distinction between analysis (partial, diagnostic) and execution (closed,
concrete).

The current spec's overdetermination design should be reframed: `myco::resolution`
becomes `myco::closure` (or `myco::discrepancy`), and the four-way component
taxonomy (redundancy, square, underdetermined, overconstrained) replaces the
current binary "determined vs. overdetermined."
