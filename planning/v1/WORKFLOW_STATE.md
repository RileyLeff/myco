# Workflow State

Current phase: post-`v1.3` checkpoint; pre-`v2` hardening is complete and the next step is the focused `v2` plan captured in `planning/v2/`

## Completed

- vertical slice compiler implemented
- Python package and examples implemented
- TinyTree end-to-end training demo implemented and verified
- introspection/explainability added
- structured constraints and soft penalties added
- dimensions and minimal units added
- compile-mode validation added
- `egg` feasibility spike added and tested
- `egg` promoted into the primary equality substrate
- planner rewired to consume the shared equality core
- full Rust, Python, and training validation rerun successfully
- slot binding kinds now have real emitted runtime semantics
- learned initial state is now implemented in emitted artifacts
- consistency policy is now explicit in the compile spec and respected by both backends
- compile modes now affect emitted artifact shape in a visible way
- explanation output now carries rendered expressions, provenance labels, and source spans
- compiled artifacts now expose typed slot-interface and trainable metadata through the Python package
- blocked-path explanations now use extracted expressions and extracted costs rather than stale candidate defaults
- equation provenance now flows through stable per-equation IDs rather than block-name lookup alone
- observation losses are now normalized by valid-point count
- the TinyTree training demo now includes a small consistency regularization term
- a `v1.3` planning note now captures the remaining pre-`v2` fixes
- equation candidates now resolve through candidate-local `egg` extraction rather than a shared output e-class
- direct `DataSeries` bindings now require dense full-horizon coverage in `v1`
- emitted Python and JAX artifacts now expose runtime validation for rollout inputs and observation payloads
- explicit runtime constraint policy now exists for mechanistic/state outputs:
  Python raises on violations and JAX surfaces `constraint_violation_loss`
- compiled artifact metadata now exposes the backend-specific constraint runtime policy
- expression parsing now supports scientific notation
- syntax diagnostics now catch unterminated and malformed constraint blocks explicitly
- emitted Python and JAX artifacts now use explicit safe-division helpers

## In Progress

- `v2` boundary migration, starting with persistence moving from source-level semantics toward inferred/compiler semantics
- choosing the first concrete `v2` plant-model family and writing the corresponding implementation plan

## Open Risks

- the remaining risks are now mostly `v2` scope choices, not pre-`v2` correctness gaps

## Next Action

1. use `planning/v2/charter.md` and `planning/v2/v2_do_this_first.md` as the near-term guide for the next compiler milestone
2. finish the `v2_do_this_first` migration by expanding binding-time workflow roles and reducing dependence on source-level quantity-role keywords
3. keep avoiding broad frontend-language expansion outside one concrete plant-model target
4. treat `planning/v_long_term/` as north-star material, not immediate implementation scope
