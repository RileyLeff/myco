# Audit Report — Appendix C: Rewrite Catalog (A–Z)

Audited against corpus as of 2026-04-22.

---

## Absorbed

Content the corpus has settled and Appendix C correctly reflects.

### A-group (ring/field axioms, A1-A10) — LOCKED

Chunk 04 `04_egraph_foundation_in_progress.md` O2.2 Bucket 1 explicitly lists "A1-A10 — ring/field axioms" as fully committed with no blocking dependencies. §17.1 source 3 ("Algebraic rewrites") is the authorization source; §17.5 describes A as "commutativity, associativity, distributivity, identity elements." Appendix C's LOCKED tag is accurate.

### B-group (constant folding, B1-B2) — LOCKED

B1 confirmed in chunk 04 Bucket 1. B2 (universal-to-literal substitution at workflow time) is noted in chunk 04 Bucket 4 as "absorbed by CC1 + V1": workflow-bound constants enter as observation-style equalities, not compile-time folds. Appendix C B2's note "per the CC1 literal-numerics lock (§4)" is consistent with the CC1 LOCKED status in chunk 04 §CC1 and spec_new §4.

### C-group (unit normalization, C1-C4) — LOCKED

Chunk 04 Bucket 1 lists "C1-C4 — unit/dimensional normalization." §17.1 source 8 ("Unit-preserving rewrites") is the authorization source. Chunk report `02_collections_iteration_report.md` does not conflict. Appendix C's LOCKED tag is accurate.

### D-group (named-type normalization, D1-D5) — LOCKED

Chunk 04 Bucket 1: "D1-D5 — named-type normalization (requires O2.1 round-trip verification, locked)." O2.1 is explicitly marked RESOLVED (2026-04-20) in chunk 04 §O2.1. §17.1 source 6 ("Named-type conversion") covers D-group. Appendix C's LOCKED tag and D3's O2.1 reference are accurate.

### E-group (function-inverse, E1-E2) — LOCKED

Chunk 04 Bucket 1 includes E1-E2. §17.1 source 5 ("Stdlib-declared function inverses") and §17.3 confirm the mechanism. E2's built-in pairs (exp/log) match §17.3 examples. spec_new §5.2 covers the round-trip verification referenced by D3 and the CC2 elimination. Appendix C's LOCKED tag is accurate.

### F-group (geometry seam, F1) — LOCKED (partially)

Chunk 04 Bucket 1 explicitly lists "F1 — `identify`-seam merge for scalar fields (unblocked by CC5)." Geometry chunk report `01_geometry_design_report.md` §2.8 confirms the `identify phi = 0 <-> phi = 2*pi` seam example matches F1. The vector/tensor seam OPEN annotation in Appendix C is consistent with chunk 01 §2.8: "Scope for v2.1: `identify` is guaranteed only for scalar fields." Appendix C accurately notes the partial OPEN.

### G-group (transcendentals, G1-G3) — LOCKED

Chunk 04 Bucket 1 includes G1-G3. A10 Arrhenius canonicalization reference (Appendix C A10) and G1's Arrhenius note are internally consistent. Appendix C's LOCKED tag is accurate.

### H-group (aggregates, H1-H2) — LOCKED

Chunk 04 Bucket 1 confirms H1-H2. The empty-collection identities in H2 match the table in `02_collections_iteration_report.md` §4.4 (`sum(empty)→0`, `product(empty)→1`, `any(empty)→false`, `all(empty)→true`, `count(empty)→0`). Appendix C's LOCKED tag is accurate.

### I-group (conditionals, I1) — LOCKED

Chunk 04 Bucket 1 lists I1. The three cases (`if true then a else b → a`, `if false then a else b → b`, `if c then a else a → a`) are natural consequence of algebraic rewriting. Appendix C's LOCKED tag is accurate.

### J-group (temporal forbidden merge, J1) — LOCKED

The no-merge-across-timesteps invariant is confirmed in chunk 04 §4 ("y[t] and y[t+1] are distinct ground terms — they are not claimed equal anywhere") and §17.1's "What does NOT merge" section. Appendix C's phrasing — "never merge across timesteps even if numerically equal at runtime" — accurately reflects Section 4 of chunk 04 and spec_new §21.3. Appendix C's LOCKED tag is accurate.

### K1 (kernel compact-support truncation) — LOCKED in corpus

Chunk 04 O2.3 (RESOLVED 2026-04-20) explicitly moves K1 to Bucket 1 (fully committed): "K1 (kernel compact-support truncation)... move to Bucket 1 (fully committed)." See also chunk 04 §O2.3: "Unblocked rewrites... K1." Appendix C correctly marks K1 LOCKED.

### L1 (smooth_min sharpness collapse) — LOCKED

Chunk 04 Bucket 1 explicitly lists "L1 — `smooth_min` → `min` forward-only." §8.9 confirms "compiler does not auto-smooth." Appendix C's LOCKED tag is accurate.

### O1 (training-mode consistency loss) — OPEN (O4.3)

Chunk 04 §O4.3 marks per-residual training emission as still open: "Standard CSE-style canonicalization would collapse them. The e-graph can hold both forms; the extraction policy must be aware. Section 12 open." spec_new §35 confirms "O4.3 per-residual training emission... still open." Appendix C's OPEN tag and "§35, chunk 04 O4.3" pending citation are accurate.

### Q-group (probabilistic marginalization, Q1-Q2) — OPEN

Chunk 04 Bucket 1 includes "Q1-Q2 — probabilistic truncation/marginalization (unblocked by CC4)." However, Appendix C marks Q as OPEN pending "§35, stochastic rewriting semantics." The chunk 04 document moved Q1-Q2 to Bucket 1 — see "stochastic rewriting semantics" as the remaining open referenced in Appendix C may be a legitimate residual design question about the full stochastic-rewriting semantics (not just Q1-Q2 firing). Appendix C's OPEN is not contradicted by corpus; it is more conservative than chunk 04's Bucket 1 placement. See Conflicts section.

### R-group (lossy-function simplification, R1-R3) — LOCKED

Chunk 04 Bucket 1 lists R1-R3. §17.6 confirms that R-group (abs/max/min/floor/relu/clamp under envelope) is default-on. Appendix C's LOCKED tag is accurate.

### S-group (opaque callables, S1-S2) — LOCKED

Chunk 04 Bucket 1 lists S1-S2. §17.1's "What does NOT merge" confirms opaque callables have no reverse rewrite. §24.1 `bind_controller` is referenced by S2. Appendix C's LOCKED tag is accurate.

### T-group (one-way convert, T1) — LOCKED

Chunk 04 Bucket 1 lists T1. §5 confirms `convert A -> B` installs forward-only. Appendix C's LOCKED tag is accurate.

### U-group (named-type stripping, U1-U3) — LOCKED

Chunk 04 Bucket 1 lists U1-U3. §3 named-type section and §17.1 source 6 confirm directionality. Appendix C's LOCKED tag is accurate.

### V-group (observation injection, V1) — LOCKED

Chunk 04 Bucket 1 lists V1. §13.8 and §13.9 confirm observation injection as a Layer-2 envelope fact (not a Layer-1 equational merge for the path itself). Appendix C's note "Not an equational merge: `path` is not rewritten to `data` in layer 1" accurately reflects chunk 04 §CC1 and §17.1 source 2. LOCKED tag is accurate.

### W1 (obligation retraction) — OPEN (O4.1)

Chunk 04 §O4.1 explicitly marks `replaces` obligation retraction as still open with three candidate semantics. spec_new §35: "O4.1 `replaces` obligation retraction (rewrite group W1 in Appendix C; three candidate semantics still open)." §17.1 also notes: "`replaces` (§8.10, §10.5) suppresses the default-generation merge... Broader retraction semantics are tracked as §35 O4.1." Appendix C's OPEN tag and "(chunk 04 O4.1, cross-ref §8.10, §10.5, §15, §16, §35)" are accurate.

### X-group (site-gated strict, X1-X2) — LOCKED

CC5 resolved on 2026-04-20 (predicate gating) and 2026-04-22 (data path). Chunk 04 §CC5 LOCKED. spec_new §35 has the CC5 data-path paragraph: "CC5 site-gated strict rewrites: data path resolved... Appendix C X splits into X1 (pole L'Hopital) and X2 (identify / quotient-induced value equality)." anti_spec.md confirms the "X-category bundling pole L'Hopital and `identify` as one rewrite shape" is retired; they are now X1 / X2 split. §17.1 source 4 (`identify` via Layer-3 site records consumed by X2) and §17.1 source 3 (algebraic rewrites for X1) are the authorization sources. Appendix C's LOCKED tag and 2026-04-22 date for data path are accurate.

### Y1-Y3 (closure policies) — LOCKED

Chunk 04 Bucket 1 lists "Y1-Y3 — `weighted_average`, `soft_select`, `hard_select` closure policies." §8.7 confirms all three. Appendix C's LOCKED tag is accurate.

### Y4 (condition_weighted) — LOCKED

Chunk 04 §O4.5 RESOLVED (2026-04-20). Chunk 04 O2.4 sub-question 3: "RESOLVED (2026-04-20): un-defer and ship." spec_new §8.7 confirms Y4 ships. Appendix C's "LOCKED (un-deferred 2026-04-20, closes O4.5)" is accurate.

### Y5 (user-defined closure policy) — LOCKED

Chunk 04 Bucket 1 lists Y5. §8.8 ("Y5: User-Defined Closure Policies") confirms the surface. Appendix C's LOCKED tag is accurate.

### Z1, Z5, Z10, Z11 — LOCKED

Z1 (affine closure) and Z5 (exp/log transform) are in chunk 04 Bucket 2 under CC4 LOCKED. Z10 (MVN Cholesky reparameterization) is confirmed in spec_new §13.6. Z11 (pushforward under invertible differentiable map) is referenced at spec_new line 2140-2141 and §27.1 (`ReparameterizedSampleable`). All four LOCKED tags in Appendix C are accurate.

### CC1 absorption claim — ACCURATE

Appendix C line 5524: "CC1 literal-numerics (§4, §4.1)." spec_new §4 and §4.1 carry the CC1 lock. Chunk 04 §CC1 confirmed LOCKED. spec_dev_notes.md has the "2026-04-20 — CC1 literal-numerics scope" entry. Accurate.

### CC2 absorption claim — ACCURATE

Appendix C line 5524: "CC2 sanity inverses (§5.2 round-trip)." §5.2 "Round-Trip Verification (O2.1)" carries the CC2 resolution (annotation eliminated; O2.1 absorbs it). Chunk 04 §CC2 confirmed ELIMINATED. Accurate.

### CC4 absorption claim — ACCURATE

Appendix C line 5526: "CC4 stochastic `~` rewrite blank (§13.8 resolved 2026-04-20)." Chunk 04 §CC4 LOCKED. spec_new §13.8 carries the distributional claim semantics. Accurate.

### §17.5 cross-reference — ACCURATE

§17.5 ("Rewrite-Rule Groups A-Z") describes A (algebraic), E (equality/merge), Y (closure-policy), Z (distribution-family) as representative groups and states "Full catalog lives in Appendix C." The A-Z catalog is the elaboration of §17.5's commitment. No conflict found.

### §17.1 authorization-source claim — ACCURATE

Appendix C line 5194-5197: "Every rule below routes through one of the eight sources." Spot checks:
- A-group → §17.1 source 3 (algebraic rewrites)
- C-group → §17.1 source 8 (unit-preserving rewrites)
- D-group, T-group → §17.1 source 6 (named-type conversion)
- E-group → §17.1 source 5 (stdlib-declared function inverses)
- F1, X2 → §17.1 source 4 (`identify` declarations)
- V1 → §17.1 source 2 (workflow constant injection)
- Y-group → §17.1 source 7 (closure-policy co-membership)

All routes through eight sources confirmed.

### §27.3 vs Z-group reserved slots — ACCURATE

Appendix C line 5500-5503: "Intermediate Z-numbers (Z2-Z4, Z6-Z9) are reserved for conjugate-posterior rewrites (§27.3 catalog) and approximate closures (Tier B: Delta, Fenton-Wilkinson, CLT, block-maxima → GEV)."

§27.3 ("Conjugate-Posterior Rewrite Catalog") enumerates: Beta-Bernoulli/Binomial, Gamma-Poisson, Normal-Normal, InverseGamma-Normal, Dirichlet-Multinomial — 6 entries. Chunk 04 Bucket 2 proposes Z7 for conjugate posteriors (7 pairs listed there, including Normal-InverseGamma, NormalInverseGamma, Gamma-Gamma). Tier B rewrites (Z8 delta method, Z9 Fenton-Wilkinson) are confirmed by chunk 04 O2.3 resolution. That the reservation spans Z2-Z4 and Z6-Z9 (7 slots for 6+ conjugate pairs + 4 Tier B rewrites) is plausible and not contradicted.

---

## Superseded

### B2 as "universal-to-literal substitution compile-time fold"

Chunk 04 Bucket 4 explicitly labels B2 as "absorbed by CC1 + V1." Appendix C B2 has been updated to reflect this framing correctly: the note says "Per the CC1 literal-numerics lock (§4) the value enters from the workflow; no literal appears in `.myco` value position." The original "universal-to-literal fold at compile time" framing in the rewrite-rule audit is superseded by this V1-style equality model. Appendix C correctly reflects the absorbed/updated semantics.

### X-category bundling (pre-2026-04-22 framing)

anti_spec.md explicitly retires "X-category bundling pole L'Hopital and `identify` as one rewrite shape" in favor of the X1/X2 split. Appendix C correctly reflects the replacement split; the old single-X framing does not appear in Appendix C.

---

## Homeless

### K1 OPEN tag in Appendix C vs LOCKED status in chunk 04

**Not homeless** — Appendix C correctly marks K1 LOCKED. (Confirmed above.)

### M1, M2 stale OPEN tags

Appendix C lines 5331-5337 mark the M-group as a whole OPEN: "**M. Series / linearization.** ... OPEN (§35 envelope machinery)."

Chunk 04 O2.3 (RESOLVED 2026-04-20): "K1 (kernel compact-support truncation), M1 (first-order Taylor), M2 (high-order drop)... move to Bucket 1 (fully committed)." The O2.3 resolution note in chunk 04 Bucket 3 sub-list is also explicit: "O2.3 resolved (ship): K1... M1 (first-order Taylor), M2 (high-order term drop)..."

Appendix C does not yet reflect the M1/M2 LOCKED status. The pending reference "§35 envelope machinery" is no longer the blocking item; the `approximate` block surface (shipped in v2.1 per O2.3) provides the authorization path.

**Recommend:** Change M-group header from `OPEN (§35 envelope machinery)` to `M1 LOCKED (O2.3 resolved 2026-04-20); M2 LOCKED (O2.3 resolved 2026-04-20)`. The authorization route is the `approximate` block (`tolerance_class: small_arg_linearization`).

### K3 OPEN tag vs v2.2+ deferral in chunk 04

Appendix C K3 (line 5317-5319): "OPEN (chunk 03; speculative — kernels report does not enumerate, but §28 machinery must accommodate)."

Chunk 04 Bucket 4 explicitly defers K3 to v2.2+: "K3 (low-rank kernel SVD / Nyström / random Fourier features) — deferred to v2.2+. Speculative; machinery exists but concrete rewrites not urgent."

Appendix C's OPEN tag implies K3 is in-scope-for-v2.1-but-pending, whereas chunk 04 has definitively deferred it to post-v2.1. These are different statuses.

**Recommend:** Change K3 from `OPEN (chunk 03; speculative...)` to `DEFERRED (v2.2+; K3 deferred in chunk 04 O2.2 Bucket 4 — speculative, low-rank kernel rewrites not urgent for v2.1)`.

### CC3 → §20 mapping in CC-summary paragraph

Appendix C line 5525: "CC3 per-residual training emission (§20; open as O4.3)."

§20 is "SCC Decomposition and Component Classification" — it covers how SCCs are classified and lowered, including the training SCC class. The CC3/O4.3 tension (overconstrained relations must survive extraction with original names) is also tracked in §17.6 (spec_new line 3313-3315) and §35 O4.3. spec_new §35 line 4875 has "§20 rewrite group O1" in the O4.3 context.

The §20 reference is arguably imprecise: the per-residual-name-preservation issue primarily concerns §17.6 (extraction policy) and §25 (training emission), not §20 itself. §20 is about SCC classification; §25 is about training emission. However, since §35's O4.3 entry itself says "§20 rewrite group O1", the reference in Appendix C is derived from there and not a standalone error in Appendix C.

This is a marginal imprecision the whole spec shares, not an Appendix C-specific problem. Noting it for completeness but not flagging as a conflict.

### Q-group conservatism gap

Appendix C marks Q1-Q2 OPEN (§35, stochastic rewriting semantics). Chunk 04 Bucket 1 lists "Q1-Q2 — probabilistic truncation/marginalization (unblocked by CC4)." The CC4 resolution in chunk 04 moves Q to Bucket 1.

However, Appendix C's OPEN tag may reflect a legitimate unresolved downstream: "stochastic rewriting semantics" in §35 (particularly phase 2 Q3/Q4 residual↔e-graph relationship) could still be a dependency before Q1-Q2 can fire correctly. The corpus does not explicitly confirm Q1-Q2 are fully locked independent of those questions. This is ambiguous rather than clearly wrong, but worth flagging.

**Recommend:** Verify with Riley whether Q1-Q2 are actually fully gated on §35 "stochastic rewriting semantics" or whether the CC4 Bucket-1 promotion supersedes that gate. If CC4 supersedes it, update Q-group to LOCKED.

### Y6 `C(N,M)` — combinatorial threshold still pending

Appendix C line 5462-5465: "OPEN (combinatorial-blowup warning threshold pending; §35)."

spec_new §8.7 confirms Y6 warns on combinatorial blowup threshold but does not enumerate the threshold. §35 does not explicitly list this as a tracked open — it is mentioned in the context of closure policies but not itemized. The reference to "§35" in the Y6 OPEN tag is implicit; §35 does not have a named Y6 item.

This is a minor cross-reference inaccuracy: §8.7 (not §35) is where the Y6 blowup-threshold open is currently described.

**Recommend:** Change Y6 pending cite from `(§35)` to `(§8.7, threshold not yet locked)` to reflect where the open actually lives.

---

## Conflicts

### Conflict 1 — M-group LOCKED vs OPEN (substantive mismatch)

**Source:** Appendix C lines 5331-5337, M-group header:
> "**M. Series / linearization.** First-order expansions and asymptotic truncation. OPEN (§35 envelope machinery)."

**Corpus authority:** `04_egraph_foundation_in_progress.md`, O2.3 resolution (line 818-820):
> "**O2.3 resolved (ship):** K1 (kernel compact-support truncation), M1 (first-order Taylor), M2 (high-order drop), Z8 (delta method), Z9 (Fenton-Wilkinson) move to Bucket 1 (fully committed)."

Confirmed also at chunk 04 sub-questions (line 836-837):
> "~~O2.3 ship call.~~ **RESOLVED (2026-04-20): ship in v2.1.** Unblocked K1, M1, M2, Z8, Z9."

**Conflict:** Appendix C shows both M1 and M2 as OPEN pending §35 envelope machinery, but chunk 04 (the authoritative source for rewrite-rule partition decisions) has explicitly moved both to Bucket 1 — fully committed, no blocking dependencies — as of 2026-04-20. The `approximate` block surface (committed per O2.3) provides the tolerance authorization that makes M1/M2 fireable.

**Recommend:** Update Appendix C M-group header to reflect the O2.3 resolution. Change from `OPEN (§35 envelope machinery)` to `LOCKED (O2.3 resolved 2026-04-20)`. Individual rule tags for M1 and M2 should be updated from bare text to `LOCKED`.

---

### Conflict 2 — K3 OPEN vs deferred to v2.2+

**Source:** Appendix C line 5317-5319:
> "- K3. Low-rank `K → U·Vᵀ` (truncated SVD, Nyström, random Fourier features). OPEN (chunk 03; speculative — kernels report does not enumerate, but §28 machinery must accommodate)."

**Corpus authority:** `04_egraph_foundation_in_progress.md`, Bucket 4 (line 827-829):
> "- **K3** (low-rank kernel SVD / Nyström / random Fourier features) — **deferred to v2.2+**. Speculative; machinery exists but concrete rewrites not urgent."

**Conflict:** "OPEN" in Appendix C implies in-scope-for-v2.1-but-unresolved, which is the language used for K2, L2, M1-M2, N1, O1, P1, Q1-Q2, W1, Y6 — all design items this release's design envelope is supposed to close. "Deferred to v2.2+" in chunk 04 means definitively out of scope for v2.1. K3 should not appear in the OPEN bucket alongside items that are in-scope; it belongs in a deferred category.

The summary table (line 5514) lists K3 in the Fuzzy-tolerance bidi count ("~7 (K1-3, M1-2, N1, Q1-2)"), which means K3 is being counted as a v2.1 rule. If K3 is deferred to v2.2+, it should not appear in the v2.1 rule count.

**Recommend:** Change K3 tag from `OPEN (chunk 03; speculative...)` to `DEFERRED (v2.2+)` and remove K3 from the Fuzzy-tolerance bidi count in the summary table (reducing that count from ~7 to ~6, and from ~10 total to ~9).

---

### Conflict 3 — Summary table double-classification of J1

**Source:** Appendix C summary table line 5511:
> `| Strict | ~24 (A1-10, B1-2, C1-4, D1-3, E1-2, F1, G1-3, H1-2, I1) | ~5 (D4-5, X1, X2, J1 forbidden) | ~29 |`

And line 5517:
> `| Forbidden | 1 (J1 temporal) | — | 1 |`

**Conflict:** J1 appears in two rows simultaneously — the Strict row's uni column and the Forbidden row's bidi column. If J1 is "Forbidden" (the faithfulness class assigned in line 5299: "Temporal invariant (forbidden merge, not a rewrite)"), it should appear only in the Forbidden row. Including it in the Strict uni column inflates the Strict count (the ~24 + ~5 bidi/uni do not list J1 explicitly in the bidi cell, but the ~5 uni does include it). The grand total "~63" then overcounts J1 by 1.

This is a double-counting error: J1 is counted once in Strict-uni (~5 includes D4-5, X1, X2, J1 forbidden = 4 named items for "~5") and once in Forbidden (1). The faithfulness class for J1 should be Forbidden only, not Strict.

**Recommend:** Remove J1 from the Strict uni column. Amend the Strict uni entry to read `~4 (D4-5, X1, X2)` and the Strict total to `~28`. The Forbidden row stays as is (1, J1). This reduces the grand total from ~63 to ~62 (plus the K3 correction above would make it ~61).

---

### Conflict 4 — Summary table M2 double-listed in two cells

**Source:** Appendix C summary table line 5514:
> `| Fuzzy-tolerance | ~7 (K1-3, M1-2, N1, Q1-2) | ~3 (O1, P1, M2) | ~10 |`

M2 appears in both the bidi column (`M1-2` meaning M1 and M2) and the uni column (M2 explicitly). This double-counts M2.

Checking the actual M-group: M1 ("First-order Taylor... around declared operating point") is a bidi-applicable rewrite (the approximation can be applied both to expand and to recognize). M2 ("Drop higher-order terms when envelope bounds their contribution below tolerance") is inherently uni (dropping is one-direction). The table intends to list M2 only in the uni column but has accidentally included it in the bidi cell via "M1-2."

**Recommend:** Correct the Fuzzy-tolerance bidi cell from `K1-3, M1-2, N1, Q1-2` to `K1-2, M1, N1, Q1-2` (removing K3 per Conflict 2 and separating M1 from M2). The uni cell already correctly lists M2 and O1 and P1. After the K3 removal and M2 de-duplication, the bidi count drops from ~7 to ~5 (K1-2, M1, N1, Q1-Q2 = 6 items but Q1-Q2 are each 1 rule so 6 named rules for "~6"; adjust to ~5 removing K3). Total remains approximately ~9 (6 bidi + 3 uni) after K3 removal.

---

### Conflict 5 — CC3 disposition: "absorbed into normative spec text" vs still OPEN

**Source:** Appendix C lines 5523-5526:
> "CC1-5 are absorbed into normative spec text: ... CC3 per-residual training emission (§20; open as O4.3)..."

The phrase "absorbed into normative spec text" followed immediately by "open as O4.3" is contradictory in tone. CC3 is not absorbed in the sense CC1, CC2, and CC4 are absorbed (i.e., the design is settled and the mechanism is in the spec). CC3 is tracked as an unresolved design tension. Saying it is "absorbed into normative spec text" suggests it is settled, but the parenthetical "(open as O4.3)" correctly says it is not.

Chunk 04's own summary table (line 1335):
> `| CC3 | Per-residual exposure for training | **TRACKED** as O4.3 (no change) |`

The corpus frames CC3 as TRACKED / still open, not absorbed.

**Recommend:** Separate CC3 from the "absorbed" claim. The CC-summary paragraph should distinguish: "CC1, CC2, CC4, CC5 are absorbed into normative spec text [with descriptions]; CC3 is tracked as open item O4.3 in §35 — per-residual training emission tension with strict algebraic collapse has not yet resolved."

---

### Conflict 6 — Fuzzy-tolerance bidi count inconsistency (~7 vs actual enumeration)

**Source:** Appendix C summary table line 5514 bidi cell: `~7 (K1-3, M1-2, N1, Q1-2)`.

Counting the items named: K1, K2, K3 (3) + M1, M2 (2) + N1 (1) + Q1, Q2 (2) = 10 items, not 7. Even at face value the claimed ~7 conflicts with 10 named items. If the intention is that these rules are individually bidi-applicable, the count should be ~10 bidi, but the total column says ~10 (bidi + uni combined). This means the bidi and uni columns cannot both be accurate simultaneously.

Looking at the text:
- K1 (compact-support truncation): uni (K → 0 is one-direction)
- K2 (separable decomposition): "bidi when exact, uni when approximate" per K2 text
- K3 (low-rank): speculative, mostly uni
- M1 (first-order Taylor): bidi-applicable (expand/recognize)
- M2 (drop higher-order): uni
- N1 (quadrature): uni (symbolic → quadrature is one-direction)
- Q1, Q2: direction depends on marginalization context

The "~7 bidi" count for this row is internally inconsistent with (a) the 10 named items, (b) several of those items being clearly uni, and (c) the total column being ~10 (implying bidi + uni ≈ 10, but the table shows bidi: ~7 and uni: ~3, which would sum to 10 only if each is counted once, yet M2 appears in both).

This is compounded by the M2 double-count (Conflict 4 above) and K3 deferral (Conflict 2 above).

**Recommend:** After resolving Conflicts 2 and 4, re-tally the Fuzzy-tolerance row from scratch against the actual rule list. Corrected enumeration would be approximately: bidi-applicable: K2, M1, Q1-Q2 (~4 items); uni: K1, M2, N1, O1, P1 (~5 items); total ~9 (with K3 deferred to v2.2+).

---

*End of audit. Six conflicts identified, two homeless items requiring spec updates, one Q-group ambiguity requiring author clarification.*
