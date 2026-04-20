# Myco v2.1 — Type Graph vs. Expression E-Graph Design Report (IN PROGRESS — STUB)

**Date:** 2026-04-20 (stub created)
**Authors:** Riley Leff, Claude (Opus 4.7)
**Reviewers:** None yet
**Status:** STUB. Captures a design question Riley surfaced live during the
consolidation-audit wait: relations form an e-graph (chunk 04 commitment) —
do *types and their relations* live in a graph too, and if so, how does that
structure interact with the expression e-graph? Undesigned. Do not treat any
of this as settled.

---

## 1. Why this chunk exists

Chunk 04 commits to an e-graph as the internal equality substrate for
*expressions / relations / terms*. What it does **not** say is what
structure the *type system* itself lives in, or how type-level equality
and implication talk to the expression e-graph.

Riley's question (2026-04-20, paraphrased): "I know the defined
relationships in a `.myco` constitute an e-graph. Do the types and their
relations also live in a graph too?"

They do. There are at least five distinct type-level relations in v2.1:

- **Refinement lattice.** `Matrix<U, n, n>` has child refinements
  `Symmetric`, `PosDef`, `Diagonal`, `Triangular`, `Orthogonal`, and
  compositions like `Symmetric ∧ PosDef`. Lattice, not tree.
- **Conversion graph.** Units (`seconds ↔ minutes × 60`), shape reshapes
  (`Tensor<U, (m, n)> ↔ Tensor<U, (m*n,)>` under caveats), precision
  casts. Directed edges, each with cost and possibly a witness function.
- **Definitional / alias equality.** `Vector<U, n>` ≡ `Tensor<U, (n,)>`.
  `Matrix<U, m, n>` ≡ `Tensor<U, (m, n)>`. Currently assumed to be
  resolved at elaboration — terms entering the expression e-graph are
  already normalized.
- **Contract / `dyn` satisfaction.** Which concrete types satisfy which
  contracts; which `dyn` witnesses close over which concrete
  implementations. Bipartite-ish graph between types and contracts.
- **Generic instantiation.** `Tensor<U, shape>` → the concrete
  `(U, shape)` monomorphs that actually appear in a program. Not a
  relation in the type-theoretic sense — more a demand-driven projection.

These relations are not currently described as a unified structure
anywhere in the v2.1 docs. They are implicit in the surface language
(the user can write `Matrix<U, n, n> where Symmetric`) but their
internal representation and the bridge to the expression e-graph is
undesigned.

---

## 2. Where the type graph meets the expression e-graph

Three interaction points, identified live:

1. **Structural view equalities.** `Symmetric.view(M)` and `M` are the
   same value when `M : Symmetric<U, n>`. That equality belongs in the
   expression e-graph (so rewrites using `Symmetric.view(M)` can also
   apply to `M` and vice-versa). The *guard* that licenses the equation
   is a type-graph query. The *equation itself* is an e-graph fact.

2. **Unit-conversion identities.** `x [s] × 60 [min/s]` and `x [s]`
   represent the same physical quantity. That's a rewrite, but it's
   conditioned on unit metadata the type graph owns. The rule is an
   e-graph rule; the preconditions are type-graph facts.

3. **Refinement-gated rewrites.** `A⁻¹ b` → `cholesky_solve(A, b)` is
   only valid when `A : PosDef`. The rewrite lives in the e-graph; the
   precondition is a type-graph query ("is this term's type in the
   `PosDef` refinement?"). Without the precondition the rewrite is
   unsound.

In all three cases the pattern is the same: **e-graph holds the
equation, type graph provides the guard.** The open design question is
how that coupling is mechanized.

---

## 3. Three options for the coupling (not locked)

### Option A — Two graphs, explicit bridge

The type graph is its own machinery (subtype/refinement lattice +
conversion graph + contract satisfaction). The expression e-graph has
rewrite rules *parameterized by type-graph queries*. At saturation
time, applying a rewrite requires a "does type predicate hold for this
e-class?" check.

- **Pros:** clean separation; type-level reasoning uses the right
  algorithms (unification, lattice meets); e-graph stays term-oriented.
- **Cons:** you pay for the predicate query at every rewrite attempt;
  the bridge API is load-bearing and easy to get wrong.

### Option B — One graph, types are terms

Everything — types, refinements, conversion witnesses — lives in the
e-graph. Requires a more expressive equational logic, closer to
dependent-type territory. Type equality is just e-class equality.

- **Pros:** uniform substrate; no bridge; type-level and term-level
  reasoning compose.
- **Cons:** e-graph blow-up; equational logic has to handle variance
  and subtyping, which e-graphs don't natively express; probably
  requires a layered or tagged e-graph.

### Option C — Two graphs, type graph compiled to e-graph rules at elaboration

Before saturation, walk the type graph and emit all the conditional
rewrites it implies as standalone rules into the e-graph. The
type-graph structure is erased at that point; the e-graph sees only
concrete rules.

- **Pros:** no runtime bridge; fast saturation; type-graph algorithms
  stay whatever they want.
- **Cons:** loses online reasoning — anything that depends on a
  type-level derivation discovered *during* saturation is impossible;
  rule set can get large after monomorphization.

No strong lean yet. Option A is the honest default; option C is the
performance-optimal version of A; option B is the most uniform but
also the most ambitious.

---

## 4. Adjacent questions this opens

- **Subtype semantics for refinements.** `PosDef <: Symmetric`:
  width-subtyping? Or refinement-subtyping with a witness function?
  The answer affects whether option A's bridge is a predicate or a
  cast.
- **Variance.** `Tensor<U, shape>` is parameterized by `U`. Does
  `Tensor<Length_m, shape> <: Tensor<Length, shape>` (subtyping on
  units)? Does `Tensor<U, (3,)> <: Tensor<U, shape?>` (existential in
  shape)? Currently not specified.
- **Contract satisfaction for generics.** A generic function
  `foo<T: HasInverse>(x: T)` — when instantiated at `T = Matrix<U, n, n>
  where Symmetric ∧ PosDef`, does the refinement affect which
  implementation of `inverse` is selected? That's a type-graph query
  that drives e-graph rewrite selection.
- **Conversion graph cost.** Chunk 05 open question Q7 asks whether
  `convert` returns structural views (cheap) or copies (expensive).
  That's an edge-cost question in the conversion graph. Belongs here
  once this chunk gets real.
- **Dyn witness caching.** `dyn` objects carry type-graph information
  at runtime. Whether two `dyn` values are interchangeable is a
  type-graph query. Interacts with Tier C distributional machinery
  from chunk 04.

---

## 5. Out of scope for this chunk

- Implementation (data structures for the type graph itself) — spec,
  not impl.
- Type inference algorithm choice (bidirectional, constraint-based,
  etc.) — orthogonal; the graph structure is what types *are*, not
  how they're *inferred*.
- Universe / kind system — we don't have one; not planning to add one
  to support this chunk.

---

## 6. Dependencies / ordering

This chunk is best tackled **after** chunks 04 / 05 / 06 land:

- Chunk 04 locks the expression e-graph substrate that the type graph
  has to bridge to.
- Chunk 05 exercises the refinement lattice most heavily (structural
  matrix subtypes) and will produce the first concrete refinement
  examples to reason about.
- Chunk 06 settles backend abstraction; some type-graph edges
  (precision casts, device placement) have backend-dependent cost, so
  the backend interface needs to exist first.

This chunk does **not** block chunk 07 (B2 joint syntax / B4 coupling
machinery) — those are orthogonal. Numbering preserved: this is 07,
the B2+B4 chunk is now 08.

---

## 7. Open questions to carry forward

- Q1. Is the type graph mechanized as option A, B, or C? (Primary
  blocker for this chunk.)
- Q2. What precisely lives in the type graph vs. the expression
  e-graph? Inventory-level breakdown for every construct in v2.1.
- Q3. Subtype semantics for refinements — predicate, cast, or
  witness-function? Interacts with Q1.
- Q4. Variance rules for generic parameters. Especially for the
  `U` parameter of `Tensor<U, shape>` (unit variance) and the
  `shape` parameter (shape subtyping / existentials).
- Q5. How does `dyn` interact with the type graph? Is a `dyn Contract`
  a type-graph node or an e-class?
- Q6. Cost model for conversion-graph edges. Interacts with chunk 06
  backend abstraction (some edges have backend-dependent cost).
- Q7. Does the type graph support online derivation during e-graph
  saturation, or is it fully compiled out (option C)?

---

## 8. Notes to self (for resuming)

- Riley surfaced this question unprompted during the consolidation-
  audit wait on 2026-04-20. Treat as a confirmed-intent topic, not a
  speculative one.
- The three-option framework (A/B/C) in §3 was Claude's framing in
  the live conversation; Riley said "good call" to filing it but has
  not yet expressed a lean.
- The "e-graph holds the equation, type graph provides the guard"
  framing in §2 is the most compact summary of the three interaction
  points. Lead with that if revisiting cold.
- Chunk 07 number was previously earmarked for B2+B4 (joint syntax /
  coupling). Bumped to 08 with Riley's approval on 2026-04-20 when
  he asked for this topic to take the 07 slot.
