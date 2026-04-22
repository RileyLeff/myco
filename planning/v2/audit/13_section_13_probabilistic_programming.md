# Audit Report — §13 Probabilistic Programming

---

## Absorbed

Corpus content that already landed in spec_new.md §13.

- **`planning/v2/v2.1_in_progress.md` §Probabilistic Programming — `~` operator and aleatoric/epistemic split:**
  > "Aleatoric (world claim, .myco). Observation noise, process noise, stochastic dynamics. The world contains randomness. Declared with `~`. Epistemic (experimenter claim, workflow). Prior distributions on unknown parameters reflecting pre-data beliefs."
  Absorbed into §13.1.

- **`planning/v2/v2.1_in_progress.md` §Probabilistic Programming — Itô/Stratonovich as generics on `~`:**
  > "`d(psi)/d(t) ~<Ito> Normal(drift_psi, alpha * psi / sqrt(dt))` ... They parameterize the operator when needed and have no effect otherwise."
  Absorbed into §13.4.

- **`planning/v2/v2.1_in_progress.md` §Probabilistic Programming — automatic marginalization:**
  > "When a stochastic discrete variable is latent ... the compiler automatically marginalizes it out of the likelihood by enumerating its support."
  Absorbed into §13.3.

- **`planning/v2/v2.1_in_progress.md` §Probabilistic Programming — stochastic dynamics compilation:**
  > "Probabilistic backend (e.g., NumPyro): `~` claims compile to direct HMC/NUTS targets ... Deterministic backend (point-estimate / MLE / MAP): `y ~ Normal(μ, σ)` compiles to a negative-log-likelihood loss term."
  Absorbed into §13.2 (Tier C handoff prose) and §31.2.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` CC4 — Tier A/B/C three-tier dispatch:**
  > "| 24 | Three-tier distributional propagation: Tier A closed-form / Tier B approximate / Tier C opaque | 12 (CC4) |"
  Absorbed into §13.2.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` CC4 — distribution capability contracts:**
  > "| 23 | Distribution capabilities decomposed into multiple contracts (`AffineSelfClosed`, `SumSelfClosed`, ...); cross-family rules are rewrite declarations | 12 (CC4) |"
  Absorbed into §13.2 and §7.2.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` CC4 — independence via e-graph structural identity:**
  > "| 25 | Independence via e-graph structural identity; joint declarations reparameterize to independent bases; copulas are Tier C | 12 (CC4) |"
  Absorbed into §13.5.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` Z10 — MVN Cholesky reparameterization:**
  > "Z10 — MVN auto-reparameterization via Cholesky to independent bases"
  Absorbed into §13.6.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` design blockers B2/B4 — coupling machinery and joint declaration syntax deferred to chunk 08:**
  > "B2 — Joint distribution declaration syntax ... Status: open. Will land in a future chunk 07 paired with B4. ... B4 — Coupling machinery (non-independence in envelope)."
  Absorbed into §13.10 (Tier 2 PPL lock).

- **`planning/v2/open_questions.md` §Probabilistic Inference — resolved decision:**
  > "Probabilistic programming is a first-class v2.1 language feature, not a post-hoc addition."
  Absorbed into §13 preamble and throughout.

- **`planning/v2/open_questions.md` §Sequential inference for time-varying discrete latents (HMMs):**
  > "A latent discrete variable with Markov transitions over time ... requires forward-backward, Viterbi, or particle filter inference — not compile-time marginalization. The v2.1 compiler detects the pattern and errors with guidance."
  Absorbed into §13.3 (failure path and latent discrete compile-error behavior reference this pattern).

- **`planning/v2/anti_spec.md` retired open question:**
  > "`~` stochastic as e-graph merge | resolved — `~` is layer-2 distributional metadata, not a merge"
  Absorbed into §13 summary and §13.9.

- **`planning/v2/anti_spec.md` retired open question:**
  > "MVN 'deferred pending vector/matrix story' | reframed — gated on B5 heterogeneous-unit resolution"
  Absorbed into §13.6 (L directly suppliable or derived at compile time) and §27.5 (Tier 2 distribution machinery).

- **`planning/v2/spec_dev_notes.md` §13 subsection accounting:**
  > "13.1 aleatoric vs epistemic ... 13.10 Tier 2 lock (coupling / joint declaration deferred to chunk 08; higher-order distributions route via §28)."
  All ten subsections present in spec_new.md §13.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` — Tier C PPL handoff protocol:**
  > "PPL backend protocol. Concrete handoff for Tier C distributional inference ... wire format for Tier C handoff (e-class identity, parametric form from envelope, layer-1 term, capability requirements, observation constraints)."
  Absorbed into §13.2 (Tier C description) and §31.2 (backend handoff protocol).

---

## Superseded

Corpus content replaced by a newer decision in spec_new.md §13.

- **`planning/v2/open_questions.md` §Closure policy — `condition_weighted` deferred:**
  > "`condition_weighted` deferred beyond v2.1. Conditioning-aware weighting requires either a `condition_of(expr)` compiler intrinsic ... Most v2.1 workflows reconcile overdetermined systems via controller plus consistency loss rather than via conditioning-aware closure."

  Superseded by spec_new.md §8.7:
  > "Y4 `condition_weighted` — weights candidates by numerical conditioning; backed by `condition_of` Levels I-III (§14)."

  `condition_weighted` was un-deferred in chunk 04 (item 32: "Y4 `condition_weighted` un-deferred and ships in v2.1; closes O4.5") and ships in spec_new.md. `open_questions.md` has not been updated to reflect this. The anti_spec.md entry confirms the resolution:
  > "`condition_weighted` deferred | resolved — ships via `condition_of` Levels I-III (chunk 04 O4.5)"

  The open_questions.md text should be struck or annotated resolved; the anti_spec.md entry already covers it.

- **`planning/v2/v2.1_in_progress.md` §Distribution contract — `sample` described as "required for simulation":**
  > "Only `log_pdf` is strictly required for HMC/NUTS inference. `sample` is required for simulation and for backends that sample latent variables. `reparameterized_sample` is optional and only matters for variational inference backends not on the v2.1 critical path."

  This is a fine-grained contract that is not reproduced in spec_new.md §13. The spec_new.md §13.8 references `D.log_pdf(data)` but has no `Distribution<U>` contract block specifying required vs optional methods. The full contract surface lives in §27 (distribution families table), not §13. The `v2.1_in_progress.md` text is not strictly superseded but is displaced to §27, making the §13 picture incomplete (see Homeless).

---

## Homeless

Corpus content relevant to §13, not accounted for in spec_new.md §13, and not committed to anti_spec.md.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` B1 — opaque `log_pdf` stdlib policy (open design question):**
  > "B1 — Opaque `log_pdf` stdlib policy. Does stdlib permit distribution families whose `log_pdf` is a structurally-opaque numerical evaluator (inverse-FFT of characteristic function, table-lookup, etc.) rather than a symbolic expression? ... Status: open. Not blocked on anything else."

  spec_new.md §33 lists B1 as an open blocker: "B1. Opaque log_pdf stdlib policy." But §13 itself does not mention B1 or describe what Tier C routing implies for distributions with opaque `log_pdf`. A user reading §13 alone has no signal that `log_pdf` transparency is an open question affecting which families can reach Tier A vs C, or that the consequence for `condition_of` analysis is undefined for opaque `log_pdf`.

  Recommend: add a one-sentence forward reference in §13.2 or §13.3 noting that the Tier A/B dispatch assumes a symbolic `log_pdf`; families with structurally opaque `log_pdf` (B1) route to Tier C pending B1 resolution, tracked in §33.

- **`planning/v2/v2.1_in_progress.md` §Distribution contract — `log_pdf` required, `sample` required for simulation, `reparameterized_sample` optional for VI:**
  > "contract Distribution<U: Unit>: fn log_pdf(x: Scalar<U>) -> Scalar<dimensionless>; fn sample(key: RngKey) -> Scalar<U>; // Optional — required only for pathways gradients (VI backends); fn reparameterized_sample(key: RngKey, params) -> Scalar<U>"

  spec_new.md §13.8 references `D.log_pdf(data)` without ever stating the contract interface. The `Distribution<U>` contract specification appears in neither §13 nor §27 in spec_new.md (searches on `contract Distribution` and `log_pdf.*required` return no results in spec_new.md). It is a stable settled decision.

  Recommend: add the `Distribution<U>` contract block to §27 (the families section) with cross-reference from §13. The three-method structure (required `log_pdf`, required `sample`, optional `reparameterized_sample`) is a concrete contract the compiler enforces and users extend; its absence from the spec is a gap.

- **`planning/v2/open_questions.md` §Workflow-side API for epistemic priors (`assume_prior`):**
  > "Parameter priors (Bayesian beliefs about unknown values) live workflow-side. The verb name and signature (`assume_prior(path, Distribution)` or similar), composition with other `assume_*`/`learn_*` verbs, per-parameter vs vectorized priors, and hierarchical-prior construction are workflow design questions."

  spec_new.md §13.1 states that epistemic uncertainty "reduces with observation via Bayesian update; participates in training" but says nothing about how epistemic priors are bound. §24.4 mentions `assume_prior` as a candidate future verb but marks it deferred. The §13.1 description of epistemic `~` appearing "at module scope or in `initial:`" is internally consistent but has no connection to any workflow mechanism that supplies the prior. This is the aleatoric/epistemic split's open end on the workflow side.

  Recommend: add a sentence in §13.1 cross-referencing §24.4 and noting that the workflow-side API for supplying priors (`assume_prior` or equivalent) is tracked as a future verb; currently, epistemic `~` at module scope participates in training via `learn_constant` with no explicit prior shape.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` Z-group rewrite catalog (Z1-Z10) — specific rewrite rules not enumerated in §13:**
  > "Z1 — Affine of Normal → Normal, via `AffineSelfClosed` ... Z7 — Conjugate posterior updates (cross-family rewrite declarations) ... Z8 — Delta method: smooth nonlinear transform → Normal (`SmoothTransformable`) — Tier B ... Z9 — Fenton-Wilkinson: sum of Log-Normals → approximate Log-Normal via moment matching — Tier B"

  spec_new.md §13.2 names the Tier B approximation methods (Delta method, Fenton-Wilkinson, CLT, block-maxima → GEV) but does not link them to the Z-group rewrite labels. The full catalog of Tier A and B distributional rewrites lives in chunk 04 only. Appendix C of spec_new.md is noted as carrying the rewrite catalog, but probabilistic Z-group rewrites should appear there for completeness.

  Recommend: confirm that Appendix C (spec_new.md) includes the Z-group (Z1-Z10) rewrites alongside A-Y groups. If not, add a §13.2 footnote cross-referencing Appendix C for the distributional rewrite catalog.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` — `SumSelfClosed` reframed as "sum rewrite registered," not "unconditionally sum-closed":**
  > "Frame `SumSelfClosed` as 'a sum rewrite is registered.' Some families sum-close unconditionally (Normal, Cauchy, Poisson); others with matching parameters (Gamma same-rate, Binomial same-p, ChiSquared always-closes-dof-adds). Parametric predicates live on the rewrite rule (CC5 site-scoped predicate machinery), not as additional contracts."

  spec_new.md §13.2 names `SumSelfClosed` as a Tier A closure contract but does not convey the conditional-rewrite nuance (same-rate Gamma, same-p Binomial). A user reading §13.2 would believe `SumSelfClosed` is a binary flag, not a "rewrite is registered (possibly conditionally)." The full semantics landed in chunk 04 and §27.1's capability table but not in §13.2 prose.

  Recommend: add a one-sentence qualification in §13.2 noting that some sum-closure rewrites carry parametric predicates (same-rate, same-p), not all contracts are unconditional; detail lives in §27.

- **`planning/v2/open_questions.md` §Sequential inference for time-varying discrete latents (HMMs) — compile-error guidance detail:**
  > "The v2.1 compiler detects the pattern and errors with guidance. Full design covers: Syntactic recognition of Markov-structured latent discrete chains. Which inference algorithms to generate ... Whether PPL machinery (Pyro's `markov`, NumPyro's `contrib.funsor`) covers enough to lean on."

  spec_new.md §13.3 says "Failed marginalization falls through to Tier B/C dispatch" but does not mention the HMM pattern specifically. The open_questions.md treatment is a deferred design pass, not a stable decision, so this is legitimately not homeless by the "open design work" carve-out. However, §13.3 should note that Markov-structured discrete latents are a recognized compile-error case rather than a silent Tier C fallthrough.

  Recommend: add a sentence in §13.3 noting that discrete latents with Markov temporal structure produce a compile error with guidance (sequential inference not yet supported); distinguish from the auto-marginalization path.

---

## Conflicts

Direct contradictions between spec_new.md §13 and corpus documents.

- **`condition_weighted` ship status:**

  `planning/v2/open_questions.md` (§Closure policy semantic interface):
  > "`condition_weighted` deferred beyond v2.1. Conditioning-aware weighting requires either a `condition_of(expr)` compiler intrinsic (parallel to `deriv`) or a compiler-provided black box — both have real cost."

  spec_new.md §8.7 (referenced by §13.2 which names "Tier A/B/C routing"):
  > "Y4 `condition_weighted` — weights candidates by numerical conditioning; backed by `condition_of` Levels I-III (§14)."

  spec_new.md §8 preamble:
  > "closure policies Y1-Y6 including un-deferred `condition_weighted` (backed by `condition_of` Levels I-III)."

  `condition_weighted` is a closure policy (§8.7), not a distributional operator, so the conflict does not arise inside §13 itself. The conflict is between open_questions.md and spec_new.md §8. It is flagged here because both the §13 Tier A/B/C dispatch description and the Tier 2 lock (§13.10) reference the closure-policy layer, and a reader of §13 cross-checking open_questions.md for policy status will find a stale deferral claim.

  anti_spec.md already records: "`condition_weighted` deferred | resolved — ships via `condition_of` Levels I-III (chunk 04 O4.5)." The open_questions.md §Closure policy entry should be updated to match.

  Recommend: strike the `condition_weighted` deferral paragraph in `planning/v2/open_questions.md` §Closure policy semantic interface (lines 525-530) or prepend "RESOLVED — ships in v2.1." The spec_new.md §8.7 text is the authoritative statement.

- **Aleatoric `~` structural position — §13.1 vs v2.1_in_progress.md example:**

  spec_new.md §13.1:
  > "Aleatoric ... `~` appears inside `temporal:` or event scope. Realized via sampling; does not reduce with more data."

  `planning/v2/v2.1_in_progress.md` §Probabilistic Programming (sapflow example):
  ```
  relation sapflow_observation:
      measured_sap_flow ~ Normal(true_sap_flow, sigma_sapflow)
  ```
  This `~` is inside a plain `relation` block, not inside `temporal:` or event scope. Yet it is labelled as an observation likelihood (a world-noise claim), which spec_new.md §13.1 would classify as aleatoric.

  The spec_new.md §13.1 structural rule ("aleatoric `~` appears inside `temporal:` or event scope") is too narrow: observation likelihoods coupling measured data to latent states are aleatoric claims that naturally live in `relation` bodies at module scope. The distinction between aleatoric and epistemic cannot be purely "inside temporal/event" vs "module scope."

  Recommend: tighten the §13.1 structural rule. The aleatoric/epistemic split is correctly conceptual; the implementation rule likely needs to be "aleatoric when the LHS is a measured/observed quantity (tied to data) or when the `~` appears in a temporal/event context; epistemic when the quantity is an unknown constant not observed per time-step." The current wording risks incorrectly classifying observation likelihoods as epistemic.
