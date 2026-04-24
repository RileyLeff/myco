# Myco v2.1 — Kernels & Spatial Optimization Design Report (IN PROGRESS)

**Date:** 2026-04-19 (draft started; discussion ongoing)
**Authors:** Riley Leff, Claude (Opus 4.7)
**Reviewers:** None yet — discussion still open
**Status:** IN PROGRESS. Kernel identity and finite assembly
(`kernel_matrix` / `gram`) are locked; ordinary `integrate` / `sum`
kernel-operator semantics are locked; exact support / locality /
truncation semantics are locked; sparse / index lowering semantics
are locked; low-rank rewrites and GP/HSGP process machinery remain
under discussion.

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
convolution operators, sparse / index lowerings).

The useful phrase is:

- **Kernel relation:** the user or stdlib parameterized relation of shape
  `A, B, out`.
- **Kernel facts:** compiler-facing facts/capability contracts such as
  `PositiveDefinite<A>`, `Stationary<L>`, `Isotropic<L>`, and
  exact support predicates / `CompactSupport<A, B>(radius)` summaries.
- **Kernel consumers:** operators that need those facts, such as
  `gram(k, points)`, GP/HSGP priors, convolution/integration operators, and
  sparse / index lowerings.

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
- Exact support / `zero_when` facts emit `zero_pattern` when the A-domain
  distance / adjacency evidence proves finite pairs are zero;
  `CompactSupport<A, A>(radius)` is one common summary that can help establish
  those facts.

PSD and PD are intentionally separate. Ordinary Cholesky consumes
`positive_definite(K)`, not merely `positive_semidefinite(K)`. If a downstream
primitive requires PD and the compiler knows only PSD, the compiler reports an
unmet `positive_definite(K)` obligation. It does not silently add jitter,
select a pivoted factorization, or route to an opaque backend. Valid routes are
explicit: prove strict PD plus distinct points, model jitter, or choose a
primitive / workflow policy that accepts PSD.

## 2.2 Kernel operators: ordinary expressions, recognized facts

Kernel operators do not get a new source construct in v2.1. The canonical
continuous form is ordinary `integrate`:

```myco
effect(x) =
    integrate(k(x, y) * source(y), y, Domain)
```

The canonical finite form is ordinary `sum`:

```myco
effect(x) =
    sum(k(x, sample.location) * sample.value * sample.weight
        for sample in observations)
```

No `kernel_apply`, `kernel_operator`, or `convolve` source sugar is required up
front. A future helper can exist only as transparent desugaring to ordinary
Myco expressions.

`integrate(expr, y, Domain)` uses only the domain's canonical measure: length,
area, volume, or counting measure where the domain is finite. Model-specific
weights stay visible in the graph:

```myco
light(z) =
    integrate(canopy_k(z, h) * leaf_area_density(h) * transmission(h),
              h,
              CanopyHeight)

effect(x) =
    sum(k(x, node.pos) * field(node.pos) * node.volume
        for node in mesh.nodes)
```

The compiler recognizes kernel operators by normalizing ordinary expressions,
not by asking the user to mark them. Recognition is allowed when the body is a
linear kernel action over the bound variable:

```myco
integrate(k(x, y) * source(y), y, Domain)
integrate(source(y) * k(x, y) * density(y), y, Domain)
sum(k(x, item.pos) * item.value * item.weight for item in items)
```

Recognized operators may emit compiler facts:

```text
kernel_integral_of(effect, k, source_factor, x, y, Domain)
kernel_sum_of(effect, k, source_factor, x, item, items)
linear_in(effect, source_factor)
operator_domain(effect) = Domain
operator_target(effect) = x
operator_measure(effect) = canonical_measure(Domain)
```

Nonlinear wrapping, such as `integrate(exp(k(x, y) * source(y)), y, Domain)`,
may be valid Myco but is not a recognized linear kernel operator and does not
receive sparse / convolution / low-rank lowering facts.

Kernel facts live on kernel relations; operator facts live on recognized
operator expressions. Use-site domain, measure, weights, boundaries, and source
factors decide which relation facts survive. For example, exact support can
emit `local_coupling(effect)`, while `zero_when` emits a finite
`zero_pattern` only when axes prove separation. A normalized nonnegative kernel
can preserve constants only when the full operator context preserves
normalization; masks or empirical weights remove that fact unless separately
proven.

Finite `sum` kernel operators are exact finite semantics. The compiler may
assemble a `kernel_matrix` and perform a matrix-vector contraction without an
approximation ledger. Continuous `integrate` kernel operators remain continuous
semantic objects until closed exactly or explicitly lowered. Quadrature,
truncation, mesh sampling, inducing points, and low-rank / HSGP-style bases are
approximations unless exactness is proven; they require workflow approximation
policy or an explicit `.myco` `approximate` model claim and must emit
provenance such as:

```text
quadrature_lowering_of(discrete_effect, continuous_effect)
approx_error(discrete_effect, continuous_effect, envelope)
relaxation_ledger_entry(discrete_effect)
```

No continuous kernel integral is silently made finite just because a backend
needs finite compute.

## 2.3 Exact support, locality, and truncation

Exact support is a predicate-shaped model fact, not a radius-only annotation.
The core vocabulary:

```text
support(k) = P(x, y)
nonzero_region(k) = Q(x, y)
zero_when(k, R(x, y))
boundary(k) = B(x, y)
boundary_smoothness(k, B) = C0 | C1 | C2 | ... | C∞ | discontinuous | unknown
tail_bound(k, outside_region, envelope)
truncation_of(truncated, original, region)
```

Closed support, nonzero region, and exact-zero predicates are distinct. Support
describes dependency geometry, `zero_when` drives exact sparse patterns, and
boundary smoothness controls gradient / event behavior.

Exact support facts may come from visible relation bodies, audited stdlib
facts, or provider-validated finite artifacts. User-authored `.myco` cannot
assert unchecked compact support:

```myco
# rejected shape, not real syntax
property k is CompactSupport(r)
```

Provider validation can prove facts about a concrete run artifact, such as
`zero_pattern(K)`, without turning the source relation into a globally compact
kernel.

Support predicates can be metric, graph, directed, anisotropic, or
domain-specific:

```text
support(k) = distance(x, y) <= r
support(k) = edge_exists(a, b)
support(k) = upstream_of(y, x) && path_length(y, x) <= r
support(k) = abs(dx) <= rx && abs(dy) <= ry && windward(x, y)
```

Structured summaries are optimization facts derived from or shipped alongside
the predicate:

```text
metric_radius(k) = r
graph_hop_radius(k) = 1
anisotropic_box(k) = (rx, ry)
directed_support(k)
local_coupling(k)
```

Boundary smoothness decides whether a support boundary creates gradient /
event obligations. Smooth compact kernels can emit differentiability facts up
to their proven order; discontinuous or unknown boundaries require ordinary
crossing policy for gradient-sensitive use.

Exact compact support and workflow truncation are separate. If a truncated
kernel is written in `.myco`, truncation is a model claim and can emit exact
support / zero facts when proven. If a workflow truncates an infinite-tail
kernel such as RBF, it is an approximation:

```text
tail_bound(rbf, distance > cutoff, eps)
truncation_of(truncated_op, original_op, distance <= cutoff)
approx_error(truncated_op, original_op, envelope)
relaxation_ledger_entry(truncated_op)
```

Tail bounds create approximation opportunities and envelopes, never exact
`zero_pattern` facts by themselves.

Downstream consequences:

```text
local_coupling(effect)
dependency_region(effect(x)) = { y | support(k)(x, y) }
zero_pattern(W[i, j]) when zero_when(k(targets[i], sources[j]))
sparse_candidate(W)
neighbor_index_candidate(k, metric_radius = r)
truncation_candidate(k, cutoff, envelope)
```

Support composition follows ordinary expression structure:

- Additive combinations use support union; exact zero requires every summand
  to be zero unless cancellation is proven.
- Multiplicative combinations use support intersection; exact zero follows
  when either factor is zero.
- Scaling preserves support with nonzero evidence, collapses support with
  zero evidence, and otherwise emits only conservative bounds.
- Source-side masks refine dependency regions only with structural or
  provider-validated zero facts, not runtime accidents.

## 2.4 Sparse / index lowering and provider patterns

Sparsity is semantic evidence; sparse storage is an execution choice. Exact
support and `zero_when` facts may emit:

```text
zero_pattern(W[i, j])
sparse_candidate(W)
dependency_region(effect(x))
neighbor_index_candidate(effect, predicate)
```

but they do not make source values `CSR`, `CSC`, block-sparse, or stored at
all.

The planner may materialize exact finite patterns:

```text
row_neighbors[i] = { j | not zero_when(k(xs[i], ys[j])) }
csr_pattern_of(P, W)
neighbor_list_for(effect, P)
pattern_from_support(P, support(k), xs, ys)
```

Exact index lowering requires complete coverage:

```text
complete_for(index, support_predicate, axes)
```

If the support predicate may hold for `(x, y)`, then `index.query(x)` must
return `y`. Soundness is optional:

```text
sound_for(index, support_predicate, axes)
```

False positives are legal when the exact predicate is checked before
accumulation. Missing possibly-nonzero pairs are not exact; ANN or any other
index that may miss pairs requires approximation policy and ledger facts.

Patterns and indexes carry phase / invalidation:

```text
pattern_phase(P) =
    compile_static | bind_static | step_static | dynamic_query
depends_on(P, facts...)
invalidates_on(P, event)
```

Planner choices include rebuilding, runtime query, dynamic sparse execution,
host routing, replan, or capability rejection when a backend cannot preserve
the phase semantics.

Sparse / index lowering is operator-general:

```text
sparse_matrix_lowering(W)
sparse_matvec_lowering(effect)
neighbor_sum_lowering(effect)
matrix_free_kernel_action(effect)
runtime_spatial_query(effect)
fixed_pattern_dynamic_values(effect)
```

The planner may materialize a matrix, materialize only row / column patterns,
or use matrix-free neighbor iteration. All legal exact forms preserve the same
support coverage and arithmetic semantics.

Workflow policy may choose among legal exact lowerings:

```text
lowering_candidate(effect, dense)
lowering_candidate(effect, csr)
lowering_candidate(effect, neighbor_list)
lowering_candidate(effect, matrix_free)
requires_capability(effect, sparse_matvec)
requires_capability(effect, dynamic_query)
cost_of(candidate) = ...
```

Storage policy cannot authorize dropped pairs, threshold sparsification,
tail truncation, or ANN misses; those are approximation policies.

Providers may supply concrete sparse patterns, neighbor graphs, spatial
indexes, or query structures. Validations produce artifact-level facts:

```text
csr_pattern_of(P, W)
complete_for(index, support_predicate, axes)
sound_for(index, support_predicate, axes)
pattern_phase(P) = bind_static
depends_on(P, axes, radius)
validated_by(P, exact_distance_check)
validated_by(P, topology_adjacency_certificate)
```

Provider validation can satisfy obligations for a concrete run artifact, but
does not grant unchecked relation-level facts such as `support(k)` or
`CompactSupport<A, B>(r)`.

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

## 5. Support / locality / sparse-index lowering locked

Characteristic length is one structured summary of exact support, not the
support model itself. Radius, hop count, directionality, and bounding boxes are
optimization facts derived from predicate-shaped support or audited stdlib
facts. Sparse / index lowering semantics are locked: exact lowerings require
complete coverage, may be conservative, carry phase / provenance facts, and
remain operator-general. Concrete backend storage kernels and cost calibration
remain implementation work.

---

## 6. Integration semantics — locked at the ordinary-expression layer

Mixing continuous domains and finite collections is handled by the existing
Myco distinction between `integrate` and `sum`. Continuous `integrate` uses the
domain's canonical measure and remains continuous semantics; finite `sum` over
collections is exact finite semantics. Kernel structure is recognized from the
ordinary expression when the compiler can normalize it into a linear kernel
action.

This closes the need for a special kernel integration syntax. The remaining
work is approximation policy for quadrature / low-rank transforms, concrete
sparse backend implementations, and GP/HSGP process consumers.

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
4. With the machinery drawn up, revisit low-rank / process-consumer questions.
   Section 5's support and sparse-index semantics and Section 6's source
   semantics are locked; low-rank approximation policy remains open.

---

## 8. Open questions deferred out of this thread

- Exact stdlib layout for kernels (module structure, naming, which kernels
  ship first)
- Whether `condition_weighted` closure policy gets resurrected with a
  `condition_of(expr)` intrinsic now that we're taking cost modeling
  seriously
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
