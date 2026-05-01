# 08 — Relation/Fn Fix

Durable summary of the fn-vs-relation lock and the associated design
decisions. Captures the ban-user-fn policy, statement-form-only
invocation, the three modes for representing runtime pluggability, the
dumb-data Python API principle, and the surfaces the follow-up sweep
must touch.

## The soul violation

The spec enumerates 8 e-graph merge sources. **Fn definitional equality
is not among them.** A user who writes

```
fn arrhenius(T, Ea) { exp(-Ea / (R*T)) }
```

produces an opaque e-node that hides its equational content from the
compiler until some rewrite pries it open. The same formula written as
a relation is fully visible to invertibility derivation, envelope
reasoning, capability inference, etc.

Consequence: whether the compiler can invert Arrhenius depends on which
keyword the user reached for. Same math, different compiler visibility.
This is the violation — Myco's premise is that the compiler derives
properties from equational content, not from user claims or keyword
choice.

The "this is marked invertible and this isn't" annotation surface is
the same violation seen from the other side: users assert capability,
compiler permission-checks user claims, compiler is no longer the
source of truth.

## The lock

1. **Ban user fn.** End users cannot declare `fn` in user code. Full
   stop.
2. **Keep stdlib fn** as the axiomatic primitive surface: `exp`, `log`,
   `sin`, `cos`, `sqrt`, arithmetic. Irreducible. Capability contracts
   declared at stdlib level are compiler axioms. Riley-only; not a
   library surface.
3. **Introduce parameterized relations** as the user-facing reuse
   mechanism. Relations already live in the e-graph; adding formal
   parameters lets users package named, reusable equational content
   without opacity.
4. **Contract methods are parameterized relations**, not a separate
   concept.
5. **Kernels are parameterized relations**, not a separate concept (§28
   becomes curation of stdlib-shipped named relations).
6. **No user capability-contract declaration surface.** The compiler
   derives invertibility / differentiability / monotonicity from body
   composition over stdlib atoms + other parameterized relations. Users
   never annotate.

## Parameterized-relation syntax

Undirected, positional, equational. No `-> ReturnType`.

```
relation arrhenius(
  T: Scalar<Temperature>,
  Ea: Scalar<Energy>,
  rate: Scalar<PerSecond>,
):
  rate = exp(-Ea / (R * T))
```

All parameters are semantically equal. The equation asserts equality;
nothing in the declaration pins direction. Workflow-side data and
upstream `.myco` relations determine which variables end up pinned and
which are derived at extraction time.

## Invocation rule

**Every parameterized relation invocation is statement-form with all N
slots explicit.** A relation body is a sequence of lines, where each
line is exactly one of:

1. **`let name: Type`** — introduces a fresh e-graph node with the
   given name. Explicit node creation.
2. **`relation_name(arg, arg, ...)`** — invokes a parameterized
   relation in statement form. Every parameter slot is filled by a
   previously-existing node (field access, universal, or a `let`-
   introduced local). The body's equations unfold into the caller's
   e-graph with the supplied substitutions. Merge-source-1.
3. **`lhs = rhs`** — asserts equality between two already-existing
   expressions. Standard equation.

No other invocation shape is legal for parameterized relations. In
particular:

- **No expression-position invocation.** `let x = foo(a, b)` where `foo`
  has arity 3 is illegal. The user writes `let x: Type` on one line and
  `foo(a, b, x)` on the next.
- **No "missing slot" conventions.** Every slot is explicit; the
  compiler never fills slots silently.
- **No anonymous intermediates.** If the user wants a name for a value,
  they introduce it with `let`.

Stdlib fns (`exp`, `log`, `+`, `*`, `min`, `max`, ...) are not
parameterized relations. They are axiomatic primitives with built-in
fn semantics and work in expression position as usual:
`rate = kmax * exp(-psi / b)` is ordinary and fine.

## Method-style dispatch sugar

For parameterized relations whose first parameter is typed `Self`, the
compiler accepts `receiver.rel(args...)` as sugar for
`rel(receiver, args...)`. Pure positional shift; no slot-inference, no
direction.

```
// sugar form:
leaf.vc.fraction(leaf.water_potential, remaining)

// desugars to:
fraction(leaf.vc, leaf.water_potential, remaining)
```

Method-style vs unsugared is a per-call-site readability choice, not a
property of the relation. Contract-method invocation usually reads
better with the dot form.

## Name collision rule

**`()` always denotes a relation invocation, never field-then-apply.**
If a type has a field `f` and an internal relation `f(...)`:

- `leaf.f` is a field access. Returns the stored value.
- `f(leaf, arg, arg)` or `leaf.f(arg, arg)` is a relation invocation.

The grammar rule: parentheses after a name always parse as invocation.
This rules out function-valued fields and higher-order-call-on-
expression-result, neither of which Myco wants.

Infix operators (`+`, `*`, etc.) are stdlib-fn sugar and behave as
expected; they are not subject to the relation-invocation rule.

## Worked example: vulnerability curve

**Library side — `hydraulics` spore:**

```myco
// hydraulics/src/vc.myco

contract VulnerabilityCurve:
  relation fraction(
    self: Self,
    water_potential: Scalar<Pressure>,
    remaining: Scalar<Dimensionless>,
  )

type WeibullVC implements VulnerabilityCurve {
  b: Scalar<Pressure>,
  c: Scalar<Dimensionless>,

  relation fraction(self, water_potential, remaining):
    remaining = exp(-((water_potential / self.b) ^ self.c))
}

type SigmoidVC implements VulnerabilityCurve {
  p50:   Scalar<Pressure>,
  slope: Scalar<Dimensionless>,

  relation fraction(self, water_potential, remaining):
    remaining = 1 / (1 + exp(-self.slope * (water_potential - self.p50)))
}
```

**Consumer — `my_model` spore:**

```myco
// my_model/src/leaf.myco
use hydraulics::{VulnerabilityCurve, WeibullVC}

type Leaf {
  kmax:            Scalar<Conductance>,
  water_potential: Scalar<Pressure>,
  vc:              VulnerabilityCurve,     // contract-typed (Mode B)
  k:               Scalar<Conductance>,
}

relation leaf_conductance on leaf:
  let remaining: Scalar<Dimensionless>
  leaf.vc.fraction(leaf.water_potential, remaining)
  leaf.k = leaf.kmax * remaining
```

Three lines in the body, three meanings:

1. `let remaining: Scalar<Dimensionless>` creates a new e-graph node.
2. `leaf.vc.fraction(leaf.water_potential, remaining)` is method-style
   dispatch, desugaring to `fraction(leaf.vc, leaf.water_potential,
   remaining)`. All three slots filled by existing nodes.
3. `leaf.k = leaf.kmax * remaining` is a standard equation; `*` is a
   stdlib axiom with fn semantics.

## Contracts as required parameterized relations

A contract declares the required relation shape; implementors provide a
relation of the same name and signature.

```
contract PhotosynthesisModel:
  relation photosynthesis(
    self: Self,
    light: PPFD,
    temperature: Scalar<Temperature>,
    co2: CO2Concentration,
    rate: Scalar<PhotosynthesisRate>,
  )

type FarquharC3 implements PhotosynthesisModel {
  vcmax25: Scalar<VcmaxUnit>,
  kc25:    Scalar<PartialPressure>,
  // ...

  relation photosynthesis(self, light, temperature, co2, rate):
    let vcmax_t: Scalar<VcmaxUnit>
    arrhenius_scale(self.vcmax25, temperature, vcmax_t)
    let An_c: Scalar<PhotosynthesisRate>
    rubisco_limited(vcmax_t, co2, An_c)
    let An_j: Scalar<PhotosynthesisRate>
    light_limited(light, An_j)
    rate = min(An_c, An_j)
}
```

Every intermediate is explicit. The compiler sees the full equation
chain; inversion, differentiation, cost analysis all compose through.

## Kernels as parameterized relations

```
relation kernel_gauss<D: Length>(
  distance: Scalar<D>,
  sigma:    Scalar<D>,
  weight:   Scalar<dimensionless>,
):
  weight = exp(- (distance * distance) / (2 * sigma * sigma))
```

Same shape as any other parameterized relation. No dedicated "kernel"
keyword. §28 becomes curation of stdlib-shipped named relations, not a
distinct semantic category.

## Three modes for representing pluggability

`.myco` supports three equally valid patterns for "which implementation
does this instance use."

### Mode A — concrete type baked into the world

The `.myco` model commits to a specific implementation.

```myco
type Leaf {
  vc: WeibullVC,   // concrete; the world-model asserts Weibull
  // ...
}
```

No per-instance or per-workflow choice. Changing family requires
editing `.myco`. Good when the modeler has committed to a specific
family as part of the world-claim.

### Mode B — contract-typed field, per-instance swapping

The `.myco` model says "a leaf has some VC." The specific choice gets
bound at workflow time, per instance.

```myco
type Leaf {
  vc: VulnerabilityCurve,   // contract-typed
  // ...
}
```

Different leaves in the same population can use different VCs.
Specialization happens per-site at compile time.

**Note:** under the dumb-data Python API (below), per-instance
heterogeneous contract binding is not expressible from Python alone —
the choice has to be made in `.myco`. See the §35 open for how this
might be rescued via sum types / discriminants; Mode B is primarily
useful today for "subclasses defined in `.myco`, selected by a
`.myco`-side switch."

### Mode C — generic type parameter

`.myco` parameterizes the type over the family; workflow picks the
family at instantiation.

```myco
type Leaf<VC: VulnerabilityCurve> {
  vc: VC,
  // ...
}
```

`Leaf<WeibullVC>` and `Leaf<SigmoidVC>` are different types. Each
population commits to one family. Monomorphization happens once at
workflow composition.

### Choosing among them

| Mode | When |
|---|---|
| A concrete | The world-claim commits to a family. |
| B contract-field | You want per-instance heterogeneity within a single population, driven by a `.myco`-side discriminant. |
| C generic param | You want family-uniformity per run but family-agnostic `.myco` source; swap families between runs without editing the model. |

Mode C is the most common for research use. Mode A is the honest default
when there's only one obvious family. Mode B is the most flexible and
the least ergonomic.

## Python API: dumb data provenance

**The Python side does not know Myco types.** The Python library is a
generic data-provenance layer. It handles:

- Compiling / loading `.myco` models
- Spawning populations of declared entities
- Binding data to named nodes (scalars, time series, tensors)
- Observing named outputs
- RNG / wall-clock / checkpointing infrastructure

The Python side does NOT handle:

- Importing spore-specific types as Python classes
- Constructing Myco instances via Python constructors
- Mirroring `.myco` contracts in a Python type registry

Binding is done by node name:

```python
import myco

world = myco.load("my_model")          # compile + load
pop = world.spawn("Leaf", n=1000)

# bind values to named nodes
pop.bind("vc.b",   values=np.full(1000, 3.0))
pop.bind("vc.c",   sampled_from=myco.uniform(1.5, 3.0))
pop.bind("kmax",   sampled_from=myco.lognormal(mu=5, sigma=0.5))

run = world.run(duration=30_days)
k_series = run.observe("k")            # time series per leaf
```

The Python library knows node names, declared types (for shape
checking), units (for coercion), and that's it. `WeibullVC`,
`VulnerabilityCurve`, and `fraction` are compile-time `.myco` artifacts
— they never cross into Python as importable symbols.

**Concrete implications:**

- Spore authors ship one artifact (the `.myco` sources + `myco.toml`),
  not two (`.myco` + a Python mirror package).
- The Python library surface grows along one axis: data primitives.
- Mode B per-instance contract binding requires a `.myco`-side
  discriminant mechanism (see §35 open).

## Stdlib boundary (locked)

- **Stdlib fn** (Riley-only): axiomatic primitives. Opaque e-nodes
  carrying capability contracts declared at stdlib level. Compiler
  trusts by fiat.
- **Stdlib parameterized relations** (Riley-only for now): named
  compositions over stdlib fns — kernels, standard scaling laws, common
  distributions' log_pdfs.
- **User parameterized relations** (anyone): compositions over stdlib
  fns, stdlib relations, and other user relations. No capability-
  contract declaration; compiler derives properties from body
  composition.

The trust boundary is "what is axiomatic" (fn), not "what is named and
reusable" (relation). If a user needs a new primitive, they request it
for stdlib inclusion.

## Cost system integration

The cost-of extraction-cost struct (§14, §19.1) interacts with the
relation/fn lock via four layers:

1. **Stdlib atom declarations** (Riley-only). Each stdlib fn ships
   capability contracts plus an abstract cost tag: operation class
   (transcendental, arithmetic, reduction, matmul, ...) + operand-shape
   schema. Axiomatic.
2. **Compositional rules** (compiler-built-in). Costs of compound
   expressions derive leaf-up: `compute` sums, `condition` propagates
   via partial derivatives, etc. Fixed algebra.
3. **Rewrite-rule cost contributions** (Riley-only, §17). Every rewrite
   carries its own cost delta. Lossy rewrites contribute to
   `approximation`; lossless rewrites contribute zero.
4. **Backend cost interpreter** (per-backend, pluggable). Translates
   abstract op class + shape into concrete weight. GPU tensor-core
   matmul vs scalar CPU transcendental, etc.

User parameterized relations need no cost annotations; cost is derived
from body composition. Direct analog of "no user capability annotations"
applied to cost.

**Cost interpreter as a contract.** Locked. `CostInterpreter` is a
contract with required parameterized relations; each backend ships an
impl. No special mechanism beyond the regular contract/relation
machinery.

**Deferred to §35:**

- Whether `memory` is a sixth field of `cost_of` or a backend-specific
  annotation.
- Approximation-cost composition for stacked lossy rewrites.
- Condition-cost representation for multi-output operations.
- Enumeration of the canonical stdlib atom set. Separate chunk.

## Design choices (resolved)

External review (Codex + Gemini, parallel) converged on all five.

1. **Undirected by syntax** (no `-> ReturnType`). Locked. Direction is
   a workflow/extraction concept, not a declaration concept.
2. **No `use` / `call` keyword at invocation.** Locked. Statement-form
   invocation is structurally unambiguous with the `()`-always-means-
   invocation rule; no disambiguator needed.
3. **Flat positional args.** Locked. Named args are a candidate future
   sugar but not load-bearing for v2.1 — deferred. (Earlier draft
   argued for named args as necessary for "which slot is the unknown";
   that framing was wrong. No slots are unknown; all N must be filled
   explicitly in statement form.)
4. **Generics via the same type-parameter machinery.** Locked.
   `relation arrhenius<U: Temperature>(...)` monomorphizes at call
   site.
5. **Contract "method" collapses into required parameterized relation.**
   Locked.

## Sub-questions (resolved)

1. **`self` is convention, not syntactic marker.** Structural rule:
   contract-required relations take the implementor instance as the
   first parameter, typed `Self`. The identifier spelling `self` is
   recommended for readability but not semantic.
2. **Free variables: banned.** Parameterized-relation bodies reference
   only formal parameters, imported names, other declared relations,
   universals, and stdlib items. No caller-scope capture.
3. **Recursion: banned in v2.1.** Both direct and indirect. Use
   `temporal` dynamics or algebraic cycles for legitimate feedback.
4. **Compiler-derived property visibility: inspection-only.** `hypha
   explain`, IDE hover, and plan-inspection surfaces expose derived
   properties with proof chains or explicit "unknown because ...".
   Never a source-level annotation.
5. **Namespacing: qualified paths + explicit import aliases.**
   Unqualified collisions are hard errors at import.

## Additional questions raised by external review

1. **Expression vs statement position.** **Resolved: statement-form
   only** for parameterized relations. Stdlib fns retain expression-
   position semantics.
2. **Local-intermediate syntax.** **Resolved: explicit `let`** on a
   separate line. No `let x = rel(...)` sugar. No anonymous
   intermediates.
3. **Y5 (custom closure policies) placement.** Still open. Codex's
   suggestion: move user-defined Y5 to the workflow layer; built-in
   Y1–Y6 stay in `.myco`. Deferred to a follow-up chunk on closure-
   policy extensibility.
4. **Distribution contract shape.** Still open. Are `log_pdf` /
   `sample` stdlib-only callable exceptions or relation-shaped
   obligations? Needs to land before §13/§27 do. Deferred to a follow-
   up chunk on probabilistic programming.
5. **Variable / relation name collision (Gemini).** **Resolved:**
   `()` always denotes relation invocation. Field access is parenless.

## Surfaces the follow-up sweep must touch

From external review (see
`planning/v2/audit/relation_fn_external_review.md` for the full list):

**spec_new.md:**
- Part I summary (L196) — reframe from "functions" to relations
- §3.1 — reframe stdlib-axiomatic boundary
- §6 — retire user fn; retain stdlib fn; rename to "Axiomatic Primitives"
- §7 — method becomes required parameterized relation
- §8 — add statement-form invocation rule + method-dispatch sugar;
  name-collision rule
- §8.8 (Y5) — reframe; consider workflow-side move
- §11.1 — reframe spatial operators as axiomatic primitives
- §13.8 — categorize distributions
- §14 — categorize intrinsics
- §17.1 — source 5 stays stdlib-only
- §17 merge count unchanged
- §24.1–§24.2 — rename `fn` parameter to `callable` to avoid collision
- §28 — rename to "Relation Curation"; strip fn-based kernel prose
- Appendix A — retire `fn` keyword, `self` reserved language
- Appendix B — remove future-user-fn placeholder
- Appendix C / Y5 — reframe

**Other files:**
- anti_spec.md — add user fn, kernel-as-function, capability-annotation,
  method-wording retirements
- mock_sperry.myco, mock_potkay.myco — full rewrite of `pub fn` blocks +
  header comments
- 03_kernels_in_progress.md — retire "kernels are ordinary `.myco`
  functions"
- 06_backend_abstraction_in_progress.md §4.5 — `bind_controller`
  terminology
- Audit reports: 06, 13, adjudication.md Batch 2 §6 — flag as resolved
  by chunk 08

## Status

Locked:
- Ban user fn; stdlib fn as axiomatic primitives
- Parameterized relations as user reuse mechanism
- Statement-form-only invocation; `let` for node creation
- Method-style dispatch sugar
- Name-collision rule (`()` means invocation)
- Three-mode pluggability story (A concrete / B contract / C generic)
- Dumb-data Python API principle
- Cost-interpreter-as-contract
- All five design choices + five sub-questions

Deferred:
- Y5 closure-policy extensibility — follow-up chunk
- Distribution contract shape (`log_pdf` / `sample`) — follow-up chunk
- Mode B per-instance discriminant mechanism — §35 open

Ready for the surface sweep once the deferred items are at least
scoped. The sweep itself does not block on their resolution; it can
land the locked items first.
