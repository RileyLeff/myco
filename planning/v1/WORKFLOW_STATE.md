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
- compile modes now affect emitted artifact shape in a visible way
- explanation output now carries rendered expressions, provenance labels, and source spans
- compiled artifacts now expose typed slot-interface and trainable metadata through the Python package
- blocked-path explanations now use extracted expressions and extracted costs rather than stale candidate defaults
- equation provenance now flows through stable per-equation IDs rather than block-name lookup alone

## In Progress

- remaining `v1.2` parity work after the compile/emitter honesty pass
- decide whether current artifact metadata is already sufficient for honest `v1`

## Open Risks

- the remaining slot-metadata question is now sufficiency, not availability

## Next Action

1. land the next `v1.2` slice:
   make a concrete sufficiency decision on artifact metadata
2. rerun external review once the remaining `v1.2` gaps are narrower
3. keep avoiding frontend-language expansion until the parity cleanup is done
