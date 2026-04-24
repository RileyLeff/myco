# Myco v2.1 — Kernels & Spatial Optimization Design Report (IN PROGRESS)

**Date:** 2026-04-19 (draft started; discussion ongoing)
**Authors:** Riley Leff, Claude (Opus 4.7)
**Reviewers:** None yet — discussion still open
**Status:** IN PROGRESS. Kernel identity and finite assembly
(`kernel_matrix` / `gram`) are locked; sparse / low-rank rewrites,
integration operators, and GP/HSGP process machinery remain under
discussion.

---

## 1. Context

v2.0 spec and v2.1 work-in-progress have no first-class notion of a "kernel"
(spatial weighting function, integral transform, or convolution-like
aggregation). Sperry-style and Potkay-style plant models need at minimum:

- light competition / canopy integration (how much light reaches depth `z`
  given neighbors and self-shading)
- hydraulic kernels (how a point water potential couples to neighbor potentials
  via the hydraulic network)
- root uptake kernels (how a root at depth `z` draws on soil water distributed
  over a vertical profile)
- (eventually) spatial interaction kernels in an ecosystem-scale simulation

Riley's framing in the session that started this thread: "good kernel support
is one of those things that takes myco from a toy or a research demo to an
actually useful modeling powerhouse."

This report is the scratchpad for that design discussion.

---

## 2. What "kernel" means here

A kernel is a parameterized relation with two input domains and one explicit
scalar output:

```myco
relation k<A, B, U>(
    x: A,
    y: B,
    out: Scalar<U>,
) {
    out = ...
}
```

The common spatial covariance / interaction specialization is point-point over
one locus:

```myco
relation spatial_k<L: Locus, U: Unit>(
    x: Point<L>,
    y: Point<L>,
    out: Scalar<U>,
) {
    out = ...
}
```

Point-point same-locus kernels are not the definition. The same kernel
identity covers continuous/continuous, discrete/discrete, and cross-domain
relations:

```myco
relation root_uptake_weight(
    root: RootSegment,
    z: Point<SoilDepth>,
    out: Scalar<dimensionless>,
) { ... }

relation neighbor_competition(
    a: Tree,
    b: Tree,
    out: Scalar<dimensionless>,
) { ... }
```

Nothing about the name `kernel` is mechanically load-bearing in Myco. There is
no `kernel` keyword, no kernel block, and no kernel type kind. Kernel identity
is a relation role plus downstream consumers (`gram`, GP priors, integration /
convolution operators, sparse assembly).

The useful phrase is:

- **Kernel relation:** the user or stdlib parameterized relation of shape
  `A, B, out`.
- **Kernel facts:** compiler-facing facts/capability contracts such as
  `PositiveDefinite<A>`, `Stationary<L>`, `Isotropic<L>`, and
  `CompactSupport<A, B>(radius)`.
- **Kernel consumers:** operators that need those facts, such as
  `gram(k, points)`, GP/HSGP priors, convolution/integration operators, and
  sparse matrix assembly.

**Decided:** universality is required. The standard library will provide the
common covariance kernels (Matérn family, squared-exponential/RBF,
rational-quadratic, Wendland compact-support, etc.), but users must be able to
write arbitrary kernel relations. Myco is not in the business of enumerating a
closed taxonomy of kernel shapes.

**Decided:** kernel facts are evidence, not trust annotations. Stdlib kernels
carry audited facts; user-authored visible relations may receive facts the
compiler can derive from body composition and stdlib atom contracts. If a
consumer requires a fact the compiler cannot establish, the use reports an
unmet obligation rather than silently treating the relation as a valid
covariance or sparse kernel.

## 2.1 Finite assembly: `kernel_matrix` and `gram`

`kernel_matrix(k, xs, ys)` is the general finite assembly surface for
two-domain kernel relations:

```myco
relation k<A, B, U>(x: A, y: B, out: Scalar<U>)

W = kernel_matrix(k, xs, ys)
```

Semantics:

```text
W[i, j] = k(xs[i], ys[j])
row_axes(W) = xs
col_axes(W) = ys
entry_unit_law(W[i,j]) = output_unit(k)
construction_provenance(W) = evaluated_pairwise(k, xs, ys)
kernel_matrix_of(W, k, xs, ys)
```

This is the right assembly for cross-domain operators such as root uptake
from root segments to soil-depth points, shade from leaves to canopy points,
or any discrete/continuous pairing. `kernel_matrix` does not emit symmetry,
PSD, or covariance facts merely because it is kernel-shaped.

`gram(k, points)` is the same-domain covariance specialization:

```myco
relation k<A, U>(x: A, y: A, out: Scalar<U>)

K = gram(k, points)
```

It is sugar for `kernel_matrix(k, points, points)` plus same-domain fact
rules:

```text
gram_of(K, k, points)
row_axes(K) = points
col_axes(K) = points
construction_provenance(K) = evaluated_pairwise(k, points, points)
```

Fact emission:

- `SymmetricKernel<A>` emits `symmetric(K)`.
- `PositiveDefinite<A>` emits `positive_semidefinite(K)`.
- `StrictPositiveDefinite<A>` plus `distinct(points)` emits
  `positive_definite(K)`.
- `CompactSupport<A, A>(radius)` emits a `zero_pattern` when the A-domain
  distance / adjacency evidence proves pairs outside support.

PSD and PD are intentionally separate. Ordinary Cholesky consumes
`positive_definite(K)`, not merely `positive_semidefinite(K)`. If a downstream
primitive requires PD and the compiler knows only PSD, the compiler reports an
unmet `positive_definite(K)` obligation. It does not silently add jitter,
select a pivoted factorization, or route to an opaque backend. Valid routes are
explicit: prove strict PD plus distinct points, model jitter, or choose a
primitive / workflow policy that accepts PSD.

---

## 3. What was rejected

- **A new `kernel` keyword or kernel kind.** Rejected in favor of "kernels are
  parameterized relations." Nothing about the user-facing surface should imply
  kernels are a distinguished construct.

- **A stdlib-only hierarchy like `SpatialKernel<Reduction, Profile>`.** Rejected
  because it is not universal — users could not express arbitrary cross-domain
  or domain-specific kernel relations in it.

- **User property declarations for kernel facts.** Retired with the broader
  property-annotation surface. The compiler may infer facts where possible
  (e.g., closure-preserving composition of stdlib kernels), and stdlib kernels
  may carry audited facts, but user-authored kernels do not get
  `PositiveDefinite` / `Stationary` / compact-support facts merely by
  assertion.

---

## 4. Ambient-locus problem (solved by existing composition)

If a kernel needs a position in a larger domain (e.g., a leaf in a canopy in an
ecosystem), how does it get ambient coordinates without hardcoding a global
spatial frame?

**Answer:** this is solved by the horse/fly composition pattern already in
v2.1. Parents expose scalar coordinate fields; children sample the parent via
`.at()`. Visibility is downward-only. No new machinery needed.

---

## 5. Sparsity / characteristic length — deferred

Riley's instinct: characteristic length (how far a kernel "reaches") belongs as
a parameter on the relation declaration in `.myco`, and its concrete value gets
bound from the workflow layer like any other parameter. Not yet fully locked.
Revisit after the e-graph / cost / unified-machinery layer is drawn up, because
the right answer depends on whether the compiler can derive effective support
from the relation body / stdlib facts vs. needs a separate evidence path.

---

## 6. Integration semantics — deferred

Mixing continuous domains (e.g., canopy height as a real interval) and discrete
collections (e.g., leaves at specific heights) in the same kernel integrand is
needed. Proposal floated: `integrate(expr for p in D)` where `D` can be a
continuous domain, a discrete collection, or a mix. Syntax not locked. Semantics
not locked. Interaction with the residual graph not specified. Revisit after
unified machinery.

---

## 7. The unified-machinery direction (this is the live thread)

The discussion opened up the question: if we declare kernel optimizations —
lossless rewrites, approximations, cost-driven substitutions — they share DNA
with:

- **overdetermination / closure policies** (two candidate values for the same
  quantity, user picks a blending rule: `weighted_average`, `soft_select`,
  `hard_select`, or custom)
- **symbolic equivalence** (algebraic rewrites that preserve meaning exactly)
- **numerical conditioning** (what the compiler currently calls the "lightweight
  machinery" for well-conditionedness — proposed to expand into a real
  cost/compute model)

Riley's framing (paraphrased): relations are beads on a string; find two
equivalent beads, tie them together, and the previously-disjoint strings get
traced through the joint. If a kernel optimization is fully equivalent (no
information lost, same interface), it should just be another relation, and the
compiler should find the equivalence automatically. Only lossy optimizations
need special handling — they are world-claims of a weaker kind.

**Three-way optimization cut proposed:**

1. **Lossless** — compiler-internal rewrites. No user surface beyond possibly
   declaring that two expressions are equivalent. The compiler is free to
   substitute.
2. **Lossy-as-model-claim** — lives in `.myco`. Extends the overdetermination /
   closure-policy machinery. User is asserting "these two are approximately
   equal in the sense I'll describe," and the approximation error is a
   modeling choice.
3. **Lossy-as-tolerance** — lives in the workflow layer. "For this binding, I'm
   willing to lose this much accuracy for this much speedup." Workflow-verbs
   grow a tolerance budget.

**Critical discovery during the follow-up investigation:** The e-graph, which
was a load-bearing commitment in v1 and survived into early v2 drafts, was
quietly edited out of the current v2 spec during a section rewrite (the
references became orphans when their defining section was rewritten, and the
fix chosen was to remove the references rather than redefine them). The
v2.1 work inherited this absence.

This is a meaningful regression, not a design pivot. The unified-machinery
design assumes an e-graph substrate. So before continuing this discussion, the
v2.1 spec needs an explicit commitment to e-graphs internally, with a clean
statement of how the "residual graph" (user-facing diagnostic view) relates to
the e-graph (internal equality substrate).

**Plan going forward (in this order):**

1. Draft the v2.1 commitment to e-graphs internally, restoring the v1
   commitment and stating the residual-graph / e-graph relationship cleanly.
2. Pass over the v2.1 docs (v2.1_in_progress.md, open_questions.md, mocks,
   other chunk reports) for anywhere that needs updating given the e-graph
   substrate is explicit.
3. Return to this kernel discussion. Design the unified machinery:
   - cost model (per-op table vs. relation facts vs. backend/provider
     capability evidence)
   - rewrite-rule surface (ordinary relations plus compiler/stdlib rewrite
     facts; no resurrected `property` annotation surface)
   - tolerance plumbing (how workflow-level tolerance reaches the compiler's
     extraction decisions)
4. With the machinery drawn up, revisit Sections 5 (sparsity) and 6
   (integration semantics) — the right answers should fall out of the
   unified view.

---

## 8. Open questions deferred out of this thread

- Exact stdlib layout for kernels (module structure, naming, which kernels
  ship first)
- Whether `condition_weighted` closure policy gets resurrected with a
  `condition_of(expr)` intrinsic now that we're taking cost modeling
  seriously
- Syntax for compact support and piecewise-defined kernels
- Whether separability is declared or inferred
- Kernel composition (kernel of kernels) — out of scope until primary
  machinery is locked

---

## 9. Notes to self (for resuming)

- Riley explicitly endorsed the e-graph-as-substrate unification direction:
  "no i think this is the right direction."
- Riley flagged that equality in the overdetermination machinery can be
  strict or fuzzy, and fuzzy equality is sometimes useful — this is a
  feature of the unified machinery, not a bug.
- Stale items the subagent flagged while surveying: spec §8.5 "structural
  introspection" language is stale; `mock_potkay.myco` still has retired
  `slot` syntax and `[t+1]` indexing. Low priority, adjacent tidies.
- The `rule` keyword was killed in v2.1, replaced by `event` for topology
  change. Any rewrite-rule design for kernel optimization must not resurrect
  `rule` — pick a different name.
