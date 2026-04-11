# Myco V2 Charter

This file is the highest-level statement of what `v2` is trying to prove.

It should be read before the more detailed `v2` notes:

- [v2_do_this_first.md](./v2_do_this_first.md)
- [ideas.md](./ideas.md)
- [open_questions.md](./open_questions.md)

## V2 Goal

`v2` should prove that Myco can support one real plant-relevant model family on top of the workflow-neutral structural compiler architecture established in `v1`.

## What V2 Is

`v2` is:

- one real scientific workflow
- one level up in realism from TinyTree
- only the compiler/runtime features that workflow actually forces into existence

## What V2 Is Not

`v2` is not:

- a general scientific platform
- a complete package ecosystem
- a finished shared Myco/OzzyDB type system
- a full symbolic theorem-proving or CAS layer
- a broad continuous-time modeling framework

Those are long-term directions, not the next proof target.

## Most Important Design Rule

The most important `v2` design rule is:

> the world model describes structure; the binding/config layer describes workflow

That boundary should be strengthened, not weakened, during `v2`.

## Likely V2 Feature Additions

Depending on the chosen model family, `v2` will likely need some subset of:

- clearer parameter semantics
- a minimal function registry
- a narrow local-solve mechanism for same-step implicit subsystems
- richer observation operators
- better indexing and binding ergonomics for real data

## Recommended Default Model Family

Unless literature and data access argue otherwise, the default recommended first `v2` target is:

- hydraulic + stomatal control

because it is:

- close to TinyTree
- likely valuable quickly
- likely to force the smallest meaningful compiler expansion

## Explicit Deferrals

The following belong after the first `v2` proof unless the chosen model family makes them unavoidable:

- semantic schemas/components as production features
- package/registry ecosystem
- uncertainty-rich core modeling
- broad demand-driven planning refactors
- large backend expansion beyond JAX-first support
- long-range Myco/OzzyDB convergence work
- theorem-prover / CAS expansion

## Success Criteria

`v2` is successful if:

- one real plant-model family is implemented cleanly
- the same world model can still support multiple workflow bindings
- the added language/runtime features feel forced by the science rather than speculative
- the result is strong enough to serve as the first paper-relevant workflow after TinyTree
