# Audit Report — Appendix A: Reserved Keywords and Syntactic Surface

Audited against corpus as of 2026-04-22.

Appendix A source: `spec_new.md` lines 5113-5160.

Corpus searched: `spec_new.md` body, `spec.md`, `v2.1_in_progress.md`,
`v2.1_chunk_reports/01`–`12`, `mocks/mock_sperry.myco`,
`mocks/mock_potkay.myco`, `spec_dev_notes.md`, `anti_spec.md`,
`open_questions_deprecated_use_spec_new.md`.

---

## Absorbed

### Declaration keywords

All sixteen declaration keywords (`type`, `node`, `universal`, `fn`,
`contract`, `relation`, `constraint`, `event`, `geometry`, `locus`,
`chart`, `topology`, `metric`, `domain`, `convert`, `use`) appear
consistently throughout `spec_new.md` in declaration-head position
and nowhere else in identifier position. Sources: §2 (`use`, `node`),
§3 (`type`, `universal`), §5 (`convert`), §6 (`fn`), §7 (`contract`),
§8 (`relation`, `constraint`), §10 (`event`), §11 (`geometry`, `locus`,
`chart`, `topology`, `metric`, `domain`).

### Type-former keywords

`Scalar`, `Tensor`, `Vector`, `Matrix`, `Collection`, `impl`, `some`,
`where` all appear in type-expression position consistently with the
Appendix A claim. `impl` and `some` positively replace the retired
`dyn` (§3.5; anti_spec.md line 11). `where` is used both as a
type-constraint binder and as a guard keyword in relations — both
usages are within the declared "type-former keyword" role.

### Body-form keywords

`let`, `if`, `else`, `for`, `in`, `trace`, `identify`, `requires`,
`replaces`, `conserved` all appear in relation/event body position in
`spec_new.md`. `conserved` appears as a type-body annotation
(`type Mass : Scalar<kg> { conserved }`, §3.7, line 371) and in
prose throughout §§3.7, 11, 16. `identify` appears in geometry
declarations and is confirmed as a declaration-level keyword in §11.2
(lines 1652-1660). `trace` is confirmed as the PDE manifold-restriction
operator in §11.1 (line 1570) — distinct from the retired
`trace(f, junction, edge)` directional-limit overload
(anti_spec.md line 30). `replaces` appears in obligation-override
declarations in §10.5 (line 1474). `requires` appears in boundary
condition blocks in §11.2 (line 1605). All consistent.

### Stochastic operator

`~` is used uniformly as the distribution-binding operator throughout
§13 (first appearance line 194 in the glossary, then extensively in
§13.1–13.10). `<Ito>` and `<Stratonovich>` appear correctly as type
parameters on `BrownianMotion` in §13.4 (lines 2264-2265):
`x ~ BrownianMotion<Ito>(...)` and `x ~ BrownianMotion<Stratonovich>(...)`.
The Appendix A description ("unit generics use `<Ito>`, `<Stratonovich>`
as contract-parameter keywords on `~`") is substantively correct;
the convention is a type parameter on the stochastic family, not on
`~` itself. Wording in Appendix A is slightly imprecise but the
intent is clear and matches §13.4.

### Reserved-but-not-assigned: `self` and `match`

`self` is used in refinement-predicate bodies (`type UnitInterval =
Scalar<dimensionless> where { 0 <= self <= 1 }`, line 266) and as a
method-receiver name in contract default implementations (§7.5,
lines 922-938). Both usages are consistent with Appendix A's claim
of "reserved for refinement-predicate body use." The method-receiver
use in §7.5 extends `self` slightly beyond what Appendix A explicitly
says, but it is not a contradiction — the reservation is broad enough.

`match` appears in `spec_new.md` body only in prose discussion
(lines 1377, 1735, 1971, 2142) where it is the English verb, never
as a `.myco` keyword token. The one inline-code occurrence (`match`,
line 5141) is Appendix A itself. Consistent with "reserved but not
yet assigned."

### Structural punctuation

`::`, `->`, `<->`, `<=`, `>=`, `<`, `>`, `==`, `!=`, `=`, `|` all
appear in the spec body in roles consistent with Appendix A. `|` is
used in prose shorthand in chunk 11 (`Prior<T> = Fixed(Scalar<T>) |
Random(some Distribution<T>)`, line 70 of `11_sum_types_enums.md`)
but the formal syntax sketch in that chunk uses brace notation, not
`|`-separated variants. Appendix A's "currently unassigned, reserved
for future pattern or pipe use" is consistent with both: `|` is in
prose only, not locked as a variant separator yet.

### Stdlib-reserved identifiers (confirmed present)

All identifiers in the Appendix A list (`exp`, `log`, `sin`, `cos`,
`tan`, `asin`, `acos`, `atan`, `sqrt`, `abs`, `sign`, `floor`,
`ceil`, `round`, `min`, `max`, `sum`, `prod`, `mean`, `std`, `var`,
`solve`, `invert`, `deriv`, `integrate`, `condition_of`, `value_in`)
appear in the spec body consistently as stdlib-atom calls. `deriv`
and `integrate` are defined in §14. `condition_of` is defined in
§14.1. `value_in` is defined in §5.3. The delegation to §27 for
distribution-family names is consistent: §27 enumerates Normal,
LogNormal, Uniform, Beta, Gamma, Exponential, ChiSquared, Cauchy,
StudentT, Laplace, HalfNormal, HalfCauchy, InverseGamma, Lévy,
Weibull, Pareto, Fréchet, Gumbel, GEV (19 univariate continuous),
Bernoulli, Categorical, Poisson, NegBinomial, Hypergeometric (5
discrete), MultivariateNormal, Dirichlet, Multinomial (3 multivariate),
and meta-families Truncated, Mixture.

### Legacy `dyn` — confirmed retired

`spec_new.md` body contains only one occurrence of `dyn` (line 346:
"Together `impl` and `some` positively replace the retired `dyn`
escape"). All other occurrences of `dyn` in the corpus are in
`spec.md` (legacy), `v2.1_in_progress.md` (stale versioning prose),
chunk reports 02 and 07 (pre-retirement), and `anti_spec.md`
(retirement record). `anti_spec.md` line 11 formally retires `dyn`.
Appendix A correctly omits `dyn`.

---

## Superseded

### `dyn` (spec.md)

`spec.md` uses `dyn` extensively (§§2.4, 2.5, 10, etc.) for
heterogeneous collections. Superseded by `impl Contract` +
`some T` in spec_new.md §3.5 and formally retired in anti_spec.md.
Appendix A correctly omits it.

### `property` (spec.md)

`spec.md` §8.5 uses `property keyword` (e.g., `property monotone:
decreasing(pressure -> plc)`, spec.md line 1405). Retired in
anti_spec.md line 24 in favor of refinement types, capability
contracts, and `constraint` blocks. `spec_new.md` forbids
user property-declaration surface (§6, line 740). Appendix A
correctly omits `property`.

### `base_unit` and `unit` (not in Appendix A — see Homeless)

`spec.md` uses `base_unit` extensively (lines 667-677). `spec_new.md`
also uses both `base_unit` and `unit` as declaration keywords in
code blocks (lines 594-605). These are NOT retired — see Homeless.

### `const N: usize` (spec.md)

`spec.md` uses `const N: usize` for compile-time scalar generics.
Retired in anti_spec.md line 21 in favor of `N: val`. `val` is
present in `spec_new.md` prose (line 244 "val and type generics")
and in mock files (`mock_sperry.myco` lines 421, 452, 514, 538).

### `param`, `slot`, `rule`, `[t+1]/[t]`, `rate()` (spec.md)

All retired per anti_spec.md. Appendix A correctly omits them.
`spec_new.md` body does not use these keywords in code-block or
declaration position.

---

## Homeless

### H1. `base_unit` — declaration keyword used in spec_new.md code blocks

`spec_new.md` §5.0 contains a myco code block (lines 593-597):
```
base_unit meter
base_unit second
base_unit kilogram
```
`base_unit` is a first-class declaration keyword — it introduces
a new orthogonal dimension axis (§5.0 prose, line 590). It is
NOT in Appendix A's declaration keyword list.

Recommend: add `base_unit` to the **Declaration keywords** list in
Appendix A.

### H2. `unit` — declaration keyword used in spec_new.md code blocks

`spec_new.md` §5.0 contains a myco code block (lines 603-605):
```
unit meter_per_second = meter / second
unit pascal = kilogram / (meter * second^2)
```
`unit` is a derived-unit declaration keyword. It is NOT in Appendix A's
declaration keyword list.

Recommend: add `unit` to the **Declaration keywords** list in
Appendix A.

### H3. `val` — compile-time scalar generic keyword used in mock files and spec prose

`spec_new.md` prose uses "val and type generics" (line 244) and
"val generic" (lines 1553, 1584). Mock files use `N: val`, `M: val`,
`L: val` as compile-time scalar generic parameters (`mock_sperry.myco`
lines 421, 452, 514, 538-539). anti_spec.md line 21 replaces the
legacy `const N: usize` with `N: val`. `val` is not in Appendix A's
type-former keywords or any other list.

Recommend: add `val` to the **Type-former keywords** list (it is a
parameter-kind qualifier analogous to how `: Unit` qualifies a type
parameter).

### H4. `approximate` — body-form keyword not in Appendix A

`spec_new.md` §15.1 (lines 2618-2626) contains a myco-notation
block:
```
approximate {
  under:           <rewrite-id>
  tolerance_class: ...
  error_bound:     ...
  body:            ...
  where:           ...
}
```
`approximate` is a body-scoped block keyword that authorizes lossy
rewrites. It appears throughout §13, §15, §17, §25. It is listed
in the §1 glossary (line 194: "`approximate` block") but is NOT in
Appendix A's body-form keywords or any other list.

Recommend: add `approximate` to the **Body-form keywords** list in
Appendix A.

### H5. `is` — type-narrowing predicate keyword used in spec body

`spec_new.md` §8.3 and §12.7 use `where x is T` as a compile-time
type-narrowing predicate (lines 1011, 1017, 1133, 2094-2114):
```
for tree in trees where tree is OldGrowth:
```
`is` functions as a keyword in this position — it is the structural
narrowing predicate. It is NOT listed anywhere in Appendix A (not in
declaration keywords, type-formers, body-forms, or reserved-but-not-
assigned). It is also not merely the English copula at these sites.

Recommend: add `is` to the **Reserved but not yet assigned** list
at minimum, or to a new "Predicate keywords" category (alongside
`where`) with the note that `is` is the type-narrowing operator in
`where x is T` filter clauses.

### H6. `enum` — sum-type declaration keyword established in chunk 11

Chunk 11 (`v2.1_chunk_reports/11_sum_types_enums.md`) locks the
enum design with this syntax (lines 84-99):
```
enum Prior<T: Unit> {
    Fixed(Scalar<T>),
    Random(some Distribution<T>),
}
```
`enum` is committed as the declaration keyword for sum types (chunk
11 line 295: "Exact syntax. `enum` keyword spelling..."). The shape
is locked even if exact variant syntax is still open. `spec_new.md`
references enums at lines 5008-5017 ("contract-typed variant field
inside an enum"), acknowledging the mechanism. `enum` is NOT in
Appendix A's declaration keyword list.

Recommend: add `enum` to the **Declaration keywords** list in Appendix
A, with a note that exact variant syntax is open (pending chunk 11
close).

### H7. `match` — chunk 11 commits exhaustive match as a language construct

While Appendix A correctly reserves `match` as "reserved for future
pattern-matching surface," chunk 11 has progressed further: it locks
exhaustive match as the required construct for enum dispatch (chunk
11 lines 107-119), with mandatory exhaustiveness checking. The
Appendix A status ("reserved but not yet assigned semantics") is
therefore understated — match is no longer merely speculative future
syntax, it is the committed mechanism for sum-type dispatch.

Recommend: update the `match` reservation note in Appendix A to
acknowledge that exhaustive match is the committed enum-dispatch
mechanism (pending final surface-syntax lock in chunk 11), replacing
"reserved for future pattern-matching surface" with something like
"reserved; committed as exhaustive sum-type dispatch; surface syntax
being finalized in chunk 11."

### H8. `observe` — PPL keyword in .myco body position not in any Appendix A list

`spec_new.md` §13.8 (line 2347) shows `observe(data, x ~ D)` as
a call that appears within `.myco` model bodies — it is a
distribution-binding operator that attaches layer-2 observation
facts. The spec glossary (line 195) lists `observe` as a vocabulary
term. It is distinct from the workflow Python `observe` verb (line
2979 calls it a "workflow verb," but §13.8 shows it invoked with
`.myco` syntax). It does not appear in Appendix A as a reserved
identifier, body-form keyword, or stdlib-reserved identifier.

The dual status (language-level syntax in §13.8, workflow-tier label
in §16) is itself ambiguous: if `observe` is a `.myco` keyword it
must appear in Appendix A; if it is purely a Python-tier verb it
does not need to. This ambiguity should be resolved.

Recommend: clarify in §13.8 whether `observe(data, x ~ D)` is
`.myco` surface syntax (making `observe` a body-form keyword to add
to Appendix A) or a pseudo-notation for a compiler mechanism (in
which case remove the call-form from §13.8 and describe the
mechanism without introducing a new parse surface).

### H9. `smooth_max`, `smooth_abs`, `smooth_step` — stdlib functions used in spec body, not reserved

`spec_new.md` §8.9 (lines 1123-1130) describes stdlib-provided
smoothing helpers: `smooth_max`, `smooth_abs`, `smooth_step`. These
are presented as user-callable stdlib functions — not user-defined,
not mere examples. They are analogous to `min`/`max` but are absent
from the Appendix A stdlib-reserved identifier list. Similarly,
`soft_select`, `hard_select`, `weighted_average`, `condition_weighted`
(§8.7 closure-policy identifiers), `argmin`, `argmax` (§12.1), and
`soft_clip`, `hard_clip`, `sigmoid` (§25 projection flavors) are all
used as stdlib calls in `spec_new.md` without reservation in Appendix A.

Recommend: extend the Appendix A stdlib-reserved list to include at
minimum the identifiers named in §8.7 (Y1–Y4 closure policies),
§8.9 (smoothing helpers), §12.1 (aggregation operators including
`argmin`/`argmax`), and §25 (projection flavors). Alternatively,
acknowledge in Appendix A that the stdlib-reserved list covers
"math atoms" only and that other stdlib namespaces (aggregation,
smoothing, projection) are governed by a separate stdlib reference.

### H10. `loss_of`, `cost_of` — compiler intrinsics in §14 not in stdlib-reserved list

`spec_new.md` §14 introduces `loss_of(residual)` (line 2505) and
`cost_of(expr)` (lines 2494, 4926) as compiler intrinsics alongside
`deriv`, `integrate`, `condition_of`. `deriv`, `integrate`, and
`condition_of` are in the Appendix A stdlib-reserved list;
`loss_of` and `cost_of` are not. All five are presented as
compiler-intrinsic calls in §14.

Recommend: add `loss_of` and `cost_of` to the stdlib-reserved
identifier list in Appendix A alongside `deriv`, `integrate`,
`condition_of`.

### H11. Spatial operators — stdlib functions used in §11 not in reserved list

`spec_new.md` §11.1 (lines 1548-1580) defines a family of stdlib
spatial operators: `grad`, `diverg`, `laplacian`, `curl`,
`normal_grad`, `limit_from` (and `trace`, which IS in the body-form
keyword list). These are stdlib-atom calls that appear extensively in
spec body prose and code blocks. None except `trace` appear in
Appendix A.

Recommend: add `grad`, `diverg`, `laplacian`, `curl`, `normal_grad`,
`limit_from` to the stdlib-reserved identifier list (or a "Geometry
stdlib" sub-list). `trace` is already in body-form keywords; its
dual identity as a body-form keyword and PDE operator may warrant a
clarifying note.

### H12. `Binomial` — distribution family used in spec body but missing from §27 Tier 1 table

`spec_new.md` references `Binomial` at lines 2209 and 4332 (in the
conjugate-posterior rewrite table for `Beta`). The Tier 1 discrete
family table (§27.1, lines 4254-4262) lists Bernoulli, Categorical,
Poisson, NegBinomial, Hypergeometric — `Binomial` is absent. Yet the
conjugate rewrite catalog names it as a likelihood family.

Recommend: either add `Binomial` to the Tier 1 discrete table in
§27.1 (it is a standard family that the Beta prior is conjugate to)
or note in §27.3 that `Binomial` is a Tier 2 family whose conjugacy
fires the same rewrite. This inconsistency in §27, if unresolved,
makes the distribution-family reservation in Appendix A incomplete.

---

## Conflicts

### C1. `<Ito>` / `<Stratonovich>` described as "unit generics" but are type parameters on a family

Appendix A (lines 5136-5137): "Unit generics use `<Ito>`,
`<Stratonovich>` as contract-parameter keywords on `~`."

§13.4 (lines 2258-2266): "SDE draws carry an integration-convention
generic: `x ~ BrownianMotion<Ito>(...)` vs
`x ~ BrownianMotion<Stratonovich>(...)`. The convention is a type
parameter on the stochastic family, not a global setting."

The Appendix A phrasing "contract-parameter keywords on `~`" suggests
these generics parameterize the `~` operator itself. §13.4 is clear
that they parameterize `BrownianMotion` (the family), not `~`. The
`~` operator takes any distribution family; `<Ito>/<Stratonovich>` are
specific to SDE families.

Recommend: rewrite the Appendix A stochastic-operator line to read:
"SDE families (e.g., `BrownianMotion`) carry an integration-convention
type parameter; `<Ito>` and `<Stratonovich>` are reserved as
integration-convention tags on SDE distribution families. Default is
`<Ito>`."

### C2. `identify` listed as body-form keyword but primarily appears as a geometry-declaration statement

Appendix A classifies `identify` under "Body-form keywords" alongside
`let`, `if`, `else`, `for`, `in`. `spec_new.md` §11.2 (lines
1652-1660) shows `identify` as a geometry-body declaration
(`identify phi = 0 <-> phi = 2 * pi`) and §17 X2 (line 3187)
describes it as a module-scope declaration ("lives at module scope,
inside geometry bodies"). It is not a general expression-body form.

This is not a false claim — `identify` can appear inside certain
scoped bodies — but classifying it as a "body-form" alongside `let`
and `if` may mislead users into thinking it is usable in relation
bodies. It is a geometry-body / module-scope declaration, not a
general body form.

Recommend: move `identify` from Body-form keywords to Declaration
keywords (it is a declaration that installs a periodic identification),
or add a parenthetical clarifying its scope: "geometry-body /
module-scope only."
