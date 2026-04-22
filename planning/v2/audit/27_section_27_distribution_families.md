# Audit Report — §27 Distribution Families (Z-group)

Audited against corpus as of 2026-04-22.

---

## Absorbed

Corpus content that already landed in spec_new.md §27.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 settled items 22-26 (CC4 cluster):**
  > "`~` is distributional metadata at Layer 2, never introduces e-graph merges; envelopes propagate via rewrites"
  > "Distribution capabilities decomposed into multiple contracts (`AffineSelfClosed`, `SumSelfClosed`, ...); cross-family rules are rewrite declarations"
  > "Three-tier distributional propagation: Tier A closed-form / Tier B approximate / Tier C opaque"

  Absorbed into §27's `Distribution<U>` contract preamble and the optional capability sub-contract list (`AffineSelfClosed`, `SumSelfClosed`, `ProductSelfClosed`, `ScaleSelfClosed`, `SmoothTransformable`, `ReparameterizedSampleable`, `Conj(X)`). The Tier A/B/C axis is spelled out in §27.5 and in §13.2.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 settled item 33 (Z-group stdlib scope resolution 2026-04-20):**
  > "Z-group stdlib scope promoted (continuous + discrete + meta-family tiers); `ScaleSelfClosed` contract added; `SumSelfClosed` reframed as 'rewrite registered'; cross-family conjugates expanded; promotions include InverseGamma, Lévy, full extreme-value family, NegBin, Hypergeometric, Mixture sugar, Truncated"

  Absorbed into §27.1 Tier 1 family tables (InverseGamma, Lévy, Weibull, Pareto, Fréchet, Gumbel, GEV promoted to the 19-family univariate list; NegBinomial and Hypergeometric in the discrete list) and §27.2 meta-families. The `ScaleSelfClosed` contract is listed in §27's capability sub-contract enumeration.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 "Contract refinements":**
  > "Frame `SumSelfClosed` as 'a sum rewrite is registered.' Some families sum-close unconditionally (Normal, Cauchy, Poisson); others with matching parameters (Gamma same-rate, Binomial same-p, ChiSquared always-closes-dof-adds). Parametric predicates live on the rewrite rule (CC5 site-scoped predicate machinery), not as additional contracts."

  Absorbed into §13.2 Tier A bullet:
  > "Some closure contracts apply conditionally on parameter alignment (`SumSelfClosed` holds for Gamma only under shared rate parameter, for Binomial only under shared success probability); §27.1 records the per-family conditions."

  And into §27.1's per-row shared-parameter annotations (`S (shared β)` on Gamma, `S (shared k degrees)` on ChiSquared, `S (n-fold → Gamma)` on Exponential).

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 "Cross-family rewrites shipping in v2.1" conjugate list:**
  > "Normal-Normal (known σ) → Normal posterior / Beta-Bernoulli → Beta posterior / Beta-Binomial → Beta posterior / Gamma-Poisson → Gamma posterior / Dirichlet-Multinomial → Dirichlet posterior / Dirichlet-Categorical → Dirichlet posterior"

  Absorbed into §27.3's conjugate-posterior rewrite catalog (Beta-Bernoulli, Beta-Binomial, Gamma-Poisson, Normal-Normal known σ, InverseGamma-Normal known μ, Dirichlet-Multinomial).

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 Tier B rewrites and meta-families:**
  > "Z8 delta method / Z9 Fenton-Wilkinson / CLT — large-n sum → approximate Normal / Block-maxima → GEV"
  > "`Truncated<D>` — wraps any `Distribution<U>` with refinement-type bounds; Tier A closed-form log-pdf via CDF normalization"
  > "`Mixture<D₁, ..., D_N | weights>` — syntactic sugar over the existing latent-discrete + Q1 auto-marginalization pattern"

  Absorbed into §27 preamble, §27.2, and §27.5. Truncated's CDF-normalization intent and Mixture's workflow-supplied weights both land.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` Bucket 2 Z-group listing (Z1-Z10):**

  Absorbed into Appendix C Z-group (lines 5471-5503). §27 cross-references Appendix C at multiple points.

- **`planning/v2/v2.1_in_progress.md:644-660` `Distribution<U>` contract and stdlib distribution list:**
  > "Only `log_pdf` is strictly required for HMC/NUTS inference. `sample` is required for simulation... `reparameterized_sample` is optional"

  Partially absorbed. §27 keeps `log_pdf` and `sample` as required; `reparameterized_sample` is folded into the optional `ReparameterizedSampleable` capability sub-contract rather than an optional method on the base contract. Unit parameterization (`Scalar<U>` throughout) preserved.

- **`planning/v2/anti_spec.md:88`:**
  > "MVN 'deferred pending vector/matrix story' | reframed — gated on B5 heterogeneous-unit resolution"

  Absorbed into §27.1's multivariate row and the "B5 matrix heterogeneous-unit resolution, chunk 05" paragraph. Family equivalences from chunk 04 (`exp(Normal) ↔ LogNormal`, `Exponential ↔ Gamma(1,λ)`, `ChiSquared(k) ↔ Gamma(k/2, 2)`, `1/Gamma ↔ InverseGamma`) land in Appendix C Z5 and via §27.1 capability-row annotations (Exponential's `S (n-fold → Gamma)`).

---

## Superseded

Corpus content replaced by a newer decision in spec_new.md §27. Should move to anti_spec.md if not already there.

- **`planning/v2/v2.1_in_progress.md:644-660` `Distribution<U>` contract as three methods (`log_pdf` required, `sample` + `reparameterized_sample` optional):**
  > "contract Distribution<U: Unit>:
  >     fn log_pdf(x: Scalar<U>) -> Scalar<dimensionless>
  >     fn sample(key: RngKey) -> Scalar<U>
  >     // Optional — required only for pathwise gradients (VI backends)
  >     fn reparameterized_sample(key: RngKey, params) -> Scalar<U>"

  Superseded by §27's three-required-method shape (`log_pdf`, `sample`, `pdf`) plus optional capability sub-contracts (`ReparameterizedSampleable` is one of them). The v2.1_in_progress form has `sample` as optional and no `pdf`; §27 makes `sample` required (for Tier C handoff) and adds `pdf` as a third required method "provided as a convenience." The `reparameterized_sample` function-on-base-contract form is replaced by a separate sub-contract.

  `Recommend:` Add to anti_spec.md: "`Distribution<U>` with two-optional-methods shape (`sample` optional, `reparameterized_sample` optional) | three-required-methods shape plus capability sub-contracts including `ReparameterizedSampleable` | §27 canonical form."

- **`planning/v2/v2.1_in_progress.md:624-642` Stdlib distributions list:**
  > "Continuous, reparameterizable: `Normal<U>`, `LogNormal<U>`, `Uniform<U>`, `StudentT<U>`, `HalfNormal<U>`, `Exponential<U>`, `Cauchy<U>`"
  > "Continuous, simplex/unit-interval: `Beta`, `Dirichlet` (both dimensionless)"
  > "Continuous, shape/scale: `Gamma<U>`"
  > "Discrete: `Bernoulli`, `Categorical`, `Poisson`, `Binomial`, `NegativeBinomial`, `Geometric`"

  Superseded by §27.1's expanded Tier 1 list (19 univariate continuous, 5 discrete, 3 multivariate). The v2.1_in_progress list omits InverseGamma, Lévy, Weibull, Pareto, Fréchet, Gumbel, GEV, ChiSquared, Laplace, HalfCauchy (all promoted under chunk 04 O2.2 sub-q 4 on 2026-04-20). The discrete list shift (`Binomial`, `NegativeBinomial`, `Geometric` → `NegBinomial`, `Hypergeometric`) is substantive: Binomial and Geometric are dropped from the Tier 1 discrete row in §27.1 (see Conflicts below), and Hypergeometric is added.

  `Recommend:` The family-set expansion itself is a legitimate supersession. The dropped `Binomial`/`Geometric` entries are flagged under Conflicts.

- **`planning/v2/open_questions_deprecated_use_spec_new.md:871-877` MVN deferral:**
  > "Deferred pending vector/matrix/container story lock. The distribution's log-pdf is standard; the typing question is how mean vectors and covariance matrices are declared"

  Superseded by §27.1's reframed "gated on B5" position. Anti_spec.md line 88 already captures this retirement. No further action.

- **`planning/v2/open_questions_deprecated_use_spec_new.md:908-913` VI reparameterization deferral:**
  > "The `Distribution<U>` contract reserves the optional `reparameterized_sample` hook for when VI arrives."

  Superseded by §27's hoisting of reparameterization from an optional-method hook to the `ReparameterizedSampleable` capability sub-contract. Sub-contract form advertises the closure at family level rather than method level, which is what Tier A dispatch needs. VI backend remains a future item; the advertisement machinery is first-class in §27.

  `Recommend:` Consider an anti_spec.md entry: "`reparameterized_sample` as optional method on `Distribution<U>` | `ReparameterizedSampleable` capability sub-contract | §27 promotes the hook to a closure advertisement."


---

## Homeless

Corpus content that is relevant to §27, not accounted for in spec_new.md §27, and not already committed to anti_spec.md. This is the highest-value bucket.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md:674-706` per-family capability table (continuous):**

  The chunk-04 capability table carries seven columns (`Dist`, `Reparam`, `AffineSC`, `ScaleSC`, `SumSC`, `ProductSC`, `SmoothXform`) with per-row annotations including footnotes that encode conditional advertisement:
  > "3. Sum closes with same rate — predicate on the rewrite."
  > "4. Infinite variance (Cauchy, HalfCauchy, Lévy) — delta method formally fails. Intentionally excluded from `SmoothTransformable`."
  > "5. Student-t `SmoothTransformable` only for ν > 2 — rewrite-level predicate."

  §27.1's table carries only the capability shorthands (D, A, S, P, Sc, ST, R, Conj(X)) and does not reproduce the footnote-level conditional predicates. For example, §27.1 shows `StudentT` with only `D` — the ν > 2 `SmoothTransformable` condition from chunk 04 is elided. `Cauchy` in §27.1 shows `D, S` but not the intentional `SmoothTransformable` exclusion, and §27.1 does not record that `Lévy` and `HalfCauchy` are intentionally excluded from `SmoothTransformable`.

  `Recommend:` Either extend §27.1's row for StudentT to note the `ν > 2` condition on `SmoothTransformable`, or add a footnote block after the table capturing the "infinite variance → intentionally not `SmoothTransformable`" policy. This is settled design from chunk 04 (not open work) and narrows what Tier B delta-method rewrites may fire. Alternatively, push the full footnoted table into the §27.4 "extended capability table" that §27.4 already promises to live in the stdlib reference — but §27 should at least name the ν > 2 / infinite-variance policy rather than silently dropping it.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md:711-727` per-family discrete table:**
  > "| Bernoulli | ✓ | — | —² | Sum of iid → Binomial (cross-family) |"
  > "| Binomial | ✓ | — | ✓³ | Same-p predicate |"
  > "| Geometric | ✓ | — | — | |"

  Chunk 04 Tier 1 discrete table lists seven families: Bernoulli, Binomial, Poisson, Categorical, Geometric, NegBinomial, Hypergeometric. §27.1 lists only five discrete families: Bernoulli, Categorical, Poisson, NegBinomial, Hypergeometric. Binomial and Geometric are dropped. This is either a deliberate trim (not documented) or an omission.

  Further, chunk 04's footnote 2 captures a cross-family rewrite ("Bernoulli same-p sum → Binomial") that §27 does not record. §27.3's conjugate catalog does list Beta-Binomial as a rewrite, which presupposes Binomial exists as a Tier 1 family.

  `Recommend:` Decide whether Binomial and Geometric are Tier 1 families or not and record the decision. The Beta-Binomial conjugate in §27.3 and the Binomial-shared-p reference in §13.2 both presume Binomial is a Tier 1 family; §27.1 should add the row. Geometric is probably intentionally cut (low priority) but anti_spec.md should note it if so. See also Conflicts.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md:729-743` multivariate conditional structure:**
  > "Dirichlet and Multinomial do not require matrix machinery (they're vector-valued); they can ship independently. MVN requires `cholesky(Σ)` from chunk 05."

  §27.1 gates all three multivariate families (MVN, Dirichlet, Multinomial) on B5, but chunk 04 makes the finer point that only MVN actually needs matrix machinery. Dirichlet and Multinomial are vector-valued; their `Σ`-free parameterizations do not depend on B5 resolution.

  `Recommend:` Refine §27.1's "gated on B5" footer paragraph to note that only MVN's `Σ` carrying depends on B5; Dirichlet and Multinomial's `α[d]` and `p[K]` parameters are plain vector-valued and not B5-gated. This matches chunk 04 and avoids over-committing the gate.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md:878-889` B1 opaque log_pdf design items:**
  > "Design items: (a) should stdlib ship these at all, or route through user-declared `approximate` blocks? (b) if yes, what autodiff infrastructure is required? (c) how do they interact with Tier C PPL backend routing?"
  > "Driving family: general α-stable (only characteristic function is closed; PDF requires numerical inverse-FFT)."

  §27 preamble and §13.2 Tier C bullet both defer B1 to §33. §33 (lines 4810-4818) lists B1 as a one-liner. The driving family (α-stable) and the three design items (stdlib admissibility, AD infrastructure, Tier C routing interaction) are named in chunk 04 but do not appear in §27 or §33.

  `Recommend:` B1 is §33 blocker, not §27 open work, so §27 itself is fine. But the B1 entry in §33 should name the driving α-stable use case and the three design items so that when B1 is un-deferred there is a record of what question it was answering. This is a §33 gap surfaced by the §27 audit, not a §27 gap.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md:755-768` conjugate catalog (fuller than §27.3):**
  > "Normal-InverseGamma → InverseGamma posterior (variance)"
  > "NormalInverseGamma (joint μ, σ²) → NormalInverseGamma posterior — flag: verify rewrite-pattern language handles joint priors; add check to chunk 04 Phase 3 topic-list pass"
  > "Gamma-Gamma → Gamma posterior"

  Chunk 04 enumerates eight conjugate pairs: the six §27.3 records plus NormalInverseGamma (joint μ, σ² prior) and Gamma-Gamma. §27.3's table has six rows; NormalInverseGamma and Gamma-Gamma are absent.

  `Recommend:` Either add NormalInverseGamma and Gamma-Gamma to §27.3's conjugate table (chunk 04 explicitly promotes them under the 2026-04-20 scope resolution), or record their exclusion with a rationale. If NormalInverseGamma is held back pending the flagged "verify rewrite-pattern language handles joint priors" check, §27.3 or §35 should note that gate. Current §27.3 states "The catalog is closed for this release" without acknowledging these two promoted-in-chunk-04 rows.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md:917-934` B4 coupling machinery consequences for Z-group:**
  > "Rewrite suppression: if `x` and `y` are copula-coupled, `x + y` cannot use `SumSelfClosed` even when both are Normal (result isn't Normal unless correlation ≠ 0 is absorbed)."
  > "Algorithm for detecting when independence-assuming rewrites may fire vs. must suppress."

  §13.5 locks independence via e-class structural identity and §13.2 Tier A fires closure contracts like `SumSelfClosed`. Chunk 04 identifies a concrete case where Z-group contracts must not fire: copula-coupled operands have distinct e-classes (so structural identity would say "independent") but are in fact coupled. The rewrite-suppression requirement is a stable constraint on §27's capability-dispatch mechanics, not open design.

  `Recommend:` Add a sentence to §27's `SumSelfClosed` / `ProductSelfClosed` / `AffineSelfClosed` bullets (or to §13.2 Tier A) noting that these contracts fire only when operands pass the e-class-identity independence check AND carry no coupling envelope fact. When B4 lands and introduces copula annotations, the closure contracts must consult the envelope. The principle is locked; only the machinery for detecting the coupling flag is pending.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md:801-804` v2.2+ deferrals:**
  > "Stable distributions with non-closed-form characteristic-function tricks beyond α-stable proper"
  > "Matrix-Normal, generalized Wishart variants"
  > "Sklar-theorem-based copula decomposition as a rewrite rule"

  These are explicit v2.2+ deferrals. §27.5 Tier 3 paragraph covers GPs, DPs, CRPs, IBPs, Beta Processes but does not mention Matrix-Normal, generalized Wishart variants, or Sklar-theorem copula decomposition. The latter three have a different character (parametric extensions of shipped families, not non-parametric process families) and likely belong in Tier 2 rather than Tier 3.

  `Recommend:` Add a sentence to §27.5 Tier 2 or §35 Tier 2 distribution machinery bullet noting that Matrix-Normal, generalized Wishart variants, and Sklar-theorem copula decomposition are explicit v2.2+ items (not Tier 2 design work). Currently these three items have no home in spec_new.md.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md:696-706` footnote on `ReparameterizedSampleable` for Student-t:**
  > "2. Implicit reparameterization (RSVI-style); standard PPL practice."

  Chunk 04's table marks Student-t, Gamma, ChiSquared, and InverseGamma with `Reparam ✓²` — implicit reparameterization. §27.1 shows `StudentT` with only `D` (no R). Chunk 04 documents that this is RSVI-style implicit reparameterization, a "standard PPL practice" surface.

  `Recommend:` Reconcile. Either §27.1 Student-t should carry `R` (implicit reparameterization, footnoted) or chunk 04's `✓²` marks should be withdrawn. Current mismatch is substantive: a downstream reader of §27.1 would not know Student-t supports implicit reparameterization and would route Student-t transforms to Tier B/C unnecessarily. See also Conflicts.

---

## Conflicts

Direct contradictions between spec_new.md §27 and any corpus document.

- **§27.1 discrete family roster vs. chunk 04 and v2.1_in_progress discrete rosters:**

  §27.1 discrete table lists 5 families: `Bernoulli`, `Categorical`, `Poisson`, `NegBinomial`, `Hypergeometric`. Chunk 04 table (§11, line 714-721) lists 7: Bernoulli, Binomial, Poisson, Categorical, Geometric, NegBinomial, Hypergeometric. v2.1_in_progress (line 636-637) lists 6: Bernoulli, Categorical, Poisson, Binomial, NegativeBinomial, Geometric.

  Binomial is absent from §27.1's discrete row but appears in §27.3 ("Beta-Binomial" conjugate), in §13.2 ("Binomial only under shared success probability"), and in Appendix C implicitly via the Binomial-conjugate rewrite. Geometric is absent from §27.1 and from §27.3 but appears in v2.1_in_progress and in chunk 04.

  `Recommend:` Add `Binomial` to §27.1's discrete table. The §27.3 Beta-Binomial conjugate and §13.2 shared-p reference both presume Binomial is a Tier 1 family. Geometric's absence is a separate call: either add it (chunk 04 commits) or add an anti_spec.md entry documenting the drop.

- **§27.1 capability rows vs. chunk 04 capability tables (multiple elisions):**

  Systematic mismatches between §27.1 shorthand rows and chunk 04 §11 tables (lines 674-721):

  - **StudentT.** §27.1: `D`. Chunk 04 line 686: `Dist, Reparam, AffineSC, ScaleSC, SmoothTransformable (ν > 2)`. §27.1 elides five capabilities; the `AffineSelfClosed` elision is substantive (Student-t `a·X + b` is a Tier A closure with location/scale shift).
  - **Normal.** §27.1: `D, A, S, ST, R`. Chunk 04 line 678 also lists `ScaleSC`. If §27 implicitly treats `AffineSelfClosed` as implying `ScaleSelfClosed` (chunk 04 contract-refinements section, line 659-665, states this implication), the elision is defensible; §27 does not state the implication.
  - **LogNormal.** §27.1: `D, P, ST`. Chunk 04 line 679: `Dist, Reparam, ScaleSC, ProductSC, SmoothXform`. §27.1 elides `R` and `Sc`. LogNormal via `exp(Normal)` is the canonical reparameterized-sampleable positive-support distribution.
  - **Poisson.** §27.1: `D, Conj(Gamma)`. Chunk 04 line 717: `Dist, SumSC (unconditional)`. §27.1 omits Poisson's unconditional sum-closure, load-bearing for Gamma-Poisson conjugate machinery.

  `Recommend:` Either state the `A ⇒ Sc` implication in §27 preamble and rework capability-row derivations accordingly, or exhaustively list the tags chunk 04 locks. Add the Student-t `AffineSelfClosed` tag (with ν > 2 note on `SmoothTransformable`), LogNormal's `R, Sc`, and Poisson's `S`. Chunk 04 is the more careful enumeration and should be the reconciliation target.

- **§27.3 conjugate catalog closure claim vs. chunk 04 promoted conjugates:**

  §27.3: "The catalog is closed for this release; additional conjugate pairs that modelers need are either derivable via `Truncated` / `Mixture` composition or route to Tier 2 (chunk 08)."

  Chunk 04 explicitly promotes Normal-InverseGamma, NormalInverseGamma (joint), and Gamma-Gamma as "shipping in v2.1" (line 764-768). §27.3 ships only six of the eight. Either the §27.3 "closed catalog" claim is accurate and chunk 04 is stale on two rows, or §27.3 is undercounting two promoted conjugates.

  `Recommend:` Decide which. If §27.3 is correct, add an anti_spec.md entry capturing that NormalInverseGamma / Gamma-Gamma were briefly on the list but cut. If chunk 04 is correct, extend §27.3 to eight rows.

- **§27 `pdf` as required method vs. v2.1_in_progress and chunk 04 contract shape:**

  §27 requires three methods: `sample`, `log_pdf`, `pdf`. v2.1_in_progress `Distribution<U>` (lines 648-654) and chunk 04 pseudocode (lines 1466-1468) both carry only `log_pdf` and `sample`. `pdf` is introduced in §27 as a "convenience" with the option to either derive from `log_pdf` or provide directly.

  This is a spec evolution, not a conflict per se — but §27 says `pdf` is *required*, which is stricter than "convenience." If `pdf` is always derivable from `log_pdf` by `exp`, making it required only adds a no-op default. If direct `pdf` is genuinely load-bearing (avoiding log/exp round-trip for specific backends), §27 should state which backends need the direct form.

  `Recommend:` Clarify §27: either (a) mark `pdf` as a default-derived method (effectively optional), consistent with the chunk 04 / v2.1_in_progress precedent; or (b) state why `pdf` is required and which backend or rewrite rule consumes it directly. Current language "required... provided as a convenience... may be derived from `log_pdf`" is internally ambiguous.

- **§27 `log_pdf` return type vs. v2.1_in_progress and chunk 04:**

  §27: "`log_pdf(params, x: Scalar<U>) -> Scalar<unitless>`"
  v2.1_in_progress (line 650): `fn log_pdf(x: Scalar<U>) -> Scalar<dimensionless>`

  The return type terminology differs: `unitless` (§27) vs `dimensionless` (v2.1_in_progress). Elsewhere in spec_new.md `dimensionless` is the canonical unit name. This is a spelling consistency call.

  `Recommend:` Use `dimensionless` in §27 to match the rest of spec_new.md's unit vocabulary. If `unitless` is intentional (e.g., to distinguish "no unit attached" from "dimensionally dimensionless"), record the distinction in §5.
