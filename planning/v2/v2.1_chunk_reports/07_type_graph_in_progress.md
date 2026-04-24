# Myco v2.1 — Type Graph vs. Expression E-Graph Design Report (LOCKED)

**Date:** 2026-04-20 (stub created); locked 2026-04-24
**Authors:** Riley Leff, Claude (Opus 4.7)
**Reviewers:** None yet
**Status:** LOCKED. The type graph is a separate static semantic substrate
from the expression e-graph. They interact through a live guard-discharge
bridge. Precompiled / cached guards are implementation optimizations, not the
semantic model.

---

## 1. Why this chunk exists

Chunk 04 commits to an e-graph as the internal equality substrate for
*expressions / relations / terms*. This chunk locks what structure the
*type system* itself lives in, and how type-level equality and implication
talk to the expression e-graph.

Riley's question (2026-04-20, paraphrased): "I know the defined
relationships in a `.myco` constitute an e-graph. Do the types and their
relations also live in a graph too?"

They do. There are at least five distinct type-level relations in v2.1:

- **Refinement implication lattice.** Matrix facts such as `symmetric`,
  `positive_definite`, `diagonal`, `triangular`, and `orthogonal` compose as
  evidence-backed facts and implication rules. Lattice, not tree; facts, not
  casts into new values.
- **Conversion graph.** Units (`seconds ↔ minutes × 60`), shape reshapes
  with named index bijections, sparse / dense materialization with proven
  pattern facts, and structural-refinement widening. Directed semantic edges
  carry witnesses, faithfulness, and obligations; realization cost is a
  lowering concern.
- **Definitional / alias equality.** `Vector<U, n>` ≡ `Tensor<U, (n,)>`.
  `Matrix<U, m, n>` ≡ `Tensor<U, (m, n)>`. Resolved at elaboration:
  terms entering the expression e-graph are already normalized.
- **Contract / `impl` satisfaction.** Which concrete types satisfy which
  contracts; which `impl Contract` monomorph sets are in scope. Runtime sizing
  is `some`, not erased `dyn`.
- **Generic instantiation.** `Tensor<U, shape>` → the concrete
  `(U, shape)` monomorphs that actually appear in a program. Not a
  relation in the type-theoretic sense — more a demand-driven projection.

These relations are now described as a unified structure in canonical
spec §18, with a guard-discharge bridge to the expression e-graph.

---

## 2. Where the type graph meets the expression e-graph

Three interaction points, identified live:

1. **Refinement-gated equalities.** A refinement fact such as
   `symmetric(M)` or `positive_definite(M)` can license a rewrite, but the
   rewrite itself belongs in the expression e-graph. The *guard* that licenses
   the equation is discharged through the type graph / fact store. The
   *equation itself* is an e-graph fact.

2. **Unit-conversion identities.** `x [s] × 60 [min/s]` and `x [s]`
   represent the same physical quantity. That's a rewrite, but it's
   conditioned on unit metadata the type graph owns. The rule is an
   e-graph rule; the preconditions are type-graph facts.

3. **Refinement-gated rewrites.** `solve(A, b)` → `cholesky_solve(A, b)`
   is only valid when facts prove `positive_definite(A)` and axis
   compatibility. The rewrite lives in the e-graph; the precondition is a
   guard-discharge query. Without the precondition the rewrite is unsound.

In all three cases the pattern is the same: **e-graph holds the
equation, type graph / fact store provides the guard.**

---

## 3. Coupling decision — LOCKED

### Accepted semantic model — two graphs, explicit bridge

The type graph is its own machinery (refinement implication lattice,
conversion graph, contract satisfaction, and generic instantiation).
The expression e-graph has
rewrite rules *parameterized by type-graph queries*. At saturation
time, applying a rewrite requires a "does this guard predicate hold for this
e-class?" check.

This is the locked semantic model. It preserves the distinction between
equality and implication: the e-graph merges values; the type graph and
monotone fact store discharge rewrite guards.

### Rejected model — one graph, types are terms

Everything — types, refinements, conversion witnesses — lives in the
e-graph. Requires a more expressive equational logic, closer to
dependent-type territory. Type equality is just e-class equality.

Rejected. It blurs "these expressions are equal" with "this value satisfies a
fact" or "this type implies another fact." That cuts against the three-layer
e-graph discipline and the matrix/refinement-fact decisions.

### Allowed optimization — precompiled / cached guards

Before saturation, walk the type graph and emit all the conditional
rewrites it implies as standalone rules into the e-graph. The
type-graph structure is erased at that point; the e-graph sees only
concrete rules.

Allowed as an implementation optimization only. The compiler may precompile
or cache guard results when sound, but online monotone fact discovery remains
semantic: facts discovered during saturation can unlock later guarded
rewrites.

---

## 4. Adjacent questions — resolved in this chunk

- **Subtype semantics for refinements.** Refinements are evidence-backed
  facts with provenance, not casts into new values and not source-level
  witness arguments. Implication rules (`positive_definite` implies
  `symmetric` and `square`, etc.) produce more facts; guarded rewrites
  consume those facts.
- **Variance.** Generic parameters are invariant by default. Parameter
  relationships can authorize conversions, rewrites, obligations, or
  dispatch, but they do not silently substitute one instantiated type for
  another.
- **Contract satisfaction for generics.** Contract satisfaction and
  `impl Contract` monomorph sets are type-graph facts. They drive rewrite
  guard discharge and dispatch, but they do not erase values into runtime
  trait objects.
- **Conversion graph cost.** Conversion legality belongs to the type graph:
  semantic edges carry witnesses, faithfulness, and obligations. Realization
  cost belongs to extraction / lowering and is parameterized by backend
  capabilities and workflow policy.
- **Online derivation.** Guard discharge may query the evolving monotone fact
  store during saturation. Type-graph structure is static, but e-class facts
  can grow monotonically and unlock later guarded rewrites.

---

## 5. Final v2.1 commitment

1. Myco has two semantic substrates: the type graph and the expression
   e-graph.
2. The type graph carries static semantic relationships: type constructors,
   aliases, contract satisfaction, `impl` monomorph sets, unit dimensions,
   conversion legality, generic instantiations, and refinement implication
   rules.
3. The e-graph carries value expressions, value equalities, rewrite results,
   conversion-result terms, and residual candidates.
4. The bridge is a live guard-discharge interface. E-graph rewrites own the
   equation; guard discharge proves whether the rewrite may fire.
5. Guard discharge may consult type-graph facts, envelope facts, matrix facts,
   unit algebra, contract satisfaction, geometry / site facts, shape-phase
   facts, backend capability facts, and adjacent keyed records where a rule
   explicitly permits them.
6. Facts discovered during saturation can unlock later guarded rewrites, as
   long as fact growth is monotone and provenance-tracked.
7. Precompiled / cached guards are allowed for performance, but they are not
   the semantic model.
8. Refinements are facts with evidence and provenance, not casts and not
   user-carried witness objects.
9. Generic parameters are invariant by default.
10. Conversion legality is separate from conversion realization cost.
11. The retired `dyn` framing does not participate in this design. Static
    heterogeneity is `impl Contract`; runtime sizing is `some`.

---

## 6. Out of scope for this chunk

- Implementation (data structures for the type graph itself) — spec,
  not impl.
- Type inference algorithm choice (bidirectional, constraint-based,
  etc.) — orthogonal; the graph structure is what types *are*, not
  how they're *inferred*.
- Universe / kind system — we don't have one; not planning to add one
  to support this chunk.

---

## 7. Dependencies / ordering

This chunk was tackled after chunk 06 landed; chunks 04, 05, and 06
supplied the expression substrate, matrix refinement examples, and backend
capability model:

- Chunk 04 locks the expression e-graph substrate that the type graph
  has to bridge to.
- Chunk 05 supplies the strongest refinement-lattice examples through
  matrix facts, envelope views, shape phases, and conversion scope.
- Chunk 06 settles backend abstraction; legal conversion edges can have
  backend-dependent realization costs, so the backend interface needed to
  exist first.

This chunk does **not** block chunk 08 (B2 joint syntax / B4 coupling
machinery); those are orthogonal.

---

## 8. Closed questions

- **Q1.** Mechanization choice. RESOLVED: two substrates with an explicit
  live guard-discharge bridge; precompiled / cached guards are optimization;
  one unified type/value e-graph is rejected.
- **Q2.** Inventory boundary. RESOLVED: type graph = static semantic
  relationships; e-graph = value equalities and rewrites; envelope = facts on
  e-classes; adjacent keyed state = keyed runtime / process records.
- **Q3.** Refinement semantics. RESOLVED: refinements are facts with evidence
  and provenance, not casts or user-carried witness arguments.
- **Q4.** Variance. RESOLVED: generic parameters are invariant by default;
  relationships across parameters are explicit facts, conversions,
  obligations, or dispatch rules.
- **Q5.** `dyn` interaction. VOID: `dyn` is retired. Static heterogeneity is
  `impl Contract`; runtime sizing is `some`.
- **Q6.** Conversion-graph cost. RESOLVED at the semantic boundary:
  conversion legality is type-graph meaning; realization cost is extraction /
  lowering and backend-policy information.
- **Q7.** Online derivation. RESOLVED: guard discharge may query evolving
  monotone facts during e-graph saturation.

---

## 9. Notes to self (historical)

- Riley surfaced this question unprompted during the consolidation-
  audit wait on 2026-04-20. Treat as a confirmed-intent topic, not a
  speculative one.
- The three-option framework (A/B/C) in §3 was Claude's framing in
  the live conversation. The locked answer is A semantically, C as
  optimization, reject B.
- The "e-graph holds the equation, type graph provides the guard"
  framing in §2 is the most compact summary of the three interaction
  points. Lead with that if revisiting cold.
- Chunk 07 number was previously earmarked for B2+B4 (joint syntax /
  coupling). Bumped to 08 with Riley's approval on 2026-04-20 when
  he asked for this topic to take the 07 slot.
