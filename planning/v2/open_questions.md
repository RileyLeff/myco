# Myco v2.1 — Open Questions

Extracted from v2.1_in_progress.md, geometry_design_report.md, and April 2026
design sessions. Organized by topic, roughly prioritized within each section.

---

## Domain Geometry — Remaining Open Questions

The core geometry subsystem is settled (see `geometry_design_report.md`):
`geometry` keyword, `Domain<G>`, `chart`, `topology`, `metric`, `locus`,
`requires`, `trace()`, locus-scoped relations with `replaces` obligation keys,
`normal_grad()`, `identify`, `assume_topology`. What remains:

### Manifold boundary conditions for 2D/3D
The `boundary coord = value:` selector and `normal_grad(field)` work for
axis-aligned boundaries in any dimension. Open:

- **Non-axis-aligned boundaries:** Circular domains, irregular coastlines,
  complex 3D surfaces. Need a boundary naming/selection mechanism beyond
  `coord = value`.
- **Additional boundary primitives:** `normal()` (vector itself), `jump()`
  (discontinuity across interface), `mean()` (average across interface)?
- **Periodic boundaries beyond `identify`:** Full design for seam handling in
  2D/3D manifolds. Vector/tensor seam transforms deferred beyond v2.1.
- **Internal interfaces between subdomains/materials.**
- **Tangential/slip conditions for vector fields.**

### Graph neighborhood iteration
Locus-scoped relations use `for c in children` and
`sum(e in incident_edges, ...)`. The settled language only has `for i in 0..N`.
Either geometry needs a dedicated neighborhood-reduction construct, or the
language settles general dynamic iteration first.

Same design question as `for i in fish` when `fish` is `dyn`-sized. Resolving
dynamic iteration for events/collections would likely resolve graph-neighborhood
iteration simultaneously.

### Compiler internals for custom metrics
- **Basis-aware tensor IR:** Custom coordinate-dependent metrics require the
  compiler to derive `g^{-1}`, `det(g)`, Christoffel symbols, and co-normals
  symbolically. Needs a tensor calculus subsystem.
- **Heterogeneous metric units:** Polar's metric `[[1, 0], [0, r^2]]` mixes
  length and angle units across elements. Dimension checker must handle
  per-element unit analysis.
- **Pole / singularity handling:** `locus pole` names the problem. Compiler
  backend must emit L'Hopital limits at poles rather than naive `1/sin(theta)`
  formulas. The `locus` declaration provides the structural information needed.

### Cross-domain coupling at embedding (ambient locus problem)
A 1D root network embedded in 3D soil interacts along its physical extent, not
at an intrinsic locus. This is a kernel coupling problem, not a geometry
problem. The geometry system stays strictly intrinsic. Deferred to kernel
coupling design.

### Plant hydraulics features needing geometry support
- **Embolism-driven edge deactivation** (topology masking — connects to dynamic
  topology / events)
- **Mixed-dimensional coupling** between 1D network and 0D/3D compartments
  (partially addressed by kernel coupling)
- **`rooted_graph` topology class** for cyclic-but-rooted structures
  (anastomosing roots, leaf vein reticulation, fungal mycelium)
- **Edge-level scientific data binding** — per-edge diameter, conductivity,
  vulnerability parameters as model quantities (separate from topology tags)

### Spatial operator catalogue
Full set of spatial operators and their dimensional signatures needed. Currently
settled: `grad`, `diverg`, `laplacian`, `curl`, `normal_grad`. May need more
for specific PDE classes.

---

## Events (Dynamic Topology)

- Can events be generic?
- Can events span multiple container types? (Currently: events live on the
  container that owns the `dyn` collection. Revisit if cross-container events
  prove necessary.)
- How does `for i in fish` iteration work when the collection is `dyn`-sized?
  (Same design question as graph neighborhood iteration above.)
- Within-event conflict tiebreaking — index order is the default, but should
  the user be able to specify a tiebreak function?

---

## Coupling & Kernels

- Is a kernel just a function used inside an `integrate` call, or does it need
  its own declaration?
- Can kernels be learned (neural slots)? Concept says yes, syntax undesigned.
- How does kernel sparsity (characteristic length scale) get communicated to
  the compiler for spatial indexing optimization?
- Is `coupling` a keyword, or just a pattern the compiler detects in
  kernel-weighted integrals?

---

## Conservation

- Scoped conservation? `{ conserved within Pond }` for open systems where
  quantities can leave via declared boundary fluxes.
- How do boundary fluxes interact with conservation? (Birds flying away with
  mass — is that a boundary condition on the container?)
- Does `{ conserved }` work for fields (continuous) as well as scalars?

---

## Type System

- Clarify that "atomic" means leaf of the containment tree (holds a numerical
  value), not "single-field."
- `where` on runtime values (e.g., `where mass > threshold`) is piecewise
  function behavior. How does this interact with differentiability? Does the
  compiler need to know about discontinuities?

---

## Compiler / Spec Maintenance

- **Spec section 6.3 temporal relations** needs updating. Currently shows
  explicit Forward Euler (`x[t+1] = x[t] + dt * rate`). Should use the v2.1
  `d(x) = expr` syntax where the compiler owns the integration scheme.
- Add lib/bin analogy framing to the spec prose (tracked in riley_spec_notes.txt).
- Closure policy semantic interface — what a policy receives (candidates,
  enumeration). Tracked in deferred_review_findings.md.
- `deriv` primitive needs to handle matrix/tensor expressions for non-Euclidean
  spatial operators.
