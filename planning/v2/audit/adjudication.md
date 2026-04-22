# Phase 1 Adjudication Log

Curated accumulator of findings from the Phase 1 per-section audits.
Drops stale-doc-only conflicts (which the archival plan handles),
collapses repeats, and flags items that need Riley's attention.

**Status vocabulary.** Two orthogonal axes: adjudication disposition
(what the design call is) and prose status (whether spec_new.md
reflects it). Labels combine both.

- `ACC` — accepted design call; spec_new.md prose not yet written.
- `ACC/W` — accepted and written into spec_new.md (or anti_spec).
- `O` — open design call; spec_new.md does not yet track it.
- `O/W` — open and written as an inline `*Open.*` note in spec_new.md
  (usually with §34 Chunk-Slotted Work bullet and a chunk report
  where the design needs one). Design call deferred; state is
  consolidated so no cross-document sync is required to find it
  again.
- `SKIP` — not actionable (stale-doc-only, redundant, or covered
  elsewhere).
- `REVIEW` — Riley flagged; needs a decision before it can become
  `ACC`, `ACC/W`, `O`, or `O/W`.
- `REFORMULATE` — adopt a variant of the recommendation (see Notes).
- blank — pending Riley's adjudication.

Full per-section reports live alongside this file as
`NN_section_*.md`. Stale-doc-only conflicts (where the only problem is
in `spec.md` or `v2.1_in_progress.md` and spec_new.md is already
correct) are noted per-batch at the end of each batch and not tabled.

---

## Batch 1 (§0-§4) — ACC/W

### §0 — What Myco Is

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | soul.md Principle 1 "reads like relationships / abstraction is leaking" test missing | add one-sentence version to §0 | SKIP | |
| H2 | MDL framing missing | add sentence about compressed relational description | SKIP | |
| H3 | Myco-is-general scope rule not captured | §0 Scope: Myco is a general language; domain libraries live external to the core | ACC/W | Scope paragraph rewritten (spec_new.md:59) |
| C1 | §0 Scope names GPU/ecosystem project directly | rewrite Scope as capability statement without naming downstream project | ACC/W | collapsed into H3 edit |

Note: H3 ACC effectively replaces the current C1 scope paragraph anyway.

### §1 — Canonical Glossary

§1 is a 14-term skeleton with no definitions written. All findings below feed the Phase 5 glossary draft.

| ID | Finding | Recommendation | Status |
|---|---|---|---|
| H1 | `node` missing | add to §1 | ACC/W |
| H2 | `binding verb` missing | add to §1 | ACC/W |
| H3 | `bound variable` / `free variable` missing | add both to §1 | ACC/W |
| H4 | `SCC` missing | one-line def pointing at §20 | ACC/W |
| H5 | `residual graph` missing | one-line def redirecting to §19 | ACC/W |
| H6 | `~` operator missing | add to §1 | ACC/W |
| H7 | `geometry` / `domain` missing | add both to §1 | ACC/W |
| H8 | `impl` / `some` missing | add both to §1 | ACC/W |
| C1 | `data contract` listed but retired | drop from §1 term list | ACC/W |
| C2 | `approximate` bare noun | rename to `` `approximate` `` block | ACC/W |

### §2 — Modules, Imports, Scope

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Re-export rule missing | land in §2 | ACC/W | |
| H2 | Circular import rule missing | land in §2 | ACC/W | |
| H3 | `initial` / `temporal` illegal at module scope not stated | add statement | REVIEW | Riley flagged; "is this stuff leftover from older temporal handling?" Possible answer: the new module-scope rule (cross-entity relations only) makes the explicit prohibition redundant. Check against §9 audit first. |
| H4 | `file-local` tier has no design backing | explain file-local or drop | ACC/W | subsumed by silent-abolish below |
| H5 | `spore` / registry / fs-to-module-path mapping unresolved | note in open_questions.md | ACC/W | added under new "Module packaging and distribution" Tier 2 section |
| H6 | three vs two visibility tiers (overlaps C1 below) | see C1 | ACC/W | subsumed by silent-abolish below |
| C1 | public/private/file-local at all, no design backing | abolish visibility from Myco entirely | ACC/W | visibility prose removed silently from §2 (spec_new.md:201-230) |

**Visibility subagent result** (`planning/v2/audit/design_review_visibility.md`).
Two genuine module-visibility occurrences in all of spec_new.md, both in §2 (lines 204 and 210). Both are prose bullets, not grammar. Zero `pub` keyword in any `.myco` code block. Four other "visibility" hits refer to the cross-scale topology rule (composites see components), which is orthogonal and not module visibility. Classification: both occurrences are incidental / cargo-culted — not load-bearing for external APIs or implementation hiding in the current skeleton. No spec section outside §2 depends on module visibility.

- **Design A (abolish visibility entirely):** only §2 edits. `pub` disappears from the grammar. File-local tier gone. Workflow binding asymmetry ("workflow ignores pub") dissolves because there is nothing to ignore. Re-export collapses to "does your module re-expose the name under its own path?" Use leading-underscore convention for "implementation detail" with no compiler enforcement.
- **Design B (implicit visibility from use-site):** worse — symbol status becomes a whole-project emergent property, breaks doc generation, non-local resolution ordering, does not solve re-export.
- **Riley's decision:** abolish silently. No pub/priv mention in the spec at all. No underscore convention. It is math, not code. Don't mention the sentiment in any doc.

Applies to §2 H3/H4/H6/C1 and §6 H3 in one move.

### §3 — Types

| ID | Finding | Recommendation | Status |
|---|---|---|---|
| H1 | "Atomic = leaf of containment tree, not single-field" missing | land in §3.3 | ACC/W |
| H2 | Named-type comparison rules for `=`, `<`, etc. missing | land rule prose in §3.3 or §7 | ACC/W |
| H3 | Scalar-value generics (`LOW: Scalar<U>` as type param) missing | clarifying sentence in §3.6 | ACC/W |
| H4 | Pre-CC1 literal in v2.1_in_progress Euros/Dollars example | cleanup note in spec_dev_notes (doc only) | SKIP | doc-only; stale-doc handled by archival plan |

Stale-doc-only: §3 C1, C2 — only affect v2.1_in_progress.md, handled by archival plan. No action on spec_new.md.

### §4 — Values and Literal Policy

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | π / e clarification missing | reformulate: π, e are ordinary stdlib-declared identifiers. No CC1 carve-out for them. Values bind at compile time via the workflow layer like any other constant. | ACC/W | §4 prose landed (spec_new.md:461+); chose path (a) "stdlib ships default bindings" — see resolution note below |
| H2 | open_questions "Literal constants in .myco" still open | mark RESOLVED pointing at §4 + anti_spec CC1 | ACC/W | |
| H3 | B2 rewrite-set consequence of CC1 unplaced | note in §4 or §17: workflow-bound numerics enter the e-graph as observation-style equalities | ACC/W | |
| S1 | dimensionless-ratio carve-out not in anti_spec | add under CC1 cluster | ACC/W | |

**Open question from Riley's H1 reformulation:** how does a stdlib constant like `universal pi: Scalar<dimensionless>` get a numeric value without the user writing an explicit `assume_constant('stdlib::pi', 3.14159265...)` in every workflow?

Resolution adopted in §4: path (a). The stdlib ships default bindings so users do not write them by hand. The specific mechanism (workflow-import hook vs. compile-time injection at stdlib reference) is a workflow-layer design detail; flagged for the workflow-verb design pass but not a Tier 1 language blocker.

Stale-doc-only: §4 C1 (chunk 04 vs §4 exception-3 naming) becomes moot once H1 reformulates away the π/e carve-out. §4 C2 (spec.md §4.11 dimensionless carve-out) handled by archival plan.

---

## Batch 2 (§5-§9) — pre-adjudicated

### §5 — Units

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | §5 has no preamble; jumps into `convert` with no def of `base_unit`, `Scalar<U>`, derived units | add §5.0 preamble (or forward-reference §3) | ACC/W | §5.0 "Unit System Fundamentals" added (spec_new.md §5.0); covers `base_unit`, `Scalar<U>`, derived unit syntax, storage invariant, no-inference policy, expression annotation, §17 forward-ref |
| H2 | Affine unit semantics missing (`20°C * 2` rules, subtraction to base-unit diff) | add as §5.4 or fold into preamble | ACC/W | §5.4 "Affine Unit Semantics" added; covers multiply-requires-base, subtraction yields base-unit diff, add-diff-to-affine, compile error on affine+affine |
| H3 | Base-unit internal storage invariant missing (why `value_in` exists) | one sentence in §5.3 or preamble | ACC/W | "Base-unit storage invariant" named paragraph in §5.0; §5.3 Summary cross-refs §5.0 to motivate `value_in` |
| H4 | Workflow-boundary unit parameter (`assume_series(..., unit='K')`) unplaced | add to §5.3 or §5.5 | ACC/W | §5.5 "Workflow-Boundary Unit Parameter" added with `assume_series` example and dimension-mismatch error; cross-refs §24 |
| H5 | No-implicit-unit-inference policy not stated | one-line policy note in §5 | ACC/W | "No implicit unit inference" named paragraph in §5.0; also reflected in §5 top-level Summary |
| H6 | Named-type arithmetic/comparison coercion rules unplaced | pick a home (§3 or §5); overlaps §3 H2 | ACC/W (via §3.3; §5 should cross-link) | |
| H7 | Expression-level unit annotation syntax `(expr) mol_m2_s` missing | add to §5.0 or §5.4 | ACC/W | "Expression-level unit annotation" named paragraph in §5.0 with `mol_m2_s` example |
| H8 | Forward-reference from §5 to §17 (unit-normalization rewrites) missing | one-line forward reference | ACC/W | Forward-ref sentence at end of §5.0 body and in §5 top-level Summary (spec_new.md §5.0 closing paragraph) |
| C1 | §5.1 "Required for conservation-group siblings" ambiguous | reword; bare `<->` always legal between same-dimension types | ACC | |
| C2 | §5.3 omits `value_in` return type | state `Scalar<ratio>` or `Scalar<dimensionless, T>` | ACC/W | §5.3 body and Summary updated: return type is `Scalar<dimensionless>` (consistent with spec-wide usage) |

### §6 — Functions

| ID | Finding | Recommendation | Status |
|---|---|---|---|
| H1 | `fn` vs `relation` distinction not stated | add paragraph to §6; or forward-ref to open_questions | O/W | Design decision resolved in chunk 08 (`08_relation_fix_whoops.md`): user `fn` banned in favor of parameterized relations; contract methods become required parameterized relations; kernels become parameterized relations. Spec prose in §6 still describes the prior `fn`-as-first-class surface — stale relative to the lock but full prose rewrite is a chunk-08 application, not a one-line fix. Canonical tracking: §6 inline `*Open (pending application).*` note + §34 Chunk 08 bullet. Applied 2026-04-22 |
| H2 | Generic function example (`arrhenius<U: Unit>`) missing | add example + one-sentence monomorphization | ACC/W | §6.1 "Generic Functions" added with `fn arrhenius<U: Unit>(...)` example and monomorphization rule (spec_new.md §6.1) |
| H3 | Function visibility / module packaging not stated | one sentence cross-ref to §2 (pending visibility subagent) | ACC/W (visibility abolished; no cross-ref needed) |
| H4 | Compiler roles for `fn` bodies (dimensional analysis, sym-diff, solver emission) not stated | short list in §6 | ACC/W | §6.2 "Compiler Roles for `fn` Bodies" added: three bullets covering dimensional analysis, symbolic differentiation (deriv lowering), and solver emission / e-graph rewriting (spec_new.md §6.2) |
| H5 | User recourse for uninferable inverses (refactor-into-composable-pieces) not stated | add sentence | ACC/W | Named paragraph "User recourse when the compiler cannot infer an inverse" added at end of §6.2; cross-refs `Invertible<_>` (§7) (spec_new.md §6.2) |
| H6 | "Kernels-as-functions" architectural decision not stated | anti_spec entry "no `kernel` keyword" or sentence in §6 | ACC/W | Sentence added to §6 body: kernels are ordinary `.myco` functions (no `kernel` keyword or kind). Anti_spec row added retiring `kernel` as a hypothetical keyword (anti_spec.md "Retired keywords / syntax" table) |
| H7 | Functions as closure-policy extensibility surface not stated | cross-ref from §6 to §8 closure policy | ACC/W | Sentence added to §6 Summary; named paragraph "Closure-policy extensibility" added in §6.2 with cross-ref to §8.7 policy Y5 (spec_new.md §6.2) |

Stale-doc-only: §6 C1, C2.

### §7 — Contracts

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Diamond-supertrait same-name field rule missing | one sentence in §7.3 or §7.4 | ACC/W | (a) hard error at contract declaration time. `contract C : A + B` with an unresolvable same-name collision between A and B is rejected immediately. §7.4 rewritten with the two-case split (diamond-via-common-Root = one obligation; different-obligations-colliding-on-name = hard error). Silent-shadow and per-impl-disambiguation rejected |
| H2 | "Contracts cannot declare `initial`/`temporal`" constraint missing | add to §7 (related to §9 H5) | ACC/W | Canonical paragraph added to §9 body ("Type bodies vs. contract bodies." named paragraph); cross-link sentence added to §7 preamble body. Both state the restriction positively: contracts are structural, type bodies own `initial:`/`temporal:`/`d(x)=`/`step(x)=`. |
| H3 | Contract default implementations missing | add §7.x (fallback-not-override rule) | ACC/W | §7.5 "Default Implementations" added. Fallback-not-override rule stated; code fragment shows a contract with a default `fn label` and a type that overrides it vs. a type that inherits the default. |
| H4 | "Uniform application to types, functions, distribution families" should be promoted to §7 preamble | promote statement | ACC/W | §7 preamble body paragraph rewritten: "Contracts apply uniformly to types, functions, and distribution families." is now the leading sentence rather than the fifth. The Summary already had it first; body paragraph now matches. |
| C1 | open_questions vs anti_spec on `condition_weighted` | update open_questions entry to RESOLVED | ACC/W | `open_questions.md` has been deprecated (`open_questions_deprecated_use_spec_new.md`). No edit to the deprecated file is required. C1 is void; status flipped to reflect that the landing point (spec_new.md) already reflects the resolved state via §8.7 Y4 prose and anti_spec. |

Stale-doc-only: §7 C2.

### §8 — Relations and Equality

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Named-type equality rule for `=` in relation bodies missing | place once §3/§5/§7 settles; overlaps §3 H2, §5 H6 | ACC/W (via §3.3; §8 should cross-link) | |
| H2 | E-graph merge as explicit mechanism for relation `=` not named | add sentence or cross-ref §16 | ACC/W | Sentence added to §8 preamble body: every `=` in a `relation` body introduces a Layer-1 e-class merge (cross-refs §16, §17 merge source 1) |
| H3 | `constraint` Layer 2 envelope placement missing | cross-ref §8.1 to §16 | ACC/W | Sentence added to §8.1 body: each `constraint` obligation attaches as Layer-2 envelope metadata on the relevant e-class (cross-ref §16) |
| H4 | `where` on `convert` bodies applying three-way classification missing | note in §8.6 or §8.3 | ACC/W | Sentence added to §8.6 closing paragraph: three-way classification applies to `where` preconditions on `convert` bodies (§5); placed in §8.6 because that section owns the three-way classification machinery |

Stale-doc-only: §8 C1, C2. Contract-default-relations note merged with §7 H3.

### §9 — State and Time

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Four initialization mechanisms (`initial` block, `assume_initial`, `learn_initial`, `learn_trajectory`) and mutual exclusion missing | add §9.x; largest gap in §9 | ACC/W | §9.3 Initialization added. Four mechanisms listed as bullets with mutual-exclusion rule, compiler diagnostic for missing/duplicate initialization, and cross-ref to §24 for verb semantics. Code snippet uses `moisture_field_capacity` (workflow-bound universal, no CC1 violation). Workflow verbs `assume_initial`, `learn_initial`, `learn_trajectory` already appear in §24 preamble. |
| H2 | `step(·)` pre-tick RHS / current-tick LHS semantics + swap-not-cycle consequence missing | two sentences in §9 | ACC/W | Two sentences added to §9.1 body after the discrete-form bullet: pre-tick RHS / current-tick LHS rule, then swap-not-cycle consequence. |
| H3 | Mixing `d(·)` and `step(·)` coexistence rule missing | one sentence in §9 | ACC/W | One paragraph (three sentences) added to §9.1 body immediately after the H2 sentences: both forms legal in same model, update discipline distinction, compiler composes without user coordination. |
| H4 | Per-path uniqueness for type-body instance expansion missing | expand §9.2 to cover both expansion sources | ACC/W | §9.2 retitled "Per-Path Uniqueness After Expansion". Body expanded: generic expansion paragraph retained; new "Type-body per-instance expansion" paragraph covers per-instance expansion colliding with module-scope or other per-instance expansions, with compile-error diagnostic naming both sources. |
| H5 | No-override rule (contracts cannot carry `initial`/`temporal`) missing | add to §9; cross-link from §7 H2 | ACC/W | Subsumed by §7 H2 landing. §9 already carries the "Type bodies vs. contract bodies." paragraph (spec_new.md lines 1169-1176) added during the §7 H2 edit. No further §9 edit needed. |
| H6 | Locus-scoped `temporal name on locus:` missing | add §9.x | ACC/W | §9.4 Locus-Scoped Temporal Blocks added. States `on locus:` applies symmetrically to `relation` and `temporal`; boundary-specific evolution example (bulk diffusion vs. surface evaporation at top_boundary); cross-ref to §11 for locus machinery; obligation-key distinction cross-ref to §9.2. Also resolves §2 H3 and §11 H10. |
| C1 | §9.2 never defines "obligation key" despite §10.2 citing it | add definition to §9.2 | ACC/W | One-sentence definition added to §9.2 Summary paragraph: obligation key is the canonical fully-qualified path string (`type_instance.field` with generic parameters bound) identifying a unique temporal, initial, or relation obligation after all expansion. |

---

## Batch 3 (§10-§14) — pre-adjudicated

### §10 — Dynamic Topology and Events

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | `when` clause and edge-triggered event semantics missing | add §10.0 or §10.1 note on `when` trigger surface | ACC/W | §10.0 "Event Triggers" added before §10.1. Covers: `when` as the trigger surface; Boolean-valued expression referencing participant and container fields; edge-triggered semantics (fires on false-to-true transition only); falling-and-rising fires twice; persistently-true fires once. Illustrative snippet uses workflow-bound universal in threshold comparison (CC1-clean). Probabilistic `when` clause cross-refs §13.1. |
| H2 | Concrete-output-type requirement for events targeting `impl`-typed collections (`event oak_recruit: -> Tree<FarquharC3>`) missing | add to §10.2 | ACC/W | "Concrete output type for `impl`-typed collections" named paragraph added to §10.2 body. States the rule, gives an `oak_recruit` example, explains that generic event expansion produces concrete types by construction, and declares omission a compile error. |
| H3 | Specific Python scheduling policy API (`policy(pending_firings, state) -> List[Firing]`) and three stdlib policies not stated | extend §10.1 with API shape + stdlib policies | ACC/W | (b) §10.1 commits to the contract and names the three stdlib policies (priority-based, random-with-seed, FIFO); Python API signature deferred to §24 (workflow verbs) since it's a workflow-layer concern. Open-question entry added in §35 tracking the signature resolution for Phase 1 batch 5 |
| S1 | v2.1_in_progress "within-event index-order tiebreak" stale | add row to anti_spec: §10.4 three-case analysis replaces index-order tiebreak | ACC/W | Row added to anti_spec.md "Retired architectural framing" table: retired = "within-event index-order tiebreak"; replacement = "§10.4 three-case exhaustive analysis"; why = ordering not needed once the three cases are classified. |
| C1 | §10.4 "no within-event ordering" vs v2.1_in_progress "tiebreak by index order" | no action on §10; archival plan + S1 handle it | SKIP | stale-doc; S1 covers it |
| C2 | §10.1 omits the resolved API shape that open_questions has | overlaps H3 | SKIP | covered by H3 |

### §11 — Geometry and Locus

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | `geometry` keyword and `Domain<G>` annotation not named in §11 | add §11.0 naming both | ACC/W | §11.0 Foundations added before §11.1. Two paragraphs: (1) `geometry G { ... }` as first-class construct naming topology/chart/metric/loci; (2) `Domain<G = SomeGeometry>` as ordinary composite-type annotation binding geometric behavior. §11.11 forward-ref for full keyword vocabulary |
| H2 | `as` clause (coord names/units/extents on domain type) absent | add to §11 (explains `bind_topology` edge-length unit validation) | ACC/W | Landed as third paragraph of §11.0 body. Covers: positional binding (first `as` name to first chart binder), required on every domain type, edge-length unit validation against `as`-clause coordinate units at workflow composition |
| H3 | `trace()` as directional-limit primitive for graph junctions absent | distinguish from manifold-restriction `trace` or rename one | ACC/W | (a) rename the graph-junction operator to `limit_from(f, junction, edge)` (Rustacean `from_X` idiom). `trace(f, boundary)` retains the manifold-restriction meaning (standard PDE trace operator). §11.1 updated with both; anti_spec entry for the old overloaded spelling |
| H4 | Locus-scoped relations with `replaces` obligation keys absent | add to §11.8 or own subsection | ACC/W | "Locus-scoped relations with `replaces` obligation keys" named paragraph added to §11.8. Explains stable obligation-key form (`balance(axial_flux)`), why stability across renaming matters, cross-refs §8.10 and §10.5 |
| H5 | `identify` (periodic seam) absent; `Sphere` underdefined without it | note `identify` at least for `Sphere` if kept | ACC/W | Note added in §11.3 immediately after the solid-vs-manifold naming rule paragraph. Covers: `Sphere` carries `identify phi = 0 <-> phi = 2 * pi`; v2.1 scope (scalar fields only); vector/tensor seam transforms deferred (§35); underlying X2 mechanism via Layer-3 site records (§17) |
| H6 | `bind_topology` schema + validation rules absent | describe schema (vertex IDs, edge list, edge-length units, vertex tags) | ACC/W | §11.5 expanded with "Schema for `rooted_tree` and `metric_graph`" block listing five schema fields (vertex IDs, edge list, edge-length units, vertex tags, root vertex). Validation bullet added (unit mismatch, missing tag coverage, vertex-ID gaps, missing root, cycles = compile errors at workflow composition). Manifold-vs-network distinction added (manifolds use `experiment.compile`, not `bind_topology`) |
| H7 | `continuous(field)` and `kirchhoff(potential, flux)` stdlib helpers absent | add one-sentence §11.8 note | ACC/W | "Stdlib junction helpers" named paragraph added at end of §11.8 body. Names both helpers, states they are stdlib convenience functions not compiler magic, and explains the expansion of each. Users may always write explicit trace equations |
| H8 | Subdimensional fields (`field name: Type over coord`) absent | add `over` keyword to §11.7 | ACC/W | "Subdimensional fields (`over` keyword)" named paragraph added at end of §11.7 body. Covers form `field name: Type over coord`, positional semantics (varies in named coord only; constant in orthogonal directions), compiler treatment, and multi-coordinate variant |
| H9 | `curl` listed as settled elsewhere but absent from §11.1 | either include in §11.1 list or flag deferred | ACC/W | ship both 2D (scalar) and 3D (vector) under one name; dimension dispatch at stdlib axiom level via case-on-val-generic in return position, mirroring §3.9/§30 `solve` dispatch on structural subtype. §11.1 updated; §35 flags case-on-val-generic formalization as small open |
| H10 | Locus-scoped `temporal` has no subsection despite summary header mentioning it | add parallel treatment to locus-scoped relations | ACC/W | One-sentence cross-ref to §9.4 added as closing sentence in §11.8 body (after the stdlib helpers paragraph). No standalone §11 subsection needed; §9.4 is the canonical home |
| H11 | Terrain-as-field deprioritization not recorded | land as anti_spec entry or §11 note | ACC/W | Row added to anti_spec.md "Dropped features" table: retired = terrain-as-field on irregular domain boundaries (v2.1); replacement = terrain-as-field on flat domain; why = correctness vs elegance distinction, flat-domain pattern covers all practical v2.1 cases, irregular boundaries deferred beyond v2.1 |
| C1 | §11.3 stdlib geometry names diverge from corpus (Ball3D new; Line1D/Rectangle2D renamed; Polar/Sphere dropped) | reconcile: either adopt §11.3 and retire old names in anti_spec, or revert §11.3 | ACC/W | adopt authoritative suffix-free catalog: `Interval`, `Circle` (S¹), `Rectangle`, `Disk`, `Sphere` (S²), `Box`, `Ball`, plus the three network geometries. Solid-vs-manifold distinction load-bearing (`Sphere` = surface; `Ball` = solid). Coord systems live on `as` clauses, not as separate geometry types (no `Polar`/`Spherical`). §11.3 rewritten; anti_spec entries added for dimensional-suffix names, `Polar`/`Spherical`-as-type, and `Sphere`-as-solid |
| C2 | §11.1 spells `laplace`; every other doc uses `laplacian` | standardize on `laplacian`; retire `laplace` | ACC/W | §11.1 updated; anti_spec entry added |
| C3 | `trace` overloaded (manifold-restriction in §11.1 vs graph-junction directional-limit in chunk 01) | resolve overload per H3 | SKIP | covered by H3 |

### §12 — Collections and Iteration

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | `max` and `min` as collection aggregation primitives missing | add to §12.1 + §12.3 (chunk 02 §4.2 settled) | ACC/W | `max`/`min` bullet added to §12.1 primitive list (scalar extrema, unit-compatible, subgradient); §12.1 Summary updated; §12 preamble Summary updated. §12.3 Summary updated; `max(empty) = -inf`, `min(empty) = +inf` bullet added to §12.3 body list; identity-element prose paragraph updated to name `max`/`min`. |
| H2 | Backend sentinel injection (`-inf`/`+inf` for masked invalid slots) missing | add to §12.3 or §21 lowering prose | ACC/W | "Sentinel injection for masked slots" named paragraph added at end of §12.3 body. Covers bitmask-liveness lowering context, JAX/PyTorch both-branches-evaluated behavior, `-inf` injection for `max`/`argmax` and `+inf` for `min`/`argmin`, alive-element semantics preservation. |
| H3 | `count` alive-element vs backing-array-length distinction missing | note in §12.1 or §12.4 | ACC/W | `count(xs)` bullet in §12.1 expanded: "number of alive elements"; added sentence that for bitmask-liveness collections `count` sums liveness bits, not backing-array capacity, with cross-ref §12.4. |
| H4 | `argmin`/`argmax` tie-break rule (deterministic, index order) missing | add to §12.2 | ACC/W | "Tie-break rule" named paragraph added to §12.2 body after the IR sum-type paragraph. States earliest index in canonical index order wins; deterministic, no runtime randomness. |
| H5 | `argmin`/`argmax` differentiability class (`subgradient`) missing | add to §12.2 or §12 compiler-role note | ACC/W | "Differentiability class" named paragraph added to §12.2 body after the tie-break paragraph. States subgradient-differentiable, gradient flows through selected element, undefined at tie points, drives A-group rewrite routing (§17), soft alternative tracked §35. `max`/`min` also noted as subgradient in their §12.1 bullet with cross-ref to §12.2. |
| C1 | §12.3 empty-collection table structurally inconsistent with chunk 02 re: min/max | fixed by H1 | SKIP | covered by H1 |

### §13 — Probabilistic Programming

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | B1 opaque `log_pdf` forward reference missing from §13 | add one-sentence note in §13.2 or §13.3: opaque families route Tier C pending B1 | ACC/W | Sentence added to §13.2 Tier C bullet: opaque families route Tier C by default; stdlib policy for what qualifies as opaque tracked in §33 as open item B1. |
| H2 | `Distribution<U>` contract interface (required `log_pdf`, required `sample`, optional `reparameterized_sample`) not stated anywhere | add contract block to §27 with cross-ref from §13 | ACC/W | Contract block already at §27 preamble (lines 3947-3994) covering required `log_pdf`, `sample`, `pdf` and optional capability sub-contracts. Cross-ref sentence added to §13.2 preamble pointing to §27 for the `Distribution<U>` contract surface. |
| H3 | Workflow-side epistemic prior API (`assume_prior`) unconnected from §13.1 | add sentence cross-referencing §24.4 (verb tracked as future) | ACC/W | Sentence added after the epistemic bullet in §13.1: workflow-side prior binding uses `assume_prior` (§24.4), which attaches a distributional fact to the e-class at training time. |
| H4 | Z-group rewrite catalog (Z1-Z10) not cross-referenced from §13 | confirm Appendix C covers Z-group; else add §13.2 footnote | ACC/W | Appendix C Z-group (Z1, Z5, Z10, Z11; Z2-Z4/Z6-Z9 reserved for conjugate/Tier B) confirmed at lines 5244-5279. Sentence added to §13.2 Tier A bullet cross-referencing the full Z-group catalog in Appendix C. |
| H5 | `SumSelfClosed` conditional-rewrite nuance (same-rate, same-p) missing | add one-sentence qualifier in §13.2 | ACC/W | Sentence added to §13.2 Tier A bullet: `SumSelfClosed` holds for Gamma only under shared rate parameter, for Binomial only under shared success probability; per-family conditions in §27.1. |
| H6 | HMM / Markov-structured discrete latents compile-error not named in §13.3 | add sentence distinguishing this from silent Tier C fallthrough | ACC/W | Sentence added to §13.3 closing paragraph: Markov-structured discrete latents (HMM-style temporal dependencies) are a compile error with diagnostic guidance, require structural handling per §28, and do not fall through to Tier C. |
| S1 | open_questions.md §Closure policy still shows `condition_weighted` deferred | strike paragraph or mark RESOLVED (anti_spec already records it) | ACC/W | open_questions.md renamed to open_questions_deprecated_use_spec_new.md on 2026-04-22; `condition_weighted` landed 2026-04-20 as Y4 un-deferred. No edit to the deprecated file required; deprecation supersedes stale content. |
| C1 | §13.1 structural rule ("aleatoric `~` appears inside `temporal:` or event scope") is too narrow; observation likelihoods in plain `relation` bodies are also aleatoric | tighten rule: aleatoric = LHS is measured/observed OR inside temporal/event; epistemic = unknown constant not observed | ACC/W | §13.1 rewritten 2026-04-22 with the tightened rule: aleatoric when LHS is measured/observed-tethered OR inside temporal/event scope; epistemic when unknown constant not observed per time-step (module scope, `initial:`, or `~` whose LHS is neither data-bound nor in temporal/event scope). Classification is compiler-derived (static graph-position + workflow-binding inspection), not user-annotated |

### §14 — Compiler Intrinsics

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | `deriv` has no subsection in §14 despite being named in the summary | add §14.4 `deriv` covering three lowering modes + SCC interaction + limitations | ACC/W | §14.4 added before §15 heading. Three ordered modes (symbolic via A-group rewrites, algorithmic compile-time chain-rule, runtime backend AD). `.mode` accessor on return matches `condition_of` pattern. `Differentiable` contract requirement stated. Runtime mode cross-references §33 B6 backend-AD ownership. |
| H2 | `condition_of` consumer mapping incomplete (extraction ranking, diagnostics missing; Level III unavailable to closure policies not stated) | expand §14.1 | ACC/W | Paragraph added to §14.1 after "Primary consumer" sentence: extraction ranking (§19) consumes Level I and II only; Level III unavailable to closure policies at extraction time (requires runtime numerical computation); diagnostic surfaces (§22) expose Level III at post-run inspection. |
| H3 | Duality worked example missing per O2.4 documentation obligation | add brief pair to §14.1 (`(exp(x)-1)/x` vs `expm1(x)/x`; linear-solve block) | ACC/W | Worked-example paragraph added to §14.1 after H2 consumer-mapping paragraph. Covers `(exp(x)-1)/x` vs `expm1(x)/x` (well-conditioned problem, ill-conditioned naive algorithm, Level II tight with `expm1`) and linear solve `A x = b` (Level I = κ(A); Level II tracks pivot quality for Gaussian elimination or Q factor for QR). Inline prose, no code fences. |
| H4 | `condition_of` return accessor name mismatch: §14.1 uses `.level`, O2.4 locks `.mode` | reconcile; pick one and log in dev_notes | ACC/W | `.mode` (Rust convention: `.level` implies ordering, `.mode` implies alternatives; tolerance classes are alternatives not severities). §14.1 updated |
| H5 | `deriv` runtime fallback gated on B6 AD-ownership not cross-referenced | add forward reference to §14 (or §14.4) | ACC/W | Covered by §14.4 runtime-mode bullet, which states "(gated on §33 B6 backend-AD ownership)" explicitly. |
| S1 | v2.1_in_progress "`deriv` always symbolic / no runtime cost" framing stale | add anti_spec entry: runtime AD is the fallback for large SCCs | ACC/W | Row added to "Retired architectural framing" table in anti_spec.md: "`deriv` always symbolic / no runtime cost" framing retired in favor of three-mode lowering per §14.4; runtime AD is the authorized fallback for SCCs too large to expand symbolically, gated on B6 backend-AD ownership. |
| C1 | `loss_of` field inventory conflict: §14.2 uses `{data_fit, constraint_violation, regularization}`; O2.4 locks `{compute, approximation, condition, truncation, discretization}` | resolve overload: one struct, two structs, or rename one intrinsic. Document and update anti_spec | O/W | Canonical tracking: new chunk 12 (`12_cost_field_unification.md`) with problem statement + three options + load-bearing questions + cross-refs. Inline `*Open.*` notes added at §14.2 and §19.1; §34 Chunk 12 bullet added. Design call deferred; state consolidated. Applied 2026-04-22 |
| C2 | `condition_of` accessor name (same as H4; tabled as C2 in §14 report) | covered by H4 | SKIP | covered by H4 |

---

## Batch 4 (§15-§19) — pre-adjudicated

### §15 — Approximate Blocks

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Orientation axis (bidirectional vs unidirectional) orthogonal to faithfulness not developed | fill §15 with 2x3 matrix (faithfulness x orientation); drop "univariate vs bivariate" placeholder | ACC/W | §15 preamble (after the three-tier summary sentence) replaced with two paragraphs: the 2x3 faithfulness x orientation matrix described with fuzzy-model (L-group, M-group) and fuzzy-tolerance (K-group, Q-group) cells named; a second paragraph maps the three-tier trust-posture labels (lossless / lossy-model / lossy-tolerance) to the strict / fuzzy-model / fuzzy-tolerance faithfulness rows. Cross-ref to Appendix C summary table for the full cell enumeration. "Univariate vs bivariate" framing dropped. |
| H2 | Envelope-narrowing makes normally-lossy rewrite lossless in context; absent from §15 | add paragraph to §15.2 or §15.3 noting envelope metadata (§16) can narrow a default-off rewrite to default-on in context | ACC/W | Paragraph added at end of §15.3 (after the "three different trust postures" sentence, before §15.4 header): states that envelope metadata (§16, Layer 2) can narrow a rewrite's error class in context; a normally lossy-tolerance block becomes default-on when the envelope proves the error bound collapses to zero; canonical location is §15.3; §17.6 carries the rewrite-predicate corollary. |
| H3 | Declaration/derivation interaction three-case semantics absent (user-declared bound vs compiler-derived) | add §15.4 or §15.2 subsection covering (a) compiler-proves-exact / (b) compiler-within-user-declaration / (c) compiler-disproves-declaration | ACC/W | New §15.5 "Declaration/Derivation Interaction" inserted after §15.4, before the Part II separator. Summary bullet + one-paragraph preamble + three labelled cases: (a) compiler proves exact — user's declaration retained in provenance, compiler's result governs; (b) compiler within user declaration — derived bound looser but within tolerance, authorized; (c) compiler disproves declaration — compile error naming both bounds and the rewrite. |
| H4 | Multi-dimensional cost vector absent; `loss_of` named fields not cross-linked | cross-ref §15.2 to §14.2 named-field `loss_of` struct | ACC | gated on §14 C1 resolution; apply once the canonical struct is picked |
| H5 | Mutual exclusion: exactly one of `under` / `tolerance_class` required; not stated in §15.1 | one-sentence rule in §15.1 | ACC/W | Three-sentence rule added to §15.1 after the `where:` bullet (before "Blocks compose by nesting"): states that exactly one of `under` and `tolerance_class` is required; defines the derivation behavior of each path; specifies that both-present and neither-present are compile errors. |
| H6 | Workflow-side sampling parameters (`run.config.loss_estimation`) absent | add sentence to §15.2 noting sampling config is workflow-side per CC1; cross-ref §24 | ACC/W | Sentence appended to the closing paragraph of §15.2 (after "lossiness is a lattice join over them, not a single authoritative source"): states that sampling parameters for empirical error estimation (sample count, seed, stratification) live workflow-side per CC1; the `.myco` `approximate` block names the rewrite and bound; `run.config` surfaces the numerical parameters (§24). |
| H7 | Named Tier B rewrites (K1/M1/M2/Z8/Z9) that fire under `approximate` blocks unlinked | cross-ref §15.1 `under` field to Appendix C | ACC/W | `under` bullet in §15.1 updated to name the rewrite groups (K-group fuzzy-tolerance, M-group, L/Q-group lossy-model, Tier B Z-group conjugates) and add "see Appendix C for the closed catalog." |
| S1 | chunk 04 `approximate` expr-infix surface (`approximate A <-> B: under: ...`) superseded by block form | add anti_spec row: expr-infix form replaced by `body:`-scoped block | ACC/W | Row added to "Retired keywords / syntax" table in anti_spec.md: `` `approximate A <-> B: under: ...` expr-infix form `` retired in favor of `` `approximate { body: ... under: ... }` block form ``; rationale: block form scopes cleanly to a named body and is what §15.1 locks. |
| C1 | §15 summary "2x2 matrix (lossy-model vs lossy-tolerance) x (univariate vs bivariate)" contradicts chunk 04 2x3 (faithfulness x orientation) | adopt 2x3 per H1 | SKIP | covered by H1 |
| C2 | §15.2 four-source framing vs chunk 04 five-layer framing | clarify §15.2 describes lossiness *sources* (where); the derivation *stack* (how compiler quantifies) lives in §14 or new §15.4 | ACC/W | Both framings land as orthogonal. §15.2 retains four sources (origin taxonomy). New §15.4 "Five-Layer Lossiness Accounting" adds the accounting stack (Layer 0 syntactic / Layer 1 equational / Layer 2 distributional envelope / Layer 3 adjacent keyed state / Layer 4 extraction cost) with worked example (`hard_clip` as source-1 projection landing at Layer-2 distributional envelope). Orthogonality note added to §15.2 cross-referencing §15.4 |

### §16 — The E-Graph

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | O4.1 `replaces` open-retraction question not cross-referenced from §16.2 | add cross-ref in §16.2 to §35 noting broader retraction semantics open | ACC/W | Sentence appended to the `replaces` bullet in §16.2: states that broader retraction semantics (whether `replaces` should admit full fact-level retraction) are tracked as open item O4.1 in §35. |
| H2 | Provenance as envelope content (union-under-merge rule) unplaced | add to Layer 2 enumeration in §16.1; note rule in §16.3 | ACC/W | Provenance (declaring construct and rewrite trace for every envelope fact) added to Layer 2 content list in §16.1. Sentence added after the Readers list in §16.3: "Provenance composes by set union when two envelope facts merge onto the same e-class; no provenance is dropped." |
| H3 | Faithfulness and orientation tags as Layer 2 merge-edge content unplaced | add to Layer 2 enumeration in §16.1 | ACC/W | Merge-edge annotations (faithfulness tag in {strict, fuzzy-model, fuzzy-tolerance, distribution-family, one-way}; orientation tag in {bidirectional, unidirectional}) added to Layer 2 enumeration in §16.1, noting these attach to the merge edge and cross-referencing §15 and Appendix C. |
| H4 | SCC decomposition results as Layer 3 unplaced | add to Layer 3 enumeration in §16.1 with cross-ref §20 | ACC/W | Layer 3 rewritten as a bulleted content-type enumeration (C1 and H4-H8 applied jointly). SCC decomposition results keyed on SCC identifier (class assignments: algebraic / stochastic / training / fixed-point / iterative-solve / stepper) added with cross-ref §20. |
| H5 | Backend emulate-mode substitutions as Layer 2 lossy-tolerance envelope facts unplaced | add sentence to §16.3 or §16.2; cross-ref §31.1 | ACC/W | Fourth writer "Backend emulation" added to §16.3 Writers list: states that when a backend emulates a missing capability under workflow authorization (§31.1), the emulation path's error class attaches as a layer-2 lossy-tolerance envelope fact on the affected e-classes. §16.3 summary and body updated to reflect four writers. |
| H6 | Workflow provider bindings as Layer 3 unplaced | add to Layer 3 enumeration in §16.1 | ACC/W | Workflow provider bindings (keyed on handle identifying which workflow-side component supplied a value, observation, or learned parameter) added to Layer 3 enumeration in §16.1 with cross-ref §24. Applied jointly with C1 Layer 3 rewrite. |
| H7 | Stochastic sampling traces as Layer 3 unplaced | add to Layer 3 enumeration in §16.1; cross-ref §13 | ACC/W | Stochastic sampling traces keyed on draw ID added to Layer 3 enumeration in §16.1 with cross-ref §13. Applied jointly with C1 Layer 3 rewrite. |
| H8 | Runtime event-trigger state as Layer 3 unplaced | add to Layer 3 enumeration in §16.1; cross-ref §10 | ACC/W | Runtime event-trigger state (keyed on event timestamp for edge-triggered `when` clauses) added to Layer 3 enumeration in §16.1 with cross-ref §10. Applied jointly with C1 Layer 3 rewrite. |
| H9 | §16.4 envelope flavors stated as settled; tensor-extension is open (chunk 05 §3.3) | add caveat to §16.4: composition rules stated for scalar case, tensor extension tracked in chunk 05 | ACC/W | Sentence appended to closing paragraph of §16.4: "The composition rules as stated cover the scalar case; tensor-shape extension (how tolerance envelopes compose across tensor-valued expressions) is tracked in chunk 05 §3.3 (matrix and tensor types)." |
| H10 | Extraction cost tuple connection to Layer 2 not stated as principle | cross-ref §19.1 from §16.4; minor | SKIP | adequately covered by §16.4 pointing to §19.1 per audit |
| C1 | Temporal subscripts `y[1]`, `y[2]` listed as Layer 3 in §16.1 but chunk 04 §4 explicitly corrects this to Layer 1 distinct ground terms | remove `y[1]`, `y[2]` from §16.1 Layer 3 list; replace with per-event copies / identity-tagged instances | ACC/W | Layer 3 enumeration rewritten as a six-bullet content-type list (per-event copies, identity-tagged instances, SCC decomposition results, provider bindings, sampling traces, event-trigger state). Closing sentence clarifies that temporal subscripts (`y[t]`, `y[t+1]`) are layer-1 distinct ground terms; each per-tick copy is its own e-class. H4, H6, H7, H8 applied in the same rewrite. |

### §17 — Equality-Introducing Machinery

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Envelope-narrowing promotion (default-off becomes default-on when error_bound collapses) absent from §17.6 | add named corollary to §17.6 | ACC/W | Corollary paragraph added at end of §17.6: "Envelope-narrowing corollary. A default-off rewrite is promoted to default-on at sites where envelope metadata (§16.1 Layer 2) collapses its certified error bound to zero. The mechanism is canonical in §15.3; this partition treats such a site as effectively default-on for the narrowed context without requiring an explicit `approximate` declaration." |
| H2 | A-Y rewrite catalog closed/extensible not stated | one sentence in §17.4 or §17.5: baseline is Appendix C; workflow extension tracked in §35 | ACC/W | Closed. Appendix C preamble updated 2026-04-22 with **Catalog closure** paragraph: "The A–Z catalog is closed for v2.1. New rewrite rules are not expressible in `.myco`; the compiler is not a user-facing rewrite-authoring surface. Post-v2.1 extensibility lands via a Rust-side plugin system invoked from workflow." Catalog extended A–Y → A–Z to accommodate the locked Z group (distribution-family rewrites). §17.5 updated (26 groups, A-Z) |
| H3 | `replaces` suppression-not-retraction framing absent from §17.1 | forward-ref note in §17.1 (or cross-link from §16.2 already covered by §16 H1) | ACC/W | Two-sentence paragraph added to §17.1 after the eight-source enumeration and the "no silent inference" note: "`replaces` (§8.10, §10.5) suppresses the default-generation merge at the declaration site; it does not retract merges already emitted before the declaration was processed. Broader retraction semantics are tracked as §35 O4.1." |
| H4 | `identify`-seam merges as CC5 site-scoped structural-predicate-gated absent from §17.2 | add sentence distinguishing module-scope `identify x = y` (unconditional) from geometry-body `identify coord_a <-> coord_b` (site-scoped gated) | ACC/W | Resolved 2026-04-22 via opus_identify_review.md recommendations. X-category split into X1 (pole L'Hopital, removable-singularity operator substitution) and X2 (identify, quotient-induced value equality). X2 installs a Layer-3 adjacent-keyed-state "site record" keyed on locus path (e.g., `seam@SphereSurface.azimuth`) carrying glue map, site predicate, and declaration provenance. Layer-1 merges become *derived consequences* of Layer-3 site records (X2 consults the record and emits a merge tagged with the site's identity). §17.1 reframed as "eight authorization sources" with direct-writer (1, 2, 3, 7, 8) vs rewrite-class-authorizer (4 identify-via-Layer-3, 5 stdlib-inverses-via-E-group, 6 convert) split. §17.2 updated with idempotency-at-merge-vs-declaration clarification. Cross-geometry pollution impossible by construction (site records owned by the geometry). §35 CC5 entry locked. Appendix C V1 tightened (observe is a Layer-2 envelope fact, not an equational merge) |
| H5 | O4.3 per-residual exposure constraint (extraction preserves original relation names) absent from §17 | forward-ref in §17.6 to §25 Training Emission | ACC/W | Sentence added to §17.6 before the envelope-narrowing corollary: "Extracted residuals preserve their original relation names under the CC3 / O4.3 constraint, so training-emission diagnostics (§25) can expose per-residual loss contributions; §35 O4.3 tracks the open tension with strict algebraic collapse." |
| H6 | Symbolic-math intrinsics (`deriv`, `integrate`) participating in rewrite system unstated | sentence in §17.4 or §17.5 cross-linking Appendix C | ACC/W | A-group bullet in §17.5 extended: "Symbolic-math intrinsics (`deriv`, `integrate`; §14.3, §14.4) participate via A-group rewrites on compositions of `Differentiable` atoms and stdlib integration-by-parts rules (Appendix C)." |
| H7 | "Merges happen only via the eight sources; no silent inference" negative statement absent from §17.1 | one sentence in §17.1 | ACC/W | Paragraph added to §17.1 after the eight-source enumeration: "No silent inference. Layer-1 merges arise only via these eight authorization sources. The compiler does not infer equality from structural shape, type identity, name coincidence, or any signal outside the enumerated authorizations. Every merge is traceable to a source tag." |
| Open | O4.6 heterogeneous `argmax` tagged handles not Tier-1 ready | do not land in §17; confirm open_questions.md tracks it | ACC/W | Confirmed tracked in §35 (line ~4821, "heterogeneous argmax tagged handles"). open_questions.md renamed to open_questions_deprecated_use_spec_new.md on 2026-04-22; no separate action required. |

### §18 — The Type Graph (stub)

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Chunk 07 three-option coupling framing (A/B/C) not in open_questions.md | add to open_questions.md Tier 0/1 | ACC/W | open_questions.md renamed to open_questions_deprecated_use_spec_new.md on 2026-04-22. Canonical tracker is the chunk 07 report (`planning/v2/v2.1_chunk_reports/07_type_graph_in_progress.md`) which holds the full A/B/C framing (§3); §34 Chunk 07 entry now names the three options and cross-refs the report. |
| H2 | Chunk 07 Q1-Q4, Q6, Q7 not in open_questions.md | add to open_questions.md Tier 1 with chunk 05 Q7 / chunk 06 cross-link for Q6 | ACC/W | Q1-Q7 live in the chunk 07 report (§7). §34 Chunk 07 entry updated to name the report as the canonical tracker. open_questions.md deprecated; spec_new.md §34 plus chunk report are the replacement. |
| H3 | Chunk 07 dependency ordering (after 04/05/06) not recorded in §34 or open_questions.md | add brief dep note to §34 chunk 07 entry | ACC/W | §34 Chunk 07 entry updated with "Depends on chunks 04 (expression e-graph substrate), 05 (refinement-lattice examples from matrix types), and 06 (backend-dependent conversion-edge costs)". |
| H4 | §0.1 Conversion-graph cost model paragraph not forward-ref'd from §18 | add when §18 stub is fleshed out; not urgent for stub | SKIP | deferred with chunk 07 |
| C1 | Stale §18 cross-references in §0.1 conservation-laws paragraph (line ~85) and §0.1 Part II intro (line ~1915-1916) — point to §18 for SCC/residual classification | replace `(§18)` with `(§20)` in both locations; also note conservation-group membership is genuine §18 concern but enforcement thread is §19.3/§20 | ACC/W | Replaced `(§18)` with `(§20)` at line 85 (conservation-laws paragraph) and line 2849 (SCC classification paragraph). Residual classification is §20's concern; §18 type-graph stub retains its own purpose. |

### §19 — Residual Graph (Projection)

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | O4.3 per-residual training-emission constraint (extraction preserves overconstrained relation names) absent from §19.2 | add bullet to §19.2 or note in §35 | ACC/W | Added "Name preservation" bullet to §19.2 after "Envelope carriage": extracted residuals carry original relation names (CC3 / O4.3); training emission (§25) depends on per-residual identity; aggressive algebraic collapse that erases relation names is forbidden; open tracking in §35. Applied 2026-04-22 |
| H2 | §19.1 cost dimensions (precision, latency, memory, approximation class) diverge from O2.4 five-field struct (compute, approximation, condition, truncation, discretization) | reconcile with §14.2 / O2.4 | O/W | Tracked in chunk 12 (`12_cost_field_unification.md`) per §14 C1 resolution. Inline `*Open.*` note at §19.1 added. §34 Chunk 12 bullet added. Applied 2026-04-22 |
| H3 | Workflow extraction-policy API shape (`run.config.extraction_policy`) unlinked from §19.1 | cross-ref §19.1 to §24 workflow verbs | ACC/W | Added sentence to §19.1 after the Pareto-front / no-default-scalar paragraph: "Extraction policy is selected workflow-side (§24) via a config surface naming the axis preference." Applied 2026-04-22 |
| H4 | Rational-denominator saturation non-termination concern absent from §19.4 termination bound | caveat to §19.4 cross-linking §35 and §26.3 | ACC/W | Added "Rational-denominator saturation" bullet to §19.4 after "Termination bound": conjugate-multiplication rewrites on rational arithmetic (§26.3) can produce non-terminating saturation chains in pathological cases; rewrite-count cap catches these; open work on non-cap-based termination argument tracked in §35. Applied 2026-04-22 |
| H5 | Closure-policy extraction-time vs saturation-time distinction unstated | sentence in §19.4 or §8.7: Y1-Y6 policies are extraction-time | ACC/W | Added "Closure-policy timing" bullet to §19.4 after "Scheduling priority": Y1-Y6 (§8.7) operate at extraction time; during saturation candidates coexist as alternative e-class representations; selection happens when extractor commits. Applied 2026-04-22. Resolves §19 C1 |
| H6 | CC5 site-scoped structural predicates (pole L'Hopital, seam rewrites) tier placement in §19.4 unclear | clarify these fire in algebraic/unit-preserving tier | ACC/W | Added "Site-gated strict rewrites" bullet to §19.4 after "Closure-policy timing": Appendix C X1 (pole L'Hopital operator substitution) and X2 (identify-via-Layer-3 site records) fire in the algebraic/unit-preserving tier; strict, so no `approximate` authorization required. Applied 2026-04-22 |
| S1 | spec.md §12.3 "canonical evaluator" framing not in anti_spec.md | add low-priority anti_spec entry | ACC/W | Added row to anti_spec.md "Retired architectural framing" table: "canonical evaluator" framing for residual retired in favor of residual as user-facing projection via cost-vector-guided extraction (§19); subsumed by "residual as core semantic object" retirement, called out for legacy-doc readers. Applied 2026-04-22 |
| C1 | §19.4 conflates closure-policy co-membership merges (saturation-time) with policy selection (extraction-time) | rewrite §19.4 bullet to distinguish both phases | SKIP | covered by H5 |
| C2 | §19.1 cost dimensions vs O2.4 named-field struct | covered by H2 | SKIP | covered by H2 |

---

## Batch 5 (§20-§24) — pre-adjudicated

### §20 — SCC Decomposition and Component Classification

§20 is a 15-line skeleton. Most findings are Homeless. Two conflicts
need Riley's attention (cross-cutting section below).

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | SCC formation pipeline (acyclic single-node lowers as forward; coupled lowers through class-dispatched path) not stated | one-bullet pipeline description in §20 | ACC | legacy spec.md §12.2 settled |
| H2 | Hierarchical SCC decomposition (`deriv` feedback to same SCC triggers inner/outer split) not restated | subsection §20.x covering P/D/X/Y partition and E0952 diagnostic | ACC | settled per chunk 04 §1622-1623 and spec.md §12.5 |
| H3 | Inter-SCC gradient composition (nested `custom_root`, IFT chain over condensation DAG) silent | bullet describing topological order + IFT chaining | ACC | spec.md §12.5 settled |
| H4 | SCC decomposition runs post-binding; same model yields different SCCs under different bindings | bullet in §20 | ACC | spec.md §12.5 settled; load-bearing for §25 multi-experiment training |
| H5 | Opaque controller factor joins its SCC as non-symbolic atom | bullet cross-referencing §24 `bind_controller` | ACC | v2.1_in_progress:902-906 settled |
| H6 | Convergence-failure two-phase gradient regime for training-classified SCCs absent | bullet or forward-ref §25 | ACC | spec.md §12.5 settled |
| H7 | Knowledge-envelope `realization` field (explicit/implicit/opaque per quantity) from spec.md §12.6 unplaced | decide v2.1 fate | REVIEW | see cross-cutting |
| H8 | O4.3 per-residual name preservation not cross-linked from training-class description | cross-ref to §19.2 / §25 | ACC | CC3/O4.3 settled |
| H9 | Per-entity SCCs vmap on GPU vs cross-entity scalar solver lowering | bullet in §20 or §21 | ACC | v2.1_in_progress:1772 settled |
| H10 | Dynamic-shape matrix SCC deferral (chunk 05 §3.3) unmentioned | caveat in §20 or §21.4 | ACC | chunk 05 settled deferral |
| H11 | SCC-role predicates (e.g., "only Newton-root e-classes") as fact source for rewrite predicates | bullet cross-linking §17 O4.2 | ACC | chunk 04 §1622-1623 |
| S1 | spec.md §12.3 four-way *equation-count* classification coexists with §20 four-way *execution-class* classification | disambiguation sentence in §20 (two orthogonal axes; §19.3 covers equation-count) | ACC | coexistence is real; phrase collision is the whole problem |
| S2 | Legacy Linear/Polynomial/General-nonlinear solver classification (spec.md §12.5) not restated, not retired | decide fate | REVIEW | see cross-cutting |
| C1 | §20 four-way (static/dynamic/stochastic/training) vs §16.1 six-way (algebraic/stochastic/training/fixed-point/iterative-solve/stepper) | reconcile | REVIEW | see cross-cutting |
| C2 | §20 flat "stochastic" class vs §13.2 / Part II tiered stochastic SCCs (Tier A/B/C) | bullet noting decomposition into three tiers | ACC | aligns with §13.2 stochastic capability contracts |
| C3 | Linear/Polynomial/nonlinear collapses into §20 four-way | covered by S2 | SKIP | |

### §21 — Lowering

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Backend capability advertising / fallback policy not referenced from §21.2 SCC lowering | bullet or forward-ref §31 | ACC | chunk 06 §4.2 settled |
| H2 | AD ownership (compile-time symbolic / runtime backend-owned hybrid) not stated in §21.2 training bullet | one sentence | ACC | chunk 06 §4.3 lean settled |
| H3 | Single-backend-per-run and callable-in-same-backend leans not stated in §21 preamble or §21.2 | preamble sentence | ACC | chunk 06 §4.5/§4.6 |
| H4 | O4.3 per-residual name preservation at lowering missing from §21.2 training-SCC bullet | one sentence | ACC | CC3/O4.3 |
| H5 | Gradient checkpointing / TBPTT horizon as workflow-configurable backend-agnostic capability absent | bullet in §21.3 | ACC | spec.md §13.2 capability settled; JAX-specific spelling retired |
| H6 | Incremental saturation under event-time topology not forward-ref'd | forward-ref §35 O4.7 | ACC | open item needs pointer |
| H7 | Mesh discretization lowering path absent from §21 | forward-ref §35 P1 | ACC | open item needs pointer |
| H8 | Structural-subtype dispatch for assembled linear solves (Cholesky/Triangular/LU) unmentioned | bullet in §21.2 dynamic-SCC | ACC | chunk 05 §4 |
| S1 | Per-backend "mask may be optional on PyTorch" framing (v2.1_in_progress:1802-1804) | add anti_spec.md row | ACC | implicit retirement; one line for legacy readers |
| S2 | spec.md §13.1 two-way plan representation (forward-derived vs solver-block) subsumed by §21 four-way | low-priority anti_spec entry | ACC | |
| C1 | §21.4 `.myco` declares N-max, workflow overrides up to ceiling vs v2.1_in_progress:1796 "workflow must supply MAX_CAPACITY" | retire legacy framing in anti_spec.md | ACC | §21.4 is newer and CC1-compat |
| C2 | §21.1 module-level static/dynamic classification vs v2.1_in_progress per-collection bind-static-vs-event-time granularity | clarify or retire per-collection form | REVIEW | see cross-cutting |
| C3 | §21.3 rolling-buffer sized mechanically by lookback vs chunk 04 §5 "user retention policy" | clarifying sentence in §21.3 distinguishing ground-term storage from e-graph GC | ACC | likely reconcilable |

### §22 — Plan Inspection

§22 is a 26-line skeleton. No conflicts; all substantive findings are
Homeless.

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Plan-report contents enumeration (SCCs, symbolic resolutions, hierarchical decomps, numerical fallbacks, execution order, temporal state, resolution frontier) absent | bullet list in §22 | ACC | spec.md §14.5:3184-3202 settled; slot-era terms translate to `bind_controller` |
| H2 | Per-quantity envelope query surface (`artifact.plan.knowledge(path)` → bounds, obligations, resolver_sets, provenance) not described | subsection or forward-ref §17 | ACC | §17 names `mycoc explain` as envelope reader |
| H3 | Hypothetical plan re-evaluation (`with_assumption`) from spec.md §14.5 has no landing site | decide ship or retire | REVIEW | see cross-cutting |
| H4 | Phase 2 Q3 residual/e-graph navigation (round-trip from residual node to e-class) not cross-referenced | cross-ref §35 Phase 2 Q3 | ACC | legitimately §22's scope |
| H5 | Dual-mode `condition_of` rendering labels (algorithmic vs problem) in `mycoc explain` not stated | bullet | ACC | chunk 04 O2.4 settled |
| H6 | Execution-order inspection | part of H1 list | SKIP | |
| H7 | Numerical-fallback reporting | part of H1 list | SKIP | |
| H8 | Obligation-key `replaces` visibility in plan report | part of H1 list | ACC | v2.1_in_progress:1300 |
| H9 | Plan-is-backend-agnostic cross-ref to Part V | optional sentence | ACC | light touch |
| H10 | Visualization surface (graph IR, Graphviz/D2/Mermaid/Cytoscape renderers) fate in v2.1 | decide scope | REVIEW | see cross-cutting |
| H11 | Workflow config for inspection policy (`run.config.loss_estimation` strict/permissive) not cross-linked | cross-ref §24 | ACC | chunk 04 settled |
| S1 | Legacy "residual graph as inspectable artifact" framing | subsumed by existing retirement | SKIP | already in anti_spec.md |
| S2 | Legacy `myco plan --dot` CLI invocation | implicit via Appendix B wholesale retirement | SKIP | low urgency |

### §23 — The `.myco` ↔ Python Boundary

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Python value providers (`myco.lognormal`, `myco.from_csv`) vs `.myco` `Distribution<U>` contracts not distinguished | preamble bullet | ACC | chunk 09 locked |
| H2 | Mode B per-instance contract-type selection is `.myco`-side constraint, not Python-side | bullet under dumb-data paragraph | ACC | chunk 09 locked |
| H3 | Workflow-only capabilities (RNG, checkpoint, restart, wall-clock) not enumerated in §23 preamble | bullet list | ACC | chunk 09 locked |
| H4 | `input_contract` visibility invariant (expanding visibility requires expanding contract) unstated | sentence in §24.1 | ACC | cross-section; lands in §24 |
| H5 | `bind_controller` opaque-factor SCC participation unstated | sentence in §24.2 or §23.2 | ACC | covered jointly with §20 H5; lands in §24.2 |
| H6 | Cross-backend callable interop caveat missing from §23.3 "trained weights plus plain contract" | caveat + forward-ref §35 Tier 2 | ACC | open item needs pointer |
| H7 | Refinement-type bounds missing from node-catalog metadata list in §23 preamble | add "refinement bounds (where declared)" to catalog field list | ACC | v2.1_in_progress training-emission lock |
| H8 | Mode B dispatcher idiom forward-reference | deferred | SKIP | depends on chunk 11 sum-type lock |
| S1 | `param` keyword superseded | already anti_spec'd | SKIP | |
| S2 | `bind_slot` / `bind_slot_metadata` superseded | already anti_spec'd | SKIP | |
| S3 | `model.params()` / `model.universals()` typed Python accessors superseded by generic node-catalog | add anti_spec.md row | ACC | low-urgency but tidy |
| S4 | Four-verb `assume`/`observe`/`learn`/`bind` grouping superseded by eight-verb taxonomy | structurally void | SKIP | §24 never commits to four-way form |
| C1 | §23 preamble names Python verbs as "`bind`, `observe`, `run`" which re-implies retired four-verb form | rewrite to name eight-verb taxonomy (or paraphrase as "`assume_*`, `learn_*`, `bind_*`, `observe`") | ACC | |
| C2 | §23.3 "callable's declared input contract" suggests contract is callable attribute; §24.1 makes it a bind-verb argument | tighten §23.3 wording to "bound under an `input_contract`" | ACC | reconcilable |
| C3 | §23.4 does not distinguish tier-1 `.myco` contract-obligation error from tier-2 bind-time mismatch | add distinguishing sentence | ACC | implicit in §7 but operational at boundary |

### §24 — Eight Workflow Verbs

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | `run.config.extraction_policy` and `run.config.loss_estimation` absent from §24.5 representative fields | add fields or cross-ref §19 / §14 | O/W | gated on chunk 12 cost-field unification; landing site depends on final field names |
| H2 | Backend version pinning missing from §24.5 | bullet or forward-ref §31 | ACC | chunk 06 §4.7 settled |
| H3 | `load` / `spawn` / `run` / `checkpoint` orchestration verbs ambiguous against §24's "complete workflow-composition surface" claim | preamble note or §24.4 line distinguishing binding verbs from orchestration verbs | ACC | chunk 09 locked |
| H4 | Python value providers on `assume_*` not distinguished from `.myco` `Distribution<U>` | sentence on `assume_*` bullets | ACC | chunk 09 locked; resolves jointly with §23 H1 |
| H5 | Bind-time type-checking invariant (shape / dtype / units at workflow composition, errors at bind time) not stated | preamble sentence | ACC | chunk 09 locked |
| H6 | `assume_prior` cross-reference | §24.4 already defers | SKIP | |
| H7 | Mode B per-instance dispatch is `.myco`-side | covered by §23 H2 | SKIP | |
| H8 | Cross-backend callable interop open, §24.2 "cross-run persistence" reads unconditional | caveat | ACC | covered jointly with §23 H6 |
| H9 | Eight verbs reduce to narrower substrate mechanisms (§17.1 merge sources, envelope facts) | preamble cross-ref to §17.1 source 2 | ACC | helps readers coming from §17 |
| S1 | `slot` / `bind_slot` / `learn_slot` / `bind_slot_metadata` superseded | already anti_spec'd | SKIP | |
| S2 | `[*]` wildcard slot inputs superseded | already anti_spec'd | SKIP | |
| S3 | Transparent-heuristic ABI superseded | already anti_spec'd | SKIP | |
| S4 | "v2.0 had slot" versioning prose | already anti_spec'd as retirement narrative | SKIP | |
| S5 | Four-verb `assume`/`observe`/`learn`/`bind` grouping | structurally void | SKIP | |
| S6 | Trained-slot serialization and rebinding | subsumed by §24.2 persistence; slot retirement covers | SKIP | |
| C1 | §24.1 `bind_controller(path, fn, input_contract, output_contract)` four-arg vs three-arg in all corpus (v2.1_in_progress:867, open_questions:347, anti_spec.md:13) | decide whether `output_contract` is signature argument or inferred from callable return type | REVIEW | see cross-cutting |
| C2 | §24.5 `run.config.dt` "referenced via `assume_constant`" excludes `assume_series` path for variable time-stepping | update bullet to include both paths | ACC | v2.1_in_progress:325-326 |

---

## Cross-cutting items needing Riley's attention

- **Visibility abolishment** (§2 H3/H4/H6/C1 and §6 H3). RESOLVED — silently removed from §2. `pub` keyword never appears. Noted in adjudication and applied.
- **π / e as stdlib constants** (§4 H1). RESOLVED in §4 via path (a): stdlib ships default bindings. Workflow-layer mechanism detail flagged for the workflow-verb design pass.
- **Named-type equality/comparison placement** (§3 H2 + §5 H6 + §8 H1). PARTIAL — landed in §3.3 as the canonical home. §5 H6 and §8 H1 should cross-link rather than restate.
- **No-initial-in-contract rule** (§7 H2 + §9 H5). Same rule, two sections. §9 is canonical home; §7 cross-links. ACCED in batch 2 adjudication.
- **§2 H3** (initial/temporal illegal at module scope). RESOLVED by §9 H6 ACC: temporal is legal on loci, so the module-scope-only prohibition is the wrong framing. Drop §2 H3.
- **Locus-scoped temporal** (§9 H6 resolves §2 H3 and §11 H10). One landing point in §9.x covers all three; §11 cross-links. Already flagged in §9 H6 and §11 H10 status cells.
- **Cost-field struct unification** (§14 C1 + §15 H4/H5 + §19 H2/C2). O/W 2026-04-22. Tracked in chunk 12 (`12_cost_field_unification.md`) with problem statement, three options (unified O2.4 struct; two intrinsics `loss_of` + `cost_of`; rename `loss_of` → `objective_terms`), five load-bearing questions, and full cross-ref list. Inline `*Open.*` notes at §14.2 and §19.1 plus §34 Chunk 12 bullet consolidate the tracking state. ACC items in batches 3 and 4 that depend on the canonical struct (§14 C1, §15 H4/H5, §19 H2/C2) stay pending until the chunk is resolved.
- **Envelope-narrowing promotion** (§15 H2 + §17 H1). Same mechanism surfaces in two sections; land in §15 (the home for the lossiness narrative) and have §17.6 cross-link. Both ACCED.
- **CC5 site-scoped predicates** (§17 H4 + §19 H6). RESOLVED 2026-04-22 via X1/X2 split. Pole L'Hopital (X1) fires at any mesh node coinciding with a declared locus pole; rewrite is a removable-singularity operator substitution, not a value-equality merge (so "unconditional vs mesh-coincident" question does not apply — X1 is operator-shape-local by definition). Identify (X2) fires only where a geometry-body `identify` declaration has installed a Layer-3 site record; X2 consults the record and emits a Layer-1 merge tagged with the site's identity. Cross-geometry pollution impossible by construction. §19 H6 tier-placement sentence still pending in Phase 1 end cleanup but the load-bearing design question is closed.

Batch 5 (§20-§24) raised the following new cross-cutting items:

- **SCC class taxonomy reconciliation** (§20 C1). §16.1:2911-2912 names six classes (`algebraic / stochastic / training / fixed-point / iterative-solve / stepper`); §20:3510-3513 names four (`static / dynamic / stochastic / training`). Both claim to be the §20 classification. §16.1's list reads as solver-strategy dispatch; §20's reads as execution-role. Two plausible resolutions: (a) keep §20's four-way execution-role taxonomy as the canonical SCC class, push solver-strategy sub-dispatch to §21 lowering; (b) adopt §16.1's six-way, splitting §20's "static" into algebraic+fixed-point and "dynamic" into iterative-solve+stepper. Option (a) is simpler and separates concerns cleanly. Needs Riley's call before §20 writeback can land.
- **`bind_controller` signature arity** (§24 C1). §24.1 lists four arguments (`path, fn, input_contract, output_contract`). All corpus sources (v2.1_in_progress:867, open_questions_deprecated:347, anti_spec.md:13) use three (`path, fn, input_contract`). The output contract is semantically real (drives admissibility and gradient flow per §24.1) but its elevation to a signature slot is novel to §24.1. Two options: (a) output contract is inferred from `fn`'s declared return type and §24.1 should reword to describe it as a property, not an argument; (b) output contract is a separate argument and anti_spec.md:13 plus v2.1_in_progress:867 need to update to the four-argument form. Needs Riley's call.
- **Linear/Polynomial/General-nonlinear solver classification fate** (§20 S2). Legacy spec.md §12.5 names these as solver dispatch for square SCCs; spec_new.md neither restates nor retires. Chunk 05 notes "SCC solver dispatch exists conceptually but Cholesky-for-PSD vs LU-for-general is not formalized." Options: (a) lift as a §21 lowering sub-classification orthogonal to §20 classes, (b) retire in anti_spec.md and let structural-subtype dispatch on matrix contracts (chunk 05 §4) carry the weight. Leaning (b) given chunk 05's structural-contract direction.
- **Knowledge-envelope `realization` field fate** (§20 H7). Legacy spec.md §12.6 describes per-quantity `realization: explicit(expr) | implicit(residual_block) | opaque(provider)` as inspectable metadata orthogonal to SCC class. Not restated in spec_new.md, not retired in anti_spec.md. Either land as a §17 / §19 envelope field (plausibly useful for `mycoc explain` output — see §22 H2) or retire. Needs Riley's call on v2.1 scope.
- **Hypothetical plan re-evaluation (`with_assumption`) fate** (§22 H3). Legacy spec.md §14.5:3217-3229 describes `plan_b = artifact.plan.with_assumption(...)` for experimental-design use cases. Carries "settled" status in legacy text but has no v2.1 landing and no retirement. Either ship as part of §22 plan inspection (the feature is small once the inspection surface is written) or retire. Needs Riley's call.
- **Visualization in v2.1** (§22 H10). Legacy Appendix B.5/B.6 describes graph IR + Graphviz/D2/Mermaid/Cytoscape renderers. Appendix B is flagged for wholesale retirement in anti_spec.md, so the specific renderer list is implicitly gone, but spec_new.md does not state whether graphical plan inspection ships in v2.1 at all or is Part VII deferred. §22 should explicitly name the answer.
- **Per-collection bind-static vs module-wide dynamic classification** (§21 C2). §21.1 classifies whole modules as dynamic if any event exists. v2.1_in_progress:1660-1666 allows per-collection bind-static inference: `some`-sized collections with no targeting events can skip mask-update machinery even in otherwise-dynamic modules. Two different granularities of the same concept. Needs Riley's call: either §21 adopts the per-collection form (and §21.4 gains a "bind-static skips mask emission" bullet) or the per-collection form retires explicitly.

---

## Batch 6 (§25-§29) — pre-adjudicated

### §25 — Training Emission

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Stdlib loss helpers `soft_penalty(weights)` and `augmented_lagrangian(weights, mu, lambda_init, mu_schedule)` unnamed in §25 | name both helpers or forward-ref Part IV | ACC | v2.1_in_progress:1082-1096 locked |
| H2 | Dual-state API shapes for `augmented_lagrangian` (PyTorch-mutable vs JAX-pure) | workflow-side; §25 cross-ref | ACC | v2.1_in_progress:1089-1093 |
| H3 | `model.residuals` workflow surface (`Residual` object shape) unnamed | name or cross-ref §31 | ACC | v2.1_in_progress:1076-1080 |
| H4 | Refinement-type bounds as workflow-visible projection-target metadata | sentence in §25 | ACC | v2.1_in_progress:1057-1060 |
| H5 | Negative list ("not auto-emitted": projection flavor, loss aggregation, dual updates, annealing) | add 4-item bullet | ACC | v2.1_in_progress:1104-1106 |
| H6 | Pre-training against hand-coded heuristic | workflow recipe; no §25 mention needed | SKIP | v2.1_in_progress:1098-1102 |
| H7 | Training-mode consistency-loss substitution rule (`lhs = rhs` → `(lhs - rhs)²`) | one-line statement in §25 | ACC | chunk 04 O1 locked |
| H8 | Opaque-callable gradient-flow in training-SCC | cross-ref §24.2 | ACC | chunk 06 §4.5 tracked |
| H9 | §25 aggregation presumes `loss_of` named fields (cost-field unification dependency) | `*Open.*` stanza on chunk 12 | O/W | gated on chunk 12 |
| H10 | Long-rollout gradient regime (checkpointing, truncated BPTT) | forward-ref workflow section or retire | REVIEW | see cross-cutting |
| H11 | Study-level training / multi-experiment joint loss | cross-ref or note out-of-scope | ACC | workflow-side per chunk 09 |
| H12 | PINN physics-residual-factor pattern (`learn_trajectory` + temporal equation) | name in §25 or cross-ref §24 | ACC | spec.md §16.4 settled |
| S1 | Compiler auto-emitted admissibility projections | already anti_spec'd | SKIP | |
| S2 | Two-phase gradient regime with convergence-penalty non-convergence fallback | decide retain vs retire | REVIEW | see cross-cutting |
| S3 | Compiler-emitted fixed loss-function menu (`obs_loss`/`consistency_loss`/etc.) | add anti_spec.md row | ACC | superseded by `loss_of` |
| S4 | Per-experiment `set_weight` Python API | workflow-side; flag-only | SKIP | study-weighting open per chunk 09 |
| C1 | Warm-start wording conflates between-timesteps (a) / `assume_constant` init (b) / `learn_constant` prior (c) | rewrite to enumerate or pick load-bearing | ACC | enumerate all three |
| C2 | Constraint enforcement collapses training-time penalty vs runtime projection into single "runtime projection otherwise" | split into three discharge regimes | ACC | compile-time / training-penalty / runtime-projection |

### §26 — Numeric Types

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Fixed-width signed/unsigned integer widths (`Int{8,16,32,64}`, `UInt{8,16,32,64}`) vs single `Integer` row | split or state intent | ACC | chunk 04 §8 granular |
| H2 | `BigInt`/`BigDecimal` extension-module status not stated in §26.1 | add note | ACC | chunk 04 settled item 14 |
| H3 | Compiler-internal e-graph use of `Rational` for constant folding | §26.3 bullet naming internal use | ACC | origin of saturation concern |
| H4 | `Complex` total-ordering resolution invalidates one §35 open sub-item | tidy §35 open-list wording | ACC | follow-up §35 edit |
| H5 | `§31.1` `host` fallback mode as opt-in escape from Rational-GPU hard error | cross-ref §31.1 | ACC | one-line addition |
| H6 | AD-ownership citation (chunk 06 B6) bookkeeping | optional parenthetical | SKIP | low priority |
| H7 | Scalar-level precision downcast (bare `convert`) vs tensor-level (requires `approximate`) | covered by C1 | SKIP | subsumed |
| S1 | User-facing `Dual<T>` numeric representation | already anti_spec'd | SKIP | |
| S2 | `Dual<T>` in chunk 04 hierarchy | covered by S1 | SKIP | |
| S3 | spec.md §4.3 one-parameter `Scalar<U>` | already anti_spec'd wholesale | SKIP | |
| S4 | Dimensionless-ratio literal carve-out | already anti_spec'd | SKIP | |
| C1 | §26.2 authorizes `Float64 -> Float32` via bare `convert`; chunks 04/05 require `approximate ... tolerance_class` | decide single authorizing surface | REVIEW | see cross-cutting |
| C2 | §26.1 single `Integer` row collapses fixed-width hierarchy | split into `Int{8..64}`/`UInt{8..64}` stdlib + `BigInt` extension | ACC | covered by H1 resolution |

### §27 — Distribution Families (Z-group)

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Per-family capability table footnotes (StudentT ν > 2 `SmoothTransformable`, Cauchy/HalfCauchy/Lévy infinite-variance exclusion) | add footnote block or per-row condition annotation | ACC | chunk 04 §11 settled |
| H2 | Discrete Tier 1 roster (Binomial/Geometric drop) | covered by C1 | SKIP | subsumed |
| H3 | Multivariate gating: only MVN needs B5; Dirichlet/Multinomial are vector-valued | refine §27.1 B5-gate footnote | ACC | chunk 04 §11 settled |
| H4 | B1 opaque log_pdf design items (α-stable driving case, 3 sub-questions) | extend §33 B1 entry | ACC | §33 gap, not §27 gap |
| H5 | NormalInverseGamma + Gamma-Gamma conjugates promoted in chunk 04 but absent from §27.3 | covered by C2 | SKIP | subsumed |
| H6 | B4 copula-coupling suppresses `SumSelfClosed`/`ProductSelfClosed`/`AffineSelfClosed` firing | add sentence to §27 or §13.2 Tier A | ACC | chunk 04 §8 locked |
| H7 | v2.2+ deferrals (Matrix-Normal, generalized Wishart, Sklar-copula decomposition) | add §27.5 Tier 2 sentence | ACC | chunk 04 §9 explicit |
| H8 | StudentT `Reparam` (RSVI implicit reparameterization) marking elision | covered by C2 | SKIP | subsumed |
| S1 | `Distribution<U>` two-optional-methods shape (pre-chunk-04) | add anti_spec.md row | ACC | superseded by 3-required + sub-contracts |
| S2 | v2.1_in_progress stdlib distribution list (pre-promotion) | supersession is legitimate; Binomial/Geometric drop is Conflict | SKIP | covered by C1 |
| S3 | MVN "deferred pending vector/matrix story" | already anti_spec'd | SKIP | |
| S4 | VI reparameterization as optional method | add anti_spec.md row | ACC | promoted to `ReparameterizedSampleable` sub-contract |
| C1 | §27.1 discrete roster (5) drops Binomial+Geometric vs chunk 04 (7) / v2.1_in_progress (6); Binomial referenced downstream (§27.3 Beta-Binomial, §13.2 shared-p) | add Binomial row; decide Geometric fate | ACC | re-add Binomial; Geometric drops with anti_spec.md entry |
| C2 | §27.1 capability rows elide chunk 04 tags (StudentT `AffineSC`, LogNormal `R`/`Sc`, Poisson `S`, etc.) | exhaustive list OR state `A ⇒ Sc` implication | ACC | exhaustively list; reconciliation target is chunk 04 |
| C3 | §27.3 "catalog is closed" contradicted by chunk 04 promoted NormalInverseGamma + Gamma-Gamma | covered by cross-cutting below | REVIEW | see cross-cutting |
| C4 | §27 `pdf` as required method (vs v2.1_in_progress + chunk 04 contract shape) | mark default-derived OR state why required | ACC | default-derived from `log_pdf` |
| C5 | §27 `log_pdf` return type `Scalar<unitless>` vs `Scalar<dimensionless>` rest of spec | rename to `dimensionless` | ACC | spelling consistency |

### §28 — Kernels

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Compact-support / characteristic-length declaration surface (contract vs refinement vs workflow) | three-option bullet in §28.3 or §35 | REVIEW | see cross-cutting |
| H2 | `condition_of` cross-reference for ill-conditioned Gram matrices | one-line §28.3 or preamble cross-ref to §14 + §30 | ACC | low urgency |
| H3 | K2 separability declared-vs-inferred open-question cluster forward-ref | one-line §28.1 or §28.3 pointer | ACC | chunk 04 sub-q 5 |
| H4 | §28 has no pointer from surface to §32 K rewrite cluster or §15.1 `approximate` blocks | final paragraph cross-ref | ACC | reader-orientation |
| H5 | Fuzzy vs strict equality for kernel equivalences | post-substrate-lock tracker | SKIP | implied open |
| H6 | K3 low-rank not acknowledged in §28.3 even as deferred | add K3 to §28.3 (third deferred concern) | ACC | chunk 04 §8 |
| H7 | Riley property-note closed | no action | SKIP | |
| H8 | K2 separability → Kronecker-structured Gram matrices (chunk 05 gap) | §28.2 sentence OR §30 Kronecker deferred-refinement note | ACC | adjacent chunk 05 gap |
| S1 | v2.1_in_progress "NEW: Coupling & Kernels" legacy chunk | add anti_spec.md row | ACC | stale in legacy docs |
| S2 | spec.md coupling-as-kernel references | covered by existing spec.md wholesale retirement | SKIP | |
| S3 | deprecated-open-questions coupling cluster | globally superseded | SKIP | |
| C1 | `Stationary` / `Isotropic` derivation mechanism underspecified | §6 / chunk 03 flag; §28.1 consistent with derivation principle | SKIP | not a §28 conflict |
| C2 | §28.3 defers "to chunk 03" but sparsity actually defers to chunk 05, integration to chunk 03 resume post chunk 04, low-rank to chunk 03 / v2.2 | tighten §28.3 venue naming | ACC | internal precision |
| C3 | §28 preamble "pending e-graph substrate lock" stale (chunk 04 locked 2026-04-20) | rewrite preamble 2nd paragraph | ACC | separate chunk 03 resume from substrate lock |

### §29 — Units Library

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Ecophys spore content inventory (water potential, gas-exchange, PPFD, LAI, soil water) | optional example list | SKIP | dev_notes carries detail |
| H2 | Unit-system mechanics live in §5, not §29 — forward-ref not explicit | one-sentence summary pointer | ACC | matches §30 pattern |
| H3 | Non-SI systems (CGS, imperial) out-of-scope implicit | add to out-of-scope enumeration | ACC | project-vs-language separation |
| H4 | Stdlib math constants (π, e) — §4 commits bindings but §29 has no pointer | one-line disambiguation | ACC | low urgency |
| H5 | Conversion-graph cost-model cross-reference to §0.1 / §35 | cross-reference | ACC | unit-conversion edges sourced from §29 |
| H6 | Same theme as H5 from chunk 07 side | covered by H5 | SKIP | subsumed |
| H7 | `Scalar<U, T = Float64>` two-parameter form interaction with core units | no action | SKIP | §26 + §5 carry forms separately |
| S1 | spec.md `units::si` compound-unit catalog (`J_mol_K`, `mol_m2_s`, etc.) | add anti_spec.md row for scope narrowing | ACC | low urgency tidy |
| S2 | Stdlib physical constants (`R`, Stefan-Boltzmann) | already anti_spec'd | SKIP | |
| S3 | mock_sperry `use physics::constants` import | covered by mock flag | SKIP | |
| S4 | mock_sperry inline-valued universal `R = 8.314` | already anti_spec'd | SKIP | |
| C1 | Derived-unit ship-list scope narrowing (compound units now in ecophys spore, mocks still import from `units::si`) | resolves on mock rewrite | SKIP | not §29 action |
| C2 | "Standard affine conversions between equivalent SI-derived spellings" wording imprecise — affine conversions are offset-based (Celsius↔Kelvin), not spelling aliases | reword to "between SI-derived units with offset" | ACC | precision |
| C3 | π/e + R/Stefan-Boltzmann split coherent but §29 does not mediate "where does the units library ship constants?" | one-line disambiguation | ACC | covered by H4 |

---

Batch 6 (§25-§29) raised the following new cross-cutting items:

- **Precision-downcast authorizing surface** (§26 C1). §26.2 authorizes `Float64 -> Float32` via bare `convert` with automatic tolerance envelope emission. Chunk 04 §7 places this rewrite in the 2×3 lossy-tolerance × bidirectional cell, authorizing it via `approximate ... tolerance_class precision_downcast`. Chunk 05 §3.5 states `convert` refuses precision downcast on matrices; `approximate` handles it. Three options: (a) bare `convert` emits tolerance envelope automatically — §26.2 framing, consistent with §15.3; (b) `approximate ... tolerance_class:` required — chunk 04/05 framing, consistent with 2×3 matrix; (c) scalar allows bare `convert`, tensor requires `approximate`, distinction stated explicitly. Leaning (a) for uniform surface unless there is a load-bearing reason for scalar/tensor asymmetry. Needs Riley's call before §26.2 writeback.
- **Distribution catalog-closure claim vs promoted conjugates** (§27 C3). §27.3 states "the catalog is closed for this release." Chunk 04 §11 explicitly promotes Normal-InverseGamma, NormalInverseGamma (joint μ, σ²), and Gamma-Gamma as shipping in v2.1. §27.3 ships six; chunk 04 promotes eight. Either §27.3 is stale (add two rows) or chunk 04 is stale on two promotions (record in anti_spec.md). The joint NormalInverseGamma case carries a flagged "verify rewrite-pattern language handles joint priors" check; if that check is unresolved, NormalInverseGamma is a legitimate gate. Leaning: ship Gamma-Gamma now; gate NormalInverseGamma on the rewrite-pattern-language check and keep it out of §27.3 with a §35 pointer. Needs Riley's call.
- **Compact-support / characteristic-length declaration surface** (§28 H1). Wendland and other compact-support kernels have no declarative home. Three candidate surfaces: (a) capability contract `CompactSupport(radius)` on the kernel function, (b) refinement type on the output value, (c) workflow-layer annotation. Chunk 03 deferred to post-substrate-lock (now lock is in). Leaning (a): compact support is a claim about the function, and the refinement-on-output surface doesn't fit (the output is just a scalar; support is a property of the domain). Workflow-layer moves a statically-checkable fact to runtime. Needs Riley's call; affects §28.1 contracts list, §32 K1 rewrite firing condition, and the chunk 03 resume agenda.
- **§28 preamble staleness** (§28 C3). "pending e-graph substrate lock" line predates chunk 04's 2026-04-20 lock. Should rewrite to distinguish settled substrate from chunk-03-specific open threads. Scope is small (one paragraph). No design decision; drop-in edit during §28 writeback.
- **Long-rollout gradient-regime disposition** (§25 H10). Legacy spec.md §14.7 describes gradient checkpointing and truncated BPTT as rollout-stability machinery. spec_new.md does not commit or retire either. Options: (a) land as §24 or §31 workflow-config surface (training-dynamics knob), (b) retire in anti_spec.md as workflow-recipe-only (user handles via backend configuration). Leaning (a) — checkpointing is load-bearing for long rollouts and the workflow config surface already exists for similar concerns. Needs Riley's call.
- **Two-phase solver non-convergence regime fate** (§25 S2). Legacy spec.md §12.5 names a solver-exceed-max-iterations regime with convergence-penalty injection plus gradient detachment via implicit-function-theorem detachment. spec_new.md neither restates nor retires. Options: (a) retain and add a §20/§21 or §25 subsection naming the regime, (b) retire as a backend-specific concern (backend's optimizer handles non-convergence). Leaning (b) — this is solver-dispatch policy and §31 backend framing owns it. Needs Riley's call.

---

## Batch 7 (§30-§34) — pre-adjudicated

### §30 — Matrix and Tensor Primitives

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Opaque-primitive e-graph semantics unspecified (leaf node, capability contracts as class-level metadata, rewrite rules consume those facts) | add paragraph to §30 (or §15) stating the opaque-node handling rule | ACC | stable design implication of the capability-contract framing |
| H2 | `inverse(A) * b -> solve(A, b)` rewrite unclassified against §17 taxonomy | classify as D-group default-on algebraic rewrite; forward-ref §17 | ACC | parallel to scalar inverse rewrites |
| H3 | `det(Matrix<U, n, n>) -> Scalar<U^n>` unit signature not stated; `trace` absent entirely | note return-type units gated on chunk 05 §3.2; add `trace` with "(unit TBD)" annotation | O/W | `trace` needed by Wishart/InverseWishart log_pdf |
| H4 | `condest` missing from §30 despite chunk 04 O2.4 lock | add `condest(A) -> Scalar<dimensionless>`; cite §14.4 Level III `condition_of` consumer | ACC | settled commitment, not open |
| H5 | `norm`, `rank`, `least_squares`, constructor family (`zeros`/`ones`/`identity`/`diag`/`stack`) absent | add forward-looking note as tentative pending chunk 05 | ACC | scopes the stub accurately |
| H6 | Structural-subtype stripping rules (`transpose(Symmetric) -> Symmetric`, `inverse(PosDef) -> PosDef`, `A·Aᵀ -> PosSemiDef`) unenumerated in §3.9 or §30 | note enumeration deferred to chunk 05 §3.4 | O/W | matrix analogue of named-type U1-U3 stripping |
| H7 | §3.9 sparse deferral conflates storage-format (chunk 06 backend) with pattern-as-type-vs-envelope (chunk 05 type system) | split the two deferral notes | ACC | pattern-vs-envelope affects §30's dispatch rule |
| H8 | Dynamic matrix shapes: open item in deprecated file, silent in §30; chunk 05 §3.8 defers to v2.2 | add scope note: v2.1 tensor shapes compile-time known | ACC | chunk 05 §3.8 settled |
| H9 | Scalar reconciliation lean (`Scalar<U> := Tensor<U, ()>`) in chunk 05 §3.1 not propagated to §3.8 | record lean as direction note or flag explicitly as blocker | ACC | chunk 05 lean is (i) |
| H10 | Envelope flavors for matrix quantities open (four flavors in chunk 05 §3.3) | cross-ref §13.6 (cholesky positive-diagonal fact) as concrete example | O/W | chunk 05 §3.3 open |
| S1 | Legacy spec.md §12 JAX-centric solver emission | wholesale-supersede covers | SKIP | anti_spec.md covers |
| S2 | Legacy spec.md §12.6 incidence-matrix as compiler-internal | covered by wholesale supersede | SKIP | |
| S3 | deprecated MVN "deferred pending vector/matrix story" | already covered by §13.6 | SKIP | |
| C1 | `PositiveDefinite` (§3.9) / `Matrix<_, PositiveDefinite>` (§30) / `PosDef<U, n>` (chunk 05) three-way naming + `Matrix<_, prop>` vs `Matrix<U, m, n>` argument-count mismatch | pick one name; pick one type-constructor form | REVIEW | see cross-cutting |
| C2 | `inverse(A)` (§30) vs `inv(A)` (chunk 05) naming; §30 omits return-type `Matrix<U^(-1), n, n>` | pick one name; add return-type unit | REVIEW | see cross-cutting |
| C3 | `eigen` "square A" (§30 general with `Complex` deferral) vs chunk 05 `eigen(Symmetric<U, n>)` only | restrict v2.1 to `Symmetric` input; general-square as "(pending §26.1 Complex lock)" | ACC | chunk 05 is the tighter scope |

### §31 — Backend Trait Surface

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | AD-ownership fork (Option A Myco-owned / B backend-delegate / C hybrid) pros/cons not in §31 or §32 | add §32.3 "AD Ownership Fork" summarizing options + Option C lean + consistency obligation | O/W | chunk 06 §4.3 design venue |
| H2 | Minimum backend trait API boundary (which ops required vs capability-advertised — e.g. is dense Cholesky required?) not named | add §32 open item citing chunk 06 §4.1 Q2 | O/W | determines how fat the trait is |
| H3 | PPL return-side message schema (sample values with provenance, gradient estimates, MCMC traces, ESS/R-hat/divergence) absent from §31.2 | expand §31.2 to enumerate return-side schema from chunk 06 §4.4 | ACC | locked design, not open |
| H4 | PPL protocol open questions: whole-model vs per-factor visibility; sample re-entry as new envelope facts | add to §32 (record sample-re-entry as design lean) | O/W | chunk 06 §4.4 |
| H5 | Opaque-callable Q1-Q4 (which backend runs callable, training-SCC gradient flow, Matrix/Tensor I/O backend, cross-run portability) | enumerate in §32; record Q1/Q3 lean same-backend, Q2/Q4 genuinely open | O/W | chunk 06 §4.5 |
| H6 | Framework-specific adapters (NumPyro-style, Pyro-style, Turing.jl, Stan-style) absent from §31.2 | one-sentence acknowledgment in §31.2 | ACC | communicates trait-symmetric mechanism |
| H7 | `supports_dynamic_shapes` capability absent from §31.1 examples; JAX/PyTorch dynamic-shape difference unaddressed | add `supports_dynamic_shapes` to §31.1 capability examples | ACC | needed for §21 alive-mask lowering |
| H8 | Chunk 06 §8 Q2/Q3/Q4/Q7 partial coverage (Q2 minimum API, Q3 fallback default, Q4 PPL form, Q7 versioning) | covered by H1-H5, H7, and versioning verification | SKIP | subsumed |
| S1 | spec.md §13.2 / §13.3 JAX-emitter-primary framing | already anti_spec'd | SKIP | |
| S2 | spec.md §13.3 slot-based backend interface method list | already anti_spec'd via slot retirement | SKIP | |
| S3 | v2.1_in_progress PyTorch-primary / JAX-secondary ordering | already anti_spec'd | SKIP | |
| S4 | spec.md §14.7 `jax.checkpoint` specifics | covered by JAX-emitter retirement | SKIP | |
| C1 | §31 summary pre-commits autodiff as minimum-API responsibility while §32 lists AD ownership as open (implicitly assumes Options B or C) | qualify summary: autodiff joins minimum API under hybrid/full-delegation | REVIEW | see cross-cutting (couples with §32 C2) |
| C2 | §31.3 "threads gradients back through Python" + §24.2 "via backend's AD facility (§31)" form a loop without resolving the mechanism | expand §31.3 to state §31 owns backend-side mechanism vs §24.2 model-side gradient semantics | ACC | direction of authority needs stating |
| C3 | §31.1 "Conservative default" for `error` fallback vs chunk 06 §4.2 "Open question: default policy" | formally lock `error` as default OR move to §32 as explicit open | REVIEW | see cross-cutting |

### §32 — Open Backend Items

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | AD ownership A/B/C details not enumerated | add §32.3 | O/W | covered by §31 H1 cross-cutting |
| H2 | PPL message-schema specifics not enumerated | add §32.4 (or expand §31.2) | O/W | dedup with §31 H3/H4 |
| H3 | `bind_controller` opaque-fn fallback (non-differentiable callable in training-time SCC) residue | add §32.5 bullet: stop-gradient / compile error / workflow opt-in | O/W | lean same-backend already in §32.1; fallback is the residue |
| H4 | Backend versioning status — per dev_notes §31.4 complete | verify §31.4 contains policy; if so chunk 06 Q7 resolved | ACC | cleanup-only per dev_notes |
| H5 | Capability advertising / fallback policy — per dev_notes §31.1 complete | verify §31.1 writing complete; if so chunk 06 Q2/Q3 resolved | ACC | cleanup-only; default-mode lock separately under §31 C3 |
| H6 | Cross-backend callable interop (cross-run portability) not surfaced in §32 body | add forward-ref in §32.1 or §32 preamble to §35 | ACC | distinct from intra-run single-backend |
| H7 | Minimum backend trait API surface not named | add §32.6 bullet — dense Cholesky required vs advertised | O/W | dedup with §31 H2 |
| S1 | spec.md §13.2 JAX-primary + spec.md §13.3 slot-based interface | already anti_spec'd | SKIP | |
| S2 | v2.1_in_progress backend-targets ranking | already anti_spec'd | SKIP | |
| S3 | spec.md §13.3 slot-based backend interface | covered by slot retirement | SKIP | |
| S4 | chunk 06 §2 "Enzyme + Rust long-term" framing | reframed (not retired) as "one possible backend" per chunk 06 §6 | SKIP | no anti_spec row needed |
| C1 | §32.1 "workflow-layer glue" escape hatch vs chunk 06 §4.6 v2.2 SCC-level dispatch — ambiguous whether glue is user-managed (two runs) or compiler-managed | add parenthetical: glue is user-managed isolation, not within-run SCC handoff | ACC | prevents compatibility trap with v2.2 |
| C2 | anti_spec.md `Dual` retirement reason "Part V commits backend-delegated AD" implies Option B while §32 leans Option C | rewrite retirement reason option-neutral: "redundant with backend AD in all three options; risks conflicting representation" | REVIEW | see cross-cutting (couples with §31 C1) |

### §33 — Design Blockers

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | B1 α-stable driving case + three sub-questions (stdlib admissibility, AD infrastructure, Tier C routing) absent from §33 B1 | expand §33 B1 to name driving case + three sub-questions | ACC | chunk 04 §11; §27 H4 flagged same gap |
| H2 | B2 three candidate syntax shapes (coupled-`~`, `JointCopula`, explicit latent-uniform); B4 blocked-on-B2+B5+B6; distribution-contract-shape gate from chunk 08 | expand §33 B2 with three syntax candidates; expand B4 with dependency chain + chunk-08 contract-shape gate | ACC | chunk 04 §11 + chunk 08 deferred list |
| H3 | B5 sub-questions in dependency order (heterogeneous-unit → shape refinement → envelope flavors ∥ structural subtypes ∥ scalar reconciliation) not in §33 | enumerate per chunk 05 §7 return-path ordering | ACC | chunk 05 §7 ordering is settled |
| H4 | B6 central fork (AD ownership) + PPL protocol detail not named in §33 or cross-ref'd | expand §33 B6 to name AD fork (lean hybrid) + PPL protocol | ACC | covered by cross-cutting; naming in §33 is local |
| H5 | B3 absence from §33 explained (absorbed into B6); should appear as anti_spec.md retirement row, not spec entry | add anti_spec row "B3 as PPL-protocol-only blocker \| absorbed into B6" | ACC | per feedback-memory: no history in spec prose |
| S1 | chunk 04 §11 "Recommended ordering" dependency reasoning | design-process, not spec content; omit from §33 correctly | SKIP | lives in chunk report |
| S2 | B3 labeled "Tier C PPL backend protocol" in old list | absorbed 2026-04-20 into B6 | SKIP | covered by H5 anti_spec row |
| C1 | §33 doesn't assign B2 chunk; §34 assigns chunk 08; chunk 04 §11 still says "future chunk 07" | reconcile chunk assignment (chunk 07 is now type-graph; B2+B4 slot reassigned) | REVIEW | see cross-cutting (couples with §34 C2) |
| C2 | §33 B2/B4 listed as parallel bullets; chunk 04 §11 says B4 blocked on B2 | add parenthetical to §33 B4 noting it is blocked on B2 closing first | ACC | dependency is settled |

### §34 — Chunk-Slotted Work

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Chunk 09 (workflow data layer) missing from §34 | add Chunk 09 entry with locked principles + open items (node-path syntax, catalog type repr, observe format menu, Mode B, parameter-inference API) | ACC | chunk 09 is live locked-principle work |
| H2 | Chunk 10 (package dependencies) missing from §34 | add Chunk 10 entry noting vocabulary + approach locked; resolver / version semantics / feature model / workspace-Python / registry open; not blocking v2.1 | ACC | chunk 10 partial-lock |
| H3 | §34 Chunk 03 "resume after substrate lock" is stale — chunk 04 substrate commitment is in spec_new.md §15-§17 | rewrite Chunk 03 entry: substrate commitment present; chunk 03 can resume; name actual open items (unified machinery design, cost model, rewrite-rule declaration, tolerance plumbing, sparsity/characteristic-length, integration semantics) | ACC | chunk 04 O4.1/O4.3/O4.6 are not kernel prerequisites |
| H4 | Chunk 03 K2/K3 sub-work depends on chunk 05 matrix primitives (Gram assembly, SVD); not stated | add one-line note to §34 Chunk 03 | ACC | chunk 03 §7 + chunk 05 §5 |
| H5 | §34 Chunk 07 "Depends on chunks 04, 05, 06" overbroad — only substrate commitment is needed; chunk 04 O4.x don't block | refine dependency clause to "substrate commitment (§15-§17, chunk 04 open O4.x don't block), chunk 05 refinement-lattice, chunk 06 conversion-edge costs" | ACC | precision fix |
| H6 | Chunk 12 (cost-field unification) dependency on locked chunk 04 O2.4 `cost_of` five-field struct not stated | add sentence: "chunk 04 O2.4's five-field `cost_of` is a locked input; option (a) would absorb `loss_of` and §19.1 vector into it" | ACC | makes the constraint visible |
| S1 | chunk 03 §2 "kernels are ordinary `.myco` functions" framing | superseded by chunk 08 "parameterized relations" lock; anti_spec.md row still uses interim "ordinary `fn` accepting two point arguments" framing | REVIEW | see cross-cutting |
| S2 | chunk 04 §11 "B2+B4 remain paired for future chunk 07" framing | slot reassigned; chunk 07 is now type-graph, B2+B4 moved out | SKIP | numbering note, not spec construct |
| C1 | §34 Summary enumerates 6 chunks (body has 7 — chunk 12 missing from summary line) | add "chunk 12 cost-field unification" to Summary sentence | ACC | within-§34 editorial gap |
| C2 | §34 Chunk 08 label carries stale "B2 + B4 joint syntax / coupling;" prefix — chunk 08 is fn-ban + parameterized-relation only; no B2/B4 content | remove "B2 + B4 joint syntax / coupling;" prefix from Chunk 08 label | ACC | stale carry-over from pre-renumbering plan |

Stale-doc-only conflicts (not tabled): spec.md §12-§13 solver/emitter framing (covered by wholesale spec.md supersede); v2.1_in_progress backend-targets paragraph and "NEW:" chunk stubs (covered by wholesale stale note); deprecated-open-questions MVN / dynamic-matrix / cross-backend-callable entries (already flagged for wholesale retirement).

---

Batch 7 (§30-§34) raised the following new cross-cutting items:

- **AD-ownership fork disposition** (§31 H1 + §31 C1 + §32 H1 + §32 C2 + §33 H4). The three-way fork (Option A Myco-owned / B backend-delegate / C hybrid) has three symptoms in this batch: (i) §31 summary pre-commits AD to the minimum trait API (implicitly Options B/C) while §32 calls the fork open; (ii) §32's preamble names the fork without enumerating pros/cons; (iii) anti_spec.md's `Dual` retirement reason cites "Part V commits backend-delegated AD" which reads as Option B decided; (iv) §33 B6 has a single-phrase entry that doesn't name the fork. Chunk 06 §4.3 records the Option C lean with rationale. Two possible paths: (a) formalize Option C lean in §31.3 / §32.3 with pros/cons + consistency obligation stated, rewrite §31 summary to qualify "autodiff joins minimum API under hybrid/delegation," rewrite anti_spec.md `Dual` reason option-neutral, expand §33 B6 with the fork + PPL protocol; (b) leave fork fully open in §32, rewrite §31 summary to remove AD from the minimum-API commitment entirely. Leaning (a) — the Option C lean is already concrete enough in chunk 06 to formalize. Needs Riley's call.
- **Matrix naming and argument-count reconciliation** (§30 C1). Three inconsistent spellings for positive-definite matrix type across spec_new.md and chunk 05: §3.9 uses `PositiveDefinite` as a lattice property name; §30 writes `Matrix<_, PositiveDefinite>` (two-argument `Matrix` with property as second arg); chunk 05 §3.1 uses `Matrix<U, m, n>` (three-argument, unit + two shape dims) and the alias `PosDef<U, n>` for the specialized form. Two separable decisions: (i) full name `PositiveDefinite` vs short alias `PosDef`; (ii) type-constructor form — is it `Matrix<U, n, n, PositiveDefinite>` (property as fourth argument), `Matrix<U, n, n> where { structural: PositiveDefinite }` (predicate bound), or a named alias `PositiveDefinite<U, n>` standing on its own? §30 currently uses a form (two-argument `Matrix<_, prop>`) inconsistent with chunk 05 §3.1's three-argument form. Leaning: keep full name `PositiveDefinite` (readability + matches §3.9 table); use named alias form `PositiveDefinite<U, n>` as shorthand for `Matrix<U, n, n>` with the structural property attached; `Matrix<U, m, n>` stays three-argument. Needs Riley's call before §30 writeback and §3.9 lattice finalization.
- **`inverse` vs `inv` naming** (§30 C2). §30 uses `inverse(A)`; chunk 05 §4 uses `inv(A)`. Chunk 05 also specifies the return type `Matrix<U^(-1), n, n>` which §30 omits. Two options: (a) `inv` (short, matches NumPy/LAPACK convention, chunk 05 short-vocabulary rationale); (b) `inverse` (spelled-out, matches §30 current usage). Leaning (a) `inv` — matches both the broader convention and chunk 05's stated short-lowercase-math-vocabulary rationale. Either choice must add the return-type unit `Matrix<U^(-1), n, n>`. Needs Riley's call before §30 writeback.
- **Fallback-default-mode formal lock** (§31 C3). §31.1 labels `error` with "Conservative default" which implies it is the decided default; chunk 06 §4.2 explicitly marks default-policy selection as open. Three options: (a) formally lock `error` as default with rationale recorded (safest, no silent performance catastrophes); (b) lock `host` (most permissive); (c) move default-policy selection to §32 as explicit open item. Leaning (a) — `error` being default matches Myco's broader "fail loudly at composition" posture. Small decision; just needs a sentence of rationale. Needs Riley's call.
- **B2/B4 chunk assignment** (§33 C1 + §34 C2). Three positions in corpus: §33 does not assign B2+B4 to any chunk; §34 Chunk 08 bullet carries stale "B2 + B4 joint syntax / coupling" label (actual chunk 08 is fn-ban + parameterized-relation only — confirmed by chunk 08 report deferred list which excludes B2/B4); chunk 04 §11 "Recommended ordering" still says "future chunk 07" (the chunk-07 slot was reassigned to type-graph work). Net state: B2+B4 is not currently assigned to any chunk. Two options: (a) accept this — B2+B4 remains a blocker with no chunk owner; open a placeholder "future chunk 13 (or similar)" for when design starts; update §34 Chunk 08 entry to remove stale "B2+B4" prefix; update chunk 04 §11 to say "chunk TBD" or similar; (b) assign B2+B4 to an explicit chunk number now and treat the fn-ban work in chunk 08 as separable. Leaning (a) — B2+B4 requires B5 + B6 + chunk-08 contract-shape question to close first; premature chunk assignment would just churn. Needs Riley's call on placeholder-chunk convention.
- **"Kernels as ordinary functions" anti_spec.md row staleness** (§34 S1). Current anti_spec.md retires the `kernel` keyword and says replacement is "ordinary `fn` accepting two point arguments and returning a scalar." That replacement framing predates chunk 08's user-fn ban. Chunk 08's actual replacement is "parameterized relation." One-row edit to anti_spec.md: change the retirement-reason text from "ordinary `fn` accepting two point arguments" to "ordinary parameterized relation" (per chunk 08 lock). No design call; drop-in edit. Flagging as cross-cutting because chunk 03's return-path text ("kernels are ordinary `.myco` functions") also needs updating when chunk 03 resumes.

---

## Batch 8 (§35-§39) — pre-adjudicated

### §35 — Other Opens

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | O4.1 three option names (in-graph versioning / Layer-3 obligation-keyed metadata / referential-truth reframe) not in §35 | expand §35 O4.1 to enumerate three options + referential-truth parallel with §10/§16.1 | ACC | Tier 0 Phase 4 per chunk 04 |
| H2 | O4.3 tension with algebraic collapse named but resolution locus (§19 extraction policy, not rewrite ban) absent | add sentence: e-graph holds both forms; §19 extraction policy selects named form under training-emission mode | ACC | covered jointly with §35 C3 |
| H3 | Workflow verb taxonomy grouping question (`bind_controller` with `bind_topology` or `learn_*`?) not in §35 | add short open item; revisit after §24 verbs fleshed out | ACC | deprecated-open-questions orphan |
| H4 | Cross-backend callable interop (workflow A trains on X, workflow B runs on Y) — ACC adjudication item not yet written | add §35 entry with cross-refs to §23.3, §31.6, §32.1 | ACC | duplicate of §23 H6 + §32 H6; dedup as single §35 add |
| H5 | Rational saturation termination missing from §35 alongside BigFloat/Rational GPU item | extend GPU-incompatibility entry: "Rational saturation termination — coprime-denominator growth, precision cap or canonical-form policy needed (§26.3, §15)" | ACC | chunk 04 §8 deferred-numeric item |
| H6 | Sequential inference for HMMs (forward-backward / Viterbi / particle filter) has no tracking home after deprecated-open-questions retirement | add §35 entry; no chunk assigned; design not yet scoped | ACC | compiler-detects-errors baseline in v2.1 |
| H7 | Workflow-side capability overrides (accept-large-enumeration, inference-backend selection, approximate-inference switches) have no tracking home | add §35 entry; intended to land with §24 verbs | ACC | deprecated-open-questions orphan |
| S1 | deprecated open-questions `condition_weighted` deferral + chunk 03 §8 | already covered by existing anti_spec.md row | SKIP | batch 5 S1 already marked void |
| S2 | O4.2 Pole L'Hopital as structural-predicate-gated rewrites | covered by CC5 resolution + anti_spec.md X-category bundling retirement | SKIP | |
| S3 | Literal-constants resolution (CC1 policy) | §35's "diagnostic surface" entry is correct — policy resolved, diagnostics surface remains open | SKIP | no supersession conflict |
| C1 | `condition_weighted` deferral (checked for conflict in §35) | no conflict — §35 correctly omits as deferred | SKIP | confirmed clean |
| C2 | CC5 block (~6 paragraphs in §35) is fully resolved content occupying an "Opens" section | (a) move CC5 block to Appendix C / §17 where resolved rewrites live, OR (b) condense to 2-sentence resolved-item summary with cross-refs | ACC | presentation gap, not design contradiction |
| C3 | §35 O4.3 points toward "§20 rewrite group O1" when chunk 04 explicitly locates resolution at §19 extraction policy | amend to name §19 extraction policy as locus (not rewrite-rule ban) | ACC | combines with H2 |

### §36 — Command-Line Interface

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | §36 uses "the `myco` CLI"; §0.1/§4.1/§22/§23.4/§32 use `mycoc`; Riley confirms the CLI is `hypha` | global rename: `myco` (in §36) and `mycoc` (elsewhere) → `hypha`; `hypha` is the single user-facing CLI | ACC | Riley 2026-04-22: "the myco cli is called hypha" |
| H2 | `hypha explain` has committed `--vs path_A path_B` flag (chunk 04 §5) + 3 open design items (e-graph/residual navigation, e-class referencing, round-trip materialization) not named in §36 | add note: partial commitment for `explain` + 3 open design items pending chunk 04 Phase 2 Q3 | ACC | prevents uniform-blank-slate reading |
| H3 | `hypha check` scope locked in §23.4 (tier-1 errors, no codegen, no workflow binding) but §36 treats it as uniformly deferred | add sentence: `check` is committed verb per §23.4; flags / exit codes still open | ACC | §23.4 is already normative |
| H4 | Chunk 08 uses `hypha explain`; rest of corpus uses `mycoc explain` | chunk 08's `hypha explain` is correct per Riley; all `mycoc explain` references elsewhere need renaming | ACC | resolved by H1 global rename |
| H5 | Two-binary split (`mycoc` compiler + `hypha` package manager) not stated in §36 | one-sentence toolchain-shape note: `hypha` is the user-facing CLI (like `cargo`, `uv`); relation to internal compiler binary left to implementation | ACC | single user-facing CLI per Riley; chunk 10's `mycoc`+`hypha` split framing needs revisit |
| H6 | §36 not cross-ref'd from §0.1 (traceability) or §22 (plan inspection) despite being the CLI-spec home for `explain` | add cross-refs back to §22 and §0.1 | ACC | stub-as-dead-end problem |
| S1 | spec.md Appendix B single-`myco`-binary model (`myco repl`, `myco plan --dot`, `myco add`, `myco publish`, `myco search`) | covered by wholesale spec.md supersede; rename `myco` → `hypha` per Riley | SKIP | no new anti_spec row |
| C1 | §36 line 5059+5063 says "the `myco` CLI spans compile, run, check, fmt, explain"; other sections use `mycoc` | rename all CLI references to `hypha` (Riley 2026-04-22) | ACC | resolved by H1 global rename |

### §37 — Dependency Management and Package Registry

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Four locked vocabulary terms (`spore`, `hypha`, `myco.toml`, `myco.lock`) absent from §37 body (appear only in §33 open-questions block) | add **Vocabulary (locked)** paragraph — one sentence per term; `hypha` is the user-facing CLI (covers compile, run, check, fmt, explain, package mgmt) | ACC | Riley 2026-04-22: `hypha` is singular CLI, not only package mgmt |
| H2 | Cargo+uv convention lock absent from §37 | add **Approach (locked)** line | ACC | chunk 10 Principle section |
| H3 | Chunk 10 open-items list (9 items: resolver, version semantics, feature model, build scripts, workspace-Python, tooling, cross-spore visibility, registry, platform metadata) not surfaced in §37 | add **Open items** list to §37 body (enumerate per §33 block + tooling) | ACC | copy-from-source-already-in-spec_new.md |
| H4 | Chunk 10 "Minimum viable package system for v2.1" subset + post-v2.1 deferral list not in §37 | add **Minimum viable scope (v2.1)** paragraph | ACC | gives concrete boundary |
| H5 | §34 omits chunk 10 (confirmed by §34 audit H2) | resolved by §34 batch-7 ACC; ensure chunk 10 entry cross-refs §37 | SKIP | already tracked in §34 H2 |
| H6 | `hypha` as CLI binary not named in §37 | one-sentence note: `hypha` is the user-facing CLI (same binary as §36); packaging sub-commands live alongside compile/run/check/explain | ACC | Riley 2026-04-22: single CLI, not a distinct package-manager binary |
| C1 | §37 body lines 5068/5072/5074 use "`.myco` packages" where rest of spec (§2, §23, §33) uses "spore" | replace with "spore" (or "`.myco` spore" on first use) | ACC | vocabulary consistency; chunk 10 locked |
| C2 | §37 Summary presents declare/resolve/publish/lock as uniform open, contradicting chunk 10 partial-lock (lockfile locked, resolver open, publishing open) | implicitly resolved by H1/H2 additions (show locked alongside open) | ACC | subsumed |

### §38 — Editor Tooling

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | chunk 10 line 197 bundles "editor integration (LSP) / doc generation / formatter / linter" under tooling-integration; LSP→§38, doc→§39, `fmt` CLI flag→§36, formatter + linter as tools have no tracking home | add "Formatter and linter" to §38 body list | ACC | chunk 10 open bundle splits across §36/§38/§39 |

### §39 — Documentation Generation and Website

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | chunk 10 flags `hypha doc` as candidate CLI sub-command spelling for doc generation; §39 doesn't reference CLI surface | brief note: doc generation is invoked as `hypha doc` | ACC | Riley 2026-04-22: `hypha` is the CLI; `hypha doc` is the locked sub-command |
| H2 | spec.md §B.5 names Mermaid and D2 as rendering targets relevant to doc/presentation contexts; §39 treats docs as static HTML only | optional cross-ref note: doc website may embed generated graph diagrams via §B.5 / §22 | ACC | cross-ref, not new commitment |

Stale-doc-only conflicts (not tabled): legacy spec.md §14.6 `condition_weighted`-as-deferred text, v2.1_in_progress `condition_weighted` deferral prose, spec.md Appendix B single-binary CLI framing, deprecated-open-questions file wholesale (handled by archival plan).

---

Batch 8 (§35-§39) raised the following new cross-cutting items:

- **CLI binary naming resolved: `hypha`** (§36 H1 + §36 C1 + §36 H4 + §37 H6 + §39 H1). Riley 2026-04-22: "the myco cli is called hypha." The user-facing CLI is `hypha` — a single binary covering compile, run, check, fmt, explain, and package-management sub-commands (analogous to `cargo` or `uv`). Resolution: (i) §36 rename "`myco` CLI" → "`hypha` CLI"; (ii) global rename `mycoc` → `hypha` across §0.1, §4.1, §22, §23.4, §32 (and any other section using `mycoc`); (iii) chunk 08's `hypha explain` is correct — all `mycoc explain` references elsewhere need updating; (iv) §39 doc-gen sub-command is `hypha doc` (locked, not open); (v) chunk 10's "two-binary split (`mycoc` compiler + `hypha` package manager)" framing needs revisit — whether `mycoc` survives as an internal compiler binary invoked by `hypha` (like `rustc` under `cargo`) or disappears entirely is an implementation detail that does not need to appear in spec prose. Applies at Phase 1 end alongside other ACC fixes. Touches §36, §37, §39 (as noted), plus §0.1, §4.1, §22, §23.4, §32 (global rename). Also: chunk 10 report should be annotated that `hypha` is confirmed as the user-facing CLI name.
- **CC5 block placement in §35** (§35 C2). The CC5 site-gated-strict-rewrites block occupies ~6 paragraphs in §35 describing fully resolved design (Layer-3 site records, X1/X2 split, cross-geometry pollution proof). It reads as completed prose in a section titled "Other Opens." Two options: (a) move the block wholesale to Appendix C (where X1/X2 rewrite rules live) or §17 (rewrite-rule substrate), leaving only a short "CC5 resolved — see Appendix C" pointer in §35; (b) condense the §35 block to a 2-sentence resolved-item note with cross-refs. Leaning (a) — the block is dense substantive design content and Appendix C / §17 is the semantic home for rewrite-rule material. §35 should be a list of genuinely open items. Not urgent; presentation-level fix. Needs Riley's call on option (a) vs (b).
- **Chunk 10 tooling-integration bundle splits across §36/§38/§39** (§38 H1 + §39 H1). Chunk 10 bundles "editor integration (LSP), documentation generation, formatter, linter" as a single open item. In spec_new.md these live in three different Part VII stubs: LSP in §38, doc gen in §39, `fmt` as a CLI flag in §36 with the formatter-tool itself having no home, linter entirely absent. No design decision needed; resolution is (a) add "formatter and linter" to §38, (b) cross-ref §36/§38/§39 from chunk 10's open-item text, (c) close the bundle. Drop-in edits only; listed as cross-cutting because it touches three sections.

---

## Batch 9 (§40, Appendices A/B/C) — pre-adjudicated

### §40 — Agent / LLM Integration

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | `v2_old/v2_do_this_first.md` line 176 mentions "less-technical users asking an agent to import and adapt an existing model" — LLM-assisted model adaptation as a use case | optional: add bullet under §40 summary for "agent-mediated model import/adapt/validate" | REVIEW | legacy file; Riley's call whether this vision is still live. If alive, it's distinct from §40's "agent skills for writing/reviewing" — implies package interop prerequisite |
| S1 | Testing/property-checking affordances note in `spec_dev_notes.md` flagged as "Could live in Part VII" | not §40's home; tracked by §35 or future Part VII stub | SKIP | per audit |
| S2 | `v2_old/open_questions.md` interactive-vs-agent-API-ergonomics question | belongs to §23/§24 Python boundary, not §40 | SKIP | per audit |
| S3 | `riley_project_note.md` LLM-data-scraping note | project-specific, not language/DX; retiring per project-vs-language separation memory | SKIP | per memory rule |

### Appendix A — Reserved Keywords and Syntactic Surface

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | `base_unit` used as declaration keyword in §5.0 code blocks (line 593-597) but missing from Appendix A | add `base_unit` to Declaration keywords | ACC | drop-in add |
| H2 | `unit` used as declaration keyword in §5.0 code blocks (line 603-605) but missing | add `unit` to Declaration keywords | ACC | drop-in add |
| H3 | `val` compile-time scalar generic kind used in mocks (`N: val`) and prose (§3, §11.1); anti_spec.md locks `val` as replacement for `const N: usize` | add `val` to Type-former keywords (parameter-kind qualifier) | ACC | drop-in add |
| H4 | `approximate` body-form block keyword used in §13/§15/§17/§25; named in §1 glossary but missing from Appendix A | add `approximate` to Body-form keywords | ACC | drop-in add |
| H5 | `is` type-narrowing predicate in `where x is T` (§8.3, §12.7) not listed anywhere in Appendix A | add `is` to Body-form keywords (or new "Predicate keywords" sub-category alongside `where`) | ACC | drop-in add |
| H6 | `enum` declaration keyword locked by chunk 11 (sum types) but missing from Appendix A | add `enum` to Declaration keywords, with note that variant-syntax details pend chunk 11 close | ACC | chunk 11 locked the keyword even if variant syntax is still open |
| H7 | `match` Appendix A status "reserved for future pattern-matching" understated — chunk 11 commits exhaustive match as enum dispatch | update reservation note: committed as exhaustive sum-type dispatch; surface syntax pending chunk 11 | ACC | tighten wording |
| H8 | `observe` dual status: §13.8 shows `observe(data, x ~ D)` call form in `.myco` body position; §16 describes it as "workflow verb"; Appendix A lists it nowhere | resolve semantic status first: is `observe` a `.myco` surface call (→ add to Body-form kw or stdlib-reserved) or a workflow-tier mechanism (→ remove call-form from §13.8 prose)? | REVIEW | ambiguity touches §13.8 + Appendix A; see cross-cutting |
| H9 | `smooth_max`/`smooth_abs`/`smooth_step` (§8.9), `soft_select`/`hard_select`/`weighted_average`/`condition_weighted` (§8.7), `argmin`/`argmax` (§12.1), `soft_clip`/`hard_clip`/`sigmoid` (§25) used as stdlib calls but not in Appendix A's stdlib-reserved list | extend stdlib-reserved list OR split into "math atoms" list + note that other stdlib namespaces (aggregation, smoothing, projection) are governed separately | ACC | leaning extend the list; one-pass add |
| H10 | `loss_of` (§14.2) and `cost_of` (§14, §25) compiler intrinsics alongside `deriv`/`integrate`/`condition_of` but missing from stdlib-reserved | add `loss_of` and `cost_of` to stdlib-reserved list | ACC | drop-in add |
| H11 | Spatial operators `grad`, `diverg`, `laplacian`, `curl`, `normal_grad`, `limit_from` used in §11.1 but missing from stdlib-reserved list | add to stdlib-reserved (possibly as a "Geometry stdlib" sub-list). Note `trace` already in body-form keywords; dual identity (body-form kw + PDE operator) warrants clarifying note | ACC | drop-in add + clarifying note on `trace` |
| H12 | `Binomial` referenced in §13 (line 2209) and §27.3 conjugate-posterior table but absent from §27.1 Tier 1 discrete family table | add Binomial to §27.1 Tier 1 discrete | ACC | §27.1 fix, not Appendix A; minor |
| C1 | Appendix A says "unit generics use `<Ito>`, `<Stratonovich>` as contract-parameter keywords on `~`" but §13.4 locks them as type parameters on SDE families (e.g., `BrownianMotion<Ito>`), not on `~` | rewrite stochastic-operator paragraph: `~` is the distribution-binding operator; SDE families carry an integration-convention type parameter `<Ito>` / `<Stratonovich>` (default `<Ito>`) | ACC | phrasing fix |
| C2 | `identify` listed under Body-form keywords alongside `let`/`if`; actually appears as geometry-body / module-scope declaration per §11.2 and §17 X2 — not a general body form | move `identify` to Declaration keywords, OR add parenthetical clarifying scope ("geometry-body / module-scope only") | REVIEW | classification call; minor, but affects how readers understand the keyword's scope |

### Appendix B — Grammar / EBNF Summary

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Chunk 08 locks concrete relation-invocation syntax (statement-form only, no expression-position invocation, method-style `receiver.rel(args)` desugar) — grammar-precursor material | cross-reference chunk 08 in Appendix B placeholder as locked prior art | ACC | adds a pointer, no new design |
| H2 | Chunk 02 locks `impl` / `some` bracket-position rules in collection type syntax | cross-reference chunk 02 in Appendix B placeholder | ACC | adds a pointer |
| H3 | Chunk 08 locks method-style dispatch sugar (`receiver.rel(args...)` → `rel(receiver, args...)` when first param is `Self`) | cross-reference chunk 08 in Appendix B placeholder | ACC | merges with H1; one combined reference |
| C1 | Appendix B scope parenthetical says "§2 through §14 (... workflow-boundary syntax)" but workflow-boundary syntax is §23 (Part III, outside §2-§14) | either widen scope to "§2-§14 plus §23" or drop the enumeration which risks going stale | ACC | tighten wording |
| C2 | Appendix B parenthetical says "stdlib calls" — but the `~` operator and distribution-family call syntax live in §13/§27, outside §2-§14 | narrow "stdlib calls" to §6 function-call syntax, or widen scope to include §27 | ACC | tighten wording; couples with C1 |

### Appendix C — Rewrite Catalog (A–Z)

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Q-group marked OPEN pending "§35 stochastic rewriting semantics" but chunk 04 Bucket 1 promotes Q1-Q2 to fully committed (unblocked by CC4) | verify with Riley whether Q1-Q2 are LOCKED (CC4 supersedes) or still gated on §35 phase-2 stochastic rewriting | REVIEW | chunk 04 says Bucket 1; Appendix C conservative. Couples with §13 open items |
| H2 | Y6 `C(N,M)` enumeration OPEN tag cites "(§35)" but the blowup-threshold open actually lives in §8.7 | change pending cite from `(§35)` to `(§8.7, threshold not yet locked)` | ACC | cross-ref fix |
| C1 | M1/M2 marked OPEN but chunk 04 O2.3 (RESOLVED 2026-04-20) moved both to Bucket 1 (fully committed); `approximate` block provides the authorization | change M-group header from `OPEN (§35 envelope machinery)` to `LOCKED (O2.3 resolved 2026-04-20)`; tag M1 and M2 individually as LOCKED | ACC | substantive tag correction |
| C2 | K3 marked OPEN but chunk 04 Bucket 4 explicitly defers K3 to v2.2+ ("speculative, low-rank kernel rewrites not urgent for v2.1") | change K3 tag from OPEN to DEFERRED (v2.2+); remove from v2.1 rule count | ACC | categorical correction |
| C3 | Summary table line 5511 double-counts J1 (listed in both Strict-uni column and Forbidden row) | remove J1 from Strict-uni column; Strict uni becomes `~4 (D4-5, X1, X2)`; Strict total `~28`; Forbidden row stays `1 (J1)`; grand total drops by 1 | ACC | arithmetic fix |
| C4 | Summary table line 5514 double-counts M2 (listed in both bidi cell "M1-2" and uni cell "M2") | Fuzzy-tolerance bidi cell correct to separate M1 from M2; uni cell keeps M2 | ACC | arithmetic fix; couples with C2 (K3) |
| C5 | CC-summary paragraph (5523-5526) says "CC1-5 are absorbed into normative spec text" then lists CC3 as "open as O4.3" — contradictory framing | separate CC3 from the "absorbed" list: CC1, CC2, CC4, CC5 absorbed; CC3 TRACKED as O4.3 open item | ACC | framing fix |
| C6 | Fuzzy-tolerance bidi count `~7` contradicts 10 named items (K1-3, M1-2, N1, Q1-2); compounded by C2 (K3) and C4 (M2) | after resolving C2 + C4: recount from scratch — bidi ≈ K2, M1, Q1-Q2 (~4); uni ≈ K1, M2, N1, O1, P1 (~5); total ~9 | ACC | re-tally after C2+C4 applied |

Stale-doc-only conflicts (not tabled): legacy spec.md pre-CC5 single-X rewrite bundling (retired in anti_spec.md), legacy spec.md `base_unit` usage matches spec_new.md §5.0 (no conflict; Homeless in Appendix A H1), legacy spec.md `dyn` usage (retired in anti_spec.md; Appendix A correctly omits).

---

Batch 9 (§40, Appendices A/B/C) raised the following new cross-cutting items:

- **`observe` dual status: `.myco` surface vs workflow verb** (Appendix A H8 + §13.8 cross-cut). §13.8 shows `observe(data, x ~ D)` as a call in `.myco` model bodies; §16 calls `observe` a "workflow verb"; Appendix A lists it in neither its body-form keyword list nor its stdlib-reserved list. The ambiguity is design-level, not editorial: either (a) `observe` is `.myco` surface syntax (add to Appendix A; §13.8 call-form is correct; keep workflow-verb naming as a convention) or (b) `observe` is purely a Python-tier verb and §13.8's call-form is pseudo-notation for a compiler mechanism that has no source-language surface (rewrite §13.8 to describe the mechanism without introducing `observe(...)` as a parse form). Leaning (a) — the `observe(data, x ~ D)` call-form reads clearly as `.myco` syntax and likely is the intended surface — but needs Riley's call. Touches §13.8 and Appendix A.
- **`identify` classification: declaration vs body-form** (Appendix A C2). Appendix A lists `identify` under "Body-form keywords" but §11.2 and §17 X2 describe it as a module-scope / geometry-body declaration that installs a periodic identification, not a general body-scoped form. Not a contradiction of behavior — `identify` still appears inside scopes — but the Body-form classification risks misleading readers into thinking it is usable in relation bodies. Two fixes: (a) move `identify` from Body-form to Declaration keywords, or (b) add a parenthetical ("geometry-body / module-scope only") on the existing listing. Leaning (a). Minor.
- **Summary table arithmetic** (Appendix C C3+C4+C6). Three independent double-counts + off-by-one errors in the summary table at lines 5507-5521 compound: J1 counted in both Strict-uni and Forbidden (C3); M2 counted in both bidi and uni cells (C4); bidi count `~7` does not match 10 named items (C6). Drop-in fix but requires re-tallying from scratch after C1 (M1/M2 relock) and C2 (K3 defer) land. Not a design question; purely editorial.
