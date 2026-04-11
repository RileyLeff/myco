# Myco V2 Open Questions

This note is about unresolved questions for the next real compiler milestone.

It is intentionally narrower than the long-term vision material. For higher-altitude questions, see:

- [../v_long_term/README.md](../v_long_term/README.md)
- [v2_do_this_first.md](./v2_do_this_first.md)

## 1. What Is The First Real Plant Model Family?

This is still the most important question.

Reasonable candidates:

- hydraulic + stomatal control
- water + carbon + allocation
- Farquhar-lite + stomatal coupling

Questions:

- which family creates real scientific value fastest?
- which family forces the smallest meaningful compiler expansion?
- which family best matches near-term data access?

## 2. How Far Do We Push The World/Workflow Separation In V2?

The key design rule is clear, but its practical implementation still needs decisions.

Questions:

- how much of the current source-level role vocabulary should be deprecated in `v2`?
- which workflow annotations should live only in config?
- which structural facts should be inferred from temporal relations vs declared explicitly at binding time?

## 3. What Should Count As A Parameter?

`v2` likely needs clearer rollout-stable quantity semantics.

Questions:

- should `parameter` become a first-class concept in the language or only in binding/runtime?
- how should fixed vs learned vs assumed parameter status be expressed?
- how should parameters differ from constants in practice, if at all, for the first real model?

## 4. What Is The Smallest Useful Function Registry?

The first real model may need named scientific functions.

Questions:

- which functions truly need registry support for the chosen model family?
- what metadata is minimally required for each registered function?
- should the first registry be entirely local/project-level, with package-backed registries deferred?

## 5. Does The First Real Model Require Local Solve Blocks?

This should be decided empirically from the target model family.

Questions:

- can the first real model be represented as an explicit step plan?
- if not, what is the narrowest useful same-step solve abstraction?
- how much solver detail should the compiler know vs defer to the backend?

## 6. What Is The Smallest Useful Observation-Operator Layer?

The TinyTree observation story is intentionally simple.

Questions:

- what non-identity observation operators are required by the chosen model family?
- how should loss configuration stay simple while becoming more realistic?
- how much observation metadata belongs in the core language vs binding vs registry?

## 7. What Is The Right V2 Indexing Story?

Real data will likely force clearer schedule/index semantics.

Questions:

- are explicit timestep indices enough for `v2`?
- when do timestamp-based alignments become necessary?
- which data adapters should be supported first: plain dicts, pandas, xarray, polars?

## 8. How Much Binding Ergonomics Is Worth Adding In V2?

The current API is explicit and semantically honest, but verbose.

Questions:

- what small convenience layer would materially help without making semantics fuzzy?
- should dataframe/xarray adapters be in-scope for the first real model?
- how much should the package optimize for interactive human use vs agent-assisted use?

## 9. How Far Should Backend Agnosticism Be Protected In V2?

The current implementation is backend-agnostic in core architecture, but JAX-first in practice.

Questions:

- what minimum backend-neutral contract should remain protected while building the first real model?
- what JAX-specific assumptions are acceptable in `v2` product work?
- which backend questions can be explicitly deferred until there is a real PyTorch or Rust-native use case?

## 10. What Should Be Explicitly Deferred?

One of the strengths of `v1` was scope control.

Questions:

- which good ideas should be explicitly declared out of scope for the first `v2` proof?
- what would signal that `v2` is drifting into platform-building rather than proving one real model family?

## Short Version

The main `v2` questions are:

- what real plant model family to target
- how to finish the world/workflow boundary shift
- how to represent parameters cleanly
- whether the target model truly needs a function registry and/or local solve blocks
- what the minimal real-data observation/indexing ergonomics should be
- how to stay JAX-first without turning the core into JAX-specific architecture
