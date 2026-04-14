# Deferred Review Findings

GPT 5.4 Pro findings accepted as actionable but not yet implemented.
These should be folded into the next spec edit pass.

## From GPT 5.4 Pro review (2026-04-13)

1. **Named-type rules for equality and comparison** — Section 4.7 defines
   named-type rules for arithmetic but not for `=`, `<`, `<=`, `>`, `>=`.
   `CarbonPool = WaterPool` should be a compile error (same dimension,
   different semantic type). Extend named-type rules to cover relations and
   comparisons.

2. **`learn_trajectory` vs t=0 initialization** — If `learn_trajectory` owns
   a quantity across all timesteps, the spec never says whether it owns t=0
   or is mutually exclusive with `initial`/`assume_initial`/`learn_initial`.
   Natural answer: `learn_trajectory` owns all timesteps including t=0,
   making it a fourth mutually exclusive initialization mechanism.

3. **Closure policy semantic interface** — The spec describes closure policies
   at the config level but never defines the semantic object a policy receives:
   what counts as a candidate path, how candidates are enumerated from an
   overconstrained block, how >2 candidates and multi-unknown blocks are
   surfaced.

4. **Transparent controller ABI for wildcard/metadata** — Learned wildcard
   slots have a precise ABI (element-local/global partition, vmap, metadata).
   Transparent controllers just "get path-rebased." Need to define how
   wildcard partitioning and metadata work for transparent controllers.

5. **Shared-controller portability in reviewer notes** — Notes suggest explicit
   `inputs` as a workaround for different sites, but section 7.1 requires
   identical model instantiation. These conflict. Fix the notes.

6. ~~**Contract wiring uniqueness**~~ — RESOLVED. With the no-bare-literals
   rule and acausal semantics, duplicate wiring is just normal overconstraint.
   No special rule needed.
