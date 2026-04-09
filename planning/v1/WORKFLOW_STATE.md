# Workflow State

Current phase: `v1.3` pre-`v2` hardening, focused on correctness and runtime-contract tightening before widening model scope

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

## In Progress

- deciding and implementing a narrow runtime policy for mechanistic/state-output constraint handling
- tracking the small parser/runtime cleanup items needed before the first real model family

## Open Risks

- mechanistic outputs and temporal state updates still do not have an explicit bound-violation runtime policy
- parser/runtime support is still narrow for the first nontrivial model family (for example scientific notation)

## Next Action

1. land the next `v1.3` slice:
   explicit runtime policy for mechanistic/state-output constraint violations
2. decide which small parser/runtime fixes should land before the first real model family
3. keep avoiding broad frontend-language expansion until the pre-`v2` fixes are done
