# Audit Report — §26 Numeric Types

Audited against corpus as of 2026-04-22.

---

## Absorbed

Corpus content that already landed in spec_new.md §26.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §8 ("Numeric hierarchy (stdlib)"):**
  > "Numeric  (contract: +, -, *, /, ==, <, zero, one) ├── Discrete ├── Exact └── Rational (stdlib — compiler + user) ├── Continuous │   ├── Float │   │   ├── Float{16,32,64} │   │   ├── BigFloat<precision> ... └── Complex"

  Absorbed into §26.1's seven-rep table (`Bool`, `Integer`, `Rational`, `Float32`, `Float64`, `BigFloat`, `Complex`) and the §26.2 base `Numeric` contract hierarchy (ring closure, total ordering, zero/one identity, backend representability).

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §8 ("`Scalar<U, T>` with default `T = Float64`"):**
  > "Quantity types take two generic parameters: unit `U` and numeric representation `T`. Mocks that write `Scalar<U>` use the `T = Float64` default."

  Absorbed into §26 preamble and §26.1: "`Scalar<U, T = Float64>` takes an explicit numeric representation parameter with `Float64` as default" and "Default `T = Float64` is per-Scalar, not module-wide."

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §8 ("BigNumber tiers"):**
  > "Rational — stdlib, load-bearing for the compiler. The e-graph uses rationals internally for constant folding during saturation to avoid float rounding (`0.1 + 0.2 = 0.3` exactly in rational, not in Float64). Also user-facing for exact symbolic work."
  > "BigFloat — stdlib, CPU-only. ... Using `BigFloat` in a relation disables GPU lowering for that subgraph."

  Absorbed into the §26.1 table (`Rational`: "§26.3 termination caveat; GPU-incompatible"; `BigFloat`: "exact rounding semantics; GPU-incompatible") and §26.3's GPU-incompatibility surface bullet.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §8 ("Saturation termination concern"):**
  > "Rational arithmetic grows denominators unboundedly (coprime additions). The e-graph needs a **precision cap** or **canonical-form simplification** to prevent non-terminating saturation. Specific policy is **Section 12 open**."

  Absorbed into §26.3: "`Rational` arithmetic is exact but unbounded. Numerator and denominator grow with each non-trivial operation; iterated exact arithmetic can blow up representation size." The cap policy itself is still tracked as open in §35.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §8 ("Generic conversions with bounds"):**
  > "`convert Celsius<T: Float> <-> Kelvin<T>`" and "Compile-time selection: `Celsius<Float32>` matches the stdlib rule ... No silent lossiness."

  Absorbed into §26.2's explicit-conversion rule: "Mixed-T arithmetic is a compile error; the user must write `convert T1 -> T2` explicitly. ... `Scalar<m, Float32>` and `Scalar<m, Float64>` do not silently promote."

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §7 lossiness table row:**
  > "Lossy-tol × Bi | ... `approximate Float64 <-> Float32 tolerance_class precision_downcast`"

  Absorbed into §26.2: "Conversion `Float32 -> Float64` is lossless; `Float64 -> Float32` emits the standard lossy-tolerance envelope (§15.3)."

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 (settled items 13, 14):**
  > "13 | Numeric hierarchy sketched; `Scalar<U, T>` with `T = Float64` default"
  > "14 | Rational + BigFloat in stdlib; BigInt / BigDecimal as extensions"

  Absorbed into §26.1. Note that the extension-only status of `BigInt` / `BigDecimal` is not restated in §26.1 but is consistent with the seven-rep stdlib enumeration.

- **`planning/v2/anti_spec.md` retired entry ("user-facing `Dual` numeric representation"):**
  > "user-facing `Dual` numeric representation | backend-owned AD | Part V commits backend-delegated AD (burn-style tensor tracks operations); user-facing `Dual` would duplicate backend machinery"

  Absorbed into §26.1: "Forward-mode AD is not a user-facing representation. Backends own AD (§31); dual numbers would duplicate what the backend already provides. Retired to anti_spec.md."

- **`planning/v2/spec_new.md` §15.2 source 3 (numeric type choices as lossiness source):**
  > "`Float64` arithmetic carries unit-in-last-place rounding; `Rational` is exact (with termination caveats, §26)."

  §26 is the target of this cross-reference; the claim lives in §15.2 and §26.3 together.

---

## Superseded

Corpus content replaced by a newer decision in spec_new.md §26. Should move to anti_spec.md if not already there.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §8 numeric hierarchy tree including `Dual<T>`:**
  > "├── Float │   │   ├── Float{16,32,64} │   │   ├── BigFloat<precision> (stdlib, CPU-only, opt-in) │   │   └── Dual<T> (autodiff)"

  Superseded by §26.1: `Dual<T>` is dropped from the user-facing hierarchy entirely. The retirement is already recorded in anti_spec.md ("user-facing `Dual` numeric representation | backend-owned AD"). No further action needed.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 open item ("Dual numbers for autodiff"):**
  > "`Dual<T>` listed in the hierarchy; its relation to the `deriv` intrinsic and autodiff rewrites needs its own pass."

  Superseded by the chunk 04 B6 backend-AD ownership lock and the anti_spec.md retirement. The open question is closed; §26.1 carries the final decision.

- **`planning/v2/spec.md` §4.3 ("`Scalar<U>` — the parameterized quantity type"):**
  > "`Scalar<U>` is the built-in parameterized type meaning 'a real number measured in unit U.' Every quantity in the world model has a `Scalar<U>` type."

  Superseded by §26 preamble: `Scalar<U>` is now `Scalar<U, T = Float64>` with an explicit numeric representation parameter. The single-parameter form is retained as the default-T surface but the underlying primitive is two-parameter. The legacy §4.3 one-parameter framing treats numeric representation as fixed; the new framing makes it per-Scalar configurable.

  `Recommend:` This is part of the broader spec.md §2.5 / §4.11 / §7 / §5.5 wholesale supersede already captured in anti_spec.md ("Stale in legacy docs (do not import): `spec.md` §2.5, §4.11, §7, §5.5, §8.5, §6.3, §12, §13.2-13.3, §14.6, Appendix A/C — supersede wholesale"). No new anti_spec.md entry required.

- **`planning/v2/anti_spec.md` retired entry ("dimensionless-ratio literal carve-out"):**
  > "dimensionless-ratio literal carve-out (`0.5`, `2.0` in a dimensionless expression) | CC1 applies uniformly: bind the ratio as a universal"

  Already retired; §26 does not reopen it. The numeric-type parameterization does not create a literal carve-out at any dimension or representation. Included here to document that §26 respects the retirement.

---

## Homeless

Corpus content relevant to §26, not accounted for in spec_new.md §26, and not already committed to anti_spec.md.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §8 ("Numeric hierarchy (stdlib)") — integer widths and unsigned:**
  > "├── Integer │       ├── Int{8,16,32,64} │       ├── UInt{8,16,32,64} │       └── BigInt (extension module, not base stdlib)"

  §26.1's table has one `Integer` row labeled "arbitrary-precision integers." The chunk 04 hierarchy splits into fixed-width signed (`Int8`/`Int16`/`Int32`/`Int64`), fixed-width unsigned (`UInt8`-`UInt64`), and arbitrary-precision (`BigInt`, extension). §26.1 collapses these into one row and implies arbitrary-precision ("exact; GPU-incompatible for arbitrary precision") but does not state that fixed-width integer types are the GPU-compatible form, nor whether signed/unsigned are distinct reps.

  `Recommend:` Clarify in §26.1 whether `Integer` is a single stdlib representation at arbitrary precision, or whether fixed-width `Int{8,16,32,64}` / `UInt{8,16,32,64}` are additional reps. The chunk 04 table is more granular and settled; §26.1's collapsed form is ambiguous about GPU compatibility for fixed-width cases.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §8 ("BigNumber tiers") — `BigInt` / `BigDecimal` as extensions:**
  > "BigInt / BigDecimal — extension modules, not base stdlib. The domain does not require them by default."

  §26.1's seven-rep table does not mention `BigInt` or `BigDecimal`, nor does it state that `Integer` in the table stands for the arbitrary-precision form and that fixed-width integer widths live elsewhere. The extension-module status of `BigInt` / `BigDecimal` is settled chunk 04 content that §26 elides.

  `Recommend:` Add a note to §26.1 stating that `BigInt` and `BigDecimal` are extension modules beyond the seven stdlib reps, matching chunk 04 settled item 14.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §8 ("BigNumber tiers") — rational used internally by the e-graph:**
  > "The e-graph uses rationals internally for constant folding during saturation to avoid float rounding (`0.1 + 0.2 = 0.3` exactly in rational, not in Float64)."

  This is a compiler-internal use of `Rational` distinct from user-facing `Rational` state. §26 describes user-facing `Rational` (termination caveat on temporal state, GPU incompatibility) but does not mention that the e-graph's own constant folder relies on `Rational` internally. The internal use is the origin of the `Rational`-denominator saturation concern in §19.4 and §35.

  `Recommend:` Add a short bullet to §26.3 or §26.1 noting that `Rational` is also used internally by the e-graph for exact constant folding during saturation, which is the source of the saturation termination concern cross-referenced in §19.4 and tracked in §35. Without this note, a reader of §26 alone cannot tell why rational-denominator growth is a whole-compiler concern rather than a user code concern.

- **`planning/v2/spec_new.md` §35 "Other Opens" ("Complex numeric representation in scope"):**
  > "`Complex` ships; open items are which contracts it satisfies (not totally ordered, so `Compare`-dependent stdlib functions exclude it; which of `Plus` / `Minus` / `Times` / `Divide` close; interaction with units in `Scalar<U, Complex>`), backend support commitments, and whether `Complex` forms a separate `Numeric` sub-hierarchy or lives alongside `Float`."

  §26.1 lists `Complex` in the seven-rep table with the note "in scope, representation and contracts open (§35)." §26.2 states "Complex T does not satisfy total ordering; stdlib functions requiring it accept only ordered T." This is one of the sub-questions §35 tags as open; §26.2 resolves it in passing. The other `Complex` opens (ring closure for Plus/Minus/Times/Divide, unit interaction in `Scalar<U, Complex>`, hierarchy placement) are deferred.

  `Recommend:` Verify §35's open-item text remains accurate after §26.2's total-ordering resolution. The `Compare` exclusion is now locked; §35 should either drop that sub-item or restate the remaining opens (ring closure, unit interaction, hierarchy placement) without it.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 open numeric-specific item ("GPU-incompatibility of BigFloat and Rational"):**
  > "Hard-error on GPU target? Fall-back-to-CPU with warning? (Leaning hard-error to avoid silent performance catastrophes.)"

  §26.3 locks the direction as hard error: "Using `Rational` in an SCC that targets a GPU backend is a workflow-composition error." §31.1 backend-fallback modes (`error` / `host` / `emulate`) provide the mechanism: `error` is the default for missing capabilities, `host` routes CPU-only families like `Rational` back to the host. §26 does not cross-reference §31.1's `host` fallback mode as an opt-in escape from the hard error.

  `Recommend:` Add to §26.3's GPU-incompatibility bullet a cross-reference to §31.1's `host` fallback mode, which §31.1 already explicitly calls out ("Useful for CPU-only families (e.g. `Rational` arithmetic, §26)"). Currently the cross-reference points from §31 to §26 but not back; §26.3 reads as an unconditional error when the workflow can opt into host fallback.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.3 (AD ownership Option B):**
  > "Delegate to backend AD. Myco emits the forward graph; backend handles backward pass via its native AD (JAX `grad`, PyTorch autograd, burn autodiff, Enzyme)."

  §26.1 states backends own AD as justification for retiring `Dual`. The backend-AD lock lives in chunk 06 §4.3 and Part V §31-32. §26.1's cross-reference is to §31, but the actual ownership decision is chunk 06 B6. This is a minor bookkeeping gap: §26.1 relies on a decision made elsewhere without citing the locking chunk.

  `Recommend:` Optional — §26.1 could add a parenthetical "(chunk 06 B6; §31)" to surface the origin of the backend-AD commitment. Low priority; the current §31 cross-ref is functional.

- **`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §3.5 — precision conversion on tensors:**
  > "Precision (`Matrix<Float64, m, n> ↔ Matrix<Float32, m, n>`). Lossy; already covered by `approximate` block surface from chunk 04 §9. Language-level `convert` probably refuses this; the `approximate` block handles it."

  §26.2 states `Float64 -> Float32` "emits the standard lossy-tolerance envelope (§15.3)," implying bare `convert` handles it. Chunk 05 §3.5 has `convert` refusing precision downcast on matrices in favor of `approximate`. There is a potential inconsistency between the scalar-level §26.2 treatment (bare `convert` emits tolerance envelope) and the tensor-level chunk 05 treatment (`convert` refuses; `approximate` required).

  `Recommend:` Reconcile §26.2 with chunk 05 §3.5. Either scalar `Float64 -> Float32` also requires `approximate`, or tensor precision downcast also goes through bare `convert` with an automatic tolerance envelope. The two shouldn't diverge on the same operation.

---

## Conflicts

Direct contradictions between spec_new.md §26 and any corpus document.

- **§26.2 `Float64 -> Float32` via `convert` vs. chunk 05 §3.5 precision downcast via `approximate`:**

  §26.2 says: "Conversion `Float32 -> Float64` is lossless; `Float64 -> Float32` emits the standard lossy-tolerance envelope (§15.3)." This is phrased as a property of `convert T1 -> T2`. Chunk 05 §3.5 says precision conversion is "Lossy; already covered by `approximate` block surface from chunk 04 §9. Language-level `convert` probably refuses this; the `approximate` block handles it." Chunk 04 §7 also places `approximate Float64 <-> Float32 tolerance_class precision_downcast` in the lossy-tolerance row, which authorizes `approximate` rather than `convert`.

  §26.2 treats precision downcast as a bare-`convert` operation that emits a tolerance envelope. Chunks 04 and 05 treat it as an `approximate` block (tolerance_class row of the 2×3 lossiness matrix). These framings disagree on the syntactic surface that authorizes the conversion.

  `Recommend:` Resolve which surface authorizes `Float64 -> Float32`. Three options: (a) bare `convert` emits tolerance envelope automatically — the §26.2 framing, consistent with §15.3's tolerance-envelope machinery; (b) `approximate ... tolerance_class:` is required — the chunk 04/05 framing, consistent with the 2×3 matrix's lossy-tolerance cell; (c) `convert` is allowed on scalars (§26.2) but not on tensors (chunk 05 §3.5), with the distinction stated explicitly. Option (a) is the simplest; option (c) creates an asymmetry between scalar and tensor operations that should be flagged in the spec if adopted.

- **§26.1 table row for `Integer` vs. chunk 04 §8 integer sub-hierarchy:**

  §26.1's table has a single `Integer` row: "arbitrary-precision integers | exact; GPU-incompatible for arbitrary precision." Chunk 04 §8 has `Integer` as a parent containing `Int{8,16,32,64}`, `UInt{8,16,32,64}`, and `BigInt` (extension). The chunk 04 sub-hierarchy treats fixed-width integers as the GPU-compatible stdlib forms and `BigInt` as the arbitrary-precision extension. §26.1's "arbitrary-precision integers | GPU-incompatible for arbitrary precision" collapses this: it implies `Integer` means arbitrary-precision and fixed-width integer types are not stdlib reps.

  `Recommend:` Clarify §26.1's `Integer` row. If the intent is to ship `Integer` as arbitrary-precision only (matching chunk 04's `BigInt` which was called an extension), the chunk 04 settled-items table entry 14 ("Rational + BigFloat in stdlib; BigInt / BigDecimal as extensions") is contradicted. If the intent is to ship fixed-width integers as stdlib and label `BigInt` as extension, §26.1's single-row "arbitrary-precision integers" label is wrong. The most defensible resolution is to split the `Integer` row into fixed-width (GPU-compatible) and `BigInt` (extension, CPU-only), matching chunk 04 §8 exactly.
