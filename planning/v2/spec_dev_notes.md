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
- 2026-04-21 update: §3 gained §3.9 Matrix Structural Subtype Lattice
  (type content). Part IV gained §30 Matrix and Tensor Primitives (STUB)
  (stdlib functions only). Part V §30 Backend → §31, §31 Open Backend
  → §32. Part VI §32 Design Blockers → §33, §33 Chunk-Slotted → §34,
  §34 Other Opens → §35. Part VII §35–§39 → §36–§40. All cross-refs
  updated.

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

2026-04-21: §0 Principles, §18 Type Graph (stub), §20 SCC Decomposition,
§21 Lowering (renumbered), §22 Plan Inspection (reframed as optional
debug affordance, not required workflow), §25 Training Emission all
stubbed in spec_new.md. Full Part II-VII renumbered.

| Add | Where | Why | Source |
|---|---|---|---|
| Matrix / Tensor types promoted | keep in current §3 Types | **DECLINED** — keep matrix/tensor content in §3 Types; don't promote. | `chunk 05` |

### Must-add subsections (load-bearing concepts hidden in one-liners)

**§3 Types** — 2026-04-20: subsections 3.1-3.8 stubbed in spec_new.md
(universal, refinement types, newtype/composite, node instantiation,
impl/some, variance, conservation groups, scalar reconciliation).
Named-type equality/comparison deferred (not yet placed).
- Named-type equality / comparison rules (DEFERRED — decide §3 vs §7 later) `[Y]`

**§5 Units** — 2026-04-21: 5.1 convert four variants, 5.2 round-trip
verification (O2.1), 5.3 `value_in` operator all stubbed.

**§7 Contracts** — 2026-04-21: 7.1 Parameterized, 7.2 Capability, 7.3
Supertraits, 7.4 Multi-contract coherence stubbed. Composition alias
+ data contracts retired to anti_spec.md (investigation 2026-04-21
found data contracts redundant with plain contracts + workflow-layer
visibility).

**§8 Relations and Equality** — 2026-04-21 (batch 1): 8.1-8.5
stubbed (constraint, let, if/else vs where, for, inline sugar).
`property` declarations retired. 2026-04-21 (batch 2): 8.6 system-
level overdetermination classification, 8.7 closure policies Y1-Y6
(includes Y6 C(N,M)), 8.8 Y5 user-defined, 8.9 smoothing as model
claim, 8.10 generated-defaults with obligation keys.

**§9 State and Time** — 2026-04-21: 9.1 `dt` provision (uses
existing binding verbs, not a new mechanism), 9.2 per-path
uniqueness after generic expansion.

**§10 Dynamic Topology and Events** — 2026-04-21: 10.1 firing-order
policy, 10.2 generic event expansion, 10.3 cross-container events
(NCA rule), 10.4 within-event tiebreaking (three-case exhaustive
under referential truth), 10.5 `replaces` monotonicity (default-
suppression not retraction; arbitrary prior-claim retraction stays
open in §34).

**§11 Geometry and Locus** — 2026-04-21: 11.1 spatial operators,
11.2 boundary conditions (Dirichlet / Neumann / Robin via
`requires` blocks; silence is not a default Neumann zero), 11.3
stdlib geometries table, 11.4 horse/fly composition, 11.5
discretization configuration at workflow composition, 11.6
compiler discretization defaults (conservative per-geometry,
smoke-test affordance), 11.7 edge-interior vs locus-scoped
fields, 11.8 default junction conditions (balance-only; continuity
opt-in), 11.9 embedding fields as regular fields (no special
keyword), 11.10 geometry coefficients via `requires` (not `hint`),
11.11 standard locus vocabulary (`boundary`, `chart`, `metric`,
`requires`).

**§12 Collections and Iteration** — 2026-04-21: 12.1 aggregation
primitives (`sum`, `product`, `any`, `all`, `count`, `argmin`,
`argmax`; softmax / weighted-sum deferred to §34 pending
collection-aggregation syntax), 12.2 tagged handles for
heterogeneous `argmax` (pool_identity + intra-pool index), 12.3
empty-collection defaults (identity elements for
`sum`/`product`/`any`/`all`/`count`; `argmax`/`argmin` empty is
a compile error), 12.4 bind-time vs event-time dynamism (N-max
machinery applies only to event-time), 12.5 per-type pool
desugaring for `impl Contract`, 12.6 iteration styles (index /
iterator / graph-neighborhood; neighborhood pending §11
geometry-side surface), 12.7 filtering with `where x is T`
(structural, not runtime predicate).

**§13 Probabilistic Programming** — 2026-04-21: 13.1 aleatoric
vs epistemic (same `~`, distinguished by structural position),
13.2 Tier A/B/C dispatch (closed-form via capability contracts /
approximate via `approximate` blocks / opaque PPL handoff), 13.3
automatic marginalization (closed-form + no downstream reference),
13.4 Itô vs Stratonovich as generic on SDE family, 13.5
independence via structural e-class identity (no naked
correlation surface), 13.6 Cholesky reparameterization for MVN
and affine-in-noise joints, 13.7 `.at()` named-field sample
access, 13.8 observation injection as envelope fact + likelihood
back-propagation through model graph, 13.9 observed samples are
envelope facts, not new merge sources (preserves §17 eight-source
enumeration), 13.10 Tier 2 lock (coupling / joint declaration
deferred to chunk 08; higher-order distributions route via §28).

**§14 Compiler Intrinsics** — 2026-04-21: 14.1 `condition_of`
Levels I/II/III with mode-tagged return and problem-vs-algorithm
duality, 14.2 `loss_of` named-field return (no auto-sum; scalar
aggregation is workflow composition), 14.3 `integrate` domain /
unit algebra / integration-by-parts as e-graph rewrite.

**§15 Approximate Blocks** — 2026-04-21: 15.1 block syntax
(`under` / `tolerance_class` / `error_bound` / `body` / `where`),
15.2 four-source lossiness derivation (stdlib atoms /
approximation blocks / numeric types / backend emulation),
15.3 three-tier cut (lossless / lossy-model / lossy-tolerance).

**§16 E-Graph** — 2026-04-21: 16.1 three-layer scoping split
named as structural principle (cross-reference §0), 16.2
monotonicity invariant (append-only; `replaces` suppresses
generation, does not retract; dead entities continue to exist
equationally), 16.3 envelope ownership (stdlib + compiler
rewrites + `observe` write; dispatch/extraction/diagnostics
read; no invalidators), 16.4 envelope flavors (entry-wise /
operator-norm / spectral / structural with per-flavor
composition rules).

**§17 Equality-Introducing Machinery** — 2026-04-21: 17.1 eight
merge sources with prose (resolved terminology collision —
"observation injection" renamed to "workflow constant injection"
to disambiguate from the probabilistic `observe` verb of §13.8;
haiku investigation confirmed two distinct mechanisms, layer-1 vs
layer-2), 17.2 `identify` vs relation `=` (identity vs equation),
17.3 function inverses via stdlib capability contracts (no user
annotation path; composition is the escape hatch), 17.4 unified
rewrite-predicate language (refinements + contracts + structural
shape + unit algebra, all compile-time), 17.5 rewrite-rule groups
A-Y (stub; full catalog in appendix, tracked under §33), 17.6
baseline partition (default-on lossless set fires always;
default-off set fires only under `approximate` blocks). §13.9
cross-updated to note terminology distinction.

**§19 Residual Graph** — 2026-04-21: 19.1 multi-dimensional cost
vector extraction (precision / latency / memory / approximation
class; Pareto front default, workflow picks point), 19.2
projection mechanics (root set, sharing policy, envelope carriage;
heuristic specifics remain Tier 0 Phase 2 Q3 in §34), 19.3
residual classification as orthogonal pair (§20 four-way SCC tag
× §8.6 three-way overdetermination tag), 19.4 saturation
termination + scheduling (default-on fires to fixed point; default-
off bounded by error budget; absolute rewrite-count cap as warning
not error; non-confluent rewrite sets compile-error at block
elaboration).

**§21 Lowering** (was §18) — 2026-04-21: 21.1 static vs dynamic
module classification (static skips runtime loop entirely),
21.2 four-way SCC lowering targets (static / dynamic /
stochastic / training, with class-dominance promotion rule),
21.3 `y[t]` / `y[t+1]` as distinct e-graph ground terms (no
template; monotonicity applies per-tick; closure policies are
per-tick-independent), 21.4 N-max slots and alive masks
(capacity at declaration, workflow override to a compile-
enforced ceiling; bitmap-based GPU-friendly dead-slot skipping;
retirement flips the bit, does not delete e-classes —
referential truth).

**§23 Boundary** (was §19) — 2026-04-21: 23.1 runtime `where`
at workflow composition (three layers of `where`: §8.3 compile,
§12.7 collection, §23.1 composition), 23.2 multi-binding
compilation (one plan, many workflows, callable-weight reuse
across runs), 23.3 cross-study callable reuse via plain
contracts (no data-contract kind needed; shared artifact is
trained weights + plain contract), 23.4 two error tiers
(`mycoc` compile vs workflow composition; runtime errors are
third tier outside this spec).

**§24 Eight Workflow Verbs** (was §20) — 2026-04-21: 24.1
`bind_controller` I/O specification (plain contracts;
controllers are workflow-only; retires `slot` machinery), 24.2
`bind_controller` gradient-flow semantics (parameter registration
at composition; backward pass through backend AD; opaque-fn
fallback for non-differentiable controllers; cross-run weight
persistence), 24.3 `bind_topology` ↔ §11 geometry (concrete
mesh, boundary identification, material coefficients, event-
time capacity override), 24.4 future verbs beyond the eight
(`bind_known_constants`, `bind_parameters`, `assume_prior`
tracked post-v2.1 as positive-statement of v2.1 scope), 24.5
run-config / workflow configuration surface (seed, backend,
verbosity, profile; referenced from verbs as strings, not baked
into plan). FiLM-style taxonomic embeddings demoted from
standalone subsection to §34 Other Opens as a general
controller-interface question (avoid polluting language with
Riley-specific project patterns while still supporting the
workflows that would use FiLM-style embeddings).

**§26 Numeric Types** (was §21) — 2026-04-21: 26.1 numeric
representation hierarchy table (`Bool` / `Integer` / `Rational`
/ `Float32` / `Float64` / `BigFloat` / `Complex`; `Dual` retired
to anti_spec.md — backend owns AD; user-facing `Dual` redundant
and risks conflict with backend AD), 26.2 default-compatibility
constraints on T (ring closure, total ordering for non-complex,
zero/one identity, backend representability; mixed-T arithmetic
is a compile error requiring explicit `convert T1 -> T2`), 26.3
Rational termination caveat (unbounded-loop warning for
Rational-typed temporal state; GPU-incompatibility collective
entry in §34 with BigFloat and arbitrary-precision Integer).
Complex number representation is in v2.1 scope but design
is open (§34); Riley-confirmed 2026-04-21.

**§27 Distribution Families** (was §22) — 2026-04-21 complete:
27.1 Tier 1 families table (19 univariate continuous, 5
discrete, 3 MVN gated on B5; capability column shorthand for
the core composable contracts). 27.2 Meta-families `Truncated<D>`
and `Mixture<D₁..D_N | weights>` with contract-passthrough rules.
27.3 Conjugate-posterior rewrite catalog (six canonical pairs:
Beta-Binomial, Normal-Normal known σ², Gamma-Poisson, Dirichlet-
Multinomial, NormalInverseGamma-Normal unknown both, Wishart-MVN
gated on B5). 27.4 Extended capability table (support / log_pdf /
moments / reparam / sampling / entropy / kl_div columns as shorthand
key). 27.5 Tier ordering — Tier 1 ships in v2.1; Tier 2 (genuinely
joint / coupling / B2 / B4 / higher-order routing through kernels)
reframed as **open design question in v2.x scope**, not "deferred
to a future version" — haiku investigation 2026-04-21 confirmed
the multivariate factorizable subset (MVN via Cholesky, Dirichlet,
Multinomial) already ships in Tier 1 multivariate; chunk 08 is the
design venue for the genuinely joint subset. Tier 3 (non-parametric
/ process-valued: GP, DP, CRP, Pitman-Yor, IBP, BP) also reframed
as open in v2.x — no formal tier boundary drawn yet; GPs route
through §28 kernels, other non-parametric families open. Both
tracked in §35 Other Opens (was §34).

**§28 Kernels** (was §23) — 2026-04-21 complete:
28.1 kernels-as-functions-with-capability-contracts (PositiveDefinite,
Stationary, Isotropic with closure rules for sum / product / scaling /
radial-wrap composition; stdlib covers Matérn ν∈{1/2, 3/2, 5/2, ∞},
squared-exponential / RBF, rational-quadratic, Wendland compact-support).
28.2 ambient-locus via composition (`Point<L>` arguments, locus fixed
at call site, not per-kernel declaration; product loci handled by
PositiveDefinite closure rule on product loci). 28.3 sparsity /
integration deferred (sparse kernel Gram representation routes through
chunk 05 B5 matrix-layer; kernel integration operators / GP posterior
machinery wait for chunk 03 e-graph substrate lock). Until chunk 03
lands, GPs route through opaque PPL handoff (Tier C).

**§29 Units Library** (was §24) — 2026-04-21:
Committed scope is SI base, SI-derived, derived-unit algebra, and
affine-conversion machinery. Domain-specific libraries (ecophysiology,
chemistry, astronomy, finance, etc.) are explicitly out of scope for
Myco core — they ship as distributable packages consuming core units.
Ecophysiology extensions (water potential, gas-exchange rates, PPFD /
radiation, LAI / canopy, soil water) accordingly moved out of the
spec and noted on riley_project_note.md as spore-library content.

**Part IV Matrix/Tensor** (unchanged; lives in §3 Types per chunk 05 decline) — 2026-04-21 partial:
- Matrix / tensor stdlib primitives: **§30 Matrix and Tensor
  Primitives (STUB)** — cholesky, lu, qr, svd, eigen, solve, inverse,
  det (note: the canonical stdlib functions only; no type content
  per the chunk 05 decline). Primitives are opaque at the e-graph
  layer, wrap backend linear-algebra kernels via Part V trait.
  Dispatch on §3.9 structural subtype lattice. `[Y]` partial —
  committed primitive names + contracts; full function-signature
  formalization deferred to chunk 05 for heterogeneous units.
- Structural subtype lattice for matrices: **§3.9 Matrix Structural
  Subtype Lattice** — 8 structural types (Symmetric / PositiveDefinite /
  PositiveSemiDefinite / Upper- or LowerTriangular / Diagonal /
  Orthogonal / Sparse / Banded) with meet composition, dispatch
  rule, and explicit chunk-05-deferred items (heterogeneous-unit
  entries, shape refinements, envelope flavors, sparse representation).
- Shape refinements as type-level predicates (distinct from structural subtypes) `[Y]`
- Heterogeneous-unit `LinearMap<From, To>` (chunk 05 open Q1) `[X, Y]`
- Sparse-pattern-as-type vs sparse-pattern-as-envelope-fact (chunk 05 Q7) `[X]`
- Matrix literal syntax (open, chunk 05 Q8) `[X]`
- Collections and tensors are orthogonal primitives (positive statement) — 2026-04-21 written up as §3.8 extension (orthogonality plus explicit note that collection-of-scalars is not auto-vector and tensor-axis is not auto-collection).
- ~~Ecosystem adjacency via collections + node state, not dynamic matrix shapes~~ — dropped; project-specific framing, not a Myco language concern (see feedback memory on project-vs-language separation).
- Explicit in-scope vs out-of-scope for `convert` on tensors (reshape / sparse↔dense in; precision / storage-order out) — 2026-04-21 written up as §3.8 extension. In-scope: reshape (element-count-preserving), sparse↔dense representation, structural-subtype widening. Out-of-scope: numeric precision (backend §31), storage-order / layout (backend), device residency (backend).

**§31 Backend Trait Surface** — 2026-04-21 complete:
31.1 capability advertising + 3 fallback modes (error / host / emulate)
with fallback scoped per-run via `run.config.backend`; emulate mode's
substitutions enter the approximation-error layer (§16). 31.2 PPL
handoff protocol — Tier C stochastic SCCs ship as opaque log-density
problems, samples come back without envelope facts about parametric
form. 31.3 opaque-callable runtime for `bind_controller` (§24.1)
handling Python interop, gradient threading for training emission,
device-residency management. 31.4 backend versioning — trait surface
Myco-versioned, backend implementations backend-versioned; plan cache
keys on `(plan, trait_version, backend_identity)`. 31.5 stochastic
e-class serialization — wire format for Tier C handoff (e-class identity,
parametric form from envelope, layer-1 term, capability requirements,
observation constraints). 31.6 no primary-backend commitment — trait-
symmetric; burn / JAX / PyTorch / CPU reference all first-class.

**§32 Open Backend Items** — 2026-04-21 complete for in-scope items:
32.1 mixed-backend policy — leans single-backend-per-run; escape hatch
is to run specialized SCC in isolation and pass outputs via workflow
glue rather than implement cross-backend marshalling. 32.2 first
concrete backend (burn / NumPy / JAX) — open, but does not affect
trait-surface design. (Performance / throughput considerations are
out of scope for the spec; see feedback memory.)

### Cross-cutting concepts (name once in §0 or principles block, reference many)

2026-04-21 batch 140-149 written up as **§0.1 Foundational Concepts**,
a new cross-cutting block between §0 and §1 Glossary. Each item has
one named paragraph and forward cross-references to the detailed
sections.

- **Conservation laws** — §0.1 paragraph (cross-refs §3.7, §8, §10, §18; no-suppression rule with constraint-declaration escape hatch via §8.1).
- **Referential truth** — §0.1 paragraph (cross-refs §15, §10.5, §16).
- **Downward-only cross-scale visibility** — §0.1 paragraph (cross-refs §3.3; inheritance explicitly not in the language).
- **Traceability / provenance** — §0.1 paragraph (cross-refs §22 mycoc explain, §17 merge tags, §13.9 observation tags; provenance durable across plan serialization).
- **Error-reporting philosophy** — §0.1 paragraph (three-tier split: mycoc / workflow-composition / runtime; tier named in error heading).
- **Capability errors at workflow composition time** — §0.1 paragraph (cross-refs §31.1 capabilities, §19.4 workflow-composition errors; includes shape and unit mismatches as same tier).
- **Three-layer scoping split** — §0.1 paragraph (layer 1 equational core, layer 2 envelope metadata, layer 3 adjacent keyed state; cross-refs Part II §15, §17, §16).
- **Determinism / reproducibility** — §0.1 paragraph ((plan, workflow, seed) triple reproducible within same backend version; bitwise across-version is optional advertised capability).
- **World-vs-experiment split** — §0.1 paragraph (aleatoric in .myco, epistemic in workflow; cross-refs §13 distributional, §24 verbs).
- **Conversion-graph cost model** — §0.1 paragraph (open; unit conversions + tensor reshapes + sparse/dense + subtype widening; tracked in §35, scoped chunk 05 Q7 / chunk 07 Q6).
- **Literal-constants diagnostic surface** (CC1 enforcement messages) — 2026-04-21 written up as §4.1 CC1 Diagnostic Surface.
- **GPU-incompatibility of BigFloat / Rational** (forward-reference from §21 to §26) — 2026-04-21 cross-refs added to §35 Other Opens (§26.1, §26.3, §31.1).
- **Monotonicity tension for `replaces` retraction** cross-references (§15 / §16 / §29) — 2026-04-21 cross-refs added to §35 Other Opens (§8.10, §10.5, §15, §16).
- **Type graph ↔ expression e-graph** bridge cross-reference (chunk 07) — 2026-04-21 confirmed already covered by §18 STUB + §34 chunk 07 entry.

### Structural / durable-principle statements

- `.myco` declares world-claims; world-claims are monotonic — 2026-04-21 already covered by §0 principles 1 and 5 (world-vs-experiment split + referential truth).
- Compiler does not auto-emit projections or solver selection (projection-free compiler) — 2026-04-21 added to §0.1 as named foundational concept; §23 cross-reference cleaned up.
- Generated code is the product — 2026-04-21 added to §0.1 as named foundational concept (plan = unit of execution, source = unit of reproduction).
- The language surface has no compiler annotations ("no `#[...]` on `.myco`") — covered in anti_spec.md (retirement); per feedback rule, negations stay in anti_spec.md, not spec.md.
- `impl` for static monomorphization vs `some` for runtime sizing — positive split — 2026-04-21 expanded in §3.5 with orthogonal-axes framing and `Collection<some (impl Plant)>` composition rule.
- Soul principle 2: workflow is separate from the model — 2026-04-21 already covered by §0 principles 1 and 2 (world-vs-experiment split + clean boundary).
- Soul principle 3: the compiler does work — 2026-04-21 already covered by §0 principle 3.
- Soul principle 4: structure regularizes — 2026-04-21 already covered by §0 principle 4.
- Hierarchical SCC decomposition as the compiler's core structural move — 2026-04-21 added to Part II preamble (top-level partition, tier-nested further decomposition, per-level solver dispatch).
- Read-order / audience / level-of-detail convention stated in preamble — 2026-04-21 **cut.** Out of scope for spec pre-build; revisit once `.myco` exists and there's real audience feedback.
- Versioning / stability policy for `.myco` (breaking-change discipline) — 2026-04-21 **cut.** Out of scope pre-build; policy is a post-implementation decision, not a design-time concern.
- Long-term goals (GPU ecosystem simulator) explicitly flagged as non-goals for v2.1 — 2026-04-21 **cut.** Already covered by §0 Scope; no additional non-goals list needed.

### Appendices (candidates)

- Reserved keyword / full syntactic-surface list — 2026-04-21 written up as **Appendix A** (declaration keywords, type-formers, body-forms, stochastic operator, reserved-but-unassigned, structural punctuation, stdlib-reserved identifiers).
- Grammar / EBNF summary — 2026-04-21 stub added as **Appendix B**; left open for post-lock pass.
- Deferred-to-v2.2 manifest (MVN multi-dim, HMM sequential, VI backends, dynamic matrix shapes, cross-backend callable interop, etc.) — 2026-04-21 **not written as spec appendix.** Per "don't pre-defer" rule, items moved into `open_questions.md` for later sorting: MVN / HMM / VI backends already in existing Deferred block, dynamic matrix shapes added to Tier 2 Type System, cross-backend callable interop added to Tier 2 Compiler Internals.
- Rewrite catalog A–Y enumeration — 2026-04-21 written up as **Appendix C** (25 groups A–Y, ~58 rules, faithfulness × orientation summary table, CC1-5 cross-cut absorption note, merge-source correspondence with §17). Unified spec per Riley request.
- Chunk 04 audit items O4.1–O4.8 carryover — 2026-04-21 open items (O4.1, O4.3, O4.6, O4.7, O4.8) given explicit §35 entries with Appendix C rewrite-group cross-refs; closed items (O4.2, O4.4, O4.5) absorbed into normative text.
- Mock rewrite obligations (Sperry / Potkay pre-release pass) — 2026-04-21 moved to `spec_verif_and_cleanup.md` as a pre-ship checklist (universal literal-strip, Potkay slot/step migration, anti_spec cross-check, `mycoc check` smoke gate). Implementation task, not spec content.

### Housekeeping for §27–§29

- §27 should note **B3 was absorbed into B6** (one-liner; reviewers noticed the missing B3) — 2026-04-21 **dropped.** Per feedback-memory rule, history breadcrumbs do not belong in spec prose; reviewers looking for B3 can check dev_notes.
- §29 should carry forward `open_questions.md` Deferred block explicitly (not just imply) — 2026-04-21 **dropped.** Miscategorized: the Deferred block (MVN, HMM, VI, stdlib distributions) is §27 content, not §29 units content, and the relevant opens are already tracked in §35 Tier 2 / Tier 3 distribution machinery entries.
- §29 should reference chunk 04 O4.1–O4.8 audit items — 2026-04-21 **dropped.** Miscategorized: chunk 04 is e-graph substrate, not units library; O4.x items are tracked under §35 per the preceding entry.

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


# riley notes for open questions:
- ohhhh a package in myco is called a spore. hell yeah
- - crates.io - like thing (bigmyco.com/spores)
- rustdoc-like thing (bigmyco.com/spores/plant_hydraulics/docs)
- i like svelte 5 for web development i'll probably use that
- not an immediate concern
- i have a placeholder up on bigmyco.com right now
- site to host main myco docs + user 