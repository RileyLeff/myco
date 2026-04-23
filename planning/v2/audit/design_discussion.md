# Design Discussion — 2026-04-23

Durable record of a design conversation that started as Phase 1 step 1
(verifying Codex contradictions from
`four_reviewer_synthesis_2026-04-23.md`) and escalated into a workflow-
interface redesign plus a directionality/time generalization. Nothing
locked. Riley wanted to sketch §9 and §24 under the new framing before
deciding whether to commit.

## Starting context

Four-reviewer synthesis surfaced three concrete textual contradictions
for verification first (Codex's A1-A3). Walkthrough plan was:

1. Verify three Codex contradictions
2. Accept 11 consensus items
3. Decide 6 strong-majority items
4. Resolve 5 splits
5. Triage Opus's 7 risks
6. Kill Pro's non-decision D1

Riley agreed to go in order. Started with A1 (`assume_initial`
orphaned between §9.3 and §24).

## A1 verification: confirmed

- §9.3 line 1307 defines `assume_initial(path, value)` as a shipping
  workflow verb with specific semantics, cross-referencing §24 for
  detail.
- §24 preamble (line 3867-3868) canonical eight verbs:
  `assume_constant, assume_series, learn_constant, learn_initial,
  learn_trajectory, bind_controller, bind_topology, observe`.
  `assume_initial` is absent.
- §24.4 future candidates: `bind_known_constants, bind_parameters,
  assume_prior`. `assume_initial` is absent here too.

Initial resolution options considered:

- (a) Promote to ninth verb (breaks "eight" references throughout)
- (b) Retire, use `assume_constant("path.initial", value)` with
  `.initial` path convention
- (c) Defer to §24.4 future

Claude leaned (b). Riley flagged a deeper concern.

## Riley's pivot: does Myco need to know about learning at all?

Riley: "from the e-graph's perspective, a value is a value whether
it's supplied by a random number someone made up, a sophisticated
neural net, or some empirical curve from someone's dataset. I think
we should tread carefully around workflow design."

The thesis Riley proposed: the `assume_*` vs `learn_*` distinction
bakes experimental intent into the verb surface, which violates
soul.md principle 2 (the workflow is a separate concern; the same
world supports many experiments). If "trainable vs fixed" is a
property of the value source and the compile mode, not of the binding
action, the current verb taxonomy is confused.

## Proposed three-verb design

Three verbs replace the eight:

```python
experiment.bind(path, source)
experiment.bind_topology(path, geometry)
experiment.observe(path, data)
```

Where `source` is a Python object of a source-type. Candidate source
types:

- `Constant(value)` — fixed scalar/vector literal
- `Series(data)` — fixed time-indexed data
- `Controller(fn, reads, writes, framework, differentiable)` — typed
  Python callable
- `Trainable(prior, init=None)` — value that participates in training
  when compiled in gradient-train or PPL mode; uses trained
  value/posterior in inference mode
- `Prior(distribution)` — distributional value for Bayesian inference
  without gradient training
- `GPPrior(kernel, ...)` — structured trainable where hyperparameters
  themselves are trainable sources

Under this scheme:

- `assume_constant` collapses to `bind(path, Constant(v))`
- `learn_constant` collapses to `bind(path, Trainable(prior))`
- `learn_initial` collapses to `bind("path.initial",
  Trainable(prior))`
- `learn_trajectory` collapses to `bind(path, Trainable(...))` with
  an appropriate source
- `bind_controller` collapses to `bind(path, Controller(fn, ...))`
- `assume_initial` disappears (no orphan, no resolution needed)
- `assume_prior` disappears (use `bind(path, Prior(D))`)

The `bind_controller` arity fight (§24 C1 in the four-reviewer
synthesis) becomes moot: contracts are fields on the Controller
source, not verb arguments.

## Source-type shape

Minimum contract:

```python
class Source:
    reads: list[Path]        # empty for pure value sources
    writes: list[Path]       # always non-empty
    capabilities: set[str]   # 'differentiable', 'sampleable', ...
    framework: str           # 'pure', 'pytorch', 'jax', 'numpyro'

    def invoke(self, *read_values) -> tuple: ...
```

The compiler uses `reads`/`writes` at compile time to build the
dependency graph. At emission time, generates marshalling code that
extracts `reads` values from plan state, calls `invoke`, writes
return values to `writes` paths.

## Stress test cases (14)

Six from Riley:

1. Python random number → stomatal openness: `bind(path,
   Constant(sampled_value))`. Trivial.
2. PyTorch NN, inference-only, reads graph subset, writes one value:
   `bind(path, Controller(torch_model.forward, reads=[...],
   writes=[...], framework='pytorch', differentiable=False))`.
3. Same but JAX: same source-type, framework tag differs. Myco
   doesn't care; §31 backend emitter does.
4. Distribution on stomatal openness: `bind(path, Prior(Normal(...)))`
   attaches Layer-2 distributional envelope.
5. Empirical model imported:
   - (a) as .myco code: use module import, bind parameters as normal
   - (b) as Python callable: use `Controller` source
6. GP over time, NumPyro PPL, observations attached:
   `bind(path, GPPrior(kernel=RBF(length_scale=Trainable(...),
   ...)))` + `observe(...)` + `compile(mode='ppl',
   backend='numpyro')`.

Eight from Claude:

7. Gradient flow through dynamic topology (differentiable controller
   driving `bind_topology` event). Tests Layer-1/Layer-3 interface.
8. Per-instance controllers (different trained NNs per vessel in a
   collection). Tests indexed-path binding.
9. Mutually coupled external callables (A reads B's output, B reads
   A's). Forces SCC crossing external compute.
10. Observation of a derived quantity. Gradient flows backward
    through `.myco` equations into upstream sources. Tests end-to-end
    differentiability.
11. Mixed modes across SCCs (gradient-train + PPL in one run). Opus
    flagged this as a risk; forces multi-backend story or
    single-mode-per-run commitment.
12. Serialization: trained controller saved to disk, loaded in later
    workflow. Tests cross-study reuse (§23.3).
13. Streaming / incremental binding: live sensor data re-bound each
    timestep without full recompile.
14. External solver as source (Pareto front finder, MIP, constraint
    solver). Same `Controller` abstraction, no neural net.

Case 6 and case 11 together surface a sharp point: **compile mode is
not binary.** At least three modes:

- Inference mode: fixed values, no learning, deterministic plan
- Gradient-training mode: backprop via JAX / PyTorch / burn against
  loss
- PPL mode: MCMC / VI against a joint distribution via NumPyro /
  Stan / Pyro

A `Trainable` source means "gradient-descend me" in gradient mode and
"sample my posterior" in PPL mode. Source-types declare which modes
they're compatible with; compile mode selects machinery.

## Graph I/O API — resolved as "no runtime graph API"

Riley raised the question: what's the API for getting info into and
out of the e-graph so external compute can talk to it?

Resolution: two principles.

**Principle A: the e-graph is a compile-time object.** External
compute never talks to the e-graph directly. At compile time, the
value-source's `reads`/`writes` path lists are the entire interface.
The compiler uses them for dependency-graph construction, SCC
scheduling, and AD wiring.

**Principle B: at runtime, marshalling is auto-generated.** The plan
has a state representation (per-path values, possibly per-time-step).
The compiler emits glue for each controller call: extract `reads`
from state → call `invoke` positionally → write return values to
`writes`. External callables see typed values in and typed values
out. No graph handle. No runtime API. No side channel.

Consequence: external callables are **completely agnostic** to Myco.
A PyTorch or JAX model written for any other pipeline plugs in as-is;
the `Controller` source is a thin adapter declaring contracts.

## Directionality discussion

Riley asked whether Myco needs a directionality concept, motivated by
the "sniper delete" thought experiment (take a fully-solved graph,
delete some values, solve the holes — which direction are you going?)
and the two-point boundary-value-problem research case (collect data
at t1 and t2, run forward from t1 and backward from t2, check
consistency).

Resolution: Myco does not need a directionality concept.

Three separable notions of "direction" got conflated:

1. **Structural directionality** — some operations are irreducibly
   one-way: NN forward pass, `abs`, `floor`, `argmax`, stochastic
   draws, non-injective functions. This is a **local property** of a
   specific operation.

2. **Temporal directionality** — time feels directional in physical
   reality but is not directional in a Myco model. `d(x) = f(x)` is a
   constraint between states at different time indices; algorithmically
   integrable either way.

3. **Computational directionality** — the compiler's choice of
   traversal order through the constraint graph during solve. Pure
   scheduling concern.

Only (1) is irreducibly directional, and it's local. (2) and (3) are
emergent from what the workflow binds.

Equations are fundamentally undirected. `y = f(x)` and `x = f^(-1)(y)`
are the same statement; the e-graph already treats them symmetrically
via E-group inverse rewrites. The union-find structure is intrinsically
symmetric.

Sniper-delete demo under this framing: take a fully-bound model,
delete some bindings, what remains is a partial constraint graph. The
compiler identifies:

- **Determined holes** — reachable from knowns via algebraic
  inversion or propagation
- **Jointly-determined SCCs** — coupled unknowns needing simultaneous
  solve
- **Underdetermined holes** — not reachable from knowns; error
- **Overdetermined subgraphs** — check consistency within tolerance
  or error

No direction involved at the semantic layer. The direction is a
solver implementation detail.

## Time-as-sequence generalization

A Myco model describes relations among indexed quantities. Index
families:

- **Scalar** — no index
- **Temporal** — continuous index with ordering (`d(x)` lives here)
- **Stepped** — discrete index with ordering (`step(x)` lives here)
- **Iterative** — discrete index without a priori ordering
  (fixed-point iteration, Gauss-Seidel, etc.)
- **Spatial** — coordinate index (grid, mesh, etc.)
- **Collection** — entity index (per-vessel, per-tree)
- **Event** — occurrence index

The compiler treats all of these uniformly at the constraint-graph
layer. Solver dispatch picks numerical machinery based on index
structure + what's bound:

- Temporal with known initial → ODE forward integrator
- Temporal with known terminal → ODE backward integrator
- Temporal with known both ends → boundary-value problem
  (shooting, collocation)
- Temporal with known at scattered times → BVP with multiple
  constraints (the two-point forward-backward agreement case)
- Stepped with known initial → discrete iterator
- Stepped with known terminal → reverse iterator
- Iterative with initial guess → fixed-point solver
- Pure algebraic (no index) → linear or nonlinear root solver
- Overdetermined algebraic → least-squares or consistency check

The user never picks the solver. They write relations and bind
values. The compiler classifies and dispatches.

## What stays directional

Only external callables. A `Controller` source advertises
directionality as a capability:

- `forward_only` — standard NN, one-way
- `invertible` — normalizing flow, explicit inverse
- `sampleable_input` — supports PPL inversion via posterior over
  inputs

Everything else (time, iteration, space, events) is index-shaped but
not directional in any sense the language needs to surface.

## Implications for spec prose (not yet applied)

**§9 "State and Time"** would become "State and Sequences." `d` and
`step` remain, but framed as sequence-constraint relations rather
than temporal operators. The `.initial` path convention (which
resolved A1) applies uniformly across sequence types.

**§24 workflow verbs** collapses to three verbs. "Eight verbs"
framing disappears. Source-type taxonomy replaces the verb-by-verb
analysis. `bind_controller` arity question (§24 C1) becomes moot.
`assume_initial` and `assume_prior` contradictions disappear.

**§20 SCC decomposition** subsumes algebraic and temporal SCCs under
one machinery. A temporal SCC is a coupled system across time
indices; an algebraic SCC is coupled at one index.

**§21 lowering** picks solver dispatch based on index structure plus
bindings. The existing four-way SCC taxonomy (static / dynamic /
stochastic / training) remains, now refined by which indices and
which solve direction are active.

## Cascade into four-reviewer synthesis items

If the three-verb redesign is adopted, several review items resolve
automatically:

- A1 `assume_initial`: disappears
- A2 `assume_prior`: disappears
- §24 C1 `bind_controller` arity: moot (contracts are source fields)
- App A H8 `observe` dual status: `observe` stays a verb; the
  epistemic/aleatoric concern is addressed by source types, not by
  the verb's placement
- §25 H10 long-rollout gradient regime: lives in compile-mode config
- §22 H3 `with_assumption`: becomes "recompile with different
  bindings," a workflow convenience not a plan-patching API

Several others become clearer:

- §20 C1 SCC taxonomy: reinforced (four-way execution roles still
  right; index-structure refines within classes)
- §26 C1 precision downcast: unchanged
- §31 C1 AD ownership: unchanged (hybrid)
- §28 H1 compact support: unchanged (capability contract)

## Where we stopped

Riley asked whether this framing matches what he was envisioning.
Claude offered to draft §24 under the three-verb design and stress-
test against all 14 cases before committing. Riley instead asked
about PyTorch/JAX directionality (did it in depth); then pivoted to
"does Myco need directionality at all" (resolved as no); then
requested this writeup.

Not yet decided:

- Commit to three-verb redesign, or keep eight-verb surface and
  resolve A1/A2 as textual fixes within the current shape
- Commit to time-as-sequence generalization, or keep temporal
  framing as primary
- If committing to three-verb: draft §24 and §9 under the new
  framing and verify against the 14 cases before touching spec_new

Proposed next step: draft a §24 replacement and §9 sketch, run them
against cases 1-14, surface any edge cases where the abstraction
leaks, then decide whether to commit.

## Open sub-questions surfaced but not resolved

- **Compile mode granularity.** Can compile mode be per-SCC (some
  parts trained by gradient, others by PPL, in one run)? Opus flagged
  this in the four-reviewer synthesis as risk C4. The three-verb
  design surfaces it again: a `Trainable` source at one SCC and a
  `GPPrior` source at another may want different compile handling.
- **Case 9 (mutually coupled external callables).** Does Myco permit
  SCCs spanning external compute, or does it require the author to
  break the cycle? Either answer has ergonomic consequences.
- **Case 13 (streaming binding).** Is re-binding without full
  recompilation a v2.1 commitment or post-v2.1?
- **Path convention for sequence indices.** If time generalizes, how
  are initial / terminal / scattered constraints spelled in paths?
  (`path.initial`, `path.terminal`, `path.at(t)`?)
- **Directionality capabilities and PPL dispatch.** A
  `sampleable_input` controller in PPL mode — does the PPL backend
  automatically compose with Myco's symbolic inversion rewrites, or
  does it need explicit coordination?
