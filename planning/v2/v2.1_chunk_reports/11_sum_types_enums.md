# 11 — Sum Types / Enums

Durable summary of the sum-type / enum design. Captures the four
independent design pressures that converge on this mechanism, a
syntax sketch, the compile-time-vs-runtime specialization story, the
relationship to contracts, the stdlib `Prior<T>` example, and the
open items.

**Status: draft, partial lock. Motivation and overall shape
committed. Exact syntax, exhaustiveness rules, lowering details, and
stdlib surface are open.**

## Why enums

Four independent pressures arrive at the same mechanism. Any one of
them is enough; together they are the clear signal that Myco needs
sum types.

1. **Number-or-distribution** — importing a model and materializing
   it deterministically or probabilistically from the same source.
   A parameter field is either a concrete `Scalar<T>` or a
   `Distribution<T>` prior. Same `.myco`, different workflow
   bindings.

2. **Mode B, per-instance heterogeneous contract dispatch** (chunks
   08, 09). A population where different instances carry different
   implementations of a contract (leaves with Weibull VC vs sigmoid
   VC). Needs a discriminant tag and per-tag relation dispatch.

3. **Finite state machines in dynamic topology.** Entities whose
   shape or behavior changes at event boundaries (seed → seedling →
   sapling → mature tree). Each stage has different fields, different
   relations. Tagged union over life stages with event-triggered
   variant transitions.

4. **Option / Result at the workflow boundary.** A workflow-supplied
   value that may be absent; a conversion that may fail. `Maybe<T>`,
   `Result<T, E>` are ordinary enums in every other language.

Contracts alone cannot cover these. Contracts give **behavioral
polymorphism** ("this thing implements `k(ψ)`"). Enums give
**structural polymorphism** ("this thing is one of several shapes").
Both are load-bearing and they compose — an enum variant can carry a
contract-typed field.

## Why contracts are not enough

The number-or-distribution case shows the gap clearly.

A tempting "contract only" solution:

```myco
contract Valued<T: Unit> {
    relation draw_value() -> Scalar<T>
}
impl Valued<T> for Scalar<T> { ... }
impl Valued<T> for some Distribution<T> { ... }
```

This breaks the PPL machinery. `.myco`'s probabilistic story (§13)
rests on `~` being **syntactically visible**: the compiler sees
`x ~ Normal(...)` and enters Tier A/B/C dispatch, injects the prior
into the log-joint, marginalizes discrete latents, and so on. If
`draw_value()` hides whether the underlying object is a scalar or a
distribution, the compiler cannot distinguish a constant from a
prior — Tier dispatch fails, log-pdf accounting breaks, observation
injection breaks.

Structural polymorphism keeps the distinction visible. An enum
`Prior<T> = Fixed(Scalar<T>) | Random(some Distribution<T>)` requires
the user (or a lifted-arithmetic sugar) to match on the variant, at
which point `~ d` appears syntactically on the `Random` branch and
the PPL machinery sees it.

The rule of thumb: if the variants share a meaningful behavioral
interface, use a contract. If the variants have fundamentally
different shapes or participate in fundamentally different compiler
machinery, use an enum.

## Syntax sketch

Declaration:

```myco
enum Prior<T: Unit> {
    Fixed(Scalar<T>),
    Random(some Distribution<T>),
}

enum LifeStage {
    Seed { age: Scalar<days> },
    Seedling { height: Scalar<m>, age: Scalar<days> },
    Mature { height: Scalar<m>, dbh: Scalar<cm>, crown_area: Scalar<m2> },
}

enum Maybe<T> {
    Some(T),
    None,
}
```

Variants can be:
- **Unit variants** (`None`).
- **Positional variants** (`Fixed(Scalar<T>)`).
- **Struct-like variants** with named fields (`Seedling { height, age }`).

Match expression (statement-form, aligned with relation/fn lock in
chunk 08):

```myco
match stage {
    Seed { age } => ...,
    Seedling { height, age } => ...,
    Mature { height, dbh, crown_area } => ...,
}
```

Matches must be exhaustive. The compiler checks at type-check time;
missing variants are a hard error.

Exact spelling of `match` vs `case` vs `when` is open. The
exhaustiveness rule is not.

## Compile-time vs runtime specialization

**Both. The compiler picks based on whether the discriminant is
statically known after workflow binding.**

### Compile-time specialization (static discriminant)

When the workflow commits a whole population to one variant at
binding time, the compiler specializes the kernel. The discriminant
becomes a compile-time constant; the match collapses to the chosen
arm; no runtime branch, no runtime tag in memory.

This covers the number-or-distribution case almost always. You
import a model, bind every `Prior<T>` parameter either as `Fixed` or
as `Random`, and the compiler emits one specialized kernel per
materialization. Zero overhead relative to hand-writing two separate
models.

### Runtime specialization (dynamic discriminant)

When the variant varies per-instance within a population, the
compiler emits a **discriminant-tagged kernel** with a branch or
mask. Each instance carries its tag; the kernel either branches on
the tag or executes all variants and masks out the inapplicable
ones. Lowering chooses between branch and mask based on backend
affordances (GPU prefers mask; CPU tolerates branch).

This covers Mode B (mixed-VC-family populations) and FSMs (life-
stage varies across trees in a forest).

### How the compiler decides

The discriminant is static at workflow composition if:

- The enum-typed field is bound to a single variant for the whole
  population (`pop.bind("vc", myco.weibull_vc(...))`)
- The population is homogeneous-by-construction (spawned with a
  single variant, no event ever mutates the variant)

Otherwise the discriminant is dynamic. The workflow composition
phase is the natural point to resolve this; the compiler then
specializes the downstream plan.

User-facing surface does not change between the two paths. Same
enum, same match. The lowering decision is invisible.

### Relationship to §21 static vs dynamic SCC classification

This mirrors the static-vs-dynamic module classification already in
§21. An enum whose discriminant is resolved at workflow composition
is part of the static module; dynamic-discriminant enums live in the
dynamic module. No new classification axis, just enums participating
in the one that already exists.

## Interaction with contracts

Enum variants can carry contract-typed fields:

```myco
enum VCBinding {
    Weibull(WeibullVC),
    Sigmoid(SigmoidVC),
    Custom(some VulnerabilityCurve),
}
```

The first two are concrete (Mode A). The third is contract-typed
(Mode B) and lets an escape hatch for VC families not enumerated at
compile time.

A contract can also be parameterized over an enum:

```myco
contract Prior<T: Unit> {
    relation materialize() -> ...
}

// Prior<T> enum can itself implement this contract,
// with a match on self inside the relation body.
```

This keeps the two concerns cleanly separated without preventing
composition.

## Stdlib surface

Several enums are universal enough to live in stdlib.

### `Prior<T>`

```myco
enum Prior<T: Unit> {
    Fixed(Scalar<T>),
    Random(some Distribution<T>),
}
```

The number-or-distribution case. Used as a parameter type when a
model wants to be deterministic or probabilistic based on
materialization.

### `Maybe<T>`

```myco
enum Maybe<T> {
    Some(T),
    None,
}
```

Absence at the workflow boundary. Unbound optional parameters, lookup
failures, sentinel-free sum-type alternative to NaN / sentinel
values.

### `Result<T, E>`

```myco
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Fallible operations at boundaries. Conversion, parsing, workflow-
side validation.

### Others

Open what else ships. Candidates: `Either<L, R>`, `NonEmpty<T>`,
`OrderedPair<T>`. All of these can be added post-v2.1; only `Prior`,
`Maybe`, `Result` look load-bearing for the core story.

## Sugar: lifted arithmetic through `Prior<T>`

Writing explicit matches in every relation body is painful when the
enum wraps a single-value variant plus a distribution variant:

```myco
temporal y:
    y = match a { Fixed(av) => av * t + ..., Random(d) => (~d) * t + ... } + b_something
```

Proposed sugar: arithmetic operators (`+`, `*`, etc.) and relation
invocation lift through `Prior<T>` automatically. `a * t + b` where
`a: Prior<Pa>` desugars to a match that injects `~` on the `Random`
branch and the concrete value on the `Fixed` branch.

This is convenience, not core semantics. It can land post-v2.1 once
the base enum mechanism is shipped. Users can write explicit matches
in the interim.

Open whether the sugar extends to all enums or only to stdlib
`Prior<T>`, `Maybe<T>`, etc. (special-cased). Rust's approach
(explicit match for user enums, `?` operator for `Result`, etc.) is a
reasonable reference.

## What this locks

- Myco adds sum types / tagged enums as a core construct.
- Variants can be unit, positional, or struct-like.
- Matches are exhaustive; the compiler enforces at type-check time.
- Lowering has two paths: compile-time specialization when the
  discriminant is static after workflow binding, runtime
  discriminant-tagged kernel when dynamic. Compiler picks.
- Enums compose with contracts; enum variants can carry contract-
  typed fields. Contracts handle behavioral polymorphism; enums
  handle structural polymorphism.
- Stdlib ships `Prior<T>`, `Maybe<T>`, `Result<T, E>` at minimum.

## Open items

- **Exact syntax.** `enum` keyword spelling, variant declaration
  syntax, match keyword (`match` vs `case` vs `when`), pattern
  syntax (binding, wildcards, guards).
- **Pattern matching power.** Flat variant matching is sufficient
  for the motivating cases; whether Myco needs nested patterns,
  guards (`if`), or-patterns, bindings is open. Start minimal.
- **Exhaustiveness diagnostics.** What the error looks like when a
  match misses a variant; whether the compiler ever inserts a
  default arm (probably not — explicit is better).
- **Generics interaction.** How enums interact with the type-generic
  system (§3.6). `Prior<T: Unit>` should just work, but the
  machinery needs to be spelled out.
- **Event-triggered variant transitions.** For FSM use (life stages),
  events mutate the variant. How this interacts with `replaces`
  obligations, the e-graph's monotonicity invariant, and the
  dynamic-topology machinery in §10. Likely clean but needs spelling
  out.
- **Relation bodies on enum-typed fields.** Can a relation body
  reference `leaf.stage.height` where `stage: LifeStage` and only
  some variants have `height`? Either the type system insists on a
  match first, or variants with a common-named field can be
  auto-projected with a soundness rule. Open.
- **Lifted arithmetic sugar.** Whether it ships in v2.1, post-v2.1,
  or never. If it ships, for which enums.
- **Serialization / workflow binding.** How Python binds an enum-
  typed field. Candidates: tagged dict (`{"tag": "Fixed", "value":
  3.0}`), factory pattern (`myco.fixed(3.0)` returns a tagged handle
  Python-side), discriminator string (`bind("a", variant="Fixed",
  value=3.0)`). Chunk 09 dumb-data principle applies; the Python side
  should not import the enum as a Python type.
- **Discriminant representation.** Integer tag, string tag, pointer-
  based tagged pointer. Backend-specific; probably not a language
  surface concern. Flag in §31.

## Relationship to other chunks

- **Chunk 08 (relation/fn lock).** Match expressions are statement-
  form invocations; `match { ... }` body contains parameterized
  relation invocations or direct equations. No expression-position
  match results. Aligned.
- **Chunk 09 (workflow data layer).** Python does not import enum
  types. Workflow binds variants via dumb-data primitives (tagged
  dict or factory-call pattern). Mode B resolved: heterogeneous
  contract dispatch is an enum with contract-typed variants, chosen
  at bind time per-instance.
- **Chunk 10 (package dependencies).** Enums cross spore boundaries
  like any other type. No special handling.
- **§3 (Types).** Enums are a new kind of composite type alongside
  newtype, struct, tuple. Probably lives in §3.x with cross-refs
  from §7 (contracts) and §10 (dynamic topology / FSM).
- **§13 (Probabilistic Programming).** `Prior<T>` is the canonical
  number-or-distribution escape hatch. PPL machinery continues to
  fire on `~ d` inside the `Random` arm.
- **§21 (Lowering).** Compile-time-vs-runtime specialization fits
  the existing static-vs-dynamic SCC classification. No new axis.

## Minimum viable enum system for v2.1

- Declaration syntax for unit, positional, struct-like variants
- Exhaustive match with flat patterns and named-field destructuring
- Stdlib `Prior<T>`, `Maybe<T>`, `Result<T, E>`
- Compile-time specialization when discriminant is static
- Runtime discriminant-tagged kernels when dynamic
- Contract-typed variant fields
- Workflow-binding primitives (dumb-data, factory pattern)

Deferred post-v2.1:

- Nested patterns, guards, or-patterns
- Lifted-arithmetic sugar through `Prior<T>`
- Event-triggered variant mutation (FSM dynamic topology) — depends
  on §10 open items

## Status

Motivation locked (four converging pressures; contracts insufficient
alone). Shape locked (tagged variants, exhaustive match, static/
dynamic lowering, compose with contracts). Syntax open. Lowering
details open. Stdlib surface partial. Workflow binding story open.
Not a blocker for the core language lock; fits as an additive
chunk. Most likely next major design call after Phase 1 Chunk-by-
chunk audit closes.
