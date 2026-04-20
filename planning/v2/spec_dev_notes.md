# spec_new.md — Developer Notes

**Purpose.** Decisions that shape `spec_new.md`'s structure or content but
aren't spec content themselves. Consolidation-time choices, scope
clarifications, rationale we want preserved but shouldn't bleed into the
normative spec prose. Entries are dated; most-recent-on-top.

---

## 2026-04-20 — Functions section locked (Option B); Macros dropped from v2.1

### Functions: Option B (inference-only for user fns; contracts for stdlib atoms)

The gap audit flagged "Functions section missing" as top-1 convergence. A
subagent investigation of chunks 04/05/06/07 confirmed that composable
capability contracts are already committed for distributions (chunk 04
§CC4: `Distribution<U>` + supertrait `requires` chain, modeled on
Rust's `Copy`/`Clone`/`Send`/`Sync`), but they have not been extended to
functions. Older function-property machinery used four metadata classes
(`bijective` / `injective_restricted` / `lossy` / `opaque`) and
`#[inverse]` attributes — both incompatible with the annotation purge.

**Decision.** Extend composable contracts uniformly to functions.
Stdlib atoms (`exp`, `log`, `sin`, `sqrt`, …) carry capability contracts
like `Invertible<_>`, `Differentiable`, `Monotone`. User functions carry
**no property-declaration surface** — the compiler derives properties
from body composition plus the declared stdlib-atom facts. No annotation
blocks, no `#[...]` attributes.

### Alternatives considered

- **A.** Extend contracts + allow user declarations (compiler-verified).
  Rejected: adds a surface users can avoid; "structure regularizes"
  pushes toward body-derivation.
- **C.** Pure inference, no stdlib declarations either. Rejected: stdlib
  needs to encode primitive facts somewhere (`log` is the inverse of
  `exp`, `exp` is differentiable with derivative `exp`); composable
  contracts are the natural home.

### Implications

- §6 Functions: declarations + generics + contract satisfaction. No
  user-facing property declarations.
- §7 Contracts: extended to note contracts apply uniformly to types,
  functions, and distribution families.
- §17 merge sources: function-inverse rewrites fire from stdlib-declared
  contract satisfaction, not from user annotations.
- Four-class invertibility metadata retired. Replaced by contracts on
  stdlib atoms.
- `#[inverse]` and related attributes retired (already in stale list;
  now formally replaced).
- Function inversion user recourse: if a user fn needs its inverse
  recognized and the compiler can't derive it, user refactors the fn
  into structurally composable pieces. No escape hatch.

### Macros: dropped from v2.1

v2.1_in_progress.md listed declarative and derive macros as a language
facility. Given the primitives now committed (strong generics,
contracts, refinement types, conservation groups, `impl`/`some`), the
boilerplate use cases macros would have solved have shrunk substantially.

**Decision.** Drop macros from v2.1. Add to deferred manifest (§30 Other
Opens). Revisit post-v2.1 if concrete boilerplate pain emerges.

### Consolidation actions taken

- Inserted §6 Functions in spec_new.md; renumbered §7–§35 accordingly
  (Part I §§7–15, Part II §§16–19, Part III §§20–21, Part IV §§22–25,
  Part V §§26–27, Part VI §§28–30, Part VII §§31–35).
- §7 Contracts one-liner updated to name function+distribution uniformity.
- §17 merge-sources one-liner updated to qualify function inverses as
  stdlib-declared.
- §30 Other Opens lists macros as dropped-from-v2.1.
- Internal ref "Part V §26" (AD ownership) updated to "Part V §27".

---

## 2026-04-20 — CC1 literal-numerics scope: zero literals in `.myco` value position

### The question

During the consolidation audit, reviewers flagged that mocks contain literal
numerics (`R = 8.314`, `1.67`, `0.98`, `0.1579`, etc.) inside relation bodies
and universal declarations. Chunk 04 CC1 locks a literal-numerics policy but
doesn't fully specify whether universals (`universal R: Scalar<J_mol_K> =
8.314`) or stdlib-provided physical constants count as violations.

### The decision

**CC1 is strict. No literal numerics anywhere in `.myco` value position.**
Physical constants do not live in the stdlib. They live on the Python
workflow side and inject at workflow time via the binding verbs.

### Exception positions (the complete list)

Literal numerics are permitted only in:

1. **Unit definitions.** Base unit declarations and derived-unit algebra.
2. **Affine conversion bodies.** `1 hour = 60 minutes` and equivalents.
3. **Shape tuples, indices, arity.** `Tensor<U, (3, 4)>`, `pathway[0]`,
   `N: val = 3` at the generic-parameter definition site. Structural
   positions, not value positions.

Symbolic constants (`π`, `e`) are named identifiers in the numeric stdlib —
they don't require an exception slot because they're not literals.

### What this means in practice

- `universal R: Scalar<J_mol_K>` — declaration only. No value attached.
  The workflow provides the value.
- Empirical fit parameters (Sperry's cavitation `a`/`b`, Potkay's
  calibration coefficients) are typed fields bound via `assume_constant`
  or `learn_constant` at workflow time.
- Physical constants (`R`, `Avogadro`, `Planck`, `c`) are provided by the
  Python workflow, same machinery as empirical parameters. The compiler
  does not distinguish the two.

### Rejected alternative

A fourth CC1 exception — "named stdlib declarations with documented
provenance" — was considered and rejected. Reasons:

- Splits `.myco` files into two trust postures (user files vs stdlib
  files), each with different literal rules. Needless complexity.
- Requires the stdlib to own a physical-constants module and keep it in
  sync with users' needs. Workflow-side injection avoids this entirely.
- "Physical constant" is not a distinction the compiler can make. The
  value `8.314` looks the same as an empirical fit coefficient. Keeping
  them on the same side of the language boundary honors that.

### Implications for consolidation

- Spec_new.md §3 (Values and Literal Policy) states the three exception
  positions cleanly, without mentioning stdlib or universals-with-values.
- Universals declare *types*, not values. Any existing mock universal
  with `= <number>` form must be rewritten as `universal X: Scalar<U>`.
- Mock rewrite pass (post-spec): both Sperry and Potkay need their
  universals stripped of values and the values moved to the Python
  workflow. Potkay also needs the slot / `[t+1]` migration flagged in
  the consolidation audit.
- CC1 wording in spec_new.md: "`.myco` permits literal numerics only in
  unit definitions, affine conversion bodies, and structural positions
  (shape tuples, indices, generic-parameter definitions). All values
  enter from the workflow."

### Open adjacent item

- The compiler diagnostic surface for CC1 violations is not specified.
  Flagged in the merged audit under "Other Opens." Not blocking for
  spec_new.md text; is blocking for implementation.

---


## Lower-priority gap items (not covered by the two-reviewer gap audit)

Three items surfaced in earlier brainstorm that the merged gap audit does not
otherwise cover. Keep flagged; decide when spec work nears them.

- **Testing / property-checking affordances for `.myco` models.** DX-ish.
  Could live in Part VII.
- **Symbolic algebra surface.** Whether users can invoke `simplify` /
  `prove` / inspect equivalences on the e-graph. If no user-visible surface,
  worth stating so explicitly.
- **Runtime introspection beyond `observe`.** Whether executing models can
  be queried mid-run.

---

## 2026-04-20 — Two-reviewer gap audit of spec_new.md (merged synthesis)

Two independent Claude Opus reviewers (X and Y) did a full gap-audit of
`spec_new.md` against `soul.md`, `riley_project_note.md`, `spec.md` (older),
`v2.1_in_progress.md`, `open_questions.md`, and every chunk report. Each
wrote ~80–100 gap rows. Raw findings:

- Reviewer X: `/tmp/gap-review-xCfDczmz/findings/reviewer_x_findings.md`
- Reviewer Y: `/tmp/gap-review-xCfDczmz/findings/reviewer_y_findings.md`

This section is the merged synthesis. Convergence tagging: `[both]` = flagged
by both reviewers independently; `[X]` / `[Y]` = one reviewer only.

### Top-3 convergence (both reviewers' top-3)

1. ~~Functions section missing.~~ **RESOLVED** — see 2026-04-20 Functions
   entry above. §6 Functions added; Option B locked.
2. **`node` instantiation + module semantics underspecified.** §2 covers
   modules/imports (Riley's addition) but `node name: Type` — how users
   actually instantiate entities — has no home. `[both]`
3. **Soul principles not stated up front.** Five soul.md principles +
   ecophysiology/GPU-simulator framing + world-vs-experiment split. Every
   downstream "why" in Parts I–V hangs on these. Belongs as §0 or preface.
   `[both]`

### Must-add sections (real structural holes)

| Add | Where | Why | Source |
|---|---|---|---|
| §0 Principles / Framing | before §1 | soul's 5 principles, target users, long-term goal, world-vs-experiment split `[both]` | `soul.md`; `riley_project_note.md` |
| Type graph placeholder (Part II) | new § in Part II, not just §29 footnote | chunk 07 commitment deserves front-door naming; resolved to add as stubby section. `[both]` | `chunk 07` |
| Matrix / Tensor types promoted | keep in current §3 Types | **DECLINED** — keep matrix/tensor content in §3 Types; don't promote. | `chunk 05` |
| Plan inspection / `mycoc explain` | new § in Part II | "generated code is the product" → users must inspect `[both]` | `v2.1_in_progress.md` |
| SCC decomposition + 4-way classification | new § in Part II | pivot for lowering / training / backend; currently unnamed `[both]` | `v2.1_in_progress.md` |
| Training emission | new § in Part III | warm-start, projection flavors, per-residual loss exposure `[both]` | recent commit "Lock training emission and constraint enforcement strategy" |

### Must-add subsections (load-bearing concepts hidden in one-liners)

**§3 Types** — 2026-04-20: subsections 3.1-3.8 stubbed in spec_new.md
(universal, refinement types, newtype/composite, node instantiation,
impl/some, variance, conservation groups, scalar reconciliation).
Named-type equality/comparison deferred (not yet placed).
- Named-type equality / comparison rules (DEFERRED — decide §3 vs §7 later) `[Y]`

**§5 Units**
- `convert` four variants (bidi `<->` vs one-way `->`, bare vs parameterized) `[X, Y]`
- Round-trip verification via bounded counterexample search (O2.1) `[X, Y]`
- `value_in` operator `[X]`

**§6 Contracts**
- Parameterized contracts `[X, Y]`
- Contract composition alias `contract C := A + B` (provisional) `[X, Y]`
- Multi-contract coherence / ordering / diamond-inheritance rules `[Y]`
- Supertrait semantics subsection `[Y]`
- Data contracts output-only details `[Y]`
- **Capability contracts** (AffineSelfClosed, SumSelfClosed, ProductSelfClosed, ScaleSelfClosed, SmoothTransformable, ReparameterizedSampleable for distributions; Invertible<_>, Differentiable, Monotone for fns) — the mechanism driving Tier A closed-form routing *and* function-inverse rewrites. Uniform composable-contract machinery. `[X, Y]`

**§7 Relations and Equality**
- `property` declarations (compiler-verified facts) `[X, Y]`
- `constraint` declarations (inequality / logical obligations) `[X, Y]`
- `let` bindings inside relation bodies `[X, Y]`
- `if` / `else` vs `where` in relation bodies (two distinct constructs) `[X, Y]`
- `for` loops in relation bodies (compile-time unfolding, distinct from runtime iteration) `[X]`
- Inline relation / inline-constraint sugar `[X, Y]`
- Three-way overdetermination classification (redundant / provably inconsistent / conditionally inconsistent) `[X]`
- Y6 C(N,M) closure-policy enumeration `[X]`
- User-defined Y5 closure policies + closure-policy interface (recent lock) `[X, Y]`
- Smoothing as a model claim (runtime `where` semantics; stdlib `smooth_*` helpers) `[X, Y]`
- Generated-defaults surface with obligation keys (`replaces balance(axial_flux)`) `[X]`

**§8 State and Time**
- `dt` provision from workflow (distinct verb-like mechanism) `[Y]`
- Per-path uniqueness after generic cartesian-product expansion `[X]`

**§9 Dynamic Topology and Events**
- Firing-order policy at workflow composition `[X, Y]`
- Generic event cartesian-product expansion (subsection, not just one-liner) `[X, Y]`
- Cross-container events: container scoping rule + nearest-common-ancestor `[X, Y]`
- Within-event tiebreaking `[Y]`
- `replaces` obligation retraction (monotonicity tension) cross-link `[Y]`

**§10 Geometry and Locus**
- Spatial operators: `trace`, `normal_grad`, `∇`, `∇·`, `∇²` `[X, Y]`
- Boundary conditions (Dirichlet / Neumann / Robin; `requires` blocks) `[X, Y]`
- Stdlib geometries table (Line1D, Rectangle2D, Ball3D, rooted_tree, metric_graph, BranchingManifold, …) `[X, Y]`
- Discretization config + Python-side override (geometry → mesh pipeline) `[X, Y]`
- Compiler's chosen discretization default (vs workflow-override) `[X]`
- Horse/fly composition pattern subsection (worked out) `[X]`
- Edge-interior vs locus-scoped field distinction `[X, Y]`
- Default junction conditions (balance-only, no continuity) `[X]`
- Embedding fields as regular fields (not special construct) `[X]`
- Geometry coefficients via `requires` (no `hint`) `[X]`
- `boundary` / `chart` / `metric` / `requires` vocabulary `[Y]`

**§11 Collections and Iteration**
- Aggregation primitives by name: `sum`, `product`, `any`, `all`, `count`, `argmin`, `argmax` `[X, Y]`
- Tagged-handle IR-level sum type for heterogeneous argmax `[X, Y]`
- Empty-collection behavior defaults `[X]`
- Bind-time vs event-time dynamism distinction `[X]`
- Per-type pool desugaring for `impl Contract` collections `[X]`
- Index-style vs iterator-style vs graph-neighborhood iteration `[Y]`
- Filtering with `where x is T` (narrowing) — dedicated subsection `[Y]`

**§12 Probabilistic Programming**
- Aleatoric / epistemic split as its own subsection `[X, Y]`
- Tier A / B / C concrete dispatch procedure `[X, Y]`
- Automatic marginalization `[X]`
- Itô vs Stratonovich generics on `~` `[X, Y]`
- Independence via structural identity (no naked correlation) — dedicated `[X, Y]`
- Cholesky reparameterization (Z10) — dedicated `[X, Y]`
- `.at()` field sampling syntax `[X]`
- Observation injection mechanism + back-propagation of likelihood `[Y]`
- Observed-samples-as-envelope-facts (not new merges) `[Y]`
- Tier 2 PPL items (recent lock) `[Y]`

**§13 Compiler Intrinsics**
- `condition_of` Level I/II/III semantics + mode-tagged return + algorithmic-vs-problem duality `[X, Y]`
- `loss_of` multi-dimensional named-field return `[X]`
- `integrate` intrinsic semantic commitments (domain, units, e-graph interaction) `[X]`

**§14 Approximate Blocks**
- Concrete block syntax: `under:` / `tolerance_class:` / `error_bound:` / `body:` / `where:` `[Y]`
- Auto-derived lossiness (four layers) `[X, Y]`
- Three-tier lossiness cut (lossless / lossy-model / lossy-tolerance) `[X]`

**§15 E-Graph**
- Three-layer scoping split (equational / envelope / adjacent keyed) as principle — restate in §0 too `[X]`
- Monotonicity invariant (no retraction, append-only) named explicitly `[X, Y]`
- Envelope ownership (who writes, reads, invalidates) `[Y]`
- Envelope flavors (entry-wise / operator-norm / spectral / structural) `[Y]`

**§16 Equality-Introducing Machinery**
- 8 merge sources as a named subsection heading with its own prose (not just a one-liner list) `[X]`
- `identify` vs relation `=` distinction (user surface + where/how written) `[X, Y]`
- Function inverses as e-graph equality source (via stdlib capability contracts; no user annotation path) `[X]`
- Unified rewrite-predicate language (subsection) `[X]`
- Rewrite-rule groups A–Y enumeration (or appendix) `[X, Y]`
- Baseline rewrite partition buckets `[X]`

**§17 Residual Graph**
- Extraction policy / cost model (multi-dimensional, not scalar) `[X, Y]`
- Residual ↔ e-graph projection mechanics (Tier 0 Q3 open) `[Y]`
- 4-way / 3-way residual classification `[Y]`
- Saturation termination / rewrite scheduling `[Y]`

**§18 Lowering**
- Static vs dynamic module classification `[X]`
- Four-way component classification after SCC (static / dynamic / stochastic / training) `[X]`
- `y[t]` / `y[t+1]` as ground terms — dedicated subsection `[Y]`
- N-max / alive-mask mechanics (how N-max is chosen; overflow handling) `[Y]`

**§19 Boundary**
- Runtime `where` semantics live at workflow composition `[X]`
- Multi-binding compilation (same `.myco`, N workflows, shared callable weights) `[X]`
- Data contracts for cross-study callable reuse `[X]`
- Capability errors at workflow composition vs compile errors `[X, Y]`

**§20 Eight Workflow Verbs**
- `bind_controller` contract I/O spec (data contract) `[X, Y]`
- `bind_controller` gradient-flow semantics + "pure workflow concept" reframe `[Y]`
- `bind_topology` ↔ §10 geometry relationship `[X]`
- Future verbs: `bind_known_constants`, `bind_parameters`, `assume_prior` `[X]`
- FiLM-style taxonomic embeddings (controller interface pattern) `[X]`
- Run-config / workflow configuration surface (`run.config.backend.fallback`, seed, etc.) `[Y]`

**§21 Numeric Types**
- Full numeric hierarchy: Bool / Integer / Rational / Float / BigFloat / Dual / Complex `[X]`
- Dual numbers (forward-mode AD) `[X]`
- Complex numbers deferral policy `[X]`
- Default-compatibility constraints on `T` in `Scalar<U, T = Float64>` — subsection, not one-liner `[X, Y]`
- `Rational` termination caveat — subsection `[Y]`

**§22 Distribution Families**
- Tier 1 list as a table (19 univariate continuous, 5 discrete, 3 MVN gated on B5) `[X]`
- `Truncated<D>` and `Mixture<D₁..D_N | weights>` meta-families `[X, Y]`
- Conjugate-posterior rewrite catalog (Beta-Binomial, Normal-Normal, Gamma-Poisson, …) `[X, Y]`
- Capability / support / moments / log_pdf / reparam table `[Y]`
- Tier ordering (Tier 1 / Tier 2 / Tier B / etc.) `[X]`

**§23 Kernels**
- Kernels-as-functions-with-properties (PositiveDefinite, Stationary, Isotropic) `[X]`
- Ambient-locus-via-composition `[X]`
- Kernel sparsity / integration deferrals `[X]`

**§24 Units Library**
- Ecophysiology stdlib extensions (respiration, photosynthesis, conductance) `[X]`

**Part IV (new)**
- Matrix / tensor stdlib primitives: cholesky, lu, qr, svd, eigen, solves, norms, det, trace, inv, transpose, `@` matmul, constructors `[Y]`
- Structural subtype lattice for matrices (Symmetric / PosDef / Triangular / Orthogonal / Diagonal / Sparse; meet / composition like Symmetric ∧ PosDef) `[X, Y]`
- Shape refinements as type-level predicates (distinct from structural subtypes) `[Y]`
- Heterogeneous-unit `LinearMap<From, To>` (chunk 05 open Q1) `[X, Y]`
- Sparse-pattern-as-type vs sparse-pattern-as-envelope-fact (chunk 05 Q7) `[X]`
- Matrix literal syntax (open, chunk 05 Q8) `[X]`
- Collections and tensors are orthogonal primitives (positive statement) `[X]`
- Ecosystem adjacency via collections + node state, not dynamic matrix shapes `[X]`
- Explicit in-scope vs out-of-scope for `convert` on tensors (reshape / sparse↔dense in; precision / storage-order out) `[X]`

**§25 Backend Trait Surface**
- Capability advertising + three fallback modes (error / host / emulate) — dedicated `[Y]`
- PPL handoff protocol detail (absorbs old B3) `[Y]`
- Opaque-callable runtime `[X]`
- Backend versioning `[Y]`
- Stochastic e-class serialization to backend primitives `[Y]`
- JAX-as-primary framing retired; burn-style trait abstraction (positive note) `[X]`

**§26 Open Backend Items**
- Mixed-backend policy statement (leans single-backend-per-run) `[Y]`
- First concrete backend to implement against (burn / NumPy / JAX) `[X]`
- Performance targets (~3,000–3,400 tree-years/sec on H100) `[X]`

### Cross-cutting concepts (name once in §0 or principles block, reference many)

- **Conservation laws** as first-class compiler property — threads types, relations, events, residuals `[both]`
- **Referential truth** / event-monotonicity / no retraction `[both]`
- **Downward-only cross-scale visibility** (parents see children; children don't see up; composition over inheritance) `[both]`
- **Traceability / provenance** of merges, rewrites, and injected values `[both]`
- **Error-reporting philosophy** / diagnostics surface (compiler vs workflow vs runtime tiers) `[both]`
- **Capability errors at workflow composition time** (distinct from `mycoc` compile errors) `[X, Y]`
- **Three-layer scoping split** stated up-front (so Parts I/III can reference) `[X]`
- **Determinism / reproducibility** guarantees across backend versions `[Y]`
- **World-vs-experiment split** (aleatoric vs epistemic) as design axis `[Y]`
- **Conversion-graph cost model** cross-references (chunk 05 Q7 + chunk 07 Q6) `[X, Y]`
- **Literal-constants diagnostic surface** (CC1 enforcement messages) `[both]`
- **GPU-incompatibility of BigFloat / Rational** (forward-reference from §21 to §26) `[X, Y]`
- **Monotonicity tension for `replaces` retraction** cross-references (§15 / §16 / §29) `[X, Y]`
- **Type graph ↔ expression e-graph** bridge cross-reference (chunk 07) `[both]`

### Structural / durable-principle statements

- `.myco` declares world-claims; world-claims are monotonic `[X]`
- Compiler does not auto-emit projections or solver selection (projection-free compiler) `[X]`
- Generated code is the product `[X]`
- The language surface has no compiler annotations ("no `#[...]` on `.myco`") `[X]`
- `impl` for static monomorphization vs `some` for runtime sizing — positive split `[X]`
- Soul principle 2: workflow is separate from the model `[X]`
- Soul principle 3: the compiler does work `[X]`
- Soul principle 4: structure regularizes `[X]`
- Hierarchical SCC decomposition as the compiler's core structural move `[X]`
- Read-order / audience / level-of-detail convention stated in preamble `[Y]`
- Versioning / stability policy for `.myco` (breaking-change discipline) `[Y]`
- Long-term goals (GPU ecosystem simulator) explicitly flagged as non-goals for v2.1 `[Y]`

### Appendices (candidates)

- Reserved keyword / full syntactic-surface list `[Y]`
- Grammar / EBNF summary `[Y]`
- Deferred-to-v2.2 manifest (MVN multi-dim, HMM sequential, VI backends, dynamic matrix shapes, cross-backend callable interop, etc.) `[X, Y]`
- Rewrite catalog A–Y enumeration `[X, Y]`
- Chunk 04 audit items O4.1–O4.8 carryover `[X]`
- Mock rewrite obligations (Sperry / Potkay pre-release pass) `[X]`

### Housekeeping for §27–§29

- §27 should note **B3 was absorbed into B6** (one-liner; reviewers noticed the missing B3) `[X, Y]`
- §29 should carry forward `open_questions.md` Deferred block explicitly (not just imply) `[X, Y]`
- §29 should reference chunk 04 O4.1–O4.8 audit items `[X]`

### Verdict

spec_new.md is materially incomplete without the top-3 convergence + matrix
promotion + type-graph placeholder + plan-inspection/SCC/training-emission.
The subsection list is migration-hygiene: if sections are filled from
one-liners without the bullet list, load-bearing concepts will silently die
during consolidation.

### Open adjacent

- None of the above are design decisions — they are layout claims. Each
  bullet that lands in `spec_new.md` will require a small decision on exact
  section/subsection placement; most are pre-slotted in the "where" column.