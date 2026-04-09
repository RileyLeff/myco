# Workflow State

Current phase: `v1.2` parity cleanup after the `egg` rollout, with compile-surface/runtime semantics now being tightened

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

## In Progress

- remaining `v1.2` parity work after the compile/emitter honesty pass
- deciding whether mode-aware emission or deeper provenance/introspection should land next

## Open Risks

- compile modes still mostly change validation/metadata rather than emitted behavior
- provenance and diagnostics still need another pass to fully explain chosen extracted paths
- planner cost reporting in explanations still needs cleanup so reported costs match extracted costs

## Next Action

1. land the next `v1.2` slice:
   either mode-aware emitted artifacts or deeper provenance/introspection
2. rerun external review once the remaining `v1.2` gaps are narrower
3. keep avoiding frontend-language expansion until the parity cleanup is done
