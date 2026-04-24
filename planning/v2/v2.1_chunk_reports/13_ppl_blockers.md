# Myco v2.1 - PPL Blocker Cluster (B1/B2/B4)

Status: LOCKED

This report closes the remaining named PPL blockers from the chunk 04
audit thread:

- B1: distribution contract shape and opaque stochastic-family policy.
- B2: joint declaration syntax.
- B4: coupling machinery for correlated / structured stochastic samples.

The lock is intentionally narrow. It commits the core semantic
mechanics that `spec_new.md` needs, while leaving the Tier 2 family
catalog and per-family capability tables as ordinary stdlib work.

## 1. Distribution Contract Shape

`Distribution` is parameterized by sample type:

```myco
Distribution<S>
```

For scalar families, S is usually `Scalar<U>`. For structured or
multivariate families, S may be a tensor, vector, simplex, or named
record-shaped sample type. This avoids treating every distribution as
if it were scalar-valued and keeps joints inside the same contract
vocabulary as univariate families.

Visible user-authored distributions expose a relation-shaped density
obligation:

```myco
relation log_density(self: Self, sample: S, out: Scalar<dimensionless>)
```

`density` / `pdf` is a default-derived convenience over
`log_density`, not an additional required core method. A stdlib family
may provide a direct closed-form density when useful, but this does
not change the contract's semantic center.

Sampling is backend/runtime capability, not an ordinary `.myco`
relation method. Sampling produces realized values in execution; it
does not assert a new graph equality or authorize symbolic rewrites by
itself.

Visible reparameterization, when available, is relation-shaped:

```myco
relation reparameterize(self: Self, base_noise: B, out: S)
```

That shape lets the compiler reason through the construction when the
body is visible, while still allowing backends to advertise audited
runtime implementations.

## 2. Opaque Stochastic Families

Opaque stochastic families are allowed only as curated stdlib/backend
capabilities. They are not a user-authored `.myco` escape hatch.

Default policy:

- Opaque families are Tier-C-first.
- They expose no symbolic `log_density` fact.
- They grant no automatic closure, derivative, conditioning, or
  independence facts through the opaque density.
- HMC / NUTS / VI require backend-certified differentiable
  opaque-log-density capability.
- Finite-difference or emulation routes require explicit workflow
  `emulate` policy.

The driving example is general alpha-stable. Special cases can rewrite
to visible families when the parameter values make that equivalence
structural:

- alpha = 2 routes through a Normal-like visible form.
- alpha = 1, beta = 0 routes through a Cauchy visible form.
- alpha = 1/2, beta = 1 routes through a Levy visible form, modulo
  the stdlib's parameterization convention.

Those rewrites are visible-family facts. The general opaque evaluator
does not inherit their symbolic properties.

## 3. Joint Declaration Syntax

The canonical semantic form for a joint draw is one structured
stochastic root plus named field projections:

```myco
joint_sample ~ PlantSizeJoint(mu, Sigma)
height = joint_sample.at("height")
diameter = joint_sample.at("diameter")
```

The source language also admits record-`~` sugar:

```myco
{ height, diameter } ~ PlantSizeJoint(mu, Sigma)
{ height: h, diameter: d } ~ PlantSizeJoint(mu, Sigma)
```

Both forms desugar to a hidden synthetic joint root and deterministic
`.at()` projections:

```myco
let __joint ~ PlantSizeJoint(mu, Sigma)
height = __joint.at("height")
diameter = __joint.at("diameter")
```

Rules:

- One record-`~` site creates one stochastic root.
- Field projections are deterministic reads from that root.
- The joint family owns dependence.
- Record syntax is the only destructuring sugar.
- Tuple destructuring and positional indexing are banned.
- Free-floating `correlate(height, diameter, rho)` is banned.

## 4. Coupling Machinery

Coupling lives as joint-envelope metadata on the structured stochastic
root. Field projections inherit that root's coupling facts.

Conceptual metadata:

```text
joint_root: __joint
family: PlantSizeJoint
fields: { height, diameter }
field_units: ...
dependence: coupled | independent_partitions(...) | dependency_graph(...) | opaque_coupling
marginals: optional per-field distribution facts
reparameterization: optional visible base-noise construction
```

Dependence policy:

- Fields from the same joint root are dependent by default.
- Distinct field names do not prove independence.
- Independence-based rewrites require explicit proof from the joint
  envelope.
- Separate stochastic roots are independent by structural default
  conditional on their visible parents; shared upstream stochastic
  parents can still induce marginal dependence through the ordinary
  graph.

Examples:

- Dense MVN: coupled; independent Normal+Normal rewrites are blocked,
  while exact coupled MVN affine rewrites may still fire.
- Block-diagonal MVN: independent partitions can be recorded; closure
  rewrites may fire across proven-independent blocks.
- Opaque copula: `opaque_coupling`; no independence closures and no
  symbolic conditioning facts unless a visible special structure is
  available.

## 5. Canonical Spec Edits

The lock is applied in `spec_new.md`:

- Section 7: contract vocabulary now uses `Distribution<S>`.
- Section 13.2: Tier A/B/C dispatch references the visible density
  relation, backend sampling capability, and opaque-family policy.
- Sections 13.5-13.7: joint roots, named projections, record-`~`
  sugar, and no naked correlation are committed.
- Section 13.10: B1/B2/B4 are closed as the Tier 2 PPL core mechanics.
- Section 27: the distribution-family contract shape is rewritten around
  sample types, visible `log_density`, backend sampling, visible
  reparameterization, and curated opaque stochastic families.
- Sections 33-35: B1/B2/B4 move out of open-blocker status; remaining
  Tier 2 work is family-catalog polish.

`anti_spec.md` records the retired forms so future cleanup does not
re-import scalar-only distributions, required `sample` / `pdf`
methods, user-authored opaque densities, free-floating correlation,
or positional joint destructuring.
