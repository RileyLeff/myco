# Audit — §14 Compiler Intrinsics

Corpus: `planning/soul.md`, `planning/v2/spec.md`,
`planning/v2/spec_dev_notes.md`, `planning/v2/riley_project_note.md`,
`planning/v2/anti_spec.md`, `planning/v2/v2.1_in_progress.md`,
`planning/v2/open_questions.md`, and chunk reports 01-07.

---

## Absorbed

Corpus content that already landed in spec_new.md §14.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  (O2.4, O4.5, ~line 1279):** `condition_of` ships as a single mode-tagged
  intrinsic covering Levels I, II, III; algorithmic/problem duality named;
  `compile_bound` vs `runtime_estimate` tagged in return; Y4
  `condition_weighted` un-deferred. Absorbed into §14.1.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  (~line 1150):** `loss_of(expr)` returns a struct of named fields, not a
  scalar; aggregation to a scalar happens at workflow extraction, not in the
  language. Absorbed into §14.2.

- **`planning/v2/v2.1_in_progress.md` (~line 807, "settled"):**
  `integrate(expr, over = domain)` — symbolic first, numerical quadrature
  fallback; unit algebra mechanical. Absorbed into §14.3.

- **`planning/v2/spec_dev_notes.md` (line 290-294):** Changelog entry
  documents §14's scope: 14.1 `condition_of` with mode-tagged return and
  duality, 14.2 `loss_of` named-field return, 14.3 `integrate` domain / unit
  algebra / integration-by-parts as e-graph rewrite. Matches §14.1-14.3
  structure.

- **`planning/v2/anti_spec.md` (line 71):** `condition_weighted` deferral
  listed as "resolved — ships via `condition_of` Levels I-III (chunk 04
  O4.5)." Consistent with §14.1 un-deferring Y4.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  (~line 1037-1058):** Three-level breakdown of `condition_of` —
  Level I scalar symbolic, Level II vector Jacobian-norm (both compile-time),
  Level III `condest`-style runtime estimate gated on B5. §14.1 reflects this
  split.

- **`planning/v2/spec.md` (~line 2260-2275):** `integrate` quadrature
  fallback, configurable strategy, Gauss-Legendre default, compilation plan
  reporting. The strategy-configurable behavior (§14.3 "integration-by-parts
  fires as a stdlib rewrite") is the spec_new.md form of this.

---

## Superseded

Corpus content replaced by a newer decision; should move to anti_spec.md if
not already there.

- **`planning/v2/v2.1_in_progress.md` (lines 1015-1021):**
  > "`condition_weighted` deferred. Weighting paths by numerical conditioning
  > requires a `condition_of(expr)` primitive ... Ship `condition_weighted`
  > post-v2.1 if demand emerges."
  Superseded by §14.1, which resolves `condition_weighted` via `condition_of`
  Levels I-III (O4.5 closed). Already noted in `anti_spec.md` (line 71) as
  resolved, so no new anti_spec.md entry is needed.

- **`planning/v2/open_questions.md` (lines 525-530):**
  > "`condition_weighted` deferred beyond v2.1 ... requires either a
  > `condition_of(expr)` compiler intrinsic ... or a compiler-provided black
  > box."
  Superseded by §14.1 (O4.5 resolved). Already noted in `anti_spec.md`
  line 71. No new anti_spec.md entry needed.

- **`planning/v2/open_questions.md` (lines 588-592):**
  > "metadata-aware policies (e.g., `condition_weighted`) are deferred
  > compiler intrinsics, not `.myco`-level features. Spec §14.6 stdlib list:
  > Drop `condition_weighted` from the v2.1 stdlib policy list."
  The old §14.6 stdlib-policy-list maintenance note is superseded by §14.1,
  which makes `condition_of` an intrinsic and `condition_weighted` a named
  consumer of its output. No separate anti_spec.md entry needed; already
  covered by the O4.5 closure note.

- **`planning/v2/spec.md` (~line 2230):** Backend-specific emission
  (`jax.jacfwd` over `custom_root`) for runtime autodiff through SCCs. §14
  spec_new.md does not mention `jax.jacfwd` — the backend-abstraction work
  (chunk 06) replaced JAX-as-primary with a backend trait. Already in
  `anti_spec.md` (line 53, "JAX-as-primary emitter retired").

- **`planning/v2/v2.1_in_progress.md` (~line 802-804):**
  > "`deriv(A, B)` — Resolved at compile time via chain rule on the
  > expression graph. No runtime cost. Always symbolic — all registered
  > functions have transparent bodies."
  The "always symbolic / no runtime cost" framing is superseded by §14's
  implied model (and by `spec.md` §9.5) where `deriv` can fall back to
  runtime AD for large-SCC paths. The `v2.1_in_progress.md` entry predates
  the hierarchical SCC decomposition design.
  **Recommend adding to `anti_spec.md`:** retire the "always symbolic" framing
  for `deriv`; document that runtime AD is the fallback for large SCCs.

---

## Homeless

Content relevant to §14, not accounted for in spec_new.md §14, and not
committed to `anti_spec.md`.

- **`planning/v2/spec.md` §9.5 (lines 2196-2294) — `deriv` full design:**
  `spec.md` contains a complete `deriv` specification: chain-rule resolution,
  three lowering modes (acyclic symbolic / SCC-external IFT / SCC-internal
  hierarchical decomposition), scope and limitations (cannot diff through
  opaque callables or underdetermined residuals), and the compilation-plan
  report. spec_new.md §14 has no `deriv` subsection at all. `deriv` is
  listed in the §14 summary blurb but has no dedicated treatment.
  **Recommend:** add §14.4 `deriv` to spec_new.md covering the three
  lowering modes, SCC interaction, scope/limitations, and compilation-plan
  reporting. This is a stable, locked design (status "settled" in
  `v2.1_in_progress.md:802`) that never migrated from spec.md to spec_new.md.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  (~line 1060-1076) — `condition_of` consumer mapping:**
  The O2.4 lock specifies a detailed consumer mapping: closure policies
  (Y4, Y5) consume `compile_bound` (Levels I-II); workflow solver diagnostics
  and runtime hooks consume `runtime_estimate` (Level III); extraction-time
  ranking consumes `compile_bound` only; `mycoc explain` shows both modes.
  spec_new.md §14.1 names Y4 `condition_weighted` as "primary consumer" but
  omits the extraction-ranking and diagnostics consumers, and does not state
  that Level III is unavailable to closure policies.
  **Recommend:** expand §14.1 to include the consumer table; clarify that
  Level III output is not available to `.myco`-level closure policies.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  (~line 1079-1081) — documentation obligation for duality worked example:**
  > "The spec's condition-number section must name the algorithmic/problem
  > duality and show one worked example of each mode."
  spec_new.md §14.1 names the duality but contains no worked example.
  The O2.4 lock explicitly required one (e.g., `(exp(x)-1)/x` vs
  `expm1(x)/x` for algorithmic; a linear-solve block for problem
  conditioning).
  **Recommend:** add a brief worked example pair to §14.1 per the O2.4
  documentation obligation.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  (~line 1060-1062) — `condition_of` return struct shape:**
  O2.4 locks `condition_of` returning `{mode: "compile_bound" |
  "runtime_estimate", ...}`. spec_new.md §14.1 says `.level` surfaces
  which tier was chosen, but uses different field naming than the locked
  design (`level` vs `mode`). The locked design tags the return with
  `mode`; the spec uses `level`. This is either a naming mismatch or an
  undocumented renaming.
  **Recommend:** reconcile the field name; either update §14.1 to use
  `mode`, or add a note that `level` is the `.myco`-facing accessor for
  the `mode` tag.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  (~line 1137-1148) — `loss_of` field list mismatch:**
  O2.4 locks five named fields on the `loss_of` return:
  `compute`, `approximation`, `condition`, `truncation`, `discretization`.
  spec_new.md §14.2 defines three different fields: `data_fit`,
  `constraint_violation`, `regularization`. These are semantically
  distinct things. The O2.4 fields are *extraction-cost dimensions*
  (numerical analysis quantities). The spec_new.md §14.2 fields look like
  *training-loss components* (ML/inference quantities). The two structs
  may both be intended, but there is no statement in spec_new.md that both
  exist, that they serve different consumers, or how they relate.
  **Recommend:** clarify in §14.2 whether `loss_of` has two distinct return
  shapes (one for extraction cost, one for training loss) or one unified
  struct. If unified, reconcile the field names with O2.4. If distinct,
  name both structs explicitly and document which is used for extraction
  ranking vs training emission.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md`
  (~lines 178-234) — AD ownership decision (Option C) still open:**
  The backend chunk identifies "AD ownership" as an unresolved fork (Options
  A, B, C) with a lean toward Option C (hybrid: Myco owns AD for compile-time
  analysis; backend owns AD for runtime execution). This directly affects
  `deriv`'s lowering guarantees: under Option A, `deriv` always lowers
  symbolically; under Options B/C, runtime gradient execution is backend-
  delegated. spec_new.md §14 does not acknowledge this dependency.
  This is an open design item in an in-progress chunk report, so it is
  not homeless in the strict sense, but spec_new.md §14 should at minimum
  contain a forward reference acknowledging that `deriv`'s runtime lowering
  is gated on the AD-ownership decision (B6).
  **Recommend:** add a note in §14 (or §14.4 once created) that runtime
  fallback for `deriv` through large SCCs depends on the AD-ownership
  decision tracked in B6.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  (~line 1306-1308) — `Dual<T>` / `deriv` relation open:**
  > "`Dual<T>` listed in the hierarchy; its relation to the `deriv`
  > intrinsic and autodiff rewrites needs its own pass."
  This is explicitly an open item in the chunk report, so it is legitimately
  in-progress. Not flagged as homeless, noted for completeness.

---

## Conflicts

Direct contradictions between spec_new.md §14 and corpus documents.

- **`loss_of` field inventory — spec_new.md §14.2 vs chunk 04 O2.4:**
  spec_new.md §14.2 defines `loss_of(residual)` as returning
  `{data_fit, constraint_violation, regularization}`.
  `04_egraph_foundation_in_progress.md` O2.4 (~line 1140-1148) locks
  `loss_of(expr)` as returning
  `{compute, approximation, condition, truncation, discretization}`.
  These are distinct field sets with distinct semantics. Neither document
  acknowledges the other's field set.
  **Recommend:** treat as a design ambiguity requiring resolution before
  §14.2 is considered stable. The O2.4 struct describes extraction-cost
  dimensions; the spec_new.md struct describes training-loss sources.
  Both may be valid constructs, but if they share the `loss_of` name with
  different signatures the language has an unresolved overload. Decide:
  (a) one `loss_of` with a unified field set; (b) two distinct intrinsics
  (`loss_of` for training, `cost_of` or similar for extraction); or (c)
  one intrinsic that returns both families under distinct sub-structs.
  Document the resolution in spec_new.md §14.2 and update anti_spec.md to
  retire whichever variant is dropped.

- **`condition_of` return accessor naming — spec_new.md §14.1 vs chunk 04
  O2.4:**
  spec_new.md §14.1 states: "`condition_of(expr).level` surfaces which tier
  the compiler chose."
  `04_egraph_foundation_in_progress.md` O2.4 (~line 1060-1062) locks the
  return struct as `{mode: "compile_bound" | "runtime_estimate", ...}`,
  not `{level: ...}`.
  **Recommend:** align the accessor name. If the field was intentionally
  renamed from `mode` to `level` during authoring of spec_new.md §14, add
  a changelog note to spec_dev_notes.md. If it was a drafting inconsistency,
  revert to `mode` per the O2.4 lock.
