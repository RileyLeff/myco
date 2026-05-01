# Four-Reviewer Synthesis — 2026-04-23

Four independent reviews on `spec_new.md`, `anti_spec.md`,
`audit/adjudication.md`, and `soul.md`, covering the ~22 REVIEW items
the adjudication surfaced for Riley's decision.

## Sources

- **Gemini** — pasted review, roughly 21-item walkthrough with
  opinionated takes per item
- **Codex** — single-model Codex run via `review` skill, output at
  `/tmp/review-wBcvo8Mp/codex_output.txt`
- **Opus** — general-purpose opus subagent run, output at
  `/tmp/review-wBcvo8Mp/claude_output.txt`
- **Pro** — pasted GPT-5.4 Pro review

Each reviewer worked independently without seeing the others' takes.

---

## Executive Summary

- **11 items have 4-way consensus** — no debate, apply as-is
- **6 items have 3-1 strong majorities** — defensible calls
- **5 items are genuinely split 2-2** — need Riley's judgment
- **~12 net-new issues** caught outside the 21 (concrete contradictions,
  architectural gaps, framing fixes)

The two items every reviewer flagged as load-bearing are `observe` (App
A H8) and `bind_controller` arity (§24 C1). Pro's one-line summary
captures the shared theme across all four: *"Myco should be the language
of model truth, plus a compiler that is brave and explicit."*

---

## Consensus items (4-0 — lock as stated)

For each: Riley's call line left blank. Default is to accept the
consensus unless Riley pushes back.

### 1. §20 C1 — SCC class taxonomy
**Consensus call:** §20 four-way (`static / dynamic / stochastic /
training`) is normative; §16.1 six-way (algebraic / fixed-point /
iterative-solve / stepper / ...) demotes to solver-strategy
sub-dispatch inside the "dynamic" class. Rewrite §16.1 header to make
this subordination explicit.
**Riley's call:**

### 2. §22 H10 — Visualization in v2.1
**Consensus call:** Don't commit to renderers in v2.1. Commit to
textual explain output + one machine-readable IR; any renderer
(Mermaid / D2 / Graphviz) is optional downstream tooling. Keep §22 /
Appendix B.5 as stubs.
**Riley's call:**

### 3. §25 H10 — Long-rollout gradient regime
**Consensus call:** Land as `§24` / `§31` run-config knob
(`gradient_regime: full-BPTT | truncated-BPTT(k) | checkpointed`).
Not model semantics. One-sentence spec commitment is enough.
**Riley's call:**

### 4. §25 S2 — Two-phase solver non-convergence regime
**Consensus call:** Retire to anti-spec. Backend-optimizer policy, not
language semantics. Surface via backend capability advertising if a
backend wants to offer it.
**Riley's call:**

### 5. §27 C3 — Conjugate catalog
**Consensus call:** Ship Gamma-Gamma now; gate NormalInverseGamma on
the rewrite-pattern-language check (pointer to §35). Don't claim
"closed catalog" while carrying a maybe-shipping joint conjugate.
**Riley's call:**

### 6. §28 H1 — Compact support / characteristic length
**Consensus call:** Capability contract on the kernel function
(`CompactSupport(radius)`). Not a workflow annotation, not an output
refinement. Radius takes `val` generic for consistency.
**Riley's call:**

### 7. §31 C1 + §32 C2 — AD ownership fork
**Consensus call:** Hybrid. Compiler owns symbolic / algorithmic
differentiation (§14.4 Level I path, rewrite-based Jacobian
construction, §C-group). Backend owns runtime AD on emitted kernels
and opaque callables. Spec prose must name the boundary explicitly
("Myco owns symbolic and algorithmic modes; runtime mode delegates to
backend AD advertised via §31.1") — without naming it, "hybrid" is
unfalsifiable.
**Riley's call:**

### 8. §31 C3 — Fallback default mode
**Consensus call:** Lock `error` as default. Silent host fallback is
the classical source of "it works on my machine at 1000x slower"
bugs. Permissive fallback becomes explicit opt-in in run-config.
**Riley's call:**

### 9. §34 S1 — Kernels-as-functions anti-spec staleness
**Consensus call:** Drop-in edit. Replace "ordinary `fn` accepting two
point arguments" with "ordinary parameterized relation accepting two
point arguments." Chunk 08's user-`fn` ban makes the current text
wrong. Purely editorial.
**Riley's call:**

### 10. Appendix A C2 — `identify` classification
**Consensus call:** Move `identify` to Declaration keywords. It
installs a Layer-3 site record at module or geometry scope; listing it
alongside body-form operators invites authors to try it inside relation
bodies.
**Riley's call:**

### 11. Appendix C H1 — Q-group lock status
**Consensus call:** Q1 and Q2 are LOCKED via chunk 04 CC4. Update the
Appendix C tag. The §35 stochastic-rewriting phase-2 text either
covers something distinct or is stale; either way it should not gate
Q1-Q2.
**Riley's call:**

---

## Strong majority items (3-1 — defensible)

### 12. §26 C1 — Precision downcast authorizing surface

**Votes:** Gemini, Codex, Pro all say **require `approximate { ...
tolerance_class: precision_downcast }` for both scalar and tensor**.
Opus dissents: bare `convert` with auto tolerance envelope, rejecting
scalar/tensor asymmetry.

**Majority argument:** Matches Myco's anti-magic philosophy. `convert`
should stay the home of lossless transformations; anything that loses
bits gets named authorization.

**Dissent:** GPU work forces downcast unavoidably; `approximate`
ceremony on every `Float64 → Float32` may ossify the boundary in prose.

**Recommended call:** Require `approximate`. Matches §3.8 precedent.
**Riley's call:**

### 13. §30 C2 — `inverse` vs `inv` naming

**Votes:** Gemini, Codex, Opus say **`inv`**. Pro says **`inverse`**
(Myco surface trends explicit-name for workflow verbs).

**Note from verification:** Three names currently coexist in the spec:
- `invert` — scalar/function inverse (Appendix A line 5182 stdlib fn)
- `inverse(A)` — matrix inverse (§30, line 4609)
- `inv=log` — parameter name in `Invertible<inv=log>` contracts

Whatever we pick, we should also reconcile `invert` (scalar) with the
matrix name. Three options: (i) `inv` for matrix + keep `invert` for
scalar, (ii) `inverse` for matrix + `invert` for scalar, (iii)
`invert` everywhere.

**Recommended call:** Pick a direction on this broader consistency
question, not just the local matrix-name question.
**Riley's call:**

### 14. Appendix A H8 — `observe` dual status

**Votes:** Gemini, Codex, Pro all say **workflow-only** (Python-side).
Data is a property of the experiment, not the world; §24 already
lists `observe` among the eight workflow verbs; §13.8 call-form
surface is the leak. Opus dissents: keep as `.myco` surface syntax.

**Majority argument (Pro, emphatically):** "This is one of the most
important boundary decisions in the whole set." Epistemic / aleatoric
split is soul.md principle 1 — don't leak data references into
world-claim prose.

**Dissent (Opus):** `observe(data, x ~ D)` reads naturally as source
syntax; the payload threads through workflow.

**Recommended call:** Workflow-only. Rewrite §13.8 to present
`observe` as mechanism / pseudonotation, not blessed source syntax.
**Riley's call:**

### 15. §20 H7 — Knowledge-envelope `realization` field

**Votes:** Codex and Pro say **keep as inspection metadata only** (not
source syntax; valuable for `explain`, "generated code is the product"
story). Gemini and Opus say **retire** (Layer-3 carries draw
provenance, field is redundant).

Actually **2-2 SPLIT**, not 3-1. Moving to the split section.

### 16. §40 H1 — Agent-mediated import/adapt vision

**Votes:** Codex, Opus, Pro all say **keep §40 modest / defer from
v2.1**. Gemini says **defer to Part VII**. Functionally the same call.

**Consensus call:** Don't add to §40 for v2.1. Revisit once package
resolver (chunk 10) and doc-generation format (§39) close.
**Riley's call:**

### 17. §20 S2 — Linear/polynomial/general-nonlinear labels

**Votes:** Codex, Opus, Pro all say **retire as user-facing
taxonomy**, survive only in backend capability advertising (Opus) or
`explain` output / backend internals (Codex, Pro). Gemini didn't
address directly.

**Consensus call:** Retire from user-facing surface. Keep in
§31-backend-capability vocabulary if useful.
**Riley's call:**

---

## Split items (2-2 — Riley's judgment needed)

### 18. §24 C1 — `bind_controller` arity

**Votes:**
- **Gemini, Pro → 3-arg** `(path, fn, input_contract)`, infer output
  from Python callable's declared return type. Avoid two sources of
  truth (DRY on typed callables).
- **Codex, Opus → 4-arg** `(path, fn, input_contract,
  output_contract)`. Inference weakens reproducibility and cross-study
  portability (Codex); input and output shapes often differ
  (observables vs decision variable) and a single contract is awkward
  (Opus).

**Pro's judgment (worth noting):** "A separate output-contract
argument creates two sources of truth and invites drift." Anti-spec's
`[*]` wildcard retirement was about "explicit I/O spec" — Codex/Opus
read that as arguing FOR 4-arg; Pro reads it as satisfied by contract
inference from Python.

**Riley's call:**

### 19. §22 H3 — `with_assumption` / hypothetical plan re-evaluation

**Votes:**
- **Gemini** → retire to anti-spec (speculative v1.0 idea not
  attached to 3-layer e-graph / 8-verb taxonomy)
- **Opus** → drop from v2.1, revisit post-v2.1
- **Codex** → ship as tooling, rename to `with_binding_override`
- **Pro** → keep capability but reframe as reproducible
  rebind/recompile helper, not plan-patching

Rough grouping: 2 cut/defer (Gemini, Opus), 2 keep-with-reframe
(Codex, Pro).

**Riley's call:**

### 20. §21 C2 — Per-collection bind-static vs module-wide dynamic

**Votes:**
- **Gemini, Opus → per-collection classification** only (modules carry
  multiple collections with different churn; module-wide flag forces
  unnatural module splits)
- **Codex, Pro → both axes, different roles**: module-level decides
  whether runtime loop exists (semantic); per-collection bind-static
  is a lowering optimization the compiler exploits (reports in
  `explain`)

The "both" camp is not the same as "per-collection" — they're saying
per-collection is a compiler optimization hidden from user semantics,
while Gemini/Opus treat per-collection as the primary semantic
statement.

**Riley's call:**

### 21. §33 C1 + §34 C2 — B2/B4 chunk assignment

**Votes:**
- **Gemini, Codex → create placeholder chunk** (Gemini: "Chunk 13:
  Joint Syntax"). Blockers need owners even if work hasn't started.
- **Opus, Pro → accept TBD / unassigned**. Don't invent chunk numbers
  early; fix stale labels; pretending unresolved machinery has a
  settled home distorts the roadmap.

**Riley's call:**

### 22. §20 H7 — Knowledge-envelope `realization` field (moved from above)

**Votes:**
- **Gemini, Opus → retire** (Layer-3 handles draw provenance;
  envelope kind is redundant)
- **Codex, Pro → keep as inspection metadata only** (not source
  syntax; valuable for `explain` and "generated code is the product"
  story)

**Riley's call:**

---

## Items only some reviewers addressed

### §35 C2 — CC5 block placement

Gemini: move to Appendix C or §17. Opus: specifically **§17** (because
that's where the authorization-source taxonomy lives; X1/X2 split is
about which Layer-1 merges are authorized by Layer-3 site records).
Codex and Pro didn't address.

**Opus's specificity is better-reasoned than Gemini's "or."**

**Riley's call:**

### §30 C1 — Matrix naming / constructor form

All four agree `Matrix<U, m, n>` is the canonical base constructor.
Split on alias form:
- Gemini: ship `PosDef<U, n>` as shorthand
- Codex: "full names in prose, short stdlib aliases if you want"
- Opus: full `PositiveDefinite<U, n>` alias; **reject `PosDef`
  abbreviation explicitly** (collides with domain jargon SPD/PD/PosDef)
- Pro: kill `Matrix<_, PositiveDefinite>` variant; aliases only on top
  if at all

**Consensus:** `Matrix<U, m, n>` is canonical; `PositiveDefinite` is
the full structural-property name; reject positional-generic variants
like `Matrix<_, PositiveDefinite>`. Alias question is mild — prefer
full spelling (Opus, Pro) over abbreviation (Gemini).
**Riley's call:**

---

## Net-new issues caught outside the 21 items

### A. Codex — three concrete textual contradictions

**A1. `assume_initial`** (verify first):
- §9.3 lists it as one of four initialization mechanisms and says
  semantics are in §24
- §24's eight verbs do NOT include `assume_initial`

**A2. `assume_prior`** (verify first):
- §13.1 references it as workflow-side prior binding
- §24.4 lists it as a future candidate verb beyond the eight

**A3. Artifact identity ambiguity**:
- soul.md: "generated source code is the product"
- §0.1 / §22: the compiled **plan** is the unit of execution
- §31: backend trait executes plans against runtime capabilities

Three readings of what Myco fundamentally is: source generator,
serializable plan IR, or both with one normative. Needs one sentence.

### B. Codex — structural gaps

**B1. Appendix A incomplete** — missing `when`, `initial`, `temporal`,
`as`, possibly `on`, `field`.

**B2. CC1 exception surface still leaks** — seam declarations like
`identify phi = 0 <-> phi = 2 * pi`, geometry/boundary spellings may
be accidental carve-outs.

**B3. Projection-free compiler boundary** — compiler auto-does
marginalization, truncation, inverse elimination, envelope narrowing.
Users will ask why auto-marginalization is fine but auto-projection is
not. Needs explicit "semantic simplification compiler may auto-do" vs
"model-shaping choice modeler must name" distinction.

### C. Opus — architectural risks

**C1. Layer-2 envelope merge-combination rules unspecified.** When two
e-classes merge in Layer 1, Layer-2 envelopes must combine.
Distributions → Bayesian update. Tolerances → worst-case union.
Structural → lattice meet. Costs → §19 doesn't say. Add §16.2
requirement that every Layer-2 kind specifies its merge rule.

**C2. Compilation determinism not committed.** soul.md principle 5
("reproducibility from recompilation") requires bit-identical or
bit-identical-modulo-naming output under rewrite-ordering variation.
§19 cost-vector extraction partially addresses this; tie-breaking
unspecified.

**C3. `bind_topology` e-class lifecycle at Layer-1 / Layer-3
interface.** When a vessel dies: do e-classes referencing its fields
persist (Layer-3 provenance updated), get GC'd, or something else?
§10 / §12 don't commit.

**C4. Within-run multi-backend silence.** v2.1 single-backend-per-run
vs SCC-level dispatch not formally scoped in §23 / §31.

**C5. `where x is T` narrowing + projection-free.** If refinement
triggers solver-transient violation, forcing a workflow projection
binding is user-visible and breaks the "projection-free" claim from
the user's perspective.

**C6. Tagged-handle dispatch openness.** `argmax` over `some T`
returns a tagged handle; spec doesn't commit open (runtime-discovered
tags) vs closed (compile-time tag universe). Compile-time-closed is
implementable; open-tag probably not.

**C7. `condition_of` Level III runtime mode.** Data-dependent
branching on condition number breaks static-module vs dynamic-module
separation; §21 lowering doesn't address.

### D. Pro — non-decisions flagged as already-resolved

**D1. §2 H3 (module-scope `initial` / `temporal`):** anti-spec already
retires module-scope per-type blocks in favor of in-type bodies;
§9.4 legalizes locus-scoped temporal blocks. Pro says "should just die
rather than be resolved into a misleading prohibition."

---

## Pro's compressed verdict

> Myco should be the language of model truth, plus a compiler that is
> brave and explicit. Shrink the surface, grow the compiler, keep
> experiment policy in the workflow, keep backend policy in the
> backend/run-config, and use `explain` / provenance / envelopes to
> surface complexity instead of leaking that complexity back into
> `.myco`.

> The two decisions I think matter most are `observe` and
> `bind_controller`. If you get those boundaries right, a lot of the
> rest will fall into place.

---

## Proposed walkthrough order for Riley + Claude

1. **Verify the three Codex contradictions first (A1, A2, A3).**
   These are textual bugs, not design forks — fastest wins.
2. **Accept the 11 consensus items.** Flag any Riley disagrees with;
   otherwise apply in task #119.
3. **Decide the 6 strong-majority items.** Default to majority unless
   Riley sees a reason to break with 3 independent reviewers.
4. **Work through the 5 genuine splits** one at a time:
   §24 C1, §22 H3, §21 C2, §33 C1/§34 C2, §20 H7.
5. **Triage Opus's 7 risks** (C1-C7) into spec-now / Phase-2-open /
   accept-silently.
6. **Kill non-decision D1.**

Once all calls are recorded on the **Riley's call:** lines, this file
becomes the closure record and task #119 applies the fixes.
