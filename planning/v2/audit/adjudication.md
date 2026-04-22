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
| H1 | `geometry` keyword and `Domain<G>` annotation not named in §11 | add §11.0 naming both | ACC | |
| H2 | `as` clause (coord names/units/extents on domain type) absent | add to §11 (explains `bind_topology` edge-length unit validation) | ACC | |
| H3 | `trace()` as directional-limit primitive for graph junctions absent | distinguish from manifold-restriction `trace` or rename one | ACC/W | (a) rename the graph-junction operator to `limit_from(f, junction, edge)` (Rustacean `from_X` idiom). `trace(f, boundary)` retains the manifold-restriction meaning (standard PDE trace operator). §11.1 updated with both; anti_spec entry for the old overloaded spelling |
| H4 | Locus-scoped relations with `replaces` obligation keys absent | add to §11.8 or own subsection | ACC | |
| H5 | `identify` (periodic seam) absent; `Sphere` underdefined without it | note `identify` at least for `Sphere` if kept | ACC | |
| H6 | `bind_topology` schema + validation rules absent | describe schema (vertex IDs, edge list, edge-length units, vertex tags) | ACC | |
| H7 | `continuous(field)` and `kirchhoff(potential, flux)` stdlib helpers absent | add one-sentence §11.8 note | ACC | |
| H8 | Subdimensional fields (`field name: Type over coord`) absent | add `over` keyword to §11.7 | ACC | |
| H9 | `curl` listed as settled elsewhere but absent from §11.1 | either include in §11.1 list or flag deferred | ACC/W | ship both 2D (scalar) and 3D (vector) under one name; dimension dispatch at stdlib axiom level via case-on-val-generic in return position, mirroring §3.9/§30 `solve` dispatch on structural subtype. §11.1 updated; §35 flags case-on-val-generic formalization as small open |
| H10 | Locus-scoped `temporal` has no subsection despite summary header mentioning it | add parallel treatment to locus-scoped relations | ACC | resolved by §9 H6 (temporal legal on loci) |
| H11 | Terrain-as-field deprioritization not recorded | land as anti_spec entry or §11 note | ACC | anti_spec entry (terrain-as-field deprioritized for v2.1) |
| C1 | §11.3 stdlib geometry names diverge from corpus (Ball3D new; Line1D/Rectangle2D renamed; Polar/Sphere dropped) | reconcile: either adopt §11.3 and retire old names in anti_spec, or revert §11.3 | ACC/W | adopt authoritative suffix-free catalog: `Interval`, `Circle` (S¹), `Rectangle`, `Disk`, `Sphere` (S²), `Box`, `Ball`, plus the three network geometries. Solid-vs-manifold distinction load-bearing (`Sphere` = surface; `Ball` = solid). Coord systems live on `as` clauses, not as separate geometry types (no `Polar`/`Spherical`). §11.3 rewritten; anti_spec entries added for dimensional-suffix names, `Polar`/`Spherical`-as-type, and `Sphere`-as-solid |
| C2 | §11.1 spells `laplace`; every other doc uses `laplacian` | standardize on `laplacian`; retire `laplace` | ACC/W | §11.1 updated; anti_spec entry added |
| C3 | `trace` overloaded (manifold-restriction in §11.1 vs graph-junction directional-limit in chunk 01) | resolve overload per H3 | SKIP | covered by H3 |

### §12 — Collections and Iteration

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | `max` and `min` as collection aggregation primitives missing | add to §12.1 + §12.3 (chunk 02 §4.2 settled) | ACC | |
| H2 | Backend sentinel injection (`-inf`/`+inf` for masked invalid slots) missing | add to §12.3 or §21 lowering prose | ACC | |
| H3 | `count` alive-element vs backing-array-length distinction missing | note in §12.1 or §12.4 | ACC | |
| H4 | `argmin`/`argmax` tie-break rule (deterministic, index order) missing | add to §12.2 | ACC | |
| H5 | `argmin`/`argmax` differentiability class (`subgradient`) missing | add to §12.2 or §12 compiler-role note | ACC | |
| C1 | §12.3 empty-collection table structurally inconsistent with chunk 02 re: min/max | fixed by H1 | SKIP | covered by H1 |

### §13 — Probabilistic Programming

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | B1 opaque `log_pdf` forward reference missing from §13 | add one-sentence note in §13.2 or §13.3: opaque families route Tier C pending B1 | ACC | |
| H2 | `Distribution<U>` contract interface (required `log_pdf`, required `sample`, optional `reparameterized_sample`) not stated anywhere | add contract block to §27 with cross-ref from §13 | ACC | |
| H3 | Workflow-side epistemic prior API (`assume_prior`) unconnected from §13.1 | add sentence cross-referencing §24.4 (verb tracked as future) | ACC | |
| H4 | Z-group rewrite catalog (Z1-Z10) not cross-referenced from §13 | confirm Appendix C covers Z-group; else add §13.2 footnote | ACC | |
| H5 | `SumSelfClosed` conditional-rewrite nuance (same-rate, same-p) missing | add one-sentence qualifier in §13.2 | ACC | |
| H6 | HMM / Markov-structured discrete latents compile-error not named in §13.3 | add sentence distinguishing this from silent Tier C fallthrough | ACC | |
| S1 | open_questions.md §Closure policy still shows `condition_weighted` deferred | strike paragraph or mark RESOLVED (anti_spec already records it) | ACC | |
| C1 | §13.1 structural rule ("aleatoric `~` appears inside `temporal:` or event scope") is too narrow; observation likelihoods in plain `relation` bodies are also aleatoric | tighten rule: aleatoric = LHS is measured/observed OR inside temporal/event; epistemic = unknown constant not observed | ACC/W | §13.1 rewritten 2026-04-22 with the tightened rule: aleatoric when LHS is measured/observed-tethered OR inside temporal/event scope; epistemic when unknown constant not observed per time-step (module scope, `initial:`, or `~` whose LHS is neither data-bound nor in temporal/event scope). Classification is compiler-derived (static graph-position + workflow-binding inspection), not user-annotated |

### §14 — Compiler Intrinsics

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | `deriv` has no subsection in §14 despite being named in the summary | add §14.4 `deriv` covering three lowering modes + SCC interaction + limitations | ACC | |
| H2 | `condition_of` consumer mapping incomplete (extraction ranking, diagnostics missing; Level III unavailable to closure policies not stated) | expand §14.1 | ACC | |
| H3 | Duality worked example missing per O2.4 documentation obligation | add brief pair to §14.1 (`(exp(x)-1)/x` vs `expm1(x)/x`; linear-solve block) | ACC | |
| H4 | `condition_of` return accessor name mismatch: §14.1 uses `.level`, O2.4 locks `.mode` | reconcile; pick one and log in dev_notes | ACC/W | `.mode` (Rust convention: `.level` implies ordering, `.mode` implies alternatives; tolerance classes are alternatives not severities). §14.1 updated |
| H5 | `deriv` runtime fallback gated on B6 AD-ownership not cross-referenced | add forward reference to §14 (or §14.4) | ACC | |
| S1 | v2.1_in_progress "`deriv` always symbolic / no runtime cost" framing stale | add anti_spec entry: runtime AD is the fallback for large SCCs | ACC | |
| C1 | `loss_of` field inventory conflict: §14.2 uses `{data_fit, constraint_violation, regularization}`; O2.4 locks `{compute, approximation, condition, truncation, discretization}` | resolve overload: one struct, two structs, or rename one intrinsic. Document and update anti_spec | O/W | Canonical tracking: new chunk 12 (`12_cost_field_unification.md`) with problem statement + three options + load-bearing questions + cross-refs. Inline `*Open.*` notes added at §14.2 and §19.1; §34 Chunk 12 bullet added. Design call deferred; state consolidated. Applied 2026-04-22 |
| C2 | `condition_of` accessor name (same as H4; tabled as C2 in §14 report) | covered by H4 | SKIP | covered by H4 |

---

## Batch 4 (§15-§19) — pre-adjudicated

### §15 — Approximate Blocks

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Orientation axis (bidirectional vs unidirectional) orthogonal to faithfulness not developed | fill §15 with 2x3 matrix (faithfulness x orientation); drop "univariate vs bivariate" placeholder | ACC | resolves §15 summary C1 too |
| H2 | Envelope-narrowing makes normally-lossy rewrite lossless in context; absent from §15 | add paragraph to §15.2 or §15.3 noting envelope metadata (§16) can narrow a default-off rewrite to default-on in context | ACC | same mechanism surfaces as §17 homeless item; land once in §15 and cross-link |
| H3 | Declaration/derivation interaction three-case semantics absent (user-declared bound vs compiler-derived) | add §15.4 or §15.2 subsection covering (a) compiler-proves-exact / (b) compiler-within-user-declaration / (c) compiler-disproves-declaration | ACC | O2.4 locked decision |
| H4 | Multi-dimensional cost vector absent; `loss_of` named fields not cross-linked | cross-ref §15.2 to §14.2 named-field `loss_of` struct | ACC | gated on §14 C1 resolution; apply once the canonical struct is picked |
| H5 | Mutual exclusion: exactly one of `under` / `tolerance_class` required; not stated in §15.1 | one-sentence rule in §15.1 | ACC | |
| H6 | Workflow-side sampling parameters (`run.config.loss_estimation`) absent | add sentence to §15.2 noting sampling config is workflow-side per CC1; cross-ref §24 | ACC | |
| H7 | Named Tier B rewrites (K1/M1/M2/Z8/Z9) that fire under `approximate` blocks unlinked | cross-ref §15.1 `under` field to Appendix C | ACC | |
| S1 | chunk 04 `approximate` expr-infix surface (`approximate A <-> B: under: ...`) superseded by block form | add anti_spec row: expr-infix form replaced by `body:`-scoped block | ACC | |
| C1 | §15 summary "2x2 matrix (lossy-model vs lossy-tolerance) x (univariate vs bivariate)" contradicts chunk 04 2x3 (faithfulness x orientation) | adopt 2x3 per H1 | SKIP | covered by H1 |
| C2 | §15.2 four-source framing vs chunk 04 five-layer framing | clarify §15.2 describes lossiness *sources* (where); the derivation *stack* (how compiler quantifies) lives in §14 or new §15.4 | ACC/W | Both framings land as orthogonal. §15.2 retains four sources (origin taxonomy). New §15.4 "Five-Layer Lossiness Accounting" adds the accounting stack (Layer 0 syntactic / Layer 1 equational / Layer 2 distributional envelope / Layer 3 adjacent keyed state / Layer 4 extraction cost) with worked example (`hard_clip` as source-1 projection landing at Layer-2 distributional envelope). Orthogonality note added to §15.2 cross-referencing §15.4 |

### §16 — The E-Graph

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | O4.1 `replaces` open-retraction question not cross-referenced from §16.2 | add cross-ref in §16.2 to §35 noting broader retraction semantics open | ACC | |
| H2 | Provenance as envelope content (union-under-merge rule) unplaced | add to Layer 2 enumeration in §16.1; note rule in §16.3 | ACC | |
| H3 | Faithfulness and orientation tags as Layer 2 merge-edge content unplaced | add to Layer 2 enumeration in §16.1 | ACC | |
| H4 | SCC decomposition results as Layer 3 unplaced | add to Layer 3 enumeration in §16.1 with cross-ref §20 | ACC | |
| H5 | Backend emulate-mode substitutions as Layer 2 lossy-tolerance envelope facts unplaced | add sentence to §16.3 or §16.2; cross-ref §31.1 | ACC | |
| H6 | Workflow provider bindings as Layer 3 unplaced | add to Layer 3 enumeration in §16.1 | ACC | |
| H7 | Stochastic sampling traces as Layer 3 unplaced | add to Layer 3 enumeration in §16.1; cross-ref §13 | ACC | |
| H8 | Runtime event-trigger state as Layer 3 unplaced | add to Layer 3 enumeration in §16.1; cross-ref §10 | ACC | |
| H9 | §16.4 envelope flavors stated as settled; tensor-extension is open (chunk 05 §3.3) | add caveat to §16.4: composition rules stated for scalar case, tensor extension tracked in chunk 05 | ACC | |
| H10 | Extraction cost tuple connection to Layer 2 not stated as principle | cross-ref §19.1 from §16.4; minor | SKIP | adequately covered by §16.4 pointing to §19.1 per audit |
| C1 | Temporal subscripts `y[1]`, `y[2]` listed as Layer 3 in §16.1 but chunk 04 §4 explicitly corrects this to Layer 1 distinct ground terms | remove `y[1]`, `y[2]` from §16.1 Layer 3 list; replace with per-event copies / identity-tagged instances | ACC | stable chunk-04 correction |

### §17 — Equality-Introducing Machinery

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Envelope-narrowing promotion (default-off becomes default-on when error_bound collapses) absent from §17.6 | add named corollary to §17.6 | ACC | same mechanism as §15 H2; land once in §15 and cross-link here |
| H2 | A-Y rewrite catalog closed/extensible not stated | one sentence in §17.4 or §17.5: baseline is Appendix C; workflow extension tracked in §35 | ACC/W | Closed. Appendix C preamble updated 2026-04-22 with **Catalog closure** paragraph: "The A–Z catalog is closed for v2.1. New rewrite rules are not expressible in `.myco`; the compiler is not a user-facing rewrite-authoring surface. Post-v2.1 extensibility lands via a Rust-side plugin system invoked from workflow." Catalog extended A–Y → A–Z to accommodate the locked Z group (distribution-family rewrites). §17.5 updated (26 groups, A-Z) |
| H3 | `replaces` suppression-not-retraction framing absent from §17.1 | forward-ref note in §17.1 (or cross-link from §16.2 already covered by §16 H1) | ACC | |
| H4 | `identify`-seam merges as CC5 site-scoped structural-predicate-gated absent from §17.2 | add sentence distinguishing module-scope `identify x = y` (unconditional) from geometry-body `identify coord_a <-> coord_b` (site-scoped gated) | ACC/W | Resolved 2026-04-22 via opus_identify_review.md recommendations. X-category split into X1 (pole L'Hopital, removable-singularity operator substitution) and X2 (identify, quotient-induced value equality). X2 installs a Layer-3 adjacent-keyed-state "site record" keyed on locus path (e.g., `seam@SphereSurface.azimuth`) carrying glue map, site predicate, and declaration provenance. Layer-1 merges become *derived consequences* of Layer-3 site records (X2 consults the record and emits a merge tagged with the site's identity). §17.1 reframed as "eight authorization sources" with direct-writer (1, 2, 3, 7, 8) vs rewrite-class-authorizer (4 identify-via-Layer-3, 5 stdlib-inverses-via-E-group, 6 convert) split. §17.2 updated with idempotency-at-merge-vs-declaration clarification. Cross-geometry pollution impossible by construction (site records owned by the geometry). §35 CC5 entry locked. Appendix C V1 tightened (observe is a Layer-2 envelope fact, not an equational merge) |
| H5 | O4.3 per-residual exposure constraint (extraction preserves original relation names) absent from §17 | forward-ref in §17.6 to §25 Training Emission | ACC | |
| H6 | Symbolic-math intrinsics (`deriv`, `integrate`) participating in rewrite system unstated | sentence in §17.4 or §17.5 cross-linking Appendix C | ACC | |
| H7 | "Merges happen only via the eight sources; no silent inference" negative statement absent from §17.1 | one sentence in §17.1 | ACC | Tier 0 audit's single biggest unstated semantic |
| Open | O4.6 heterogeneous `argmax` tagged handles not Tier-1 ready | do not land in §17; confirm open_questions.md tracks it | ACC | open-design; no §17 action |

### §18 — The Type Graph (stub)

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | Chunk 07 three-option coupling framing (A/B/C) not in open_questions.md | add to open_questions.md Tier 0/1 | ACC | |
| H2 | Chunk 07 Q1-Q4, Q6, Q7 not in open_questions.md | add to open_questions.md Tier 1 with chunk 05 Q7 / chunk 06 cross-link for Q6 | ACC | |
| H3 | Chunk 07 dependency ordering (after 04/05/06) not recorded in §34 or open_questions.md | add brief dep note to §34 chunk 07 entry | ACC | |
| H4 | §0.1 Conversion-graph cost model paragraph not forward-ref'd from §18 | add when §18 stub is fleshed out; not urgent for stub | SKIP | deferred with chunk 07 |
| C1 | Stale §18 cross-references in §0.1 conservation-laws paragraph (line ~85) and §0.1 Part II intro (line ~1915-1916) — point to §18 for SCC/residual classification | replace `(§18)` with `(§20)` in both locations; also note conservation-group membership is genuine §18 concern but enforcement thread is §19.3/§20 | ACC | internal-ref fix from renumbering |

### §19 — Residual Graph (Projection)

| ID | Finding | Recommendation | Status | Notes |
|---|---|---|---|---|
| H1 | O4.3 per-residual training-emission constraint (extraction preserves overconstrained relation names) absent from §19.2 | add bullet to §19.2 or note in §35 | ACC | |
| H2 | §19.1 cost dimensions (precision, latency, memory, approximation class) diverge from O2.4 five-field struct (compute, approximation, condition, truncation, discretization) | reconcile with §14.2 / O2.4 | O/W | Tracked in chunk 12 (`12_cost_field_unification.md`) per §14 C1 resolution. Inline `*Open.*` note at §19.1 added. §34 Chunk 12 bullet added. Applied 2026-04-22 |
| H3 | Workflow extraction-policy API shape (`run.config.extraction_policy`) unlinked from §19.1 | cross-ref §19.1 to §24 workflow verbs | ACC | |
| H4 | Rational-denominator saturation non-termination concern absent from §19.4 termination bound | caveat to §19.4 cross-linking §35 and §26.3 | ACC | |
| H5 | Closure-policy extraction-time vs saturation-time distinction unstated | sentence in §19.4 or §8.7: Y1-Y6 policies are extraction-time | ACC | resolves §19 C1 |
| H6 | CC5 site-scoped structural predicates (pole L'Hopital, seam rewrites) tier placement in §19.4 unclear | clarify these fire in algebraic/unit-preserving tier | ACC | |
| S1 | spec.md §12.3 "canonical evaluator" framing not in anti_spec.md | add low-priority anti_spec entry | ACC | subsumed by existing "residual as core semantic object" retirement; optional |
| C1 | §19.4 conflates closure-policy co-membership merges (saturation-time) with policy selection (extraction-time) | rewrite §19.4 bullet to distinguish both phases | SKIP | covered by H5 |
| C2 | §19.1 cost dimensions vs O2.4 named-field struct | covered by H2 | SKIP | covered by H2 |

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
