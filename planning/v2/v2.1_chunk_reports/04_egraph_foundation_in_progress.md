# Myco v2.1 — E-Graph Foundation Design Report (IN PROGRESS)

**Date:** 2026-04-20 (draft started; discussion ongoing)
**Authors:** Riley Leff, Claude (Opus 4.7)
**Reviewers:** None yet — discussion still open
**Status:** IN PROGRESS. This note exists so the thread survives context
compaction. Do not treat as settled. The final spec commitment text is not
yet written; this captures the foundational decisions that the commitment
will be built from. Resume from Section 12 ("What is settled vs what is still
open") and Section 13 ("Audit cross-cutting findings").

---

## 1. Context

v1 of Myco had an explicit commitment to e-graphs as the internal equality
substrate (section §6.2, built on the `egg` Rust crate). During the v2 spec
rewrite, the defining section was rewritten and the e-graph references became
orphans. The chosen fix was to remove the orphans rather than redefine them.
The v2.1 work-in-progress inherited this absence.

This is a regression, not a design pivot. The unified-machinery direction for
kernels (see `03_kernels_in_progress.md` §7) explicitly assumes an e-graph
substrate, and multiple v2.1 surfaces (closure policies, function inverses,
named-type conversions, `identify`, obligation fulfillment, events) assume it
implicitly.

This chunk report captures the foundational design decisions that the v2.1
commitment section will be built from. It is **not** the commitment text
itself — that gets written after Phases 2-4 finish locking (see Section 12).

Kernel chunk report — `03_kernels_in_progress.md` — was the entry point into
this thread. The unified-machinery direction surfaced the e-graph as missing.
Kernel-specific resolution resumes after this foundation is locked.

Two audits were performed during this discussion:
- **Rewrite-rule audit** (`/tmp/rewrite_rule_audit.md`) — enumerated ~60
  rewrites the v2.1 materials implicitly demand, grouped by equality mode.
- **Type-conversion surface audit** (`/tmp/type_conversion_audit.md`) —
  audited all conversion mechanisms in the v2.1 materials; found only one
  surface (`convert`) actually in the spec, with gaps around fallibility and
  invertibility tiers.

---

## 2. The foundational commitment

**The e-graph is the internal equality substrate of Myco.**

This is recovered from v1 §6.2 verbatim: *"the e-graph is the equality core of
Myco, not the entire semantic system; temporal links, observations, provider
bindings, provenance, non-equational constraint metadata live in adjacent keyed
structures."* The v2.1 work extends this but does not replace it.

**The residual graph is a user-facing projection of the internal e-graph, not
a separate thing.** The current v2 spec conflates these; the fix is to state
cleanly that the residual graph is the diagnostic *view* that diagnostics and
errors reference, while the e-graph is the substrate the compiler operates on.
Extraction policy determines what the residual graph looks like at any given
moment.

**Placement in the spec (Phase 1, Q1).** Two placements:
- A brief statement early in the spec so readers encounter the commitment
  before they read surfaces that depend on it.
- A detailed technical section late in compiler internals where the merge
  rules, rewrite rules, and extraction semantics are fully specified.

The IN/OUT partition of these two placements is deferred (Phase 1, Q2) until
after Phases 2-4 lock, because what should be committed versus what should be
filed as open-question depends on decisions still in progress.

---

## 3. Three-layer scoping split (Phase 2, Q1 — LOCKED)

The e-graph is **one of three layers** of semantic state. Being precise about
which layer owns what prevents category errors.

### Layer 1 — Equality substrate (the e-graph proper)

- All value-producing expressions across all relations, timesteps, samples.
- Each e-class represents a set of expressions claimed (or proven) equal.
- Merges introduced by the sources in Section 6.
- Closure-policy candidates are co-members of a single e-class; extraction
  picks among them per the closure rule.

### Layer 2 — Envelope metadata attached to e-classes (moves under merge)

Non-equational annotations that are *attached to classes* and *combine under
merge*. Crucially, they live **with** the graph, not outside it.

- **Non-equational constraints** — inequalities (`≥ 0`), domain bounds,
  type-level predicates. Attached to the class carrying the constrained
  expression. Intersection rule under merge: if class A requires `x ≥ 0` and
  class B requires `x ≤ 10`, the merged class requires both.
- **Provenance** — which relation, which file, which line introduced each
  node. Union rule under merge: the merged class lists all sources.
- **Uncertainty annotations** for stochastic nodes (see Section 13 — this
  interacts poorly with rewriting and needs its own resolution).
- **Faithfulness and orientation tags on merge edges** (see Section 8).

### Layer 3 — Adjacent keyed structures (genuinely separate)

- **Provider bindings / coloring** — workflow-time knowns/unknowns state.
  The "coloring book" metaphor: the `.myco` file is a book of outlines; the
  workflow colors it in. Not on the graph, but keyed by e-class identity.
- **Observation values before injection** — at `.myco` time they are
  external data; at workflow time, injection `y[2] := 5.3` merges `y[2]`
  with the literal's class, moving the equality into Layer 1.
- **Stochastic sampling traces** — the runtime history of `~` draws. Live
  in a trace structure keyed by draw-node identity.
- **SCC decomposition results** — derived from the e-graph at lowering time.
  Distinct artifact.
- **Runtime event-trigger state** — event semantics operate *on* the graph
  (mutating it by adding facts) but the scheduler and trigger state is
  adjacent.

### What is NOT in Layer 3 (corrections from initial proposal)

Early drafts of this discussion put temporal indexing, non-equational
constraints, and observations in Layer 3. This was wrong on all three.

- **Temporal indexing:** `y[1]`, `y[2]`, `y[3]` are just distinct ground
  terms in Layer 1. Nothing claims them equal; they never merge. No special
  "temporal layer" is needed. (See Section 4.)
- **Non-equational constraints:** Layer 2, not Layer 3. They attach to
  classes, not to external structures.
- **Observations before injection:** Layer 3 is correct. **After** injection,
  they become Layer 1 equalities. The coloring happens at a boundary crossing.

---

## 4. Temporal handling — no special machinery needed

A common worry: if `y[t]` and `y[t+1]` are both in the graph, does the e-graph
risk merging them and destroying the model?

**No.** The e-graph only merges expressions claimed (or proven) equal. `y[t]`
and `y[t+1]` are distinct ground terms — they are not claimed equal anywhere.
They coexist peacefully as distinct nodes. The temporal relation
`y[t] = m*x[t] + b` instantiated at `t = 1` produces ground terms referring to
`y[1]`, `x[1]`, etc.; at `t = 2`, distinct ground terms referring to `y[2]`,
`x[2]`. Never merged.

**Consequence:** temporal is not a special axis. It is just indexing that
produces distinct terms. Riley's framing: "y[2] has a structural relation to
y[1] as described by the graph and you can follow the path or paths there."
No temporal layer; just a richer set of nodes in Layer 1 with paths between
indexed instances.

---

## 5. Referential truth & dynamic topology (Phase 2, Q1 — LOCKED)

**Things do not know they are dead.** Entity existence at timestep `T` is
defined by *whether any relation at T references the entity*. No alive/dead
flag. No tombstoning.

### Worked example: bunny #50 in an ecosystem simulator

- **t = 0..999.** Relations like `bunny_50.energy[t+1] = bunny_50.energy[t] -
  spent(...)` produce per-timestep nodes referencing bunny #50. Bunny is
  alive because things talk about it.
- **t = 1000.** An event fires: `fox_j eats bunny_50 at t=1000`. This event
  is itself a relation — a node in the graph. Bunny #50's gravestone.
- **t ≥ 1001.** No new relations reference bunny #50's state. No updates
  happen because nothing updates it.
- Query "where is bunny #50 at t=500?" — relations reference it at that
  timestep; walk them.
- Query "where is bunny #50 at t=1500?" — no relations reference it.
  Answer: absent / no applicable facts.

### Events add facts; never retract

The e-graph is monotonic in facts. Events only ever *add* new nodes and new
equalities (including end-of-existence facts like `eaten(bunny_50, t=1000)`).
They never retract equalities added earlier.

A weaker form of "removal" is that the graph may garbage-collect nodes no
current or future relation references, subject to user memory policy. But
this is memory management, not semantic retraction — the facts remain
"true" for the window they applied to; they are just dropped from memory.

### Lowering to execution

- **JAX-like (big sparse, static shapes):** the compiler allocates N-max
  slots and derives the alive-mask from "which entities have active relations
  at timestep T." Mask falls *out of* the graph; it is not stored on the
  graph.
- **PyTorch-like (dynamic shapes):** per-timestep allocation only for
  entities with active references. Memory freed per user retention policy.

Both lowerings compile the same graph semantics to different execution
substrates. The graph itself is substrate-independent.

### Consequence for retraction semantics

Earlier framing proposed (a) relation-removal-with-rebuild or (b)
tombstoning as options for "retraction." Both wrong. Correct framing:
retraction does not exist; events add facts (including end-of-existence
facts), and the graph's interpretation at any timestep is determined by
which facts reference which entities at that time.

**Resolved follow-up:** the old obligation-retraction framing is replaced by
an adjacent obligation ledger. `fulfills <obligation_key>` selects candidate
fulfillments before e-graph emission; no fact is retracted.

---

## 6. Merge sources — what puts expressions into the same e-class

Enumerated, in roughly descending order of obviousness:

1. **Explicit relation equations.** Every `a = b` in a relation. Non-negotiable.
2. **Observations injected at workflow time.** `provider.bind("y[2]", 5.3)`
   merges `y[2]` with the literal's class.
3. **Algebraic and symbolic-math rewrites.** `x + 0 ≡ x`, `x * 1 ≡ x`,
   commutativity, associativity, distribution. Compiler-supplied. The
   baseline rewrite set is committed in the spec; users can extend via
   workflow configuration (Section 12 open question).
4. **User-declared `identify`.** Direct equivalence declaration (geometric
   seam merges are the canonical case).
5. **Function-inverse pairs.** If `f` and `g` are declared mutual inverses
   (and the round-trip is verified where possible), the rewrites
   `f(g(x)) ≡ x` and `g(f(x)) ≡ x` fire during saturation.
6. **Named-type conversion round-trips.** Bidirectional conversions
   (`convert A <-> B`) register the round-trip equivalence when it is
   provable or (in a fallback path) tested. One-way conversions
   (`convert A -> B`) register only the forward rewrite.
7. **Closure-policy co-membership.** When two relations both define the
   same quantity, both definitions become co-members of one e-class; the
   closure policy selects among them at extraction time. Merge happens
   regardless of policy.
8. **Unit-preserving rewrites.** `5 m + 300 cm ≡ 8 m`. Unit system
   contributes rewrite rules that normalize to canonical form.

### What does NOT merge

- Distinct timesteps of the same symbol (`y[t]`, `y[t+1]`) — distinct
  ground terms.
- Distinct stochastic samples of the same distribution — each `x ~ Normal`
  draw is a distinct draw-node.
- Type-envelope claims (`x: Length` with `≥ 0`) — envelope metadata on the
  class, not a merge.
- Approximate equalities under a tolerance budget that has not authorized
  the merge — candidate rewrite, not registered unless workflow permits.

---

## 7. Faithfulness × orientation — two orthogonal axes (Phase 2, Q2 — PARTIAL LOCK)

Every equality-introducing rule is tagged with a value from each of two
orthogonal axes.

### Axis 1 — Faithfulness (what the user claims)

- **Lossless** — equality is exact. Algebraic axioms, unit normalization,
  declared bijective conversions, observation injection.
- **Lossy-as-model-claim** — user declares two expressions approximately
  equal *under a named modeling assumption*. The assumption is tracked
  through provenance. Example: quasi-steady-state enzyme kinetics.
- **Lossy-as-tolerance** — the equivalence holds within a tolerance budget
  supplied at workflow time. The rewrite is a candidate; it fires only
  when the budget permits. Example: Gaussian kernel truncation at ±3σ.

### Axis 2 — Orientation (which direction the rule fires)

- **Bidirectional** — both `A → B` and `B → A` register.
- **Unidirectional** — only one direction registers during saturation.
  (Note: once a unidirectional rewrite fires, the resulting equality in
  the e-graph is symmetric — this is standard e-graph semantics.
  "Orientation" refers to which rules fire, not whether equalities are
  symmetric.)

### The 2×3 matrix (with representative examples)

| Cell | Physical / semantic | Numeric |
|------|---------------------|---------|
| Lossless × Bi | `convert Celsius<T: Float> <-> Kelvin<T>` · `identify` (seam) · `convert Pa <-> MPa` | `Bool <-> Int{0,1}` · `Rational <-> Float<T>` (when exactly representable) |
| Lossless × Uni | `convert Biomass -> CarbonMass` · `unwrap(Length<Soil>(x)) -> x` | `Int32 -> Int64` · `Float32 -> Float64` · `Int32 -> Rational` |
| Lossy-model × Bi | `approximate Enzymes <-> MichaelisMenten under quasi_steady_state` | `approximate Float64 <-> Fixed<16,16> under precision_regime_X` |
| Lossy-model × Uni | `approximate full_canopy -> beer_lambert under well_mixed_canopy` | `approximate Float64 -> Int under model_says_integer` |
| Lossy-tol × Bi | `approximate exp(x) <-> 1+x+x²/2 tolerance_class small_arg_linearization` | `approximate Float64 <-> Float32 tolerance_class precision_downcast` |
| Lossy-tol × Uni | `approximate full_gaussian -> compact_gaussian tolerance_class kernel_truncation` | `approximate Int64 -> Int32 tolerance_class integer_narrow_when_bounded` |

### Envelope-narrowing interaction

A normally-lossy rewrite can become lossless in context when envelope
metadata (Layer 2) proves the value is in a regime where the loss vanishes.
Example: `Float64 -> Float32` is normally lossy-tolerance; if the e-class
envelope proves `value ∈ [0, 1]`, the conversion is lossless in that
context and fires without consuming the tolerance budget.

This is why the three-layer structure matters: the narrowing is possible
only because Layer 2 travels with the class.

### Relationship to the kernel report's three-tier cut

The kernel report §7 proposed a three-tier cut (lossless / lossy-as-model /
lossy-as-tolerance). That is **one axis of two**; orientation is the
orthogonal axis the kernel report did not call out. The 2×3 framing is a
superset.

---

## 8. Numeric types in the e-graph

The spec hand-waves numeric typing: most mocks write `Scalar<U>` without a
numeric-representation parameter. The e-graph foundation commitment includes
pinning this down.

### Numeric hierarchy (stdlib)

```
Numeric  (contract: +, -, *, /, ==, <, zero, one)
├── Discrete
│   ├── Bool
│   └── Integer
│       ├── Int{8,16,32,64}
│       ├── UInt{8,16,32,64}
│       └── BigInt                     (extension module, not base stdlib)
├── Exact
│   └── Rational                        (stdlib — compiler + user)
├── Continuous
│   ├── Float
│   │   ├── Float{16,32,64}
│   │   ├── BigFloat<precision>         (stdlib, CPU-only, opt-in)
│   │   └── Dual<T>                     (autodiff)
│   └── Complex
│       └── Complex{32,64}
└── Decimal
    └── BigDecimal                      (extension module, not base stdlib)
```

### `Scalar<U, T>` with default `T = Float64`

Quantity types take two generic parameters: unit `U` and numeric
representation `T`. Mocks that write `Scalar<U>` use the `T = Float64`
default.

### Generic conversions with bounds

Conversions take contract bounds on `T`:

```myco
# stdlib — Float only, lossless bidirectional
convert Celsius<T: Float> <-> Kelvin<T>:
    body: value + 273.15

# user extension — opt into integer representation with explicit loss
approximate Celsius<T: Int> -> Kelvin<T>
    under: integer_temperature_truncation
    body: value + 273
```

Compile-time selection: `Celsius<Float32>` matches the stdlib rule;
`Celsius<Int32>` requires the user extension and carries the assumption
tag through provenance. No silent lossiness.

### BigNumber tiers

- **Rational** — stdlib, load-bearing for the compiler. The e-graph uses
  rationals internally for constant folding during saturation to avoid
  float rounding (`0.1 + 0.2 = 0.3` exactly in rational, not in Float64).
  Also user-facing for exact symbolic work.
- **BigFloat** — stdlib, CPU-only. For numerical conditioning investigation.
  Using `BigFloat` in a relation disables GPU lowering for that subgraph;
  error or warning is **Section 12 open**.
- **BigInt / BigDecimal** — extension modules, not base stdlib. The domain
  does not require them by default.

### Saturation termination concern

Rational arithmetic grows denominators unboundedly (coprime additions).
The e-graph needs a **precision cap** or **canonical-form simplification**
to prevent non-terminating saturation. Specific policy is **Section 12 open**.

---

## 9. Vocabulary gap and the proposed `approximate` surface

### The gap

Current v2.1 has syntactic surface for **only** the lossless row of the 2×3
matrix:

- `convert <->` (lossless × bi)
- `convert ->` (lossless × uni)
- `identify ~` (lossless × bi, geometric-scoped)
- Relation `=` (lossless × bi, strict equality claim)

It has **no** syntactic surface for lossy-model or lossy-tolerance, despite
discussing them in prose and depending on them implicitly for closure
policies, kernel optimization, and numerical conditioning. This is a real
gap the commitment must address.

### Proposed `approximate` block

One keyword, four cells of the 2×3:

```myco
approximate <expr_A> <op> <expr_B>:      # op ∈ { <-> , -> }
    under: <assumption_name>              # for lossy-model
    tolerance_class: <tol_name>           # for lossy-tolerance
                                          # exactly one of under / tolerance_class
    error_bound: <bound_expression>       # optional, verified when tractable
    body: <rewrite_definition>            # the actual substitution
    where: <predicates>                   # optional domain restriction
```

### Semantics by case

- `approximate <-> under:` — merges classes with an **assumption tag** on
  the merge edge (Layer 2). Always active; provenance tracks assumption
  usage.
- `approximate -> under:` — registers a forward-only rewrite with
  assumption tag.
- `approximate <-> tolerance_class:` — generates a **candidate rewrite**
  that fires only when the workflow tolerance budget permits. Classes stay
  distinct unless fired.
- `approximate -> tolerance_class:` — same, forward only.

### No fuzzy-equality operator

No `≈` or `~=` inline operator. Inline fuzzy equality hides *why* the
approximation is justified; the `approximate` block forces the user to name
the assumption (`under:`) or tolerance class (`tolerance_class:`), which
is the information the e-graph and provenance system actually need.

---

## 10. Auto-derived lossiness

**The compiler can characterize approximation extent from graph structure.**
This is a compile-time capability, not a new keyword.

### Methods, stacked

1. **Symbolic residual analysis.** Given `A ≈ B`, compute `A - B`
   symbolically (via e-graph rewriting) and simplify. Combined with
   envelope bounds on free variables, this gives concrete numerical bounds
   on approximation error.
2. **Interval arithmetic propagation.** Propagate envelope intervals
   through the expression tree of each candidate path; compare output
   intervals.
3. **Condition number estimation.** Structurally derive how sensitively
   output depends on input perturbation through the expression graph.
4. **Declared `error_bound:`** — fallback when 1-3 cannot be made precise.
5. **Sampling.** Runtime comparison on representative inputs. Last resort.

### Extraction cost becomes a tuple

```
cost = (compute_cost, approximation_loss, ...)
```

Extraction policy combines the tuple per workflow preferences:

```python
run.config.extraction_policy = {
    "compute_weight": 1.0,
    "loss_weight": 10.0,         # prefer less loss over fewer ops
    "loss_cap": 0.01,            # hard ceiling on accumulated loss
}
```

Paths exceeding `loss_cap` are filtered out entirely. Under the cap,
ranking combines compute and loss weighted per the policy.

### The `loss_of(expr)` / `condition_of(expr)` intrinsic

Exposed to:
- User-written closure policies (the deferred `condition_weighted` policy
  uses this directly — it is the machinery that was missing to implement it).
- Diagnostics (`mycoc explain path_A --vs path_B` prints both loss bounds).
- Internal extraction ranking.

Returns a structured result:
`{upper_bound, derivation_method, envelope_used}`.

### Connection to the overdetermined-systems machinery

Closure policies sit at extraction-time over multi-candidate e-classes. The
auto-derived-lossiness capability provides the *ranking input* that
`condition_weighted` needs. This unifies:

- Multiple approximate paths to a value (2×3 framing)
- Overdetermined relations giving the same quantity (closure policies)
- Numerical conditioning investigation (`BigFloat` re-runs)

into one extraction-cost-tuple mechanism.

---

## 11. What is settled versus what is still open

### Settled (this chunk report's scope)

| # | Decision | Section |
|---|----------|---------|
| 1 | E-graph is the internal equality substrate of Myco | 2 |
| 2 | v1 §6.2 scoping sentence recovered verbatim | 2 |
| 3 | Residual graph is a projection of the e-graph, not separate | 2 |
| 4 | Commitment placed in two locations: brief early + detailed late | 2 |
| 5 | Three-layer state split: substrate / envelope / adjacent | 3 |
| 6 | Non-equational constraints live in Layer 2 (envelope), not Layer 3 | 3 |
| 7 | Temporal indexing needs no special machinery | 4 |
| 8 | Entity existence is referential truth: things do not know they're dead | 5 |
| 9 | Events add facts (including end-of-existence); never retract | 5 |
| 10 | Merge sources enumerated (8 categories) | 6 |
| 11 | Faithfulness × orientation as two orthogonal axes (framing B) | 7 |
| 12 | Envelope-narrowing affects contextual lossiness | 7 |
| 13 | Numeric hierarchy sketched; `Scalar<U, T>` with `T = Float64` default | 8 |
| 14 | Rational + BigFloat in stdlib; BigInt / BigDecimal as extensions | 8 |
| 15 | `approximate` block as the syntactic surface for cells 3-6 | 9 |
| 16 | Auto-derived lossiness via stacked symbolic/interval/structural analysis | 10 |
| 17 | Extraction cost becomes a tuple combining compute and loss | 10 |
| 18 | `loss_of(expr)` / `condition_of(expr)` intrinsic exposed | 10 |
| 19 | Round-trip merge verification: trust `convert <->`; hard-error on provably-false declarations; no escape hatch needed | 12 |
| 20 | `.myco` permits literal numerics only in unit defs, affine-conversion bodies, and symbolic π/e; all other values are workflow-bound | 12 (CC1) |
| 21 | `#[verified_externally]` annotation eliminated; O2.1 absorbs it | 12 (CC2) |
| 22 | `~` is distributional metadata at Layer 2, never introduces e-graph merges; envelopes propagate via rewrites | 12 (CC4) |
| 23 | Distribution capabilities decomposed into multiple contracts (`AffineSelfClosed`, `SumSelfClosed`, ...); cross-family rules are rewrite declarations | 12 (CC4) |
| 24 | Three-tier distributional propagation: Tier A closed-form / Tier B approximate / Tier C opaque | 12 (CC4) |
| 25 | Independence via e-graph structural identity; joint declarations reparameterize to independent bases; copulas are Tier C | 12 (CC4) |
| 26 | No naked correlation claims — every distributional claim carries structural content | 12 (CC4) |
| 27 | Rewrite predicates unified: value-bound, structural (site), and future fact-source predicates share one predicate language | 12 (CC5) |
| 28 | Pole L'Hopital and `identify`-seam merges fire as site-scoped rewrites; no new rewrite category | 12 (CC5) |
| 29 | `approximate` tier ships in v2.1 proper; unblocks K1, M1, M2, Z8, Z9 | 11 (O2.3) |
| 30 | Auto-derived lossiness ships full four-layer stack (symbolic / interval / condition / sampling); multi-dimensional `loss_of` with named fields; workflow owns sampling policy; `approximate` block complements derivation; hard error on proven-violated declarations; Y4 unblocked | 11 (O2.4) |
| 31 | `condition_of` ships as a single mode-tagged intrinsic covering Levels I-III; algorithmic conditioning (I/II) is compile-time bound from graph + envelopes; problem conditioning (III) is runtime `condest`-style estimate; duality named in spec; closure policies consume algorithmic mode, workflow diagnostics consume runtime mode | 11 (O2.4 layer 3) |
| 32 | Y4 `condition_weighted` un-deferred and ships in v2.1; closes O4.5 | 11 (O2.2 sub-q 3 / O4.5) |
| 33 | Z-group stdlib scope promoted (continuous + discrete + meta-family tiers); `ScaleSelfClosed` contract added; `SumSelfClosed` reframed as "rewrite registered"; cross-family conjugates expanded; promotions include InverseGamma, Lévy, full extreme-value family, NegBin, Hypergeometric, Mixture sugar, Truncated | 11 (O2.2 sub-q 4) |
| 34 | Design blockers enumerated and split (2026-04-20): B1 opaque log_pdf policy, B2 joint declaration syntax, B4 coupling machinery, B5 matrix types, B6 backend abstraction (absorbs former B3); each must resolve before corresponding deferred families ship | 11 (Design blockers) |
| 35 | Matrix / tensor types committed to v2.1 under completeness preference; chunk 05 shape polymorphism locked (Option C — `Tensor<U, shape>` primitive with `Vector` / `Matrix` shape-refined aliases); unblocks Wishart / InverseWishart, MVN, Level III `condition_of` type surface, Gram matrices, first-class linear-system solves | 05 |
| 36 | Backend abstraction factored into chunk 06 (absorbs B3 Tier C PPL protocol); unified burn-style backend trait covering PPL inference, numerical linear algebra, GPU lowering, opaque callables, and AD ownership; independent of chunk 05 structural questions but concretizes lowering targets for chunk 05 primitives | 06 |

### Open within Phase 2 (merge criterion) — must resolve before the commitment can be fully written

**O2.1 — Round-trip merge verification strategy. RESOLVED (2026-04-20).**

The question collapsed once the `approximate` surface (Section 9) was
introduced. Syntax choice *is* the faithfulness declaration:

- `convert <->` means "user claims lossless bidirectional."
- `approximate <->` means "user claims approximate under assumption or
  tolerance."

The user has already picked the lane. The compiler's job is to honor the
declaration, not re-adjudicate it.

**Resolution:** trust `convert <->` declarations; the compiler runs a
bounded counterexample search (symbolic reduction + light sampling) at
compile time; provably-false declarations are **hard errors**; inconclusive
checks accept the declaration silently. No `trust_me_bro` escape hatch
needed because nothing is being refused — the compiler is only catching
lies, not adjudicating unprovable truths.

**Cases:**
- *Kelvin/Celsius.* Symbolic reduction proves identity → merge registered,
  no diagnostics.
- *Float32/Int32 wrongly declared `convert <->`.* Symbolic reduction finds
  `round(3.7) ≠ 3.7` counterexample → hard error; user must fix to
  `approximate -> under: rounding_loss`.
- *LatLon/Mercator.* Symbolic reduction inconclusive → merge registered;
  optional property-test run at workflow layer if user opts in.

**Key insight (Riley):** the `approximate` surface absorbed the "what if
it's actually approximate" question. What remains is catching
*misdeclarations* — which is proof of negation (one counterexample
suffices), vastly easier than the original proof-of-identity framing
implied.

**O2.2 — The baseline rewrite set. PARTITION DRAFTED (2026-04-20).**

Decision principle: *"commit a baseline that covers everything the
v2.1 spec and open questions actually need; reserve the right to add in
future versions; allow users to configure rewrites from the Python workflow
layer."* Stronger philosophical anchor from Riley: *"I want all of these
in v2.1 before we begin implementing. If you think there are some worth
deferring until we've covered more of the open questions that's fine by me,
it should just be recorded that we need to come back to them."*

The audit enumerated ~60 rewrites across 25 groups (A-Y) in the appendix.
CC4's resolution adds a new group Z (distributional envelope propagation).
Partition below.

### Bucket 1 — Fully committed, no blocking dependencies (~45 rules)

All gating uses value-bound envelope predicates (committed), site-scoped
predicates (CC5 locked), or surfaces already locked in v2.1:

- **A1-A10** — ring/field axioms
- **B1** — literal constant folding
- **C1-C4** — unit/dimensional normalization
- **D1-D5** — named-type normalization (requires O2.1 round-trip
  verification, locked)
- **E1-E2** — function-inverse round-trip (requires function registry,
  present in spec)
- **F1** — `identify`-seam merge for scalar fields (unblocked by CC5)
- **G1-G3** — transcendental simplifications
- **H1-H2** — aggregate/collection identities
- **I1** — conditional rewrites
- **J1** — temporal anti-rewrite (non-rewrite invariant, resolved in
  Section 5)
- **L1** — `smooth_min` → `min` forward-only
- **O1** — train-mode consistency-objective substitution (residual
  identity resolved by `ResidualSite`)
- **Q1-Q2** — probabilistic truncation/marginalization (unblocked by CC4)
- **R1-R3** — lossy-function simplification (`abs`, `max`/`min`, `floor`/
  `relu`/`clamp` forward-only under envelope)
- **S1-S2** — opaque callable forward edges
- **T1** — one-way `convert`
- **U1-U3** — named-type stripping under arithmetic
- **V1** — observation injection (CC1 expands scope: workflow-bound
  constants enter via V1-style equalities)
- **X1** — pole L'Hopital structural rewrite (unblocked by CC5)
- **Y1-Y3** — `weighted_average`, `soft_select`, `hard_select` closure
  policies
- **Y4** — `condition_weighted` closure policy (unblocked via O2.4's
  `condition_of` Level I/II algorithmic-conditioning mode)
- **Y5** — user-defined custom closure policy (extensibility surface)
- **Y6** — C(N,M) enumeration for overconstrained blocks (with
  combinatorial-blowup warning threshold)

### Bucket 2 — New additions from CC4 (Group Z — distributional envelope propagation)

These did not exist in the audit because the audit predates the CC4
resolution. They are the rewrites that let envelopes propagate through
deterministic operations acting on stochastic operands.

- **Z1** — Affine of Normal → Normal, via `AffineSelfClosed`
- **Z2** — Sum of independent Normals → Normal, via `SumSelfClosed` +
  e-graph structural-independence check
- **Z3** — Product of independent Log-Normals → Log-Normal, via
  `ProductSelfClosed`
- **Z4** — Sum of same-rate Gammas → Gamma; sum of Poissons → Poisson;
  sum of χ² → χ² (`SumSelfClosed` on respective families)
- **Z5** — Affine of Cauchy / Student-t / Uniform → same family
- **Z6** — Exp of Normal ↔ Log-Normal (inverse pair)
- **Z7** — Conjugate posterior updates (cross-family rewrite
  declarations): Normal-Normal (known σ), Beta-Bernoulli, Gamma-Poisson,
  Dirichlet-Multinomial
- **Z8** — Delta method: smooth nonlinear transform → Normal
  (`SmoothTransformable`) — **Tier B, depends on O2.3**
- **Z9** — Fenton-Wilkinson: sum of Log-Normals → approximate Log-Normal
  via moment matching — **Tier B, depends on O2.3**
- **Z10** — MVN auto-reparameterization via Cholesky to independent
  bases

**Z-group stdlib scope — RESOLVED (2026-04-20) under completeness
preference:** *"i would rather go for correctness and completeness in
the spec so we don't end up with loose ends everywhere."* Scope is
promoted substantially beyond the initial proposed minimum; items that
require substantial new design (not just implementation volume) are
explicitly deferred with named blockers (see §11 "Design blockers
B1-B5" below).

### Contract refinements

- **Add `ScaleSelfClosed`** contract alongside `AffineSelfClosed`.
  Positive-support families (LogNormal, Gamma, Exponential) close
  under positive scaling `a·X` but break under shift (support bounded
  from below). `AffineSelfClosed` implies `ScaleSelfClosed`. No
  separate `ShiftSelfClosed` (empty meaningful case).
- **Frame `SumSelfClosed` as "a sum rewrite is registered."** Some
  families sum-close unconditionally (Normal, Cauchy, Poisson);
  others with matching parameters (Gamma same-rate, Binomial same-p,
  ChiSquared always-closes-dof-adds). Parametric predicates live on
  the rewrite rule (CC5 site-scoped predicate machinery), not as
  additional contracts. Keeps contracts all-or-nothing per CC4 while
  allowing conditional rewrites.

### Tier 1 v2.1 stdlib — continuous univariate

| Family | Dist | Reparam | AffineSC | ScaleSC | SumSC | ProductSC | SmoothXform |
|--------|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Normal | ✓ | ✓ | ✓ | ✓ | ✓ | — | ✓ |
| LogNormal | ✓ | ✓ | — | ✓ | —¹ | ✓ | ✓ |
| Uniform | ✓ | ✓ | ✓ | ✓ | — | — | ✓ |
| Beta | ✓ | ✓ | — | — | — | — | ✓ |
| Gamma | ✓ | ✓² | — | ✓ | ✓³ | — | ✓ |
| Exponential | ✓ | ✓ | — | ✓ | ✓³ | — | ✓ |
| ChiSquared | ✓ | ✓² | — | ✓ | ✓ | — | ✓ |
| Cauchy | ✓ | ✓ | ✓ | ✓ | ✓ | — | —⁴ |
| Student-t | ✓ | ✓² | ✓ | ✓ | — | — | ✓⁵ |
| Laplace | ✓ | ✓ | ✓ | ✓ | — | — | ✓ |
| HalfNormal | ✓ | ✓ | — | ✓ | — | — | ✓ |
| HalfCauchy | ✓ | ✓ | — | ✓ | — | — | —⁴ |
| **InverseGamma** | ✓ | ✓² | — | ✓ | — | — | ✓ |
| **Lévy** | ✓ | ✓ | ✓ | ✓ | ✓ | — | —⁴ |
| **Weibull** | ✓ | ✓ | — | ✓ | — | — | ✓ |
| **Pareto** | ✓ | ✓ | — | ✓ | — | — | ✓ |
| **Fréchet** | ✓ | ✓ | — | ✓ | — | — | ✓ |
| **Gumbel** | ✓ | ✓ | ✓ | ✓ | — | — | ✓ |
| **GEV** | ✓ | ✓ | ✓ | ✓ | — | — | ✓ |

Notes:
1. Log-normal sum via Fenton-Wilkinson — Tier B rewrite (Z9), not
   self-closure.
2. Implicit reparameterization (RSVI-style); standard PPL practice.
3. Sum closes with same rate — predicate on the rewrite.
4. Infinite variance (Cauchy, HalfCauchy, Lévy) — delta method
   formally fails. Intentionally excluded from `SmoothTransformable`.
5. Student-t `SmoothTransformable` only for ν > 2 — rewrite-level
   predicate.

**Bold** = promoted from the original deferral list under Riley's
completeness preference.

### Tier 1 v2.1 stdlib — discrete univariate

| Family | Dist | Reparam¹ | SumSC | Notes |
|--------|:---:|:---:|:---:|-------|
| Bernoulli | ✓ | — | —² | Sum of iid → Binomial (cross-family) |
| Binomial | ✓ | — | ✓³ | Same-p predicate |
| Poisson | ✓ | — | ✓ | Unconditional |
| Categorical | ✓ | — | — | |
| Geometric | ✓ | — | — | |
| **NegBinomial** | ✓ | — | ✓³ | Same-p predicate; count-overdispersion |
| **Hypergeometric** | ✓ | — | — | Sampling without replacement |

1. Discrete families don't satisfy `ReparameterizedSampleable`.
   Gradient-based inference routes to score-function or Gumbel-softmax
   via PPL backend; not a stdlib contract.
2. Bernoulli same-p sum → Binomial (cross-family rewrite).
3. Same-p predicate.

### Tier 1 v2.1 stdlib — multivariate

**Conditional on chunk 05 (matrices) completing:**

| Family | Dist | Reparam | AffineSC | Notes |
|--------|:---:|:---:|:---:|-------|
| MVN | ✓ | ✓ (Cholesky) | ✓ | Auto-reparameterized via Z10; needs matrix chunk |
| Dirichlet | ✓ | ✓¹ | — | Conjugate for Multinomial |
| Multinomial | ✓ | — | — | |

1. Stick-breaking / implicit reparameterization.

Dirichlet and Multinomial do not require matrix machinery (they're
vector-valued); they can ship independently. MVN requires
`cholesky(Σ)` from chunk 05.

### Tier 1 v2.1 stdlib — meta-families

- **`Truncated<D>`** — wraps any `Distribution<U>` with
  refinement-type bounds; Tier A closed-form log-pdf via CDF
  normalization (covers audit Q2 probabilistic truncation). Ships.
- **`Mixture<D₁, ..., D_N | weights>`** — syntactic sugar over the
  existing latent-discrete + Q1 auto-marginalization pattern. Reduces
  to `component ~ Categorical(w); x ~ D[component]` under the hood.
  Ships as sugar.

### Cross-family rewrites shipping in v2.1

**Conjugate posterior updates (Z7):**
- Normal-Normal (known σ) → Normal posterior
- Beta-Bernoulli → Beta posterior
- Beta-Binomial → Beta posterior
- Gamma-Poisson → Gamma posterior
- Dirichlet-Multinomial → Dirichlet posterior
- Dirichlet-Categorical → Dirichlet posterior
- **Normal-InverseGamma** → InverseGamma posterior (variance)
- **NormalInverseGamma** (joint μ, σ²) → NormalInverseGamma posterior
  — flag: verify rewrite-pattern language handles joint priors; add
  check to chunk 04 Phase 3 topic-list pass
- **Gamma-Gamma** → Gamma posterior

**Family equivalences (strict rewrites, both directions where
meaningful):**
- `exp(Normal(μ, σ²))` ↔ `LogNormal(μ, σ²)`
- `log(LogNormal(μ, σ²))` → `Normal(μ, σ²)` (one-way by construction)
- `Exponential(λ)` ↔ `Gamma(1, λ)`
- `ChiSquared(k)` ↔ `Gamma(k/2, 2)`
- Sum of n iid `Bernoulli(p)` → `Binomial(n, p)`
- `1/Gamma(α, β)` ↔ `InverseGamma(α, β)`

**Tier B (approximate) rewrites:**
- Z8 delta method (any `SmoothTransformable` family + smooth
  nonlinear transform)
- Z9 Fenton-Wilkinson (LogNormal sum)
- CLT — large-n sum → approximate Normal (predicate: `n > threshold`)
- **Block-maxima → GEV** — `max(X_1, ..., X_n) ~ GEV` as `n → ∞`,
  analogous to CLT. Uses `approximate` surface.

### Deferred — requires substantial new design (not implementation volume)

Keyed to design-blocker list in §11 below. All five blockers must be
designed before the corresponding families can ship.

| Item | Blocker |
|------|---------|
| General α-stable (non-Lévy, non-Cauchy, non-Normal α) | B1 — Opaque-log_pdf stdlib policy |
| Copula<F, G, ...> and all coupling machinery | B2 — Joint declaration syntax; B3 — Tier C PPL backend protocol; B4 — Coupling machinery |
| Wishart / InverseWishart | B5 — Matrix types (chunk 05) |
| MVN log_pdf / Cholesky reparameterization (Z10) | B5 — Matrix types (chunk 05) |
| Level III runtime `condition_of` for solve blocks | B5 — Matrix types (chunk 05) |

**Also deferred to v2.2+ (lower priority, not blocker-driven):**
- Stable distributions with non-closed-form characteristic-function
  tricks beyond α-stable proper
- Matrix-Normal, generalized Wishart variants
- Sklar-theorem-based copula decomposition as a rewrite rule

### Bucket 3 — Committed but gated on specific open resolutions

These ship in v2.1 with the commitment stating "activates when X resolves":

| Rule | Gated on | Notes |
|------|----------|-------|
| K2 | Kernels chunk report | Kernel separability; needs kernel-properties surface |
| L2 | Smoothing-surface lock | `where` → sigmoid in user-written smooth form only |
| N1 | Kernels chunk §6 | Numerical quadrature substitution; needs integration semantics |
| P1 | Open-questions §B.3 item 9 | Mesh discretization stencils — architectural call: e-graph rewrite vs pre-e-graph codegen |
| W1 | O4.1 | obligation fulfillment ledger; resolved in spec_new.md §8.11 / §10.5 |

**O2.3 resolved (ship):** K1 (kernel compact-support truncation), M1
(first-order Taylor), M2 (high-order drop), Z8 (delta method), Z9
(Fenton-Wilkinson) move to Bucket 1 (fully committed).

### Bucket 4 — Absorbed or deferred

- **B2** (universal-to-literal substitution) — **absorbed** by CC1 + V1.
  Workflow-bound constants enter as observation-style equalities at
  composition time. No longer a compile-time fold.
- **K3** (low-rank kernel SVD / Nyström / random Fourier features) —
  **deferred to v2.2+**. Speculative; machinery exists but concrete
  rewrites not urgent.
- **Z families beyond proposed stdlib minimum** (Weibull, Pareto,
  Wishart, copula machinery) — **deferred to v2.2+** unless explicitly
  widened.

### Sub-questions still open inside O2.2

1. ~~O2.3 ship call.~~ **RESOLVED (2026-04-20): ship in v2.1.** Unblocked
   K1, M1, M2, Z8, Z9.
2. ~~O2.4 ship call.~~ **RESOLVED (2026-04-20): ship full four-layer
   stack.** Multi-dimensional `loss_of`; workflow-side sampling policy;
   `approximate` block complements derivation; hard error on
   proven-violated declarations. Unblocked Y4's missing machinery.
3. ~~Y4 un-defer call.~~ **RESOLVED (2026-04-20): un-defer and ship.**
   `condition_of` ships with Levels I-III as a single intrinsic; Y4
   consumes the algorithmic-conditioning mode. See condition-number
   scope lock below.
4. ~~Z-group stdlib scope.~~ **RESOLVED (2026-04-20): promoted scope
   under completeness preference.** Added `ScaleSelfClosed` contract;
   reframed `SumSelfClosed` as "sum rewrite registered"; promoted
   InverseGamma, Lévy, Weibull, Pareto, Fréchet, Gumbel, GEV,
   NegBinomial, Hypergeometric, Mixture sugar, Truncated meta-family;
   added Normal-InverseGamma, NormalInverseGamma, Gamma-Gamma
   conjugates; added block-maxima-GEV Tier B rewrite. Deferrals
   (α-stable, copulas, Wishart/InverseWishart, MVN, Level III runtime
   `condition_of`) all keyed to design blockers B1-B5 (§11) that must
   resolve first. See Z-group stdlib scope table in Bucket 2 above.
5. **K2 / N1 kernels-chunk dependency.** Confirm commit-pending-kernels-
   resume (not defer-to-v2.2).
6. **P1 mesh discretization.** Open-questions §B.3 item 9 architectural
   call — resolve inline with O2.2 or keep as standalone open?

Two remaining sub-questions close O2.2 when answered. (Plus design
blockers B1-B5 — see immediately below. Those are prerequisite work,
not strictly part of O2.2, but they must land before parts of the
Z-group scope become concrete.)

### Design blockers — B1-B5

The Z-group stdlib scope resolution (sub-question 4) promoted most
families to Tier 1 but deferred five items that require substantial
new design, not just implementation volume. Each is tracked as a
design blocker here; proceeding past v2.1 planning without resolving
these produces refactor pain later.

**B1 — Opaque `log_pdf` stdlib policy.** Does stdlib permit
distribution families whose `log_pdf` is a structurally-opaque
numerical evaluator (inverse-FFT of characteristic function,
table-lookup, etc.) rather than a symbolic expression?

- Driving family: general α-stable (only characteristic function is
  closed; PDF requires numerical inverse-FFT).
- Consequences: layer-3 `condition_of` can't analyze through opaque
  `log_pdf`; gradient-based inference (HMC, NUTS, VI) requires
  autodiff through the numerical evaluator; sets precedent for future
  opaque-log_pdf families.
- Design items: (a) should stdlib ship these at all, or route through
  user-declared `approximate` blocks? (b) if yes, what autodiff
  infrastructure is required? (c) how do they interact with Tier C
  PPL backend routing?
- **Status:** open. Not blocked on anything else.

**B2 — Joint distribution declaration syntax.** How do users declare
joint distributions whose components have non-factorizable dependence
in `.myco`?

- Driving use case: copulas. Covered cases
  (shared-ancestor-correlation, decomposable joints via MVN, and
  Tier C opaque declarations) are locked in CC4; the syntactic
  surface for declaring them isn't.
- Three candidate shapes surveyed (`couple(x, y) via ...`,
  `(x, y) ~ JointCopula(...)`, explicit `u ~ Uniform; x = inv_cdf(F_x, u)`).
- Integrates with: `~` distributional claim, CC1 (copula params
  workflow-bound), B6 / chunk 06 PPL backend protocol, MVN/matrix
  types once B5 resolves.
- **Status:** open. Will land in a future chunk 07 paired with B4.
  Blocks B4.

**B3 — Tier C PPL backend protocol. ABSORBED (2026-04-20) into B6 /
chunk 06 backend abstraction.** Factoring rationale: CC4's Tier C
routing is one of several backend-routing concerns (alongside
numerical linear algebra in chunk 05, GPU lowering for collections,
opaque callables via `bind_controller`, and AD ownership). Designing
it in isolation would have produced a PPL-specific protocol that
couldn't leverage shared backend infrastructure. Chunk 06
(`06_backend_abstraction_in_progress.md`) owns the unified backend
trait surface; PPL protocol is §4.4 there.

**B4 — Coupling machinery (non-independence in envelope).** CC4
locked independence via e-graph structural identity — but copulas
explicitly couple quantities with distinct stochastic ancestors. The
envelope / rewrite machinery needs:

- Coupling annotations that cross ancestor boundaries.
- Rewrite suppression: if `x` and `y` are copula-coupled, `x + y`
  cannot use `SumSelfClosed` even when both are Normal (result isn't
  Normal unless correlation ≠ 0 is absorbed).
- Distinction between "MVN-style reparameterizable" coupling (Z10
  handles it) and "irreducibly joint" coupling (Tier C).
- Algorithm for detecting when independence-assuming rewrites may fire
  vs. must suppress.
- **Status:** open. Will land in a future chunk 07 (joint distributions
  / coupling) paired with B2 (joint syntax). Blocked on B2 (syntax
  determines what coupling looks like in the graph), B5 / chunk 05
  (MVN-case needs matrix types), and B6 / chunk 06 (Tier C handoff
  protocol).

**B5 — Matrix / tensor types and linear-algebra primitives.** First-
class matrix/tensor types with decompositions, norms, solves,
structural subtypes.

- Driving items: Wishart / InverseWishart, MVN log_pdf + Cholesky
  reparameterization (Z10), Level III runtime `condition_of` (type
  surface), Gram matrices for kernel methods, linear-system solves
  as first-class modeling primitive, Jacobian-based inference.
- Audit confirms (2026-04-20) no matrix machinery in v2.1 today.
  MVN, Wishart, and linear-algebra primitives explicitly deferred in
  `open_questions.md:853-859`.
- **Status:** chunk report locked at
  `planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md`.
  Matrix / tensor source semantics are now committed: `Tensor<U,
  shape>` primitive; `Scalar`, `Vector`, and `Matrix` rank-refined
  spellings / aliases; compiler-facing matrix facts; envelope views;
  structural fact lattice; tensor `convert`; dynamic shape phases;
  finite matrix assembly; and primitive fact contracts. Backend
  execution remains chunk 06.

**B6 — Backend abstraction.** Unified backend-routing surface for
PPL inference (absorbs former B3), numerical linear-algebra
execution, GPU lowering, opaque callables, and AD ownership.

- Factoring rationale: CC4 Tier C routing, chunk 05 matrix primitive
  execution, chunk 02 collection aggregation GPU kernels, and
  `bind_controller` callable gradient-flow all share one underlying
  concern — workflow selects a backend at run-time configuration,
  compiler emits backend-agnostic IR, backend lowers to concrete
  kernels (burn-style trait pattern).
- Primary open questions: AD ownership fork (Myco-owned symbolic
  vs. backend-delegated vs. hybrid — lean hybrid), minimum backend
  trait API vs capability-advertised optional ops, fallback policy
  (error / host / emulate), PPL backend protocol concrete form
  (compiler-emitted + backend-returned + serialization), opaque
  callable gradient-flow semantics, mixed-backend policy (lean
  single-backend-per-run for v2.1), versioning.
- **Status:** stub chunk report created at
  `planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md`.
  Independent of chunk 05 structural questions but needed for
  chunk 05 primitives to have concrete lowering targets.

### Recommended ordering for B1-B6

Post-split structure (2026-04-20 update): B3 absorbed into B6
(backend abstraction chunk 06); B2 + B4 remain paired for a future
chunk 07 (joint distributions / coupling).

Dependencies suggest:

1. **B5 (matrix types, chunk 05).** Foundational type-system
   extension. Independent of the others on type-system questions.
   Unblocks MVN, Wishart, Level III `condition_of` type surface,
   kernel methods in one pass. Shape polymorphism already locked
   (Option C); remaining questions are heterogeneous units,
   envelope flavors, structural subtypes, shape refinements.
2. **B6 (backend abstraction, chunk 06) in parallel with B5.**
   Independent of B5 on structural questions but concretizes what
   chunk 05's primitives lower to. Absorbs B3. Primary fork is AD
   ownership (lean hybrid).
3. **B1 (opaque log_pdf policy) in parallel with B5 and B6.**
   Orthogonal policy decision with small implementation footprint.
4. **B2 + B4 (future chunk 07, joint distributions / coupling)
   after B5 and B6.** Joint syntax + coupling machinery. Depends on
   B5 (MVN/matrix-coupling sub-case) and B6 (Tier C handoff
   protocol). Probably cannot start in earnest until both land.

This ordering keeps B5, B6, B1 parallelizable (three independent
threads) and serializes only the genuinely dependent coupling
chunk.

**O2.3 — Baseline `approximate` tier. RESOLVED (2026-04-20): SHIP in v2.1.**

Riley's call: *"yeah let's do it. ship this, seems well-reasoned and
principled, unlocks a lot of stuff."*

The `approximate` block surface from Section 9 is committed v2.1 syntax.
Users can write `approximate <expr_A> <op> <expr_B>` with `under:`,
`tolerance_class:`, `error_bound:`, `body:`, and `where:` fields; compiler
honors these as approximate-tier rewrites with declared error bounds;
extraction-time cost tuples factor the bounds.

**Unblocked rewrites** (move from Bucket 3 to Bucket 1 in O2.2 partition):
K1 (kernel compact-support truncation), M1 (first-order Taylor), M2
(high-order term drop), Z8 (delta method), Z9 (Fenton-Wilkinson).

**O2.4 — Auto-derived lossiness tier. RESOLVED (2026-04-20): SHIP full stack
in v2.1.**

Riley's call: *"yeah that all looks good to me. wanna lock it in?"* after
walking through the design. All four layers of Section 10's stack ship in
v2.1 proper. Resolution has five parts:

1. **Ship all four derivation layers** — symbolic residual analysis,
   interval arithmetic propagation, condition-number estimation, and
   runtime sampling. They stack: the compiler tries layer 1 first,
   falls through as each becomes inconclusive. Each layer produces
   bounds with a recorded `derivation_method`.

   **Layer 3 (condition-number estimation) ships with Levels I-III
   under a single `condition_of` intrinsic.** The three levels reflect
   the **algorithmic/problem duality** in numerical analysis:

   - **Level I (algorithmic, scalar).** Scalar expressions w.r.t.
     scalar inputs. Symbolic ∂e/∂x_i propagated through layer 2
     interval arithmetic. Compile-time bound. Example: choosing
     between `(exp(x) - 1)/x` and `expm1(x)/x` — the latter is
     better-conditioned near `x=0`.
   - **Level II (algorithmic, vector).** Vector-valued expressions
     w.r.t. vector inputs. Jacobian operator-norm bounds via same
     machinery, aggregated by user-selectable norm. Compile-time
     bound.
   - **Level III (problem conditioning).** For linear-solve / implicit
     SCC blocks where `K p = b` or similar, κ(K) is estimated at
     runtime via `condest` on the assembled matrix at the current
     linearization point. Compile-time returns a mode marker; actual
     κ is filled in at runtime. Not a compile-time bound — interval
     condition number of an interval matrix is too loose to use.
     **Level III runtime machinery is gated on B5 (matrix types,
     chunk 05);** the compile-time mode marker and API shape ship in
     v2.1 either way, but the actual `condest` lowering requires
     matrix operator-norm primitives.

   **`condition_of` returns a mode-tagged struct:**
   `{mode: "compile_bound" | "runtime_estimate", ...}`. The mode
   makes the algorithmic/problem distinction explicit without
   requiring the user to juggle two primitives. Same intrinsic,
   documented duality.

   **Consumer mapping:**
   - Closure policies (Y4 `condition_weighted`, user-defined via Y5)
     read `compile_bound` mode from algorithmic candidates (Levels
     I-II). Level III doesn't surface here because closure policies
     rank candidate algorithms, not candidate problems.
   - Workflow-side solver diagnostics and runtime hooks read
     `runtime_estimate` mode (Level III). Used for logging,
     ill-conditioning escalation (`BigFloat` re-run), preconditioner
     selection.
   - Extraction-time ranking consumes `compile_bound` only.
   - `mycoc explain` shows both modes with labels.

   **Documentation obligation.** The spec's condition-number section
   must name the algorithmic/problem duality and show one worked
   example of each mode. Avoiding the naming makes the polymorphism
   feel ad-hoc; naming it makes it principled.

2. **Sampling parameters are workflow-side.** Non-determinism,
   seed management, and sample budget are workflow concerns, not world
   claims — consistent with CC1 (no workflow values in `.myco`):

   ```python
   run.config.loss_estimation = {
       "sampling": {"n_samples": 1000, "seed": 42,
                    "strategy": "stratified"},
       "interval_arithmetic": {"precision_bits": 64},
       "condition_number": {"norm": "L2", "regularization": 1e-12},
       "policy": "strict",  # or "permissive"
   }
   ```

3. **`approximate` block stays as a user surface.** The compiler cannot
   always derive tight bounds from graph structure alone. Users retain
   the `approximate` block from Section 9 for cases where:
   - **Domain knowledge.** Physicist knows asymptotic behavior the
     compiler cannot prove (e.g., Reynolds-number-dependent turbulence
     closure error bounds).
   - **Black-box terms.** Opaque callables (neural controllers, external
     solvers) have no symbolic form for the compiler to analyze.
   - **Literature-sourced bounds.** Published error analysis for a
     specific method (e.g., Fenton-Wilkinson's second-moment error on
     Log-Normal sums).
   - **Tolerance budgets.** User declares a target tolerance and the
     compiler verifies the approximation falls within it.

   The `approximate` surface and auto-derivation are **not redundant**;
   they are complementary. Auto-derivation handles what the compiler can
   prove; `approximate` handles what the user knows and the compiler
   cannot prove.

4. **Declaration/derivation interaction semantics.** When both exist for
   the same expression, the compiler checks consistency:

   - *Compiler proves exact (error = 0).* Promote to Tier A silently.
     User's `approximate` declaration is retained as a weaker fact but
     extraction treats the path as lossless. Diagnostic available on
     request.
   - *Compiler proves bound within user declaration.* Honor the user's
     declaration. The declared bound governs extraction cost even if
     the compiler-derived bound is tighter. Rationale: user is
     asserting a modeling commitment, not a measurement.
   - *Compiler proves declaration violated* (derived bound exceeds
     declared bound under the stated envelope). **Hard error.**
     Consistent with O2.1's policy for `convert <->` misdeclarations:
     lies are caught; unprovable claims are accepted.
   - *Compiler inconclusive* (layers 1-3 fail; layer 4 sampling within
     budget agrees with declaration or cannot refute it). Accept the
     declaration with a diagnostic. Workflow `policy` knob controls
     whether to require stricter evidence ("strict") or accept
     ("permissive"). Default: permissive.

5. **Loss is multi-dimensional.** Extraction cost is not a scalar. Named
   dimensions:

   ```
   cost = {
       compute: ...,
       approximation: ...,  # |A - B| bound from layers 1-4
       condition: ...,      # sensitivity to input perturbation
       truncation: ...,     # series/integral truncation error
       discretization: ..., # mesh/quadrature error
   }
   ```

   `loss_of(expr)` returns a struct of named fields, not a scalar.
   Closure policies and extraction may operate on any subset — e.g.,
   `condition_weighted` consumes the `condition` field; tolerance-
   budgeted extraction consumes `approximation`. Section 10's cost-tuple
   framing is generalized to this named-field form. Aggregation to a
   scalar for ranking happens only at the final extraction step under
   workflow-configured weights.

**Unblocked immediately:**

- **Y4 (`condition_weighted` closure policy)** — the missing machinery
  was `condition_of(expr)`. Layer 3 provides it. Y4 un-defer now gated
  only on the stdlib-scope question (sub-question 3 below).
- **Workflow surface for loss policy** — `run.config.extraction_policy`
  and `run.config.loss_estimation` are v2.1 workflow verbs.

**Reframed.** Section 10's description of cost-as-tuple is accurate but
incomplete; the named-field generalization supersedes it. Section 10
will be updated during spec-writing to reflect the multi-dimensional
framing.

### Resolved — Phase 2, Q3 (residual ↔ e-graph relationship)

Resolved in `spec_new.md` §19.2 and §25 by separating residual identity
from residual computation.

- `ResidualSite` records the user-facing claim / obligation identity:
  relation name, obligation key, source / workflow provenance, units,
  axes, e-class anchors, status, and objective-term kind.
- `ResidualRealization` records the extracted executable expression or
  block selected under `cost_of`.
- Extraction may share realizations through algebraic simplification
  and CSE, but must not merge site identities.
- Diagnostics expose both views: by site for source claims and by
  realization for executable compute / cost / backend lowering.
- Workflow objective aggregation consumes residual sites, and fit /
  inference / score composition errors on unhandled active residuals
  rather than inventing a hidden least-squares objective.

### Open — Phase 2, Q4 (knowledge envelope ownership) — not yet started

- What owns envelope metadata: the class, the node, or a symbol table keyed
  by class?
- Merge rules for envelope (intersection for constraints, union for
  provenance, max for uncertainty?).
- Conflict resolution when merge produces an inconsistent envelope (empty
  intersection of constraints).
- Exposure of envelope to user code (can a user query "what constraints
  apply to this e-class"?).

### Open — Phase 3 (apply foundational decisions to the topic list)

Each of the following surfaces needs a pass that articulates its e-graph
semantics:

- **Units** — unit-preserving rewrites; unit normalization as merge
  source; unit algebra as rewrite system.
- **Types with From/Into** — the audit found no From/Into surface in v2.1;
  the unified surface is `convert`. Needs reconciliation with
  function-registry's four invertibility classes
  (`bijective`, `injective_restricted`, `lossy`, `opaque`) per
  `spec.md:2006-2021` and two-level inverse-verification protocol at
  `spec.md:2147-2172`.
- **Over- and underdetermined systems** — closure policies as extraction
  over e-classes; how underdetermination surfaces when extraction has
  insufficient information (symbolic return versus error).
- **Symbolic math** — how much of symbolic simplification sits in the
  baseline rewrite set; what derivative rewrites (`deriv` intrinsic) look
  like.
- **Collections** — map/reduce/fold over e-graphs of indexed terms;
  lowering to vectorized execution.
- **Continuous versus discrete representation** — how the e-graph handles
  the integration-semantics question from kernel report §6.
- **Functions and contracts** — opaque callables as rewrite blockers;
  contract-generic functions as rewrite-rule families instantiated per
  concrete type.

### Open — Phase 3, cross-cutting additions beyond original list

- **Temporal** (mostly covered in Section 4 and 5; a pass to confirm
  nothing else needs to change).
- **Stochastic `~`** — currently a rewrite blank. Until the stochastic
  semantic invariant is stated (open question Tier 0 §B.3 item 6), no
  rewrites can fire across `~` edges, including unit normalization on a
  stochastic quantity's parameters. This is a bigger blocker than it
  looks.
- **Inequalities** — envelope metadata handles static inequalities;
  conditional inequalities (e.g., "`x ≥ 0` when event E has fired") need
  their own semantics.
- **Opaque callables** — blockers for rewriting; still allow substitutivity
  (`f(a) ≡ f(b)` when `a ≡ b`).
- **Events** — see Phase 4 below.
- **SCCs** — derived from the e-graph at lowering time; the decomposition
  is not itself in the graph.
- **Learning targets** — training emission must preserve original relation
  names through extraction (per-residual exposure); see Section 13.

### Open — Phase 4 (audit-specific items)

**O4.1 — Obligation fulfillment ledger. RESOLVED (2026-04-26).** The
deletion framing is gone. Compiler/package rules emit `ObligationSite`s;
relations, temporal blocks, event effects, and package defaults provide
candidate fulfillments. The ledger selects explicit/default candidates before
layer-1 e-graph emission, retains suppressed defaults for diagnostics, and
never retracts an already-emitted fact. Canonical text lives in
`spec_new.md` §8.11 / §10.5.

**O4.2 — Pole L'Hopital and `identify`-seam as structural-predicate-gated
rewrites. RESOLVED (2026-04-20) via Section 12 CC5.** Resolved as
site-scoped rewrite predicates: rewrite predicates unified across
value-bound and structural fact sources; no new rewrite category. The 2×3
faithfulness × orientation matrix is preserved; structural facts join
envelope facts as one more predicate source. Extensible to future fact
sources (SCC role, temporal index, event state, refinement type) without
taxonomic sprawl.

**O4.3 — Per-residual exposure for training emission. RESOLVED.**
Overconstrained relations survive extraction as `ResidualSite`
identities. Standard CSE-style canonicalization may still share the
chosen `ResidualRealization`, but the source relation name / obligation
key remains on the site and is what `objective_terms(residual)` consumes.
Workflow composition errors on active residual sites that are not handled
by objective, exact, projection, inference, provider-check, or explicit
ignore policy.

**O4.4 — Stochastic `~` as a rewrite blank. RESOLVED (2026-04-20) via
Section 12 CC4.** `~` is distributional metadata at Layer 2, never
introduces merges; envelopes propagate via rewrites with three tiers
(closed-form / approximate / opaque); capability decomposition via
contracts; independence via e-graph structural identity. What remains are
stdlib-scope questions (which families ship with which contracts, which
cross-family rules ship with v2.1), which fold into O2.2's baseline
rewrite-set partition.

**O4.5 — `condition_weighted` closure policy. RESOLVED (2026-04-20) via
O2.4.** `condition_of` ships Levels I-III as a single mode-tagged
intrinsic; Y4 consumes the algorithmic-conditioning mode
(`compile_bound` on Levels I-II). Deferral closed. See O2.4 condition-
number scope lock for the full design.

**O4.6 — Heterogeneous selection / `Selected<T>`. RESOLVED
(2026-04-26).** The old "heterogeneous e-class membership" framing is
gone. Selector primitives (`argmax`, `option_argmax`, `argmax_all`, and
their `argmin` twins) return `Selected<T>` handles. Selection identity and
provenance live in Layer-3 `SelectedSite` records; projected fields are
ordinary Layer-1 expressions. Homogeneous and heterogeneous collections
share the same surface, and type-specific field access requires explicit
match / narrowing.

**O4.7 — Event-driven topology mutation.** Events add nodes, edges,
equivalences. This is covered at the semantic level by Section 5 (events
add facts). Operationally, the e-graph's saturation must handle
incremental additions without re-running from scratch. Implementation
concern; not a design blocker but needs a note.

**O4.8 — Spatial operator lowering.** Kernels, integrals, convolutions.
Deferred to the kernel thread resumption. This chunk report provides the
foundation they will build on.

### Open — numeric-specific items deferred from Section 8

- **GPU-incompatibility of BigFloat and Rational.** Hard-error on GPU
  target? Fall-back-to-CPU with warning? (Leaning hard-error to avoid
  silent performance catastrophes.)
- **Rational saturation termination.** Precision cap or canonical-form
  simplification policy. Specific numeric cap / heuristic undecided.
- **Dual numbers for autodiff.** `Dual<T>` listed in the hierarchy; its
  relation to the `deriv` intrinsic and autodiff rewrites needs its own
  pass. Likely interacts with the symbolic-math items above.

### Open — Phase 1, Q2 (structural shape of commitment section)

Deferred until above resolve. Once Phases 2-4 are locked:
- Which items get full treatment in the commitment versus reference to
  open-question entries.
- Whether merge criterion and rewriteability are one subsection or two.
- Whether the forward-reference list of surfaces-that-depend-on-the-e-graph
  lives in the commitment section or as a separate cross-reference table.

---

## 12. Audit cross-cutting findings and resolutions

The rewrite-rule audit surfaced cross-cutting items that did not fit the
strict/fuzzy/one-way taxonomy cleanly. Five (CC1–CC5) came directly from
the audit synthesis (reproduced in the appendix below). Additional
findings (directional absence, obligation fulfillment, `condition_weighted`
deferral) are tracked via O4.x entries in Section 11.

**Quick status:**

| CC | Topic | Status |
|----|-------|--------|
| CC1 | Hard-coded constants in `.myco` | **LOCKED** — workflow-bound; three structural exceptions |
| CC2 | `#[verified_externally]` annotation | **ELIMINATED** — consequence of O2.1 |
| CC3 | Per-residual exposure for training | **LOCKED** — `ResidualSite` identity + shared `ResidualRealization` |
| CC4 | Stochastic `~` edges | **LOCKED** — envelope + propagation + capability contracts |
| CC5 | Pole L'Hopital / `identify`-seam gating | **LOCKED** — option 2 (site-scoped rewrite predicates) |

Audit findings not in the CC1–CC5 numbering but still cross-cutting:

- Directional rewrites absent from kernel-report three-tier cut — **resolved** in Section 7 (framing B).
- Obligation fulfillment ledger — O4.1 resolved in `spec_new.md` §8.11 / §10.5.
- `condition_weighted` closure deferred — tracked as O4.5.

---

### CC1 — Hard-coded constants in `.myco` → workflow-bound. LOCKED.

Reframed from "universal-as-binding ambiguity" to a stronger commitment:
**`.myco` does not permit naked numeric values.** Hard-coded numerics are a
workflow concern, not a world claim.

**Rule.** `.myco` permits literal numerics in exactly three structural
positions:

1. **Unit definitions** — `1 km = 1000 m`. The `1000` is part of the type
   system, not a value.
2. **Affine-conversion bodies** — `Celsius <-> Kelvin: value + 273.15`. The
   `273.15` is part of the conversion's definition.
3. **Pure mathematical constants** — π, e. Symbolic references in `.myco`;
   folded to float at lowering time. Not a value, a reference to a known
   irrational.

Everything else — gas constant, Faraday constant, Boltzmann, species
parameters, observation data — is workflow-bound:

```python
run.bind_known_constants(constants.physics)   # R, k_B, N_A, c, ...
run.bind_parameters(my_measured.sperry_params) # V_max, K_m, ...
```

The parser flags naked floats in `.myco` bodies with an error pointing at
the workflow surface.

**Consequence for the rewrite set.** The audit's B2 (universal-to-literal
substitution) ceases to be a compile-time fold. Workflow-bound values enter
the e-graph as observation-style equalities at workflow composition —
semantically identical to `observe(path, data)`, just delivered by a
different workflow verb.

**Philosophical lock.** `.myco` is about how the world works (structure);
workflow is about how you use it (values). The surfaces do not mix.

---

### CC2 — `#[verified_externally]` annotation. ELIMINATED.

**What it was.** A v2 spec annotation on function inverses where the
compiler couldn't symbolically prove round-trip identity, but a property-
test-style sanity check passed. The inverse was admitted to the e-graph
carrying the annotation as a "trust-me-it's-been-tested" tag.

**Why it's eliminated.** It embedded a workflow concern (how much testing
was done) into the surface language — the pattern Riley's CC2 concern
was flagging in general. Under O2.1's resolution:

- User declares `convert <->`; compiler runs bounded counterexample search;
  either proves-false (hard error) or accepts silently.
- No "mostly-verified" middle state requiring syntactic distinction.
- Property-test-level verification, if desired, is a workflow-side concern:
  `run.verify_convert_inverses(level="property_test", n_samples=1000)`.

Annotation is gone. No syntax surface for it.

---

### CC3 — Per-residual exposure for training emission. RESOLVED.

Overconstrained relations survive extraction with their *original
relation names* as `ResidualSite` identities. The extractor may choose a
shared `ResidualRealization`; site identity is not keyed by the extracted
expression. This resolves the CSE/canonicalization tension without
blocking optimization.

---

### CC4 — Stochastic `~` edges. LOCKED.

**Framing.** `x ~ D(params)` is a distributional claim, not an equality.
`~` never introduces e-graph merges; only `=` and algebraically-derived
equivalences do. The distribution is **Layer 2 envelope metadata** on `x`'s
e-class, and it **propagates through the graph via rewrites** — not via
inert metadata storage.

**Propagation mechanism.** When a deterministic operation (`a*X + b`,
`X + Y`, `exp(X)`, ...) acts on a stochastic operand, a rewrite creates a
new e-class whose envelope is computed from the operand envelopes and the
operation. The propagation rules are keyed on envelope metadata; they do
not fire unless preconditions hold.

**Three-tier support map.**

- **Tier A — Closed-form exact.** Stable/self-closed families under
  specific operations. Examples:
  - Affine of Normal → Normal: `a*X + b ~ Normal(a*μ + b, |a|*σ)`
  - Sum of independent Normals → Normal (means and variances add)
  - Product of independent Log-Normals → Log-Normal
  - Sum of same-rate Gammas → Gamma; Sum of Poissons → Poisson; Sum of χ²
  - Affine of Cauchy / Student-t / Uniform → same family, shifted/scaled
  - Exp of Normal ↔ Log-Normal (inverse pair)
  - Conjugate posterior updates (Normal-Normal known σ, Beta-Bernoulli,
    Gamma-Poisson, Dirichlet-Multinomial) as cross-family rewrites
- **Tier B — Approximate propagation.** Requires the `approximate` surface
  (Section 9) under declared assumption or tolerance. Examples:
  - Sum of independent Log-Normals → approximate Log-Normal via
    Fenton-Wilkinson moment matching
  - Smooth nonlinear transform → Normal via delta method (linearization at
    the mean), error bound derivable from f'' and σ²
  - Central-limit applications → approximate Normal, count- and moment-
    conditioned
  - Bimodal output approximated as single Gaussian via moment matching
- **Tier C — Opaque propagation.** No closed-form resolution; envelope on
  the output e-class is `OpaqueDistribution(underlying_expr)`; inference
  routes to the PPL backend for sampling. Examples:
  - Arbitrary nonlinear transforms of non-Gaussian distributions
  - Sums of heterogeneous families (Gamma + Beta, etc.)
  - Products or ratios of arbitrary independent distributions (except
    specific known pairs)
  - Irreducibly joint declarations (copulas)

**Capability decomposition via contracts.** The `Distribution<U>` contract
stays lean; self-closure capabilities are separate contracts that families
satisfy where applicable:

```
contract Distribution<U>:
    log_pdf(value: U) -> LogDensity
    sample(rng) -> U

contract ReparameterizedSampleable<U> requires Distribution<U>:
    reparameterized_sample(rng) -> U

contract AffineSelfClosed<U> requires Distribution<U>:
    affine_transform(scale: U, shift: U) -> Self

contract SumSelfClosed<U> requires Distribution<U>:
    sum_independent(other: Self) -> Self

contract ProductSelfClosed<U> requires Distribution<U>:
    product_independent(other: Self) -> Self

contract SmoothTransformable<U> requires Distribution<U>:
    smooth_transform(f: Function<U, U>) -> Distribution<U>
```

Rewrite dispatch is keyed on contract satisfaction: if `envelope(X)`
satisfies `AffineSelfClosed`, the `a*X + b` rewrite fires; otherwise the
envelope degrades to opaque. **No `optional` methods** — contracts are
all-or-nothing; capability decomposition is how partial support is
expressed.

Capability decomposition is not an antipattern for this use case. It
mirrors Rust's `Copy`/`Clone`/`Send`/`Sync` and Haskell's `Semigroup`/
`Monoid`/`Functor`. Antipattern boundaries (combinatorial blowup,
fragmentation without reuse, leaky boundaries) are passed here: each
contract has independent use, boundaries are sharp.

**Cross-family rules belong in the rewrite-rule declaration surface, not
in contracts.** Conjugate pairs (Normal prior + Normal likelihood → Normal
posterior) are properties of a pair of families, not of one. These are
declared rewrites, tabulated in the stdlib.

**Bulk-impl sugar not applicable.** A subagent audit confirmed the prior
species-template monomorphization sugar
(`02_collections_iteration_report.md:502-507`) generates many concrete
instances across a family from one template. It does not apply to
distribution self-closure impls, where the math differs per family and must
be hand-written.

**Independence and correlation via e-graph structure.** The
`sum_independent` rewrite requires the two operand e-classes to be
structurally distinct stochastic ancestors. Operand classes same or
ancestrally-linked → use `AffineSelfClosed` instead. This handles the
`y = x + x` case (P1) and the general shared-ancestor case (hierarchical
models, random effects, shared drivers) uniformly: propagation is
bottom-up through independent base draws, and correct covariance falls
out of the deterministic decomposition.

**Joint declarations.** Three cases:

1. *Shared-ancestor correlation.* Users declare base RVs + deterministic
   transforms. The e-graph handles propagation via bottom-up
   decomposition. Covers hierarchical models, random effects, shared
   drivers — almost all of scientific modeling.
2. *Decomposable joint declarations* (`(x, y) ~ MVN(μ, Σ)`, log-MVN, etc.)
   are reparameterized at compile time to independent bases via Cholesky
   or family-specific transforms. User-facing syntax stays joint; internal
   representation uses independent bases plus deterministic transform.
   Standard non-centered-parameterization trick from PPLs.
3. *Irreducibly joint declarations* (copulas, non-reparameterizable joints)
   are Tier C: declared with an explicit coupling family, sampleable via
   backend, no closed-form propagation through arithmetic. Copula
   parameters (correlation coefficients) are workflow-bound per CC1 — the
   structure lives in `.myco`, the values do not.

**"Naked correlation" as a first-class concept is eliminated.** A bare
`correlation(x, y) = 0.5` is not a well-formed probabilistic claim;
correlation is a summary statistic of a joint distribution, not a joint
distribution. Every distributional claim in `.myco` carries structural
content: a base draw, a decomposable joint, or an explicit coupling family.

**Envelope-conflict resolution** (two envelope claims on the same e-class
from different paths) is handled by the overdetermination / closure-policy
machinery already in the spec. Exact match is a no-op; disagreement invokes
a user-declared policy (posterior product, moment-match, stricter-wins, or
custom). This is a faithfulness check, not a merge.

**Aleatoric / epistemic split preserved.** Noise models stay in `.myco`
(world claims); parameter priors stay workflow-side (experimenter claims).
`v2.1_in_progress.md:608-622` remains correct; nothing here disturbs it.

**O4.4 reframed.** Phase 4's O4.4 ("stochastic `~` is a rewrite blank")
closes with this locking. What remains are stdlib-scope questions (which
families ship with which contracts, which cross-family rules ship with
v2.1) — these move to O2.2's baseline rewrite-set partition.

---

### CC5 — Pole L'Hopital / `identify`-seam structural gating. LOCKED.

Two rewrite flavors surfaced during the audit that do not fit value-bound
gating (envelope proves `x ≥ 0`, etc.):

- *Pole L'Hopital.* At mesh nodes coincident with a declared `locus pole`,
  rewrite `laplacian(f)` from the naive `1/sin(θ)` form (which diverges
  there) to its L'Hopital limit form (which is finite). The rewrite is
  gated on the structural fact "this mesh node is a declared pole";
  elsewhere (sin(θ) ≠ 0) the naive form is correct and the rewrite must
  not fire.
- *`identify`-seam merges.* At mesh nodes coincident with a declared seam
  (`identify phi=0 <-> phi=2*pi` on a cylindrical geometry), merge the
  scalar field e-classes at the seam. Elsewhere, no merge. Again gated on
  structural predicate.

**Resolution: option 2 (site-scoped rewrite predicates).** Rewrites already
carry predicates that the saturator evaluates as preconditions. Value-bound
predicates (`envelope(x) proves x ≥ 0`) and structural predicates
(`site(f) ∈ locus("pole")`) share the same firing semantics — both are
preconditions checked before applying a rewrite. The only difference is
what facts they consult. Rather than introduce a fifth rewrite category,
**broaden what predicates can reference.**

```
rewrite abs_nonneg:
    pattern: abs(x)
    predicate: envelope(x) proves x >= 0      # value-bound
    replacement: x
    faithfulness: strict

rewrite pole_laplacian:
    pattern: laplacian(f)
    predicate: site(f) ∈ locus("pole")         # structural
    replacement: lhopital_limit_laplacian(f)
    faithfulness: strict

rewrite seam_merge:
    pattern: (field_at(phi=0), field_at(phi=2*pi))
    predicate: site ∈ declared_seam            # structural
    replacement: merge
    faithfulness: strict
```

**Consequences:**

- The faithfulness × orientation 2×3 matrix (Section 7) stays clean. No
  fifth lane.
- One unified predicate language with multiple fact sources (envelope,
  site, type-context, event-state, refinement type, SCC role, temporal
  index, ...). Future predicate categories slot in as new fact sources
  without requiring new rewrite-category taxonomy.
- Value predicates can be evaluated purely from Layer 2 envelope metadata
  (local to the e-class). Structural predicates require coordination with
  Layer 3 adjacent metadata — specifically the site-to-e-class keying
  from Section 3. This coordination is needed in either option; option 2
  makes it natural by expressing it uniformly in the predicate pipeline.
- Mirrors the `egg` framework's conditional-rewrite idiom: conditions are
  closures the saturator queries with available facts.

**Extensibility check.** The design absorbs future predicate flavors
without new categories:

- **SCC-role predicates** — "apply only to e-classes that are the solved
  root of a Newton SCC"
- **Temporal predicates** — "apply at timestep `t=0` only" (initial
  conditions)
- **Event-state predicates** — "apply only after event `E` has fired"
- **Refinement-type predicates** — "apply only when operand has refinement
  type `Positive`"

Each is a predicate with a different fact source. Treatment is uniform.

**O4.2 closes with this locking.**

---

## 13. Implications for other v2.1 design surfaces

Once the e-graph commitment locks and this chunk report resolves, the
following docs need update passes:

- **`spec.md` §4 (Types)** — add numeric hierarchy; adjust `Scalar<U>` to
  `Scalar<U, T>` with default.
- **`spec.md` §6** — restore the e-graph commitment (two placements, per
  Section 2).
- **`spec.md` conversion section (§ around 174-225 in `v2.1_in_progress.md`
  per the audit)** — reconcile `convert` with function-registry four
  invertibility classes; clarify that `where` on `convert` is the
  fallibility knob; state the round-trip-merge rule.
- **`03_kernels_in_progress.md`** — update the three-tier cut to reference
  the 2×3 framing (strict/model/tolerance is one axis, orientation the
  other); the unified-machinery direction (§7) becomes concrete with the
  auto-derived-lossiness capability.
- **`open_questions.md` Tier 0** — close items resolved here; move open
  items tracked above into explicit Tier 0 entries; add the
  previously-missing numeric-type items.
- **Mocks** — confirm `mock_sperry.myco` and `mock_potkay.myco` do not
  violate the referential-truth framing (they currently predate it);
  adjust any static-world assumption that conflicts.
- **Probabilistic programming surface** (locked in commit 8d41756) — update
  to reflect CC4 resolution: `Distribution<U>` contract stays lean; add
  sibling self-closure contracts (`AffineSelfClosed`, `SumSelfClosed`,
  `ProductSelfClosed`, `SmoothTransformable`) and cross-family rewrite
  declarations for conjugate pairs; document envelope propagation as
  Layer-2 metadata flowing through e-graph rewrites; document MVN
  auto-reparameterization to independent bases.
- **Constants/workflow surface** — CC1 lock means the parser rejects naked
  numerics in `.myco` bodies outside the three structural exceptions;
  workflow gains `bind_known_constants` / `bind_parameters` verbs
  (if not already present) and the audit's B2 rule migrates from
  compile-time fold to observation-equality injection at composition.
- **Section 10 of this report / spec `approximate` section** — O2.4's
  named-field loss generalization supersedes the cost-tuple framing.
  `loss_of(expr)` returns a struct (`approximation`, `condition`,
  `truncation`, `discretization`, ...), not a scalar. Workflow gains
  `run.config.loss_estimation` (sampling budget, seeds, precision,
  policy) and `run.config.extraction_policy` (dimension weights, caps).
  The `approximate` block stays as a user-facing surface that
  complements compiler derivation (domain knowledge, black-box terms,
  literature bounds, tolerance budgets); proven-violated declarations
  are hard errors.
- **Spec condition-number / `condition_of` section (new)** — must
  name the algorithmic/problem conditioning duality, show one worked
  example of each mode (e.g., `(exp(x)-1)/x` vs `expm1(x)/x` for
  algorithmic; a linear-solve block for problem), document the
  mode-tagged return struct, and cross-reference Y4's consumption of
  the algorithmic mode. Level II (vector algorithmic) needs Jacobian-
  norm rollup specified.
- **Workflow solver-instrumentation surface (new)** — verbs for
  consuming Level III runtime `condition_of` output: logging hooks,
  ill-conditioning escalation (`on_ill_conditioned: escalate_to_bigfloat`
  or similar), preconditioner selection. Scope cut is small; design
  can ride on existing `run.config.*` pattern.
- **Kernel chunk report (`03_kernels_in_progress.md`)** — §8 open
  item "Whether `condition_weighted` closure policy gets resurrected
  with a `condition_of(expr)` intrinsic now that we're taking cost
  modeling seriously" closes: yes, both ship in v2.1.
- **New chunk report `05_matrices_in_progress.md`** — B5 blocker
  tracker. First-class matrix/tensor types, decompositions (Cholesky,
  LU, QR, SVD), norms, solves, structural subtypes
  (Symmetric / PosDef / Diagonal / Sparse), unit propagation through
  linear algebra. Unblocks Wishart / InverseWishart, MVN log_pdf +
  Z10 Cholesky reparameterization, Level III runtime `condition_of`
  (condest on assembled solve matrices), Gram matrices for kernel
  methods, Jacobian-based inference. B5 must resolve before chunk
  04's Z-group scope lands these families.
- **New chunk report `06_backend_abstraction_in_progress.md`** —
  B6 tracker; absorbs former B3 (Tier C PPL protocol). Unified
  burn-style backend trait covering PPL inference, numerical
  linear-algebra execution, GPU lowering, opaque callables
  (`bind_controller` gradient-flow), and AD ownership fork
  (Myco-owned symbolic vs. backend-delegated vs. hybrid — lean
  hybrid). Independent of chunk 05 structural questions; concretizes
  lowering targets for chunk 05 primitives. Owns workflow surface
  for backend selection, capability advertising, fallback policy,
  and versioning.
- **Future chunk report 07 (joint distributions / coupling)** —
  B2 + B4 unified (B3 removed from this pair, landed in chunk 06).
  Joint distribution declaration syntax; coupling machinery
  (rewrite suppression across coupled ancestry,
  MVN-reparameterizable vs. irreducibly-joint distinction).
  Unblocks copulas. Depends on B5 (MVN/matrix-coupling sub-case)
  and B6 (Tier C handoff protocol).
- **Future chunk report or open-question entry (B1)** — opaque
  `log_pdf` stdlib policy. Whether stdlib ships families with
  structurally-opaque numerical `log_pdf` evaluators (α-stable via
  inverse-FFT of characteristic function, table lookups, etc.);
  autodiff requirements if so; interaction with Tier C routing.
  Independent of B2/B3/B4/B5; can run in parallel.
- **Z-group scope in `spec.md` / stdlib docs** — once Z-group is
  written into the spec, reflect the full Tier 1 list (Normal,
  LogNormal, Uniform, Beta, Gamma, Exponential, ChiSquared, Cauchy,
  Student-t, Laplace, HalfNormal, HalfCauchy, InverseGamma, Lévy,
  Weibull, Pareto, Fréchet, Gumbel, GEV, Bernoulli, Binomial,
  Poisson, Categorical, Geometric, NegBinomial, Hypergeometric,
  Dirichlet, Multinomial, MVN conditional on B5, `Truncated<D>`,
  `Mixture<...>`) with per-family contract annotations
  (`AffineSelfClosed`, `ScaleSelfClosed`, `SumSelfClosed`,
  `ProductSelfClosed`, `SmoothTransformable`, `ReparameterizedSampleable`).
  Cross-family rewrites: the conjugate set (Normal-Normal,
  Beta-Bernoulli/Binomial, Gamma-Poisson, Dirichlet-Multinomial /
  Categorical, Normal-InverseGamma, NormalInverseGamma,
  Gamma-Gamma) plus family equivalences (`exp(Normal) ↔ LogNormal`,
  `Exponential ↔ Gamma(1,λ)`, `ChiSquared(k) ↔ Gamma(k/2, 2)`,
  iid-Bernoulli-sum → Binomial, `1/Gamma ↔ InverseGamma`) plus
  Tier B (delta method, Fenton-Wilkinson, CLT, block-maxima→GEV).

---

## 14. Notes to self (for resuming)

- Riley explicitly endorsed the two-axis framing (faithfulness ×
  orientation) over the three-tier kernel cut: *"i also agree that B is
  better, i like the idea that faithfulness and orientation are distinct."*
- Riley's correction on temporal and dead things: *"y[2] doesn't equal
  y[1] so it doesn't collapse anything — y[2] has a structural relation
  to y[1] as described by the graph... do things need to know they're
  dead? feels like extra stuff on top of a self-describing graph."*
  This locked Section 4 (temporal needs no special machinery) and Section
  5 (referential truth).
- Riley endorsed the layered-state framing (three-layer split) after
  pushing back on the initial proposal that put non-equational constraints
  outside: *"those should be attached to types, which are part of the
  graph in my eyes."*
- Riley's philosophy on the baseline rewrite set: *"configurability +
  extensibility + sane defaults with a nice base toolkit."* Commit a
  baseline; allow workflow-layer extension.
- Numeric hierarchy: Riley approved with request to include BigNumber
  thinking; resolution was Rational + BigFloat in stdlib, BigInt +
  BigDecimal as extension modules.
- Vocabulary gap observation came from Riley: *"the spec discusses lossy
  conversions and fuzzy equality but i'm not sure we actually have a
  vocabulary for that in the syntax?"* Answer: no, we were missing it; the
  `approximate` surface fills it.
- Auto-derived lossiness was Riley's question: *"is there a way we can get
  the compiler to recognize the extent of the approximation based on the
  graph itself?"* Answer: yes, via stacked symbolic/interval/structural
  analysis.
- Phase 1 Q2 (structural partition of commitment section) deferred per
  Riley: *"no, i didn't want to worry about the contents of the sections
  until we've actually settled on the foundational design questions, list
  of topics, and audit findings. once we're settled on those things, then
  we can worry about how to structure the writeup."*
- Per the kernel chunk report: after this foundation locks, return to the
  kernel discussion and draw up the unified machinery proper.
- Stale items adjacent to this thread (flagged earlier but not resolved):
  spec §8.5 "structural introspection" language is stale; `mock_potkay.myco`
  still has retired `slot` syntax and `[t+1]` indexing. Low priority; can
  ride a cleanup pass.
- The `rule` keyword was killed in v2.1, replaced by `event` for topology
  change. Any rewrite-rule-design naming in future work must not resurrect
  `rule`; use `rewrite`, `approximate`, or similar.
- Riley's CC1 reframing: *"hard-coded constants just straight up don't
  belong in the .myco. ... if you're using things from our constants
  library, the actual values get injected into the graph from the python
  side, just in nicely-managed way ... i fear that if we expose a way to
  hard-code finite values into a .myco people are going to abuse it to put
  workflow-like stuff directly in the myco."*
- Riley's CC2 stance: *"i'm generally very suspicious of adding annotations
  to the syntax, often that signals a workflow level concern bleeding into
  the syntax."* This is a durable design principle — future work should
  interrogate any annotation as possible workflow-bleed.
- Riley's CC4 vision for stochastic propagation: *"information gets
  propagated around the e-graph fully! ... if you say x is gaussian and
  y = m*x+b and m is 5 and b is unknown, what is y, i want to be able to
  say y is whatever the most combined, simplest way to express the
  interaction of the x distribution, b symbolically, and 5 numerically ...
  i want to be able to add multiple distributions to the graph and have
  them combine."* This drove the CC4 resolution beyond the initial
  "metadata-only (c)" framing.
- Riley on contract design for capabilities: *"if you can't meet the
  requirements of the contract, you don't meet it. ... i think a better
  pattern would be to have more contracts with more permutations."* No
  `optional` methods — capability decomposition is the right shape.
- Riley on naked correlations: *"do we need to support case 3 at all? i
  thought we were eliminating hard-coded values, and if the user wanted to
  declare a relationship between x and y symbolically does that not just
  get us back to 'the e-graph handles it'?"* This eliminated the fourth
  case I initially proposed; every distributional claim now carries
  structural content.
- Riley on CC5 option 2: confirmed site-scoped rewrite predicates over a
  new rewrite category. Rationale: unified predicate language with
  multiple fact sources scales to future predicate flavors without
  taxonomic sprawl; value-bound and structural predicates share firing
  semantics and differ only in what facts they consult.
- Riley on Y4 / Level III scope: *"i would rather ship 1-3 together so
  we don't have any dangling things leftover. much cleaner to reason
  about up front."* Motivated the full Level I-III commitment rather
  than deferring Level III. After walkthrough of the algorithmic/
  problem duality in numerical analysis, Riley confirmed the unified
  `condition_of` with mode-tagged return is the right shape: *"yep
  sounds beautiful go for it."* Key framing insight: algorithmic vs
  problem conditioning is a real duality in the math (the product
  bounds total error), so a polymorphic intrinsic is principled, not
  ad-hoc. Closure policies and extraction read compile-time mode;
  runtime solver diagnostics read runtime-estimate mode; the split is
  by use-site, not by primitive name.
- Riley on O2.4 (auto-derived lossiness): *"i want to ship all 4 layers.
  how many samples, seed management, determinism are surfaces for
  exposure to the workflow python side. for user-declarable approximation
  info, are there cases where they would want to declare something we
  can't solve from the compiler or could we do compiler only? what
  happens if the compiler can prove something that violates the user's
  declaration, can we error on that?"* Resolution: compiler + user are
  complementary (declaration needed for domain knowledge, black-box,
  literature bounds, tolerance budgets); proven violations are hard
  errors (same discipline as O2.1); sampling non-determinism is
  workflow-side. Loss is multi-dimensional with named fields, not a
  scalar — aggregation to rank happens at extraction under
  workflow-configured weights.
- Riley's completeness preference (overriding the initial v2.2-deferral
  strategy for Z-group): *"i would rather go for correctness and
  completeness in the spec so we don't end up with loose ends
  everywhere."* This reframed deferrals as "blocked on named design
  items (B1-B5)" rather than arbitrary v2.2 punts. Future deferral
  proposals should be tested against this preference: is the deferral
  because the work is genuinely blocked on an undesigned piece, or
  because it's just implementation volume? Only the former is a
  legitimate defer.
- Riley on matrices: *"strongly feel like we should be able to do
  matrices in myco. this is a real missing piece for real actual
  modeling and math."* Commit to first-class matrix/tensor types
  (B5) before v2.1 closes. Chunk report 05 exists as stub. This also
  locks Level III runtime `condition_of` (needs `condest` on assembled
  solve matrices), MVN log_pdf + Z10 Cholesky reparameterization,
  Wishart / InverseWishart, Gram matrices for kernel methods, and
  first-class linear-system solves.
- Five design blockers identified (B1 opaque log_pdf policy, B2 joint
  declaration syntax, B3 Tier C PPL backend protocol, B4 coupling
  machinery, B5 matrix types). Recommended ordering: B5 first
  (foundational, unblocks Level III lowering + MVN + Wishart + kernel
  linear-algebra + Jacobian inference); B1 in parallel (orthogonal,
  policy decision with small implementation footprint); B2+B3+B4
  together as unified chunk report (deeply intertwined — syntax
  influences machinery influences protocol), depends on B5 for
  MVN/matrix-coupling sub-case.

---

## 15. Return path after this report closes

**Immediate (design blockers from §11 B1-B6)** — the Z-group scope
resolution promoted families under the completeness preference, but
the remaining blockers still need design rather than just
implementation volume. B5 is now closed; B6 is the main follow-on
because it supplies execution protocols for the committed matrix
surface.

1. **B5 — Matrix / tensor types. RESOLVED.** Chunk 05 locked
   (`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md`).
   Matrix / tensor source semantics, fact contracts, and finite
   matrix assembly are committed. Unblocks MVN log_pdf + Z10 type
   surface, Gram matrices, first-class linear-system solves, and the
   matrix-facing side of Wishart / InverseWishart; actual execution
   still depends on B6.
2. **B6 — Backend abstraction.** Chunk 06 stub
   (`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md`).
   Absorbs former B3. Burn-style Backend trait; AD ownership fork
   (lean hybrid); capability advertising; fallback policy; PPL
   backend protocol; opaque callable gradient flow; mixed-backend
   policy (lean single-backend-per-run); versioning. Concretizes
   lowering targets for chunk 05 primitives.
3. **B1 — Opaque `log_pdf` stdlib policy (in parallel with B6).**
   Whether stdlib ships families with structurally-opaque
   numerical `log_pdf` evaluators (general α-stable, table-lookup,
   etc.); autodiff requirements; interaction with Tier C routing.
4. **B2 + B4 — joint syntax + coupling machinery (future chunk 08,
   after B6).** The MVN/matrix-coupling sub-case now has a stable
   matrix surface; Tier C handoff protocol still depends on B6.
   Unblocks copulas.

**Phase continuation (chunk 04 scope):**

4. Resolve the two remaining O2.2 sub-questions: K2 / N1 kernels-chunk
   dependency (commit-pending-kernels-resume vs. defer-to-v2.2); P1
   mesh discretization architectural call (e-graph rewrite vs.
   pre-e-graph codegen). (O2.1, O2.3, O2.4, Y4 un-defer, and
   Z-group stdlib scope all resolved 2026-04-20.)
5. Phase 2 Q3 — residual ↔ e-graph relationship. RESOLVED via
   `ResidualSite` / `ResidualRealization`.
6. Phase 2 Q4 — envelope ownership.
7. Phase 3 — topic list pass (units, types, over/underdetermined, symbolic
   math, collections, continuous/discrete, functions/contracts, temporal,
   stochastic, inequalities, opaque callables, events, SCCs, learning
   targets).
8. Phase 4 — remaining audit-specific items (O4.7 incremental
   saturation, O4.8 spatial operator lowering). O4.2 / O4.3 / O4.4 /
   O4.5 already resolved; O4.1 resolved by the obligation ledger; O4.6
   resolved by `Selected<T>` selector semantics.
9. Phase 1 Q2 — structural partition of commitment section.
10. Write the actual commitment section text into the spec (two
    placements).
11. Pass over v2.1 docs for updates (Section 13 list).
12. Return to the kernel discussion with unified machinery concrete.
13. Resolve the deferred `Riley Note: check what the deal is with
    properties` in `03_kernels_in_progress.md:48`.


## riley note

convo was getting compacted, here's the rewrite rule discussion that hasn't been committed yet: 
❯ sort them into groups and enumerate all of them for me, with brief blurbs where explanation is merited                                                                                                  

  Read 1 file (ctrl+o to expand)

⏺ Grouped into 25 groups, ~61 rules total. Organized by natural category (not faithfulness × orientation — those tags appear per-group). Format: group name · brief where needed · numbered rules.

  ---
  Strict / lossless rewrites

  A. Ring/field axioms — bidirectional algebraic truths that every rewriter needs. Load-bearing for canonical-evaluator selection, residual simplification, and SCC invariance.

  - A1. x + 0 → x, 0 + x → x
  - A2. x * 1 → x, 1 * x → x
  - A3. x * 0 → 0, 0 * x → 0 (gated: 0 dimensionless unless dimension matches per §4.6)
  - A4. x - x → 0, x / x → 1 (latter gated on x ≠ 0 via envelope bounds)
  - A5. Associativity of + and * (bidirectional)
  - A6. Commutativity of + and * (bidirectional)
  - A7. Distributivity: x*(y+z) ↔ x*y + x*z (bidirectional; extraction picks expanded vs factored form by cost)
  - A8. -(-x) → x
  - A9. x^0 → 1 (gated x ≠ 0), x^1 → x
  - A10. x^a * x^b → x^(a+b), (x^a)^b → x^(a*b) (needed for Arrhenius canonicalization)

  B. Constant folding — trivial but spec doesn't enumerate; needed for envelope collapse to point values.

  - B1. Literal arithmetic and transcendentals at identity points: 2+3→5, exp(0)→1, log(1)→0, sin(0)→0, cos(0)→1, sqrt(1)→1
  - B2. universal bound to literal: R → 8.314 after workflow binding (ambiguity flag — see cross-cutting CC1)

  C. Unit / dimensional normalization — base-SI internal representation; dimension-exponent arithmetic. Load-bearing for unit-equality merges.

  - C1. Literal-with-unit → base SI: 0 degC ↔ 273.15 K, 0.75 MPa ↔ 750000 Pa
  - C2. to_base(to_base(x)) → to_base(x) (idempotence)
  - C3. Dimension exponent vector arithmetic on *, /, sqrt, integer powers
  - C4. Dimensionless 0 / 1 collapse across unit signatures

  D. Named-type normalization — convert equalities at the magnitude level; name-preserving arithmetic.

  - D1. Bare convert FishMass <-> DetritusMass treats them as same-magnitude in the e-graph (different name) when both share a conserved parent
  - D2. Bidirectional convert installs both forward and backward rewrites
  - D3. Inverse convert round-trip: convert(convert(x)) → x when verified per O2.1
  - D4. Same-named-type addition preserves the name: CarbonPool + CarbonPool → CarbonPool
  - D5. Named + anonymous-matching-dimension addition preserves the name (lossless in e-graph, name-join-semilattice with anonymous as ⊥)

  E. Function-inverse round-trip elimination — requires declared or registered inverse.

  - E1. For declared-bijective/injective-restricted f with explicit inverse: f⁻¹(f(x)) → x, f(f⁻¹(y)) → y (gated on envelope bounds proving the input lies in f's declared domain)
  - E2. Built-in inverse pairs: exp(log(x)) → x (gated x > 0), log(exp(x)) → x (always)

  F. Geometry-specific strict merge — scalar-field seam identification.

  - F1. identify phi=0 <-> phi=2*pi merges scalar-field e-classes at the seam (open_questions calls this a "literal e-class merge"). Vector/tensor seams deferred.

  G. Transcendental simplifications (gated) — standard identities with bounds requirements.

  - G1. exp(a)*exp(b) → exp(a+b), log(a*b) → log(a)+log(b) (gated a,b > 0), exp(a)^b → exp(a*b) (Arrhenius canonicalization)
  - G2. Trig fundamentals: sin(0) → 0, cos(0) → 1, tan(0) → 0; Pythagorean sin(x)² + cos(x)² → 1 (e-graphs handle cleanly where naive rewriters blow up)
  - G3. Idempotency of lossy ops: abs(abs(x)) → abs(x), abs(-x) → abs(x), min(x,x) → x, max(x,x) → x

  H. Aggregate / collection identities — linearity rules for reductions; needed for GPU-reduction factoring.

  - H1. sum(0) → 0, product(1) → 1, sum(x+y) → sum(x)+sum(y), sum(c*x) → c*sum(x) when c is loop-invariant
  - H2. Empty-collection: sum(empty)→0, product(empty)→1, any(empty)→false, all(empty)→true, count(empty)→0

  I. Conditional rewrites — constraint-analysis-gated.

  - I1. if true then a else b → a; if false then a else b → b; if c then a else a → a

  J. Temporal invariant (anti-rewrite) — forbidden merge, not a rewrite.

  - J1. x[t] and x[t-1] never merge across timesteps even if numerically equal at runtime. Resolved in Section 5 (Referential truth framing covers this naturally; these are distinct ground terms).

  ---
  Fuzzy / tolerance-gated rewrites

  K. Kernel truncation — the headline fuzzy demand from the kernel report.

  - K1. K(a,b) → 0 when distance(a,b) > L_char for compact-support/rapidly-decaying kernels (Gaussian at ±3σ, Matérn, spline compact support). Turns O(N²) integrals into O(N·k).
  - K2. Separable decomposition: K((x₁,y₁),(x₂,y₂)) → K_x(x₁,x₂) * K_y(y₁,y₂) when declared or inferred (lossless when exact; fuzzy when approximate)
  - K3. Low-rank K → U·V^T (truncated SVD, Nyström, random Fourier features). Speculative — kernels report doesn't enumerate, but machinery demands they fit.

  L. Smoothing rewrites — user-written smooth form may simplify.

  - L1. smooth_min(a, b, large_sharpness) → min(a, b) when sharpness exceeds a tolerance threshold. Reverse direction (min → smooth_min) is forbidden — Riley's "no silent smoothing" lock.
  - L2. where p then a else b → a*sigmoid(k*p) + b*(1-sigmoid(k*p)) — only in user-written smooth form, never auto-fired

  M. Series / linearization — first-order expansions and asymptotic truncation.

  - M1. First-order Taylor: f(x) → f(x₀) + f'(x₀)*(x-x₀) around declared operating point
  - M2. Drop higher-order terms when envelope bounds their contribution below tolerance

  N. Numerical quadrature substitution — every PDE passes through this.

  - N1. integrate(f, var, lo, hi) → quadrature_n(...) for user-tunable n when symbolic fails

  O. Training-time consistency-loss substitution — mode-conditional.

  - O1. In train mode: overconstrained lhs = rhs becomes loss += w * (lhs - rhs)²

  P. Mesh discretization (continuous → discrete) — tolerance-gated by mesh resolution h.

  - P1. grad(field) → fd_stencil, laplacian(field) → 5-point or 9-point stencil, etc. Open-questions §B.3 item 9 explicitly flags whether these are e-graph rewrites or pre-e-graph codegen.

  Q. Probabilistic truncation / marginalization — interacts badly with ~ (Section 13).

  - Q1. Latent-discrete-with-finite-support → logsumexp_i[...] auto-marginalization
  - Q2. Continuous distribution + refinement-type bound → truncated distribution with normalized log-pdf

  ---
  One-way / directional rewrites

  R. Lossy-function simplification — forward only under bound tightening.

  - R1. abs(x) → x when envelope proves x ≥ 0
  - R2. max(a,b) → a when envelope proves a ≥ b; same pattern for min
  - R3. floor, relu, clamp — forward only under bound tightening; never invertible

  S. Opaque function applications — no reverse rewrite possible.

  - S1. f(x) where f is opaque — forward edge only; no recovery of x from f(x)
  - S2. bind_controller-attached callable: g(inputs) → output forward only (black-box)

  T. One-way convert — explicit user-declared non-invertible transform.

  - T1. convert Plaintext -> Ciphertext { ... } installs forward rewrite only

  U. Named-type stripping under arithmetic — required for type checking; directional because names can't be re-inferred.

  - U1. LeafArea * CarbonFlux → anonymous Scalar<umol_s> (multiplication strips the name)
  - U2. CarbonPool / CarbonPool → anonymous Scalar<ratio> (same-type division strips the name)
  - U3. Temperature - Temperature → anonymous Scalar<K> (affine subtraction breaks named-type symmetry)

  V. Observation injection — ground-truth data pinning.

  - V1. observe(path, data) installs path = data as a forward observation factor; data doesn't get rewritten by inferred constraints

  W. Obligation fulfillment — ledger selection, not rewrite. O4.1 resolved.

  - W1. relation X on locus fulfills flux_condition(axial_flux): ... satisfies the named obligation; generated defaults are candidate fulfillments and unselected defaults are not emitted as facts

  X. Structural-predicate-gated strict — the cross-cutting finding. Strict/lossless but gated on structural predicate, not value bounds.

  - X1. Pole L'Hopital: at any mesh node coinciding with declared locus pole, rewrite laplacian(f) from the naive 1/sin(θ) form to its L'Hopital limit (finite). Also characterizes identify-induced
  merges when they depend on coordinate predicates.

  ---
  N-way rewrites (closure policies — extraction-time, not rewrite-time)

  Y. Closure policies — extraction strategies for an e-class with multiple equally-valid evaluators. User picks via closure_config.

  - Y1. weighted_average(c1,...,cN) → mean (simplest; arithmetic mean of candidate outputs)
  - Y2. soft_select(preference_list, sharpness) → Σ softmax(rank_i/sharpness) * candidate_i
  - Y3. hard_select(preference_list) → pick highest-ranked by name; non-differentiable (rejected in train mode unless discarded paths have no learned parameters upstream)
  - Y4. condition_weighted — deferred in v2.1; needs condition_of(·) intrinsic. With auto-derived lossiness from Section 10, machinery now exists; relocation from "deferred" is O4.5.
  - Y5. User-defined custom policy — any .myco function taking candidates + hyperparameters, returning a forward value. Extensibility surface.
  - Y6. General C(N,M) enumeration for overconstrained blocks (N > M+1) — planner enumerates all maximal square subsystems; policy receives the set. Combinatorial-blowup warning threshold needed.

  ---
  Cross-cutting items (not rewrites themselves; flags for resolution)

  - CC1. Universal-as-binding ambiguity. assume_constant("R", 8.314) — does this install a strict rewrite R → 8.314 in the e-graph, or pin R in adjacent metadata per v1 §6.2? Cost-driven extraction
  wants to see the constant; scoping principle wants it adjacent. Needs resolution.
  - CC2. Sanity-only inverses. convert <-> where the inverse is #[verified_externally] (sanity-check only) — strict or fuzzy/tolerance-gated? Currently ambiguous.
  - CC3. Per-residual training emission vs CSE canonicalization. RESOLVED by `ResidualSite` identity plus separately shared `ResidualRealization`.
  - CC4. Stochastic ~ edges. No rewrites can fire across a ~ edge until the stochastic semantic invariant is stated. Full blocker for rewriting any stochastic quantity's parameters. (Tracked as O4.4.)
  - CC5. Pole L'Hopital / identify-seam category. Strict but structural-predicate-gated. Fifth category or sub-flavor of strict? (Tracked as O4.2.)

  ---
  Totals by faithfulness × orientation:

  ┌────────────────────────┬─────────────────────────────────────────────────────────────┬────────────────────────────────────┬───────┐
  │                        │                             Bi                              │                Uni                 │ Total │
  ├────────────────────────┼─────────────────────────────────────────────────────────────┼────────────────────────────────────┼───────┤
  │ Strict                 │ ~24 (A1-10, B1-2, C1-4, D1-5, E1-2, F1, G1-3, H1-2, I1, J1) │ ~3 (D4-5 name-pres, X1 structural) │ ~27   │
  ├────────────────────────┼─────────────────────────────────────────────────────────────┼────────────────────────────────────┼───────┤
  │ Fuzzy-model            │ —                                                           │ ~2 (L1-2)                          │ ~2    │
  ├────────────────────────┼─────────────────────────────────────────────────────────────┼────────────────────────────────────┼───────┤
  │ Fuzzy-tolerance        │ ~7 (K1-3, M1-2, N1, Q1-2)                                   │ ~3 (O1, P1, M2)                    │ ~10   │
  ├────────────────────────┼─────────────────────────────────────────────────────────────┼────────────────────────────────────┼───────┤
  │ One-way (lossless uni) │ —                                                           │ ~11 (R1-3, S1-2, T1, U1-3, V1, W1) │ ~11   │
  ├────────────────────────┼─────────────────────────────────────────────────────────────┼────────────────────────────────────┼───────┤
  │ N-way extraction       │ —                                                           │ ~6 (Y1-6)                          │ ~6    │
  ├────────────────────────┼─────────────────────────────────────────────────────────────┼────────────────────────────────────┼───────┤
  │ Forbidden              │ 1 (J1 — temporal)                                           │ —                                  │ 1     │
  └────────────────────────┴─────────────────────────────────────────────────────────────┴────────────────────────────────────┴───────┘

  Grand total: ~57-61 depending on how you count sub-rules.

  ---
  For the O2.2 decision, my suggested cuts:

  - Commit to v2.1 (baseline ≈ 40 rules): all of A, B, C, D, E, F, G, H, I, J, K1, L1, R, S, T, U, V, Y1, Y2, Y3, Y5. Covers everything mocks actually exercise plus the headline kernel truncation.
  - File Tier 1 open (≈ 10 rules): K2, K3, M1, M2, N1, O1, P1, Q1, Q2, Y6. Machinery-demanded but not load-bearing for v2.1 shipping.
  - File Tier 0 open (remaining rules blocked on other decisions): L2 (depends on smoothing surface lock), X1 (depends on O4.2 category), CC1-5. W1 is resolved by O4.1's obligation ledger.
  - Defer to v2.2: Y4 condition_weighted (depends on O4.5 cost-model machinery), speculative kernel rewrites (K3 low-rank specifics).

  Want to accept the cuts as proposed, adjust batch-by-batch, or go rule-by-rule?

✻ Worked for 2m 15s

❯ i want all of these in the v2.1 before we begin implementing. if you think there are some worth deferring until we've covered more of the open questions that's fine by me, it should just be recorded  
  that we need to come back to them. for the cross-cutting items: CC1 - i'm strongly considering reframing this. i am thinking that hard-coded constants just straight up don't belong in the .myco. i'm  
  wondering if it would be better to make this a workflow concern: if you're using things from our constants library, the actual values get injected into the graph from the python side, just in         
  nicely-managed way (e.g. on a loaded myco some method like bind_known_constants or something like that). i fear that if we expose a way to hard-code finite values into a .myco people are going to     
  abuse it to put workflow-like stuff directly in the myco. do you see the vision? for cc2: what is #[verified externally]? i'm generally very suspicious of adding annotations to the syntax, often that 
   signals a workflow level concern bleeding into the syntax. can you explain this further? for cc3: sounds good please keep track of this cc4: explain this more? cc5: explain this more?                

⏺ Good batch of questions. Taking each in turn.

  ---
  "All of these in v2.1" — locked. I'll adjust the rule cuts so everything ships in v2.1; items blocked on open questions get structurally committed but gated-off until the open resolves. Tracking list
  gets updated.

  ---
  CC1 — Hard-coded constants in .myco

  I see the vision, and I think it's right. The symmetry is clean: .myco is "what's true about the world" (structural); workflow is "how you use it" (values). A naked 8.314 in .myco is a value — it
  belongs on the workflow side.

  Proposed rule: .myco permits literal numerics in exactly three structural positions:
  1. Unit definitions — 1 km = 1000 m. The 1000 is part of the type system.
  2. Affine-conversion bodies — Celsius <-> Kelvin: value + 273.15. The 273.15 is part of the conversion's definition.
  3. Pure mathematical constants — π, e. Symbolic in .myco; folded to float at lowering time. Not a value, a reference to a known irrational.

  Everything else — gas constant, Faraday constant, Boltzmann, plant-specific parameters — is bound from workflow:

  run.bind_known_constants(constants.physics)   # binds R, k_B, N_A, c, ...
  run.bind_parameters(my_measured.sperry_params) # binds V_max, K_m, ...

  Consequence for the rewrite set:
  - B2 (universal-to-literal substitution) becomes a workflow-injected fact, not a compile-time fold. Once the workflow binds the value, it enters the e-graph as an observation-style equality. Same
  semantics as V1 (observation injection), just sourced from bind_known_constants instead of observe.
  - A significant philosophical lock: no naked floats in .myco bodies. The parser can flag them as errors with a pointer to bind_known_constants.

  This also resolves CC1 as you framed it: it's not an ambiguity, it's a redirection. Constants are observations, just delivered by a different workflow surface.

  ---
  CC2 — #[verified_externally]

  Your instinct is correct.

  What it was: an annotation the v2 spec introduced (spec.md:2147-2172 region) to mark function inverses where the compiler couldn't fully verify identity but a "sanity check" passed
  (property-test-style round-trip on sample inputs). The inverse was allowed into the e-graph, carrying the annotation as a "trust-me-it's-been-tested" tag.

  Why it's problematic: it's exactly the syntax-bleed pattern you're flagging. The annotation embeds a workflow concern (how much testing was done) into the syntax.

  Good news: it's eliminated by O2.1. Under our resolution:
  - User declares convert <->; compiler runs counterexample search; either it proves-false (hard error) or it doesn't (merge registered).
  - No "mostly-verified" middle state. No annotation needed.
  - If the user wants property tests as an extra layer, that's a workflow concern: run.verify_convert_inverses(level="property_test", n_samples=1000).

  I'll note this in the chunk report under O2.1's resolution so we don't forget.

  ---
  CC3 — resolved

  Resolved by `ResidualSite` identity plus separately shared `ResidualRealization`.

  ---
  CC4 — Stochastic ~ edges

  ~ is the probabilistic assignment operator. x ~ Normal(μ, σ) means "x is distributed as Normal(μ, σ)." Semantically distinct from =:

  - x = μ means x equals μ (deterministic equality).
  - x ~ Normal(μ, σ) means x is a sample from the distribution with those parameters.

  What rewrites should fire across ~:
  - Algebra on distribution parameters: Normal(μ + 0, σ) → Normal(μ, σ) — the parameter tree is ordinary deterministic expression.
  - Reparameterization: x ~ Normal(μ, σ) ↔ x = μ + σ*z where z ~ Normal(0, 1). Identity, useful for autodiff.
  - Marginalization: ∫ Normal(μ, σ) dx = 1; latent integration when declared.
  - Distributional identities: Normal(0,1) under affine transformation composes analytically.

  What rewrites should NOT fire:
  - Can't substitute x for μ just because x ~ Normal(μ, σ). The relationship is distributional, not equality.
  - Can't replace a random variable with its mean in an arbitrary expression (breaks the stochastic semantic).
  - Can't merge e-classes across ~ edges as if they were = edges.

  Why it's a blocker right now: the spec doesn't formally state what ~ means to the e-graph. Without that, the compiler can't know which of the four categories above any given rewrite falls in. Even
  Normal(μ + 0, σ) → Normal(μ, σ) is technically unsafe if the spec hasn't committed to "algebra on parameters is a pure deterministic rewrite inside the distribution expression."

  The concrete open question (Tier 0 §B.3 item 6, "State the stochastic invariant"): does x ~ D cause the e-graph to create:
  - (a) A distinct kind of edge (distributional) between x and D, with rewrite rules that know about the edge type?
  - (b) A regular equality edge to a special Sample<D> node?
  - (c) No edge; x and D are unrelated in the e-graph, and the distributional fact lives in adjacent metadata (layer 3) keyed by x?

  I lean (c) for cleanness — ~ is fundamentally not an equality claim, so it shouldn't live in the equality substrate. The distribution gets attached to x's e-class as envelope-layer metadata (layer 2),
   and inference machinery (Bayesian updates, variational methods, sampling) operates on the distributional metadata, not via e-graph merges.

  This also cleanly separates deterministic rewriting (always available) from probabilistic inference (a separate compiler pass over the distributional annotations).

  ---
  CC5 — Pole L'Hopital / identify-seam structural gating

  The issue: most rewrites are either unconditional (x + 0 → x) or value-gated (abs(x) → x when envelope proves x ≥ 0). Pole L'Hopital and identify-seam merges are neither — they fire conditional on a
  structural predicate.

  Pole L'Hopital, concrete:
  - User declares a Sphere geometry with locus pole at the north pole.
  - The generic laplacian(f) expansion contains 1/sin(θ) terms.
  - At a mesh node coincident with the declared pole, sin(θ) = 0, so the expansion is NaN.
  - But the laplacian's mathematical value at the pole is finite — it's the L'Hopital limit.
  - So: at mesh nodes coincident with locus pole, rewrite laplacian(f) from the naive form to its L'Hopital limit form.

  identify-seam, concrete:
  - User declares identify phi=0 <-> phi=2*pi on a cylindrical geometry.
  - At mesh nodes coincident with the seam (both phi=0 and phi=2*pi instances), merge the scalar field e-classes.
  - Elsewhere in the geometry, no merge.

  Why these are a new category: the gating condition isn't a value-bound (envelope won't help), it's a structural fact about where you are in the geometry. The compiler needs to know "this e-node is
  associated with mesh site S, and mesh site S is a declared pole" to fire the rewrite.

  Two options:
  1. Separate rewrite category: "structural-predicate-gated." Explicit fifth lane. New predicate format that refers to geometric/topological properties.
  2. Site-scoped rewrites. Rewrites aren't global patterns; they have a scope predicate. Most rewrites have scope=global; pole rewrites have scope=where_site_in_locus("pole"). Unifies the category.

  I lean option 2 — it's a generalization of existing rewrite mechanisms rather than a new lane, and it handles both cases (pole and seam) naturally. Value-bound gating is already a form of predicate;
  adding structural predicates to the predicate language is additive.

  Either way: the machinery exists outside the e-graph per se. The e-graph stores equivalences; the predicate engine (geometry-aware) decides which sites a rewrite applies at. The rewrite fires
  per-site, affecting the e-classes associated with that site's mesh nodes.

  ---
  Summary of calls needed:

  - CC1. Confirm constants-via-workflow and my proposed three structural exceptions (unit defs, conversion bodies, π/e). Should I update the report?
  - CC2. Accepting as resolved-by-O2.1-consequence. Just updating the report to note #[verified_externally] is out.
  - CC3. Resolved by residual sites.
  - CC4. Want option (c) — distributional fact as layer-2 envelope metadata, not an e-graph edge? Or push for (a) or (b)?
  - CC5. Option 2 (site-scoped rewrite predicates) or option 1 (new category)?

  Ranks by how much they unblock: CC4 is the biggest — everything stochastic is gated on it. CC1 is clean and doesn't really block but strengthens the whole design.

✻ Brewed for 2m 4s

❯ re: CC4, my vision is that information gets propagated around the e-graph fully! so if we know, for example, x ~ gaussian(u,s), then the paths that you can reach through x *inherit stochasticity via  
  x*. does that make sense? like if you say x is gaussian and y = m*x+b and m is 5 and b is unknown, what is y, i want to be able to say y is whatever the most combined, simplest way to express the     
  interaction of the x distribution, b symbolically, and 5 numerically. does that make sense? so i want to be able to add multiple distributions to the graph and ahve them combine. and i know some      
  distributions play more nicely with this than others, in terms of how they behave on multiplication, addition, etc. so it's good to think about this up front in terms of what we *can* propagate, of   
  what we can support -- what straight up doesn't have a solution, what's partial, what can we build to support, etc? i think this might be good to consider with the probabilistic programming angle we  
  wrote about previously, if you're unfamiliar with the decisions made there you might want ot dispatch a subagent to look into it and give you a report.
