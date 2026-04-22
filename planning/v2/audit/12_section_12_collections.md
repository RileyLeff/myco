# §12 Collections and Iteration — Audit Report

**Section audited:** spec_new.md §12 (§12.1–§12.7)
**Primary closed chunk:** `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md`

---

## Absorbed

Content from the corpus that landed in spec_new.md §12.

- `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §2.1 — "`impl` means 'each element implements this contract; concrete type resolved at compile time.'" Absorbed into §12 intro and §12.5.

- `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §2.2 — "`some` means 'the collection's size changes at runtime via events.'" Absorbed into §12 intro and §12.4 (bind-time vs event-time split).

- `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §2.3 — "`impl` + `some` compose." Absorbed into §12 intro.

- `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §3.2–§3.4 — Iterator-style and index-style iteration (`for x in collection`, `for i in 0..N`, nested/mixed iteration). Absorbed into §12.6.

- `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §3.3 — Graph-neighborhood style (`for n in node.neighbors`). Absorbed into §12.6 (with explicit pending note for §11 geometry surface).

- `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §3.5 — `where x is T` type-based filtering with code example. Absorbed into §12.7.

- `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §4.1/§4.2 — `sum`, `product`, `any`, `all`, `count`, `argmin`, `argmax` as stdlib aggregations. Absorbed into §12.1.

- `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §4.3 — Tagged handle `(pool_id, index)` for heterogeneous `argmax`, multiplexed field access, runtime sum type note. Absorbed into §12.2.

- `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §4.4 — Empty-collection identity defaults (`sum=0`, `product=1`, etc.) and `argmin`/`argmax` empty as a compile error. Absorbed into §12.3.

- `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §2.2 — Bind-time vs event-time dynamism, N-max slot machinery applies only to event-time. Absorbed into §12.4.

- `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §2.3 (frontend desugaring) — Per-type pool desugaring of `impl Contract` collections. Absorbed into §12.5.

- `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §7.4 — `softmax`/`weighted_sum` deferred pending collection-aggregation syntax. Absorbed into §12.1 as a forward reference to §35.

- `planning/v2/open_questions.md` — "Restricting the type set" and "`softmax` as a primitive" filed under Tier 2 Collections section. Both are correctly cross-referenced: restricting-type-set is a future refinement not in §12; softmax is deferred to §35.

- `planning/v2/anti_spec.md` — `dyn` retired in favor of `impl Contract` + `some`. §12's use of `impl` and `some` throughout is consistent with this retirement.

- `planning/v2/v2.1_in_progress.md` line ~818 — `any`, `all`, `count`, `argmin`, `argmax` generator-expression syntax. Absorbed into §12.1/§12.3.

- `planning/v2/spec_dev_notes.md` — §12 changelog entry documents all subsections written 2026-04-21, confirming integration is complete and intentional.

---

## Superseded

Content replaced by a newer decision in spec_new.md §12.

- `planning/v2/spec.md` §2.5 (`dyn` keyword for heterogeneous collections) — "When a collection must contain elements with different contract implementations, use `dyn`." Superseded by `impl Contract` in spec_new.md §12.5 and §12 intro. Already listed in `anti_spec.md` ("retired keyword" table, "`dyn` → `impl Contract`"). No further action needed.

- `planning/v2/spec.md` line 371 — "Variable-length collections are out of scope." Superseded by `some` and the bind-time/event-time dynamism design in spec_new.md §12.4. Already flagged in `anti_spec.md` ("Stale in legacy docs — spec.md §12"). No further action needed.

- `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §4.3 — "Return type: The element type. For heterogeneous collections, the contract type (can narrow with `is` check)." The chunk report frames `argmax` as returning the element itself. spec_new.md §12.2 supersedes this with the tagged-handle framing (`pool_identity, intra_pool_index` pair), which is the settled, more precise formulation. The chunk report's "return the element" wording is informal and does not contradict spec_new.md; the spec is more specific. Not a conflict.

- `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §5.1 — Describes the four-verb workflow split (`assume` / `observe` / `learn` / `bind`) and retention of `assume_constant`/`assume_series`. This is workflow-layer content, not §12 content. Not superseded within §12 scope.

---

## Homeless

Corpus content relevant to §12, not accounted for in spec_new.md §12, and not already committed to anti_spec.md.

- `Recommend:` `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §4.2 and §4.4 — `max(expr for x in collection)` and `min(expr for x in collection)` are listed as named aggregation primitives ("New primitives" in §4.2) and their empty-collection behavior is explicitly specified (`min`/`max` empty = undefined, requires guard, in §4.4 table). spec_new.md §12.1 lists only `sum`, `product`, `any`, `all`, `count`, `argmin`, `argmax`. `max` and `min` are absent from §12.1's primitive list, and §12.3's empty-collection defaults table also omits them. The chunk report is the authoritative closed design. This is a stable decision that never made it into §12.1 or §12.3. The gap is real: `max_height = max(t.height for t in trees)` is valid per the chunk but has no spec home.

- `Recommend:` `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §4.4 "Lowering note" — "On JAX/PyTorch, `jax.numpy.where`/`torch.where` evaluates both branches regardless of the condition. The backend emitter must inject safe sentinels into invalid mask slots: `-inf` for `max`/`argmax`, `+inf` for `min`/`argmin`." This is a compiler-implementation constraint, not an open design question. It is a direct consequence of the empty-collection compile-error rule and the N-max/alive-mask lowering (§12.3, §12.4, §21). No sentence in §12 mentions sentinels or the branch-evaluation behavior. Assessment: stable decision, load-bearing for compiler correctness, belongs in §12.3 or §21 lowering prose.

- `Recommend:` `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §4.5 — `count` semantics: "`count(collection)` for a `some`-sized collection means the number of valid (alive) elements, not the backing array length." spec_new.md §12.1 defines `count(xs)` as "cardinality, `Scalar<dimensionless>`" without distinguishing alive-element cardinality from backing-array length. The distinction is invisible to the user but is a stable, documented decision that should appear in §12.1 or a note in §12.4.

- `Recommend:` `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §4.3 — tie-breaking rule: "Ties broken by index order (deterministic)." spec_new.md §12.2 describes tagged-handle machinery but does not state how `argmin`/`argmax` break ties. A deterministic tie-break rule is a stable user-visible semantic.

- `Recommend:` `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §4.3 — differentiability class of `argmin`/`argmax`: "`subgradient`. Gradient flows through the currently-selected element. Discontinuous at switchover points (same class as `max(a, b)` at `a = b`)." spec_new.md §12 does not mention the differentiability class of `argmax`/`argmin`. This is a stable, user-relevant semantic (affects training usage) that has no §12 home.

- `Recommend:` `planning/v2/open_questions.md` (Tier 0, "Collections and the e-graph") — "Is an array of N quantities N separate e-classes, or one e-class of an array? How does indexing interact with e-class membership?" This is flagged as a Tier 0 open question and is legitimately open design work. Not homeless per audit instructions for in-progress work. Included here only to note: if and when the e-graph substrate section lands, §12 will need a cross-reference. No action on §12 now.

- `Recommend:` `planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` — "H1-H2 — aggregate/collection identities" listed as rewrite rule candidates. These are open design work in an in-progress chunk. Not homeless under the audit rules. Noted for cross-reference once chunk 04 resolves H1-H2.

---

## Conflicts

Direct contradictions between spec_new.md §12 and any corpus document.

- `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §4.3 vs spec_new.md §12.3 — The chunk report's empty-collection table (§4.4) lists `min`/`max` as "undefined — requires guard," implying they exist as collection aggregation primitives. spec_new.md §12.3 empty-collection table lists only `sum`, `product`, `any`, `all`, `count`, `argmin`, `argmax` — `min` and `max` are absent. If `min`/`max` are not aggregation primitives (per §12.1's list), there is nothing to specify a guard for; if they are primitives (per chunk 02), §12.3's table is incomplete. The tables are structurally inconsistent: one assumes `min`/`max` exist as collection aggregations, the other's list does not include them. `Recommend:` Decide whether `min` and `max` are first-class collection aggregation primitives (chunk 02 says yes) and add them to §12.1 and §12.3 accordingly, or explicitly retire them from the collection-aggregation surface and add to anti_spec.md. The chunk report's §4.2 and §4.4 represent the settled design session output, so the most likely correct resolution is to add `min`/`max` to §12.1 and §12.3.
