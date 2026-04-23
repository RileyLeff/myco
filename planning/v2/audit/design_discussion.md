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

## Partial evaluation and the inspect API

Riley raised a capability question: if a finished, well-constrained
run has a handful of values sniper-deleted, leaving those nodes
symbolic, and the workflow then asks "given everything else, what do
we know about missing value `x`", the user should get back a
symbolic expression with as much information as possible already
propagated through it. Even when `x` is underdetermined, the answer
should carry every constraint the graph can impose on it.

Claude's response: this capability is already implicit in the
existing e-graph machinery. §19's A-group and E-group rewrites
perform symbolic simplification and algebraic rearrangement during
saturation. Extraction can return the best form of any node
regardless of whether it terminates in a ground value. The missing
piece is a user-facing API.

### Four worked examples

1. **ODE with unbound initial.** Everything else resolved; initial
   condition `y0` left symbolic. Inspect of `y.terminal` returns an
   expression in `y0` plus absorbed constants. All parameter values
   are folded in; only the one genuinely free variable remains.

2. **Algebraic inversion.** `z = f(x, y)` bound, `y` bound, `x`
   sniped. Inspect of `x` returns the inverted expression if `f` is
   invertible in `x`, otherwise returns the unsatisfied residual
   `f(x, y) - z = 0` as the best available statement.

3. **Mutual expression.** Two variables `a, b` coupled by an
   equation, both unbound. Inspect of `a` returns `a` expressed in
   terms of `b` (and vice versa) — the graph's best one-equation
   statement about either.

4. **Deeply partial.** Three linked equations, one variable unbound.
   Propagation collapses the chain so the inspect result surfaces
   only the genuinely required free symbols, not every intermediate
   node.

### Proposed API shape

```python
result = experiment.inspect("path.to.node")
# Returns a structured object:
#   result.expression        — best symbolic form post-saturation
#   result.free_variables    — set of paths still unresolved
#   result.status            — ground | symbolic | overdetermined | inconsistent
#   result.value             — optional numeric reduction if ground
#   result.depends_on        — paths whose bindings fed the result
#   result.reduction_trace   — optional rewrite history (explain mode)
```

CLI equivalent: `hypha explain path.to.node` rendering the
expression as a tree or flat form. Rendering knobs likely needed:

- Depth limit for expression printing (collapse deep subtrees to
  named intermediates on request)
- Free-variable ranking so the most constraint-heavy symbols appear
  first
- Toggle between surface-syntax rendering and internal IR form

### Spec implications

- §22 gains an `inspect` verb or experiment-object method, with its
  return type documented as a structured symbolic result
- §23 (Python boundary) needs the object to marshal cleanly, with
  the `expression` field exposed as a printable + walkable structure
- §19 may need a normative statement that extraction must terminate
  even when free variables remain; current prose assumes ground
  extraction

### Tie to the directionality discussion

Inspect is direction-free by construction. It queries the graph's
current best knowledge of any node at any time. That is precisely
the capability Riley wanted when he rejected solve-direction as a
first-class concept: the user asks "what does the graph know", and
the answer is whatever the rewrites can produce.

## `myco.prove()` — symbolic truth-claim verification

Riley asked for a companion entry point: given a `.myco` model plus
a workflow, can we ask "is this expression true" and optionally get
a show-your-work trace. Same underlying machinery as inspect, but
with a different surface question.

### Shape

```python
result = myco.prove("x + y > 0", experiment=exp, show_work=True)
# Returns:
#   result.verdict     — proven | refuted | undetermined | contingent
#   result.counterexample — if refuted, a witness binding
#   result.conditions  — if contingent, the assumptions under which
#                        the claim holds (e.g., "if z > 0")
#   result.trace       — ordered list of rewrite steps that led to
#                        the verdict (show_work=True)
```

### Example targets

- `prove("x + y > 0")` — standard algebraic claim
- `prove("photosynthesis monotonic in light")` — expressed against
  a user-exposed predicate library; backend checks that
  `d(photo)/d(light) >= 0` is derivable
- `prove("conservation of C holds at every step")` — lifts the
  runtime conservation-check machinery into a compile-time
  statement, useful for documentation or spec-level guarantees
- `prove("y.terminal is bounded")` — claims about asymptotic
  behavior, answerable when the graph has enough structure

### What machinery it reuses

- E-graph saturation over A/E/CC rewrite groups decides equality
  classes and normalizations
- Symbolic differentiation (already needed for gradient training)
  handles monotonicity / sign claims
- Envelope metadata from layer 2 supplies interval and
  positivity witnesses when exact equality is not derivable
- Extraction produces the minimal counterexample when the claim is
  refuted

### What it does not guarantee

`prove` is not a general theorem prover. It succeeds on claims that
Myco's canonical rewrites can reach, and returns `undetermined` on
anything else. That status is informative: it tells the user "the
graph's current rules cannot settle this", which is a different
answer from "false". Users can extend reach by adding registered
lemmas (future direction) or by strengthening bindings.

### Spec implications

- New top-level workflow verb or library function (name TBD; leaning
  library function exposed as `myco.prove` or
  `experiment.prove`)
- Needs a canonical predicate vocabulary: what exactly does
  "monotonic in x" compile to? This is a surface-syntax decision,
  maybe a small predicate DSL
- `undetermined` verdict must be first-class; the API cannot coerce
  it into a boolean

### Why pair inspect and prove

Both queries are views over the same saturated e-graph. Inspect asks
"what expression represents this node"; prove asks "does this
expression evaluate to true". The implementation is one extraction
pass plus a decision procedure over the extracted form. Shipping
them together avoids the trap of building two separate query
subsystems.

## Overdetermined handler API design

Riley flagged that the overdetermined-constraint surface will get
messy in complicated cases if we do not think about it up front.
Agreed. Four distinct situations all fall under the same umbrella,
and they need different API treatment.

### The four situations (restated)

1. **Exactly redundant.** Two equations express the same constraint
   post-simplification. Detectable at compile time by the e-graph.
   Harmless, but worth reporting.

2. **Approximately consistent.** Residual below user-configured
   tolerance. Runtime observation. Not an error; the user may want
   the residual surfaced as a diagnostic.

3. **Provably inconsistent.** Residual provably exceeds any
   reasonable tolerance, or the symbolic form reduces to `0 = c`
   for nonzero `c`. Compile-time error when provable, runtime error
   otherwise.

4. **Two-point BVP-style.** The system is well-posed but requires a
   solver that treats the endpoint constraints symmetrically. Not
   an error at all; a dispatch signal to §20's solver-class
   machinery.

The design risk: one API surface for all four, and users cannot
tell which situation they are in.

### Separation of concerns

The API should distinguish three axes:

- **When is the situation detected.** Compile time (structural
  redundancy, provable inconsistency) vs runtime (residual
  evaluation).
- **What kind of constraint collided.** User equation vs observed
  data vs boundary condition vs conservation law. These have
  different expected tolerances and different recovery strategies.
- **What the user wants to do about it.** Accept with residual
  reporting / fail hard / re-dispatch to a different solver class.

### Proposed shape

Compile-time reporting goes through a diagnostics channel, not
runtime error. The compiler produces an overdetermined-subsystem
report listing each collision with structural information:

```
report.subsystems[i] = {
    .paths              — which nodes are over-constrained
    .collision_kind     — redundant | potentially_conflicting | bvp
    .constraints        — list of participating constraints with
                          source location (line in .myco or verb in
                          workflow)
    .residual_expression — symbolic residual when computable
    .suggested_handling — advisory (e.g., "use bvp_solver",
                          "drop one observation")
}
```

Runtime reporting lives on the experiment result:

```python
exp_result.consistency_report
  .per_subsystem[i]
    .observed_residual     — numeric
    .tolerance_used        — numeric
    .verdict               — within_tol | exceeds_tol | nan
    .contributing_paths    — which bindings fed the residual
```

The user configures handling through a dedicated verb in the
workflow, not through solver options:

```python
# Declarative configuration, not an imperative callback
on_overdetermined(
    path="plant.biomass",           # specific subsystem or pattern
    tolerance=1e-6,
    kind="observation",             # or "equation" or "boundary"
    when_within_tol="silent",       # silent | warn | record
    when_exceeds_tol="error",       # error | warn | record
)
```

Pattern matching over paths keeps the verbose case from exploding:
one `on_overdetermined(path="*", ...)` sets a default; specific
paths override.

### Why a dedicated verb and not solver options

Overdeterminedness is a property of the *model and its bindings*,
not of the solver. If the user swaps solvers, their tolerance
policy for an observed value should not change. Tying handling to
the workflow keeps the language honest about what kind of
statement is being made.

### Query API for drilling in

Complicated cases need a way to ask "which constraints fought each
other in subsystem 3". Suggested methods on the report:

```python
subsystem.participants()         — all paths
subsystem.constraint_graph()     — how paths are linked
subsystem.residual_explained()   — symbolic decomposition of the
                                   residual by contributing
                                   constraint
subsystem.drop_one_candidates()  — which single-constraint removals
                                   would resolve the collision
```

The last one is particularly useful in debugging: the user asks
"which observation is causing the inconsistency" and gets a ranked
list.

### BVP as a separate path

BVP-style overdetermination should not travel through the same
error / warning channel at all. The compiler recognizes the
structural pattern (boundary constraints on both endpoints of an
integration) and routes the SCC to a BVP solver. The
overdetermined report notes this as `collision_kind = bvp` with
`suggested_handling = "routed to bvp_solver"` for transparency, but
no user action is required.

This is consistent with the "compiler chooses solver class, user
chooses semantic content" policy from §20.

### Spec implications

- §15 or §20 gains an overdetermined-subsystem section covering
  detection, classification, and dispatch
- New workflow verb `on_overdetermined` with the signature above,
  or equivalent
- §22 consistency-report object documented with both compile-time
  and runtime fields
- §23 marshals the report into Python with usable walker APIs
- `hypha explain` CLI grows an `explain-overdetermined` subcommand
  for interactive drilling

### Why this matters for the three-verb redesign

Under the proposed three-verb surface, observations and equations
collide through the same mechanism (constraints pushed into the
e-graph). The overdetermined handler is the primary user-facing
surface where this unification becomes visible. Getting the API
right is part of validating the three-verb framing: if the API
feels natural, the unification was the right call; if it needs
separate treatment for bind-collisions vs observe-collisions, the
verbs were not really the same thing.

## Overdetermination reframed: world-layer equalities are the feature

An earlier draft of this file proposed that the compiler should
refuse to compile `.myco` worlds containing two equalities for the
same quantity (e.g., `x = this` and `x = that` with distinct RHS).
Riley corrected this: those multi-statements are the whole point.
They bridge subsystems in the e-graph so information flows between
them. The `x` is one `x` everywhere it appears; both equalities
hold simultaneously as claims about the world, and the e-graph
propagates consequences through both.

With that correction, the design picture changes:

**Inconsistency always originates at the workflow layer.** A
well-formed `.myco` is internally consistent by construction (and
any internal redundancy is informative, not pathological). The
moment real observations are attached to the world, the combined
system can become inconsistent — because instruments are noisy,
models are approximate, conservation holds up to measurement
precision rather than exactly, and any honest ontology of real
processes generates this.

**The user has four legitimate responses**, in roughly decreasing
order of modeling hygiene:

1. **Model the noise explicitly in `.myco`.** User writes
   `x_obs = x + epsilon` with `epsilon ~ Normal(0, sigma)` as a
   world claim. The disagreement becomes a named, distributed
   quantity and the inference machinery handles it. This is the
   right answer whenever a defensible noise model exists. The
   domain knowledge ends up in the world where it belongs.

2. **Statistical inference mode.** Even without an explicit noise
   term, the user can declare "treat world equations as likelihood
   terms, not hard constraints" via inference mode (MLE /
   Bayesian). World equations become soft; residuals become
   likelihood contributions. This is the default answer for "I
   have noisy data, I want a fit". Lives at the compile-mode
   level, not per-node.

3. **Fixed-menu numerical strategies for direct-observation
   conflicts.** When multiple *direct* observations of the same
   path disagree (three thermometers on one quantity) with no
   model in between, there is no likelihood function to hide
   behind. Here the small fixed workflow-level menu applies:
   `hard_error | best_conditioned | lowest_residual |
   accept_arbitrary`. No custom Python. Specifically no `average`
   or `weighted_average` in the menu because those encode
   reliability claims about the instruments, and reliability
   claims are world-level (they go in `.myco` via option 1).

4. **Tolerance-based hard mode.** Keep world equations as hard
   constraints, declare a tolerance band, fail when data violates
   by more than that. For cases where the world law is trusted and
   the data is the thing under validation.

### Which layer handles which case

Options (1) and (2) are the primary answers and cover most real
statistics. Option (3) is a narrow workflow-side affordance for
genuine observation-vs-observation disagreement with no model
mediating. Option (4) is what `on_overdetermined` with
`hard_error` means in practice.

### The `observe` default matters

Default semantics of `observe(path, data)` without any additional
config should be hard-constraint-with-tolerance, because that is
the least-magical interpretation. To get statistical behavior, the
user either (a) models noise explicitly in `.myco`, or (b) flips
inference mode. No silent likelihood coercion. This keeps the
semantics honest: if the user has not said "this is noisy", Myco
does not assume it is.

### Why no custom Python handler for strategy 3

Four reasons, unchanged from the earlier draft:

1. Preserves the world-is-truth soul. Custom handlers smuggle
   model content into the workflow.
2. Bounded API surface makes reproducibility trivial. A workflow
   is fully described by its (small) config.
3. Forces better modeling. Every time a user reaches for a custom
   handler, the correct fix is almost always a missing world
   equation.
4. Custom Python handlers are how scientific code ends up
   non-reproducible across machines and environments.

### Acid test for the fixed menu

Any strategy that involves a *choice about data reliability* is
domain knowledge and must be in `.myco`. Any strategy that
involves purely *numerical disambiguation* can live in the
workflow. `best_conditioned` and `lowest_residual` pass (about
numerics); `weighted_average` fails (weights encode reliability).

### Implications for the overdetermined handler API

The earlier overdetermined-handler section still applies, with one
scope tightening: `on_overdetermined` is specifically for case (3)
above. Cases (1), (2), (4) are handled by other mechanisms
(`.myco` world claims, inference mode, tolerance declarations).
That sharpens what belongs in the `on_overdetermined` verb
and prevents it from becoming the junk drawer for every form of
data-world friction.

## Streaming execution and the retraction-vs-eviction distinction

Gemini's follow-up review raised a concern: "no retraction" plus
Layer 3 adjacent keyed state implies a memory leak by design in
long-running simulations (a million-tick run with frequent
entity creation and destruction will explode the keyed-state
layer). The right response dissolves the concern by separating
two things that were previously conflated.

### The distinction

- **Retraction** applies to the *symbolic* layer: world
  equalities, stated claims, conservation laws, type contracts.
  These never get unsaid. The e-graph's monotone growth is the
  append-only record of what the model claims is true.
- **Eviction** applies to the *materialized* layer: cached values
  at specific indices, specific entity instances, working-set
  state. These are deterministic functions of the symbolic
  structure plus inputs. Dropping a cached value at `t = 500` does
  not retract anything; the same value can be re-materialized by
  running the program again.

"No retraction" was conflated with "no eviction" in earlier
readings. Pulling them apart: append-only is a property of
*claims*, not of *memory*. The memory-leak concern evaporates
under the correct reading.

### The execution model this implies

Myco compiles to a *streaming executor with a bounded working
set*, not an eagerly unrolled trace. This is closer in flavor to
how a SQL query planner works (compact logical plan, streaming
physical execution) or array-language fusion than to a traditional
interpreter. The compiled plan is `O(program size)`; resident
state is `O(minimum window required by active computation)`.

Long-running simulations do not hold the whole trajectory in
memory. They hold the symbolic program plus the sliding window
required by active reads, accumulators, and outstanding queries.

### What determines the window size

- **Backward lookback**: `y[t - k]` forces `k` steps of retention
- **Aggregators with incremental form**: `total = total + flux`
  collapses to O(1) retention via the A/E rewrite groups
- **Aggregators without incremental form** (e.g., quantile over
  history): must retain the window or refuse streaming mode
- **Active `inspect()` queries**: on-demand re-materialization of
  specific indices from the symbolic structure
- **Event lookbacks with bounded horizon**: known window
- **Entity lifetimes**: once no live read references an entity's
  past state, that state is evictable

### Compiler passes this requires

1. **Footprint analysis.** Given a model and a horizon, the
   compiler computes an upper bound on the minimum working-set
   size. Models whose footprint is unbounded under streaming mode
   are either rejected or compiled to bounded-horizon mode
   (explicitly, with the user notified).

2. **Unrollable-vs-dynamic classification.** Static structure
   (const generics, bounded loops, fixed meshes) unrolls eagerly
   if small, stays compactly represented if large. Dynamic
   structure (events, data-dependent flow) is materialized lazily
   at the sliding window's leading edge.

3. **Accumulator rewrite.** Detect global properties that admit
   an incremental form and rewrite automatically. Already in
   scope for the A/E rewrite groups; needs to become normative.

### Symbolic verification without unrolling

If the `.myco` says "conservation holds at every step" and the
symbolic recurrence admits an induction proof via the e-graph's
rewrite rules, `prove()` can succeed *without ever running the
model*. Compile-time invariant checking becomes a first-class
capability, not a runtime probe. This is the deeper version of
the `prove()` story: the structure is often enough.

Riley's intuition that you could verify a large .myco's
legitimacy symbolically, before unrolling, lands here. It is the
same machinery as partial evaluation, applied to claim-checking
instead of residual extraction.

### Index families and unroll strategy

This is where the "time as sequence / index family" generalization
earns its keep internally, even if the surface stays time-primary.
Each family has a different unroll strategy:

- **Spatial** (mesh): unroll once at compile time, fixed working
  set
- **Temporal**: streaming with sliding window
- **Collection**: depends on iteration order (map vs scan vs
  global reduction)
- **Event**: on-demand with a default bound; refuse streaming if
  the event rate is unbounded
- **Iterative** (solver): inner-loop, transient working set that
  does not escape the SCC

The user-facing surface does not need to know this taxonomy; the
compiler does. Codex's pushback against publicly renaming §9 stands:
the unification is a compiler-internal concern.

### Dynamism gates plannable horizon

Pure static structure can be unrolled arbitrarily far ahead (or
compacted symbolically). Stochastic events let the compiler bound
the envelope (worst-case working set) but not the specific
trajectory. Data-dependent control flow needs branch-by-branch
worst-case analysis. True external-input dependence caps
look-ahead at whatever the input buffer provides. The footprint
analysis pass walks this hierarchy.

### History-bound cases

Some models genuinely need the whole trajectory: full-posterior
MCMC over time, long-window autocorrelation, global optimization
over time-indexed parameters. These compile to *bounded-horizon*
mode rather than streaming, and pay the memory cost honestly. The
compiler reports which mode it selected and why. No silent
coercion.

### Spec commitment needed

Current §19 (extraction) and §16.2 (append-only state) are
ambiguous about whether Myco commits to streaming execution. For
the memory story to hold, the spec needs a normative statement
along the lines of:

> The compiled artifact is a streaming executor. Materialized
> state is cached and evictable; only symbolic claims are
> append-only. The compiler computes an upper bound on working-set
> size at compile time, or compiles to bounded-horizon mode and
> reports the choice to the user.

That one commitment resolves Gemini's memory concern and
simultaneously strengthens `prove()` into a compile-time
invariant-checker.

### Future optimization: speculative execution on idle cores

Noted as future work, not first build. The streaming executor
naturally exposes dynamism points (where the sliding window can
speculate on branch outcomes). Because `.myco` computation is
pure, speculative work is trivially rollback-able: discard the
cached results, keep the committed ones. Variants:

- **Two-sided speculation**: run both branches in parallel on
  idle cores, keep the winner. Simple, potentially wasteful.
- **Profile-guided speculation**: track historical branch
  frequencies per site, speculate the more common one.
- **Oracle-guided speculation**: small predictor (learned or
  heuristic) picks the branch to speculate.

GPU behavior is different. GPUs do not do branch prediction in
the CPU sense. SIMT warps handle divergence by executing both
sides serially with masking; this is closer to unconditional
two-sided speculation at the hardware level, but its cost is
wall-clock serialization rather than wasted cores. The right
GPU-side optimization is *divergence reduction* through batching
coherent work together (sort entities by likely branch, partition
kernels by branch state). Speculation on idle cores and
divergence reduction on GPU are complementary, not substitutes.

### Retraction-vs-eviction one-liner for the spec

The normative rule that needs to land in the spec:

> Symbolic claims are append-only (no retraction). Materialized
> state is a cache (freely evictable). Any apparent conflict
> between these is a conflation.
