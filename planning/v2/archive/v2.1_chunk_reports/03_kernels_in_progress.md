# Myco v2.1 — Kernels & Spatial Optimization Design Report (IN PROGRESS)

**Date:** 2026-04-19 (draft started; discussion ongoing)
**Authors:** Riley Leff, Claude (Opus 4.7)
**Reviewers:** None yet — discussion still open
**Status:** IN PROGRESS. Kernel identity and finite assembly
(`kernel_matrix` / `gram`) are locked; ordinary `integrate` / `sum`
kernel-operator semantics are locked; exact support / locality /
truncation semantics are locked; sparse / index lowering semantics
are locked; low-rank / feature approximation semantics are locked;
process-prior / GP-HSGP consumer semantics are locked. Concrete
backend implementations, PPL serializers, approximation-family
catalogs, and non-GP process-law catalogs remain implementation /
catalog work.

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

## 2.5 Separability, features, and low-rank approximation

Low-rank is not one semantic category. Myco distinguishes:

- exact separability,
- exact finite feature expansion,
- approximate feature / low-rank expansion.

Exact separability is an algebraic fact over product structure:

```text
k((x1, z1), (x2, z2)) = kx(x1, x2) * kz(z1, z2)
separable_kernel(k, product_axes=[X, Z])
kernel_factors(k) = [kx, kz]
product_domain(D, [X, Z])
product_axes(points, [X_points, Z_points])
kronecker_factorization(K, [Kx, Kz])
```

Kernel separability, product-domain / axis structure, and source
separability are distinct. Product grids can authorize exact Kronecker Gram
lowerings; product quadrature can authorize tensor-product operator lowerings;
fully separated operator action additionally requires source separability.

Exact finite feature expansions are exact representations:

```text
k(x, y) = sum_m phi_m(x) * lambda_m * phi_m(y)
exact_feature_expansion(k, Phi, Lambda)
feature_map(Phi, domain=A, feature_axis=M)
rank_bound(gram(k, points)) <= M
```

They may be recognized from visible `.myco` relations or shipped as audited
stdlib facts. Feature maps, modes, eigenfunctions, inducing points, and random
features are ordinary relations or workflow artifacts, not a new source
`basis` construct.

Approximate feature expansions cover Nyström, inducing points, random
features, truncated spectral / HSGP, truncated SVD, and related methods:

```text
approx_feature_expansion(k_M, k, Phi_M, Lambda_M)
approximation_scope(k_M, scope)
approx_error(k_M, k, envelope)
relaxation_ledger_entry(k_M)
rank_bound(gram(k_M, points)) <= M
```

Approximation scope is explicit:

```text
approx_relation(k_M, k, DomainA x DomainB, envelope)
approx_matrix(K_M, K, axes, norm, envelope)
approx_operator(T_M, T, source_space -> target_space, envelope)
```

A lower-scope approximation does not imply a higher-scope one. A truncated SVD
of one Gram matrix is not a relation-level kernel fact; a relation-level
approximation does not imply an operator bound unless measure / source-space
facts support that propagation.

Envelopes propagate only through named implication rules. Preserved structural
facts such as symmetry, PSD, support, conservation, or rank are emitted only
when the construction proves them. Error bounds do not silently authorize
substituting approximate objects into exact obligations.

Low-rank covariance constructions may emit PSD and rank facts:

```text
K_M = Phi * Lambda * transpose(Phi)
nonnegative_diagonal(Lambda)
positive_semidefinite(K_M)
rank_bound(K_M) <= M
```

They emit `positive_definite(K_M)` only with explicit full-rank evidence or an
explicit positive diagonal / noise component. No low-rank approximation
silently adds jitter or upgrades PSD to PD.

Authorization routes:

- exact compiler rewrites when separability / feature structure is proven,
- source `approximate` model claims,
- workflow approximation policy.

A finite-feature relation written directly in `.myco` is just the model unless
it is explicitly related to a richer kernel by approximation provenance.

Nyström / inducing-point methods are finite intermediary-axis approximations:

```text
Z = inducing_axis
Kxz = kernel_matrix(k, X, Z)
Kzz = gram(k, Z)
K_approx = Kxz * solve(Kzz, transpose(Kxz))
nystrom_approximation(K_approx, K, inducing_axis=Z)
approximation_scope(K_approx) = finite_matrix(X)
```

All solver obligations on `Kzz` remain ordinary matrix obligations. If
`positive_definite(Kzz)` is unknown, ordinary inverse / Cholesky routes report
it; PSD-compatible primitives or explicit stabilization are chosen visibly.

Random-feature approximations are workflow artifacts with random-draw
provenance, not source-model stochastic roots:

```text
random_feature_approximation(k_M, k, feature_axis=M)
feature_draw(Phi_M, distribution, seed)
probabilistic_error_bound(k_M, k, confidence=0.99, envelope)
reproducible_artifact(k_M) | stochastic_plan_artifact(k_M)
```

HSGP is a spectral truncation workflow approximation over explicit bounded
domain and boundary-condition facts:

```text
spectral_family(k, domain, boundary_condition)
mode_set(Phi_M, domain, boundary_condition, count=M)
orthonormal_modes(Phi_M, measure)
lambda_nonnegative(Lambda_M)
spectral_truncation_of(k_M, k, modes=Phi_M)
approx_feature_expansion(k_M, k, Phi_M, Lambda_M)
tail_bound(k, modes_not_in(Phi_M), spectral_envelope)
```

Process-prior machinery consumes these facts. HSGP is one workflow
approximation pattern over general spectral / feature-expansion semantics, not
a source-language mechanism.

## 2.6 Process priors and GP / HSGP consumers

Process priors are workflow-side sources over indexed `.myco` contracts. The
source model declares indexed relations, fields, and contracts; the workflow
binds a `ProcessPrior<I,V>` by naming index slots and value slots explicitly:

```python
workflow.bind(
    "Plant.vulnerability_curve",
    ProcessPrior(
        index=["psi"],
        value="loss",
        law=GaussianProcess(mean=Zero(), kernel=matern52),
    ),
)
```

`index` and `value` are not input/output directions. They identify the finite
projection axis and sample component coupled by the process law. The compiler
does not infer them by relation position.

Process-valued uncertainty is a sample shape / identity fact, not a third
uncertainty kind:

```text
uncertainty kind: epistemic | aleatoric
sample shape: scalar | vector | record | field/process |
              graph-indexed family | temporal path
```

Core facts:

```text
process_root(P)
process_index_contract(P) = I
process_value_contract(P) = V
projection_of(x_i, P, index_i)
same_process_root(x_i, x_j)
process_coupling(P, kernel_or_law)
epistemic_process(P) | aleatoric_process(P)
```

The general source object is `ProcessPrior<I,V>`. A process law advertises
finite-projection capabilities:

```text
ProcessLaw<I,V>
FiniteProjectionLaw<I,V>
ClosedFormConditionable<I,V>
ProjectionLogDensity<I,V>
ProjectionSampleable<I,V>
ApproximationFamily<I,V>
```

A GP law requires `mean: I -> V` and `covariance/kernel: I x I ->
Covariance<V>`. It emits a finite joint over demanded projections when model
reads, observations, or prediction queries require them:

```text
points = observed_points union required_model_points union prediction_points
K = gram(kernel(P), points)
mu = mean(P, points)
finite_process_joint(y_points, mu, K)
```

Projections from one process share one stochastic root; they are not independent
roots created at each point. Observations condition those projections rather
than equationally merging them with data:

```text
projection f_i = P.at(x_i)
observation y_i = f_i + eps_i
eps_i ~ NoiseLaw(...)
likelihood_term(log_density(noise, y_i - f_i))
```

Exact observation is explicit, and noisy observation requires an explicit noise
law. There is no silent nugget, jitter, or observation noise.

Structured values are first-class. `ProcessPrior<I,V>` supports scalar, vector,
tensor, enum-narrowed record, and named-record values. For structured `V`,
finite projections flatten into `(projection, component)` axes:

```text
process_component_axis(P) = [height_gain, leaf_area_gain]
joint_process_axis(P) = projection_axis x component_axis
entry_unit_law(K[(i,a),(j,b)]) = unit(value[a]) * unit(value[b])
```

Structured covariance validity is proven over that flattened domain through a
product-domain kernel fact, an audited `PositiveOperatorValuedKernel<I,V>`, or
visible construction rules such as separable output kernels, LMC /
coregionalization, or shared-latent factor construction. User source cannot
assert covariance validity without evidence.

Process priors dispatch after finite projection construction:

```text
ProcessPrior bound
-> finite projections demanded
-> finite joint / operator problem constructed
-> A: closed-form process conditioning
-> B: authorized approximation
-> C: whole stochastic SCC / process inference task
```

Tier C receives the whole stochastic SCC, including process projections,
observations, downstream deterministic relations, constraints, approximation
facts, and kernel/process-law facts. Posterior predictive means, covariances,
draws, and diagnostics are workflow results with provenance; they do not mutate
the source process or become new global process facts unless a closure rule
proves such a process.

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
work is approximation policy for quadrature, concrete sparse / low-rank backend
implementations, and concrete process-inference backends.

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
4. With the machinery drawn up, revisit process-consumer questions. Section
   5's support and sparse-index semantics, Section 6's source semantics, and
   Section 2.5's low-rank / feature approximation semantics are locked.

---

## 8. Open questions deferred out of this thread

- Exact stdlib layout for kernels (module structure, naming, which kernels
  ship first)
- Whether `condition_weighted` closure policy gets resurrected with a
  `condition_of(expr)` intrinsic now that we're taking cost modeling
  seriously
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
