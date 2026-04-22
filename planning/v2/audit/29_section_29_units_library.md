# Audit Report — §29 Units Library

Audited against corpus as of 2026-04-22.

§29 is a 20-line stub scoping the core units library to SI base units,
common SI-derived units, standard affine conversions between equivalent
spellings, and dimensionless-ratio handling, with domain-specific
packages (ecophysiology, chemistry, finance, astronomy) declared out
of scope. The section enumerates SI base units (m, kg, s, A, K, mol, cd)
and SI-derived units (N, Pa, J, W, C, V, Ω, Hz, etc.) and forward-
references §5 for derived-unit algebra. Unit-system mechanics (base
dimensions, derived-unit algebra, affine semantics, `value_in`, `convert`
variants, workflow-boundary unit parameter) live in §5, so §29 is
deliberately narrow.

---

## Absorbed

Corpus content that already landed in spec_new.md §29.

- **`planning/v2/spec_dev_notes.md` §430-436 (§29 scoping note, 2026-04-21):**
  > "Committed scope is SI base, SI-derived, derived-unit algebra, and
  > affine-conversion machinery. Domain-specific libraries (ecophysiology,
  > chemistry, astronomy, finance, etc.) are explicitly out of scope for
  > Myco core — they ship as distributable packages consuming core units."

  Absorbed verbatim into §29's summary and the "out of scope" paragraph.

- **`planning/v2/riley_project_note.md` §1-19 (scope boundary):**
  > "Things that DO belong in Myco core: general language surface
  > ..., the general units machinery (SI base + derived, the affine-
  > conversion facility) ... Things that do NOT belong in Myco core:
  > any unit, function, model shape, or stdlib item that only makes
  > sense in a plant-physiology / ecology / Riley's-research context."

  Absorbed into §29's domain-specific-library paragraph. The project-
  vs-language separation (per Riley's feedback memory) is honored.

- **`planning/v2/spec.md` §4.1 — SI base unit inventory:**
  > "pub base_unit kilogram ... meter ... second ... kelvin ... mole
  > ... ampere ... candela"

  Absorbed into §29's base-unit enumeration `(m, kg, s, A, K, mol, cd)`.

- **`planning/v2/spec.md` §4.2 — SI-derived unit inventory:**
  > "pub unit newton = kilogram * meter / second ** 2
  >  pub unit pascal = newton / meter ** 2
  >  pub unit joule = newton * meter
  >  pub unit watt = joule / second"

  Absorbed into §29's derived-unit list `(N, Pa, J, W, C, V, Ω, Hz, etc.)`.
  The derived-unit algebra itself lives in §5 per §29's forward reference.

- **`planning/v2/v2.1_chunk_reports/10_package_dependencies.md` §14-16
  (spore vocabulary):**
  > "Spore — a Myco package. Ecosystem-level unit of distribution."

  Implicitly absorbed: §29 states domain libraries "ship as distributable
  Myco packages." The word "spore" is not used in §29 (correctly, since
  §29 is language-surface prose and spore vocabulary lives in the
  packaging chapter).

---

## Superseded

Corpus content replaced by a decision now in spec_new.md §29.

- **`planning/v2/spec.md` §4.2 — `units::si` module with extensive
  compound-unit catalog:**
  > "use units::si::{ megapascal as MPa, mole_per_square_meter_second
  > as mol_m2_s, ratio, }"
  > "pub unit mol_m2_s = mole / meter ** 2 / second
  >  pub unit mmol_m2_s = 1e-3 * mol_m2_s
  >  pub unit J_mol = joule / mole
  >  pub unit J_mol_K = joule / mole / kelvin"

  Superseded by §29's scope narrowing: compound SI-derived spellings
  used by ecophysiology (`mol_m2_s`, `J_mol_K`, `mmol_m2_s_MPa`) are
  out of core and move to the ecophys spore. §29 commits to "common
  SI-derived units" only.

  `Recommend:` Not in anti_spec.md. Low urgency; the supersession is
  a scope narrowing, not a retirement of machinery. If anti_spec.md
  is ever refined, an entry "ecophysiology compound units in core
  stdlib | distributable spore package | project-vs-language
  separation" would fit.

- **`planning/v2/spec.md` §4.11 — stdlib-provided physical constants
  (`R`, `Stefan-Boltzmann`, reference temperatures, with inline
  default values):**
  > "Universals are valued inline in the `.myco` file and represent
  > constants that are well-established (the gas constant, reference
  > temperatures, the Stefan-Boltzmann constant). They are overridable
  > from Python."

  Superseded by CC1 (workflow-injected constants). Already retired in
  `anti_spec.md`: "stdlib physical constants (`R`, `Avogadro`, etc.) |
  workflow-injected via `assume_constant`." No further action.

- **`planning/v2/mocks/mock_sperry.myco` line 42:**
  > "use physics::constants::{stefan_boltzmann, specific_heat_air}"

  Superseded by CC1. The `physics::constants` stdlib module is retired;
  these values enter via `assume_constant`. Already covered by the
  anti_spec.md physical-constants retirement. `mock_sperry.myco` is
  already flagged for rewrite in `anti_spec.md` "Stale in legacy docs"
  (line 99).

- **`planning/v2/mocks/mock_sperry.myco` line 290 — inline-valued
  universal:**
  > "universal R: Scalar<J_mol_K> = 8.314           // gas constant"

  Superseded by CC1 (universals declare types, not values). Already
  covered by `anti_spec.md` "universals carrying values" retirement.

---

## Homeless

Corpus content relevant to §29, not in spec_new.md §29, and not in
anti_spec.md. §29 is a very short stub; the Homeless bucket is heavy.

- **`planning/v2/spec_dev_notes.md` §434-436 — ecophys spore content
  inventory:**
  > "Ecophysiology extensions (water potential, gas-exchange rates,
  > PPFD / radiation, LAI / canopy, soil water) accordingly moved out
  > of the spec and noted on riley_project_note.md as spore-library
  > content."

  The specific ecophys categories that leave core are named in
  dev_notes but not in §29. §29 names the domain (ecophysiology) but
  not the categories of units it carves out. This is load-bearing for
  a reader trying to tell which units land in core and which in the
  spore.

  `Recommend:` Either add a one-sentence example list to §29 (e.g.,
  "water potential, gas-exchange rates, radiation flux, canopy
  geometry, soil water") or leave as-is and accept that dev_notes
  carries the detail. Low urgency.

- **`planning/v2/spec_new.md` §5.0-5.5 — unit-system fundamentals
  live in §5, not §29:**
  §29 forward-references §5 ("via derived-unit algebra (§5)") but
  does not explicitly state that the unit-system mechanics (base_unit
  keyword, Scalar<U>, base-unit storage invariant, affine semantics,
  value_in, convert, workflow-boundary unit parameter) are all §5
  content. A reader arriving at §29 expecting "the units library" may
  be surprised that §29 is only the stdlib-contents scope statement
  and all mechanics live in §5.

  `Recommend:` Add one sentence to §29's summary: "Unit-system
  mechanics (base dimensions, derived-unit algebra, affine semantics,
  `convert` variants, `value_in`) live in §5; §29 commits only the
  core stdlib's unit inventory and scope." Matches the pattern used in
  §30 Matrix and Tensor Primitives (which explicitly defers type-layer
  design to §3.9).

- **`planning/v2/spec.md` §4.2 — core-vs-custom unit-package pattern:**
  > "Non-SI systems (CGS, imperial, etc.) are defined the same way. A
  > CGS package would either define its own base units (introducing
  > independent dimensions) or define CGS units as derived from SI
  > base units (allowing cross-system conversion)."

  §29 scopes core to SI but does not explicitly state that non-SI
  systems (CGS, imperial) are also out of core and ship as packages.
  By the project-vs-language separation principle non-SI belongs
  outside core on the same footing as ecophysiology; §29's current
  prose leaves this implicit.

  `Recommend:` Add non-SI systems (CGS, imperial) to the out-of-scope
  sentence: currently enumerates "ecophysiology, chemistry, astronomy,
  finance"; could extend to "ecophysiology, chemistry, astronomy,
  finance, non-SI alternate systems (CGS, imperial)".

- **`planning/v2/spec_new.md` §4 line 540-546 — stdlib default
  bindings for mathematical constants (π, e):**
  > "Mathematical constants. π, e, and similar fixed reals are
  > ordinary stdlib-declared identifiers ... The stdlib ships default
  > bindings so users do not write them by hand."

  §29's scope statement ("SI base units, common SI-derived units,
  standard affine conversions, dimensionless-ratio handling") does
  not mention mathematical constants. §4 commits that the stdlib
  ships their default bindings; §29 is the natural home for "which
  module ships them." This is an unresolved cross-reference: §29
  could host `stdlib::math::{pi, e}` or the equivalent, or §29 could
  explicitly state that mathematical constants live in a separate
  stdlib surface.

  `Recommend:` Add a one-line pointer to §29 stating that math
  constants (π, e) are stdlib identifiers declared alongside the
  numeric stdlib (§26), not the units library, since they are
  dimensionless and belong with numeric types.

- **`planning/v2/spec_new.md` §0.1 / §35 — conversion-graph cost
  model open item:**
  > "Conversion-graph cost model. Open. Unit conversions, tensor
  > reshapes, sparse or dense representation transitions, and
  > structural-subtype widenings all carry costs that the compiler
  > ..."

  §29 does not cross-reference the conversion-graph cost model even
  though unit conversions are one of its four enumerated edge types.
  The core unit library is the primary source of the unit-conversion
  edges.

  `Recommend:` Add a cross-reference in §29 pointing to §0.1 / §35
  for the conversion-graph cost model open item. Not a new commitment,
  a cross-reference.

- **`planning/v2/v2.1_chunk_reports/07_type_graph_in_progress.md`
  §30, Q6:**
  > "Conversion graph. Units (`seconds ↔ minutes × 60`), shape
  > reshapes ..."
  > "Q6. Cost model for conversion-graph edges."

  Same theme as above, from the chunk 07 side. Belongs under §0.1 /
  §35 tracking. Not §29-homeless by the "open design" rule, but the
  unit-conversion edge type sourced from §29 content deserves the
  cross-reference.

  `Recommend:` Same as the above bullet.

- **`planning/v2/spec_new.md` §26 — `Scalar<U, T = Float64>` two-
  parameter form:**
  > "`Scalar<U, T = Float64>` with explicit `T` parameter and
  > `Float64` default. `Rational` for exact constant folding (with
  > termination caveats). `BigFloat`."

  §29's ecophys spore uses `Scalar<U>` (implicit `T = Float64`)
  throughout in the mocks. The `Scalar<U, T>` form interaction with
  the core units library is not stated in §29. Likely a non-issue
  since unit declarations are T-independent, but worth a sentence if
  ever the two parameters compose non-trivially.

  `Recommend:` No action. Cross-reference unnecessary; §26 carries
  the two-parameter form and §5 carries `Scalar<U>`.

---

## Conflicts

Direct contradictions between spec_new.md §29 and the corpus.

- **Derived-unit ship-list scope.** §29 enumerates `(N, Pa, J, W, C,
  V, Ω, Hz, etc.)` as common SI-derived units. `planning/v2/spec.md`
  §4.2 ships compound units like `J_mol_K`, `mol_m2_s`, `mmol_m2_s`,
  `MPa_m3_mmol` as part of `units::si`. These compound spellings are
  not "common SI-derived" but compound names for ecophysiology use.
  §29's scope narrowing (spec_new.md) moves them out of core and into
  the ecophys spore; `spec.md` §4.2 and the mocks still assume they
  live in `units::si`.

  This is a scope conflict, not a design conflict. `spec.md` is
  legacy and the mocks (`mock_sperry.myco`, `mock_potkay.myco`) are
  explicitly flagged for rewrite in `anti_spec.md` (lines 98-99).

  `Recommend:` No §29 action. The conflict resolves on mock rewrite.

- **Affine-conversion scope sentence.** §29 says "Standard affine
  conversions between equivalent SI-derived spellings." The phrase
  "equivalent SI-derived spellings" is ambiguous: affine conversions
  (Celsius ↔ Kelvin, Fahrenheit ↔ Kelvin) are between units of the
  same dimension but different offset/scale, not between "equivalent
  spellings" (which reads as same-magnitude aliases, handled by bare
  `convert <->` per §5.1).

  `planning/v2/spec.md` §4.4 and `spec_new.md` §5.4 are both clear
  that affine units are offset-based (Celsius, Fahrenheit, gauge
  pressure). §29's wording "equivalent SI-derived spellings" is
  imprecise.

  `Recommend:` Reword §29 to "Standard affine conversions between SI
  temperature units (Celsius ↔ Kelvin, Fahrenheit ↔ Kelvin) and
  gauge-vs-absolute pressure." Or more tersely: "Standard affine
  conversions between SI-derived units with offset (e.g., Celsius ↔
  Kelvin)."

- **`planning/v2/open_questions_deprecated_use_spec_new.md` lines
  816-822 vs. anti_spec.md CC1 strict:**
  > "CC1 (spec §4, anti_spec 'Dropped features') bans literal
  > numerics in value position. Physical constants and mathematical
  > constants (π, e, `R`, `stefan_boltzmann`, etc.) are ordinary
  > stdlib-declared universals whose values enter at compile time
  > via the workflow binding verbs. The stdlib ships default
  > bindings for mathematical constants so users do not write them
  > by hand."

  The deprecated open_questions says "stdlib ships default bindings
  for mathematical constants" and groups π, e with R, Stefan-Boltzmann
  as "ordinary stdlib-declared universals." `spec_new.md` §4 line
  540-546 retains the math-constants default-bindings story for π
  and e. `anti_spec.md` line 50 retires *physical* constants (R,
  Avogadro) entirely, sending them to workflow injection via
  `assume_constant`, per CC1.

  The split (math constants: stdlib-default-bound; physical constants:
  workflow-injected) is coherent but §29 does not mediate it. A
  reader asking "where does the units library ship physical
  constants?" gets no answer from §29 (correct: nowhere, physical
  constants are not units). A reader asking "where do math constants
  live?" also gets no answer from §29.

  `Recommend:` Not a §29 conflict strictly, since §29 is about the
  units library and constants live in §4 / §26 / stdlib math module.
  But §29 could include a one-line disambiguation: "Physical and
  mathematical constants are not units and do not ship under §29; see
  §4 for the CC1 injection rule."
