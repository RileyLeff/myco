# spec_new.md — Developer Notes

**Purpose.** Decisions that shape `spec_new.md`'s structure or content but
aren't spec content themselves. Consolidation-time choices, scope
clarifications, rationale we want preserved but shouldn't bleed into the
normative spec prose. Entries are dated; most-recent-on-top.

---

## 2026-04-20 — CC1 literal-numerics scope: zero literals in `.myco` value position

### The question

During the consolidation audit, reviewers flagged that mocks contain literal
numerics (`R = 8.314`, `1.67`, `0.98`, `0.1579`, etc.) inside relation bodies
and universal declarations. Chunk 04 CC1 locks a literal-numerics policy but
doesn't fully specify whether universals (`universal R: Scalar<J_mol_K> =
8.314`) or stdlib-provided physical constants count as violations.

### The decision

**CC1 is strict. No literal numerics anywhere in `.myco` value position.**
Physical constants do not live in the stdlib. They live on the Python
workflow side and inject at workflow time via the binding verbs.

### Exception positions (the complete list)

Literal numerics are permitted only in:

1. **Unit definitions.** Base unit declarations and derived-unit algebra.
2. **Affine conversion bodies.** `1 hour = 60 minutes` and equivalents.
3. **Shape tuples, indices, arity.** `Tensor<U, (3, 4)>`, `pathway[0]`,
   `N: val = 3` at the generic-parameter definition site. Structural
   positions, not value positions.

Symbolic constants (`π`, `e`) are named identifiers in the numeric stdlib —
they don't require an exception slot because they're not literals.

### What this means in practice

- `universal R: Scalar<J_mol_K>` — declaration only. No value attached.
  The workflow provides the value.
- Empirical fit parameters (Sperry's cavitation `a`/`b`, Potkay's
  calibration coefficients) are typed fields bound via `assume_constant`
  or `learn_constant` at workflow time.
- Physical constants (`R`, `Avogadro`, `Planck`, `c`) are provided by the
  Python workflow, same machinery as empirical parameters. The compiler
  does not distinguish the two.

### Rejected alternative

A fourth CC1 exception — "named stdlib declarations with documented
provenance" — was considered and rejected. Reasons:

- Splits `.myco` files into two trust postures (user files vs stdlib
  files), each with different literal rules. Needless complexity.
- Requires the stdlib to own a physical-constants module and keep it in
  sync with users' needs. Workflow-side injection avoids this entirely.
- "Physical constant" is not a distinction the compiler can make. The
  value `8.314` looks the same as an empirical fit coefficient. Keeping
  them on the same side of the language boundary honors that.

### Implications for consolidation

- Spec_new.md §3 (Values and Literal Policy) states the three exception
  positions cleanly, without mentioning stdlib or universals-with-values.
- Universals declare *types*, not values. Any existing mock universal
  with `= <number>` form must be rewritten as `universal X: Scalar<U>`.
- Mock rewrite pass (post-spec): both Sperry and Potkay need their
  universals stripped of values and the values moved to the Python
  workflow. Potkay also needs the slot / `[t+1]` migration flagged in
  the consolidation audit.
- CC1 wording in spec_new.md: "`.myco` permits literal numerics only in
  unit definitions, affine conversion bodies, and structural positions
  (shape tuples, indices, generic-parameter definitions). All values
  enter from the workflow."

### Open adjacent item

- The compiler diagnostic surface for CC1 violations is not specified.
  Flagged in the merged audit under "Other Opens." Not blocking for
  spec_new.md text; is blocking for implementation.

---
