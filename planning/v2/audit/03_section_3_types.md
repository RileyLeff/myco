# Audit: spec_new.md §3 ("Types")

Corpus files checked: `planning/soul.md`, `planning/v2/spec.md`,
`planning/v2/spec_dev_notes.md`, `planning/v2/riley_project_note.md`,
`planning/v2/anti_spec.md`, `planning/v2/v2.1_in_progress.md`,
`planning/v2/open_questions.md`, and chunk reports 01-07.

---

## Absorbed

Content from the corpus that already landed in spec_new.md §3.

- **`v2.1_in_progress.md` (lines 83-113): named-type newtype and conservation group design.** The `type FishMass : Mass` hierarchy with `{ conserved }`, cross-sibling arithmetic prohibition, bare-convert siblings, and compiler balance checking at rule boundaries all appear in §3.7 verbatim.

- **`v2.1_in_progress.md` (lines 117-169): refinement types with `where { predicate }`.** The `type UnitInterval = Scalar<dimensionless> where { 0 <= self <= 1 }` pattern, runtime-universal bounds, and stochastic composition via `~` are all reflected in §3.2.

- **`v2.1_in_progress.md` (lines 236-258): named generic argument rule (positional for 1 param, named required for 2+).** The rule and its rationale appear in §3.6 by cross-reference to §3.1's summary and are referenced throughout.

- **`v2.1_in_progress.md` (lines 286-291): `impl Contract` replacing `dyn` for static-type heterogeneity; `some T` for runtime sizing.** The two-axis decomposition and the compose rule (`Collection<some (impl Plant)>`) appear in §3.5.

- **`v2.1_in_progress.md` (lines 93-99): `node name: Type` instantiation and identity semantics.** Durable identity surviving timesteps and e-graph merges, plus the distinction from type aliasing, appear in §3.4.

- **`anti_spec.md` (line 11): `dyn` retired in favor of `impl Contract` + `some`.** §3.5 positively replaces `dyn` with the two-operator design and explicitly names the retirement.

- **`spec_dev_notes.md` (2026-04-21 note): §3.9 Matrix Structural Subtype Lattice added.** The lattice table (Symmetric, PosDef, PSD, Upper/Lower Triangular, Diagonal, Orthogonal, Sparse, Banded), meet composition rules, and dispatch rule for `solve` appear in §3.9.

- **`open_questions.md` (Type System section, "Dynamic matrix shapes").** The open question about fixed-shape vs dynamic-shape interaction with the structural lattice is acknowledged in §3.9 under "Shape refinements (§3 generics) — Fixed-shape in refinement syntax, dynamic in runtime-bound."

- **`spec.md` §3.1 (legacy): scalar newtype declaration syntax `type Depth: Scalar<m>`.** This appears in §3.3 as the canonical single-field nominal wrapper form.

- **`open_questions.md` (Type System): `where` on runtime values resolved.** Runtime `where` staying piecewise-hard, smoothing as model claim, stdlib smooth helpers -- these are reflected in the design notes referenced from §3.2's handling of runtime predicates.

---

## Superseded

Corpus content replaced by a newer decision in spec_new.md §3.

- **`spec.md` §3.2 generic types: uses `const N: usize` keyword.** Quote: `"type Soil<const M: usize> where M >= 2"`. Superseded by `v2.1_in_progress.md` and spec_new.md §3 which use `N: val`. Already in `anti_spec.md` (line: `| \`const N: usize\` | \`N: val\` | cleaner val-generic spelling |`).

- **`spec.md` §1.1 (library vs model modules): uses `dyn` keyword.** Quote: `"It may contain unresolved generics and \`dyn\` contract references"`. Superseded by §3.5 which retires `dyn` in favor of `impl Contract` + `some`. Already in `anti_spec.md`.

- **`spec.md` §3.1: `<:` used in structural introspection.** Quote: `"The \`<:\` operator is used in structural introspection (section 5.5) to filter by type"`. Superseded by §3.5 and `anti_spec.md` line: `| structural introspection (\`<:\` predicate, §5.5/§8.5) | nothing | closure policies see values + hyperparameters only |`. Already in `anti_spec.md`.

- **`v2.1_in_progress.md` (lines 519-522): `property name: continuous_invariant` as a user-facing surface.** Quote: `"property name: continuous_invariant. Continuous properties like monotonicity. Status: settled."` Superseded by `anti_spec.md`: `property` declarations are retired; user functions carry no property-declaration surface; compiler derives from body composition. Already in `anti_spec.md`.

- **`v2.1_in_progress.md` (lines 366-369): contract composition alias `contract X = A + B`.** Quote: `"Status: provisional — include if it proves useful, defer otherwise."` Superseded by `anti_spec.md`: `| contract composition alias (\`contract C := A + B\`) | nothing | multi-contract satisfaction + supertraits cover the bundle case |`. Already in `anti_spec.md`.

---

## Homeless

Corpus content relevant to §3, not accounted for in spec_new.md §3, and not committed to `anti_spec.md`.

- **`open_questions.md` (Type System, "Clarify that 'atomic' means leaf of the containment tree").** Quote: `"Clarify that 'atomic' means leaf of the containment tree (holds a numerical value), not 'single-field.'"` This is flagged in `v2.1_in_progress.md` line 88-89 as an open clarification on the settled `type Foo : Bar` newtype. It is not present anywhere in spec_new.md §3.3 (Newtype and Composite Types), which just says "single-field nominal wrappers." The distinction matters because a type with one field that is itself composite is atomic in the containment sense only if it holds a number, not if it holds a struct. Recommend: land in §3.3 as a one-sentence clarification: "single-field" means the type holds exactly one scalar value at the leaf of the containment tree, not that the wrapping type has one field in a general structural sense.

- **`spec.md` §4.7 / `v2.1_in_progress.md` (lines 970-978): named-type rules for equality and comparison operators.** Quote from spec.md: `"Named-type compatibility for equality and comparison. Both sides of =, >=, <=, >, < must be named-type-compatible ... CarbonPool = WaterPool is a compile error."` Quote from open_questions.md: `"Named-type rules for equality and comparison. Spec section 4.7 defines named-type rules for arithmetic but not for =, <, <=, >, >=."` The spec.md §4.7 actually does specify the comparison rule, but spec_new.md §3.3 says only "Named-type comparison rules cross-link §7" -- and §7's body contains only the one-liner "Named-type comparison rules." Neither §3.3 nor §7 states the actual rule. The named-type comparison rule (both sides of a relation must be named-type-compatible or one must be anonymous with matching dimensions) is a stable settled decision that has not materialized as prose in spec_new.md. Recommend: land in §3.3 or in a standalone §3.3.1. The cross-link to §7 is insufficient -- §7 says nothing normative on this point.

- **`v2.1_in_progress.md` (lines 180-183): `convert` body contains `universal rate: Scalar<dimensionless> = 1.08`.** This is a stale example that predates the CC1 literal-numerics lock. It is not a spec_new.md §3 item per se (convert lives in §5), but the stale example that violates CC1 propagates a false belief about what is legal in a convert body. The CC1 exception positions in spec_new.md §4 list "affine conversion bodies" -- and a parameterized `convert` body is not an affine conversion body in the sense of §4's exception (unit/affine offset declarations like `1 hour = 60 minutes`). Recommend: file as a `spec_dev_notes.md` cleanup note; the example in `v2.1_in_progress.md` needs the `= 1.08` removed and the `rate` declared as a universal injected from the workflow, consistent with CC1 and §3.1 (universals carry no value in `.myco`).

- **`open_questions.md` (Type System): "Named-type rules for equality and comparison -- Extend named-type rules to cover relations and comparisons."** This is listed explicitly as open, but it is a stable settled decision in `spec.md` §4.7 (lines 970-978 quoted above) and `v2.1_in_progress.md`. The decision is not marked "open" in the corpus; the `open_questions.md` entry is a tracking note for the migration of that prose into spec_new.md. Recommend: the open_questions.md entry should be closed once the prose lands in §3.3 or §7 (see bullet above). No new open question needed.

- **`v2.1_in_progress.md` (lines 241-258): scalar value generic parameters (compile-time-constant Myco-typed bounds).** Quote: `"Value parameters extend to compile-time-constant scalar values of any declared Myco type ... type Bounded<U: Unit, LOW: Scalar<U>, HIGH: Scalar<U>> = Scalar<U> where { value >= LOW, value <= HIGH }"`. Spec_new.md §3.6 (Generic Parameter Variance) covers variance rules but the surface syntax for scalar-value generics (the distinction between integer `val` parameters and typed scalar parameters like `LOW: Scalar<U>`) is not stated in §3. The `Bounded<U, LOW, HIGH>` pattern is a stable settled design. Recommend: land a clarifying sentence in §3.6 or a new §3.6.1 noting that val generics extend to compile-time scalar values of any Myco type, not only integer counts. This is load-bearing for refinement type library patterns.

---

## Conflicts

- **`v2.1_in_progress.md` (line 181) vs spec_new.md §3.1 + §4 CC1.** The `v2.1_in_progress.md` `convert` example includes `universal rate: Scalar<dimensionless> = 1.08` -- a literal numeric in value position inside a universal declaration. spec_new.md §3.1 states: "CC1: no literal value in `.myco`." spec_new.md §4 states: "Zero literal numerics in value position. Three exception positions: unit definitions, affine conversion bodies, structural positions." A parameterized convert body is not one of the three exception positions, and a universal-with-value violates CC1 as confirmed by `spec_dev_notes.md` (lines 116-117): `"universal R: Scalar<J_mol_K> — declaration only. No value attached."` and `anti_spec.md` (line 45): `"universals carrying values (universal R: Scalar<U> = 8.314) | universal R: Scalar<U> declaration only; value from workflow | CC1 scope"`. The `v2.1_in_progress.md` example is a pre-CC1-lock artifact that contradicts all of these. Recommend: the `v2.1_in_progress.md` example is stale and should be updated to remove the literal value; it is listed in `anti_spec.md` as stale material but the specific `convert` body example is not called out there yet.

- **`v2.1_in_progress.md` (line 519-522): `property name: continuous_invariant` with `Status: settled` vs `anti_spec.md` (line 24) which retires `property` declarations.** The `v2.1_in_progress.md` section still reads `"Status: settled."` while `anti_spec.md` says `property` declarations are retired and `spec_new.md §6 already forbids user property-declaration surface`. This is a direct status conflict inside `v2.1_in_progress.md`. Recommend: `v2.1_in_progress.md` is listed in `anti_spec.md` as a stale source to not import from; the `property` item is covered. No spec_new.md change required, but the `v2.1_in_progress.md` stale-list entry in `anti_spec.md` could be made more specific.
