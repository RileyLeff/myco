# Audit: §7 Contracts

Auditing `spec_new.md` §7 (subsections 7.1–7.4) against the full corpus.

---

## Absorbed

Content already reflected in spec_new.md §7.

- **`planning/v2/v2.1_in_progress.md` (line 333)** — `"Multi-contract satisfaction — \`: A + B + C\`"`: §7 summary line states `"multi-contract satisfaction (\`: A + B + C\`)"`. Absorbed.

- **`planning/v2/v2.1_in_progress.md` (line 349)** — `"Supertraits — \`contract B : A + ...\`"`: §7.3 covers `contract B : A` with diamond resolution. Absorbed.

- **`planning/v2/v2.1_in_progress.md` (line 371)** — parameterized contracts (`contract CanopyModel<R: CanopyRadiation>`): §7.1 covers parameterized contracts with `Invertible<T>`, `Convert<From, To>`, `Distribution<U>`. Absorbed.

- **`planning/v2/spec_dev_notes.md` (line 23)** — `"Extend composable contracts uniformly to functions. Stdlib atoms … carry capability contracts like \`Invertible<_>\`, \`Differentiable\`, \`Monotone\`."`: §7.2 names these three function-side contracts and states they `"drive function-inverse rewrites (§17 merge source 5) and \`deriv\` / \`condition_of\` intrinsics (§14)."` Absorbed.

- **`planning/v2/spec_dev_notes.md` (line 16)** — distribution-side capability contracts (`Distribution<U>` + supertrait chain CC4): §7.2 lists `AffineSelfClosed`, `SumSelfClosed`, `ProductSelfClosed`, `ScaleSelfClosed`, `SmoothTransformable`, `ReparameterizedSampleable` and names Tier A closed-form routing. Absorbed.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` (line 523)** — `"Distribution capabilities decomposed into multiple contracts"`: reflected in §7.2's distribution-side supertrait list. Absorbed.

- **`planning/v2/open_questions.md` (line 563)** — `"Add data contracts (output-only), multi-contract satisfaction … supertraits"` to spec contracts section: data contracts retired to anti_spec.md (see Superseded); the remaining two items (multi-contract + supertraits) are absorbed in §7.

- **`planning/v2/anti_spec.md` (line 46)** — `"contract composition alias (\`contract C := A + B\`)"` retired: §7 omits this construct. The retirement decision is faithfully not present in §7. Absorbed (by omission).

---

## Superseded

Corpus content replaced by a newer decision; should be (or already is) in anti_spec.md.

- **`planning/v2/v2.1_in_progress.md` (line 366)** — `"Contract composition alias — \`contract X = A + B\`… Status: provisional — include if it proves useful, defer otherwise."` This provisional stance is superseded: `anti_spec.md` (line 46) retires the alias as redundant. Already in anti_spec.md — no action needed.

- **`planning/v2/v2.1_in_progress.md` (lines 317–330)** — `"Data contracts (output-only)… Status: settled."` Superseded: `anti_spec.md` (line 25) retires `DataContract` / "data contract" as a distinct contract kind, noting plain contracts + output-type annotations are sufficient. The retirement text in anti_spec.md cites the investigation that closed the question. Already in anti_spec.md — no action needed.

- **`planning/v2/open_questions.md` (lines 343–358)** — `"Callable reuse across studies (shared-controller portability) — RESOLVED (data contracts)"`. Superseded: the underlying data-contract mechanism is now retired; `bind_controller` visibility is governed by a plain contract. The open question itself is marked resolved but still frames the solution in data-contract terms. This is corpus drift (open_questions.md references the old framing). The relevant retirement is already in anti_spec.md. The open_questions.md entry is stale prose, not a live spec claim — no anti_spec action needed, but open_questions.md should not be imported.

- **`planning/v2/spec.md` (lines 440–595, §3.4)** — the detailed contract-invocation semantics (wiring pattern, flattener rule, wired vs invoked intermediates, named arguments, disambiguating wiring from constraining). These are spec.md §3.4 content, not §7 material; spec_dev_notes.md notes `spec.md §7` is wholesale superseded. In context of §7 specifically, any spec.md §7 content (Slots, line 1652) is superseded by the slot retirement in anti_spec.md. Already in anti_spec.md — no action needed.

- **`planning/v2/v2.1_in_progress.md` (line 394)** — function annotation blocks (`invertibility: bijective`, `differentiability: smooth`). Superseded by spec_dev_notes.md decision: no annotation surface; capability contracts on stdlib atoms replace user-declared annotations. Retirement of `#[...]` attributes and four-class metadata is already in anti_spec.md (lines 30–35). No further action needed.

- **`planning/v2/open_questions.md` (line 525)** — `"\`condition_weighted\` deferred beyond v2.1."` Superseded: `anti_spec.md` (line 71) notes this is `"resolved — ships via \`condition_of\` Levels I-III (chunk 04 O4.5)"`, and chunk 04 line 532 confirms Y4 is un-deferred. The open_questions.md deferral prose is stale. Not a §7 issue (condition_weighted is a closure policy in §8), but flagged for completeness. Already reflected in anti_spec.md.

---

## Homeless

Corpus content relevant to §7, not accounted for in spec_new.md §7, and not committed to anti_spec.md.

- **`planning/v2/v2.1_in_progress.md` (line 361)** — `"Diamond inheritance is fine because contracts carry no implementation — if two ancestors declare the same-named field with the same type, it's a no-op; different types on the same name is a compile error."` §7.3 states diamond resolution by contract identity (one obligation per supertrait) but does not specify the no-op vs compile-error rule for same-name fields encountered via two paths. The distinction matters: same name + same type is a no-op; same name + different type is a coherence error. §7.4 covers the `A + B` coherence case but not the supertrait diamond case specifically.
  Recommend: add a one-sentence note to §7.3 (or §7.4) specifying that diamond-inherited same-name same-type fields merge to one obligation (no-op) while same-name different-type fields emit a coherence error naming the conflicting ancestors.

- **`planning/v2/v2.1_in_progress.md` (line 484)** — `"Contracts cannot declare \`initial\`/\`temporal\`… interface and implementation cleanly separated."` Contracts are purely structural; state evolution belongs to types. This is a meaningful constraint on what a contract body may contain, absent from §7.
  Recommend: add a sentence to §7 (body-level, under 7.1 or a short §7.0 overview) noting that contract declarations are restricted to typed field obligations and supertraits; `initial`, `temporal`, and implementation bodies are not valid in a contract declaration.

- **`planning/v2/spec_dev_notes.md` (line 44)** — `"§7 Contracts: extended to note contracts apply uniformly to types, functions, and distribution families."` The §7 summary line does state this, but the subsections 7.1–7.4 do not include an explicit statement about the uniform application scope — only §7.2's capability-contracts subsection implicitly covers it through examples.
  Recommend: promote the uniform-application statement (types, functions, distribution families) into the §7 preamble prose so the scope is stated before the parameterized/capability subsections.

- **`planning/v2/v2.1_in_progress.md` (line 309)** — `"Default relation implementations. Included if and only if the implementing type does not provide its own."` Contract default implementations (§3.4.1 in spec.md) are load-bearing but absent from §7 of spec_new.md. Default relations are a property of the contract mechanism, not of the type system.
  Recommend: add a short §7.x on contract default implementations (the fallback-not-override rule) to spec_new.md §7. The concept is currently housed only in spec.md §3.4.1, which anti_spec.md flags as wholesale-superseded.

- **`planning/v2/v2.1_chunk_reports/07_type_graph_in_progress.md` (lines 135–154)** — `"Contract satisfaction for generics. A generic function … which implementation of \`inverse\` is selected? That's a type-graph query … not impl."` This identifies an open design question about how the type graph resolves contract satisfaction for generic instantiations. This is an in-progress open question, not a stable decision, so it is not homeless per the chunk-report exclusion rule. No flag needed.

- **`planning/v2/spec_dev_notes.md` (line 219)** — `"Named-type equality / comparison rules (DEFERRED — decide §3 vs §7 later)"`. This is an explicitly deferred placement decision, not a missing stable decision. Not homeless.

---

## Conflicts

Direct contradictions between spec_new.md §7 and corpus documents.

- **`planning/v2/anti_spec.md` (line 71)** vs **`planning/v2/open_questions.md` (line 525)**:
  - anti_spec.md: `"\`condition_weighted\` deferred — resolved — ships via \`condition_of\` Levels I-III (chunk 04 O4.5)"`
  - open_questions.md: `"\`condition_weighted\` deferred beyond v2.1. Conditioning-aware weighting requires either a \`condition_of(expr)\` compiler intrinsic … Both have real cost."`
  These two documents directly contradict each other on whether `condition_weighted` ships in v2.1. The chunk 04 report (line 532) confirms the un-deferral: `"Y4 \`condition_weighted\` un-deferred and ships in v2.1."` The open_questions.md entry is stale.
  Recommend: update open_questions.md to mark the `condition_weighted` entry resolved (consistent with anti_spec.md and chunk 04). This is not a §7 conflict but surfaces here because the closure policy topic is adjacent and the contradiction is found only in corpus cross-reference during §7 audit.

- **`planning/v2/v2.1_in_progress.md` (line 366)** — `"Contract composition alias — \`contract X = A + B\`… Status: provisional"` vs **`planning/v2/anti_spec.md` (line 46)** — `"contract composition alias (\`contract C := A + B\`) … nothing"`:
  v2.1_in_progress.md marks the alias provisional (may ship); anti_spec.md retires it with no replacement. These are in direct conflict. Spec_new.md §7 correctly omits the alias (following anti_spec.md). The v2.1_in_progress.md entry is stale.
  Recommend: no §7 change needed. Annotate or note in v2.1_in_progress.md that the composition alias is retired per anti_spec.md.
