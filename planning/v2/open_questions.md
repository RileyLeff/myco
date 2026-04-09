# Myco V2 Open Questions

This note is a companion to [ideas.md](./ideas.md).

The ideas note captures direction.

This file captures the design questions that still feel unresolved or intentionally open.

It is not a checklist. The point is to preserve uncertainty honestly so that `v2` can be shaped by real scientific targets rather than by momentum alone.

## 1. What Is The First Real Plant Model Family?

This is still the most important open question.

Reasonable candidates include:

- hydraulic + stomatal control
- water + carbon + allocation
- Farquhar-lite + stomatal coupling

Questions:

- Which family creates the most scientific value soonest?
- Which family forces the smallest but most meaningful `v2` compiler expansion?
- Which family fits best with currently accessible data?
- Which family is ambitious enough to matter but narrow enough to finish?

## 2. What Should Count As A Parameter?

`v1` can treat many fixed values as constants, but `v2` likely wants a clearer parameter role.

Open questions:

- Should parameters be a distinct quantity kind?
- Should they be grouped separately from external forcing and state?
- How much trainability belongs in the core versus in backend-specific wrappers?
- Should parameter bundles be attachable to semantic schemas like `Leaf` or `Tree`?

## 3. What Is The Right Layer For Semantic Schemas?

There is a likely future distinction between:

- physical/base quantity types
- semantic component schemas
- functions and relations

Open questions:

- Should things like `Leaf`, `Tree`, or `Sperry_2017_Tree` be called types, schemas, bundles, or components?
- How much nesting or recursive composition should be allowed?
- How much of this should be erased before equality/planning?
- Should schemas be instantiation templates, importable packages, or both?

## 4. What Belongs In The Function Registry?

The registry is one of the most important likely `v2` additions, but it is still underdefined.

Open questions:

- Which functions should remain expressible directly as surface arithmetic?
- Which functions deserve registry-backed identities?
- What metadata should every registered function carry?
- How much invertibility information is needed to be useful?
- Should registry entries declare backend implementations directly, or lower through a backend-neutral intermediate form?

## 5. How Should Local Solve Blocks Work?

Same-step algebraic loops will become unavoidable for real plant models.

Open questions:

- What should the surface syntax of a solve block look like?
- How explicit should unknowns and solver hints be?
- Should solve blocks live inside the model language or as registry-backed components?
- How much should the compiler know about solver internals versus simply wiring backend hooks?

## 6. How Rich Should Observation Operators Become?

Observation operators are likely essential for real workflows.

Open questions:

- Should operators be simple expressions, registry-backed functions, or both?
- How should they attach to sparse or irregular schedules?
- How should uncertainty, weights, or sigmas be represented?
- When does an observation operator become a small model in its own right?

## 7. What Is The Right Indexing Story?

Sparse and irregular data need a clearer story than "short vectors with masks."

Open questions:

- Should indexing be timestep-based, timestamp-based, or both?
- Should schedules be reusable named objects?
- How should multiple sparse series align against one compile grid?
- Should interpolation ever live inside Myco, or always happen before binding?

## 8. How Ergonomic Should Binding Become?

The current Python API is explicit and semantically honest, but somewhat verbose.

Open questions:

- Should convenience layers live inside `myco` or in add-on packages?
- How much bulk binding should be allowed before semantics get fuzzy again?
- Should `pandas` / `xarray` adapters be core, optional, or external?
- How much should the package optimize for human ergonomics versus agent ergonomics?

## 9. How Broad Should The Constraint System Get?

There is likely a large future design space for constraints.

Open questions:

- Which constraint kinds matter most for the first real model?
- Which constraints are compile-time checks versus runtime projections versus penalties versus assertions?
- How much of the constraint design should live in the core language versus the registry?
- Is there a useful generic constraint interface, or is it better to keep a typed family of explicit constraint kinds?

## 10. How Far Should Backend Agnosticism Go?

Myco core should probably stay backend-agnostic, but the implementation is currently JAX-first.

Open questions:

- What is the minimum backend-neutral contract the core should emit?
- Which runtime semantics belong in each backend instead of in the core?
- How much should backend-specific optimization shape the surface language?
- When would PyTorch or a Rust-native backend become worth supporting seriously?

## 11. What Does A Healthy Registry Ecosystem Look Like?

If Myco grows packages and registries, versioning and dependency semantics will matter.

Open questions:

- How should models declare package dependencies?
- Should registries be local, remote, or both?
- How explicit should version pinning be?
- How much should published packages expose internals versus only public schemas/functions?

## 12. What Should Stay Out Of Scope?

One of the strengths of `v1` was scope control.

Open questions:

- What should be explicitly deferred even if it sounds exciting?
- Which ideas belong in domain libraries rather than in the core?
- How far can discrete-time plus local solve blocks take the science before continuous-time becomes unavoidable?
- What would signal that Myco is drifting into a general CAS or general workflow system instead of staying a scientific compiler?

## Short Version

The biggest unresolved themes are:

- the first real model family
- parameter semantics
- semantic schemas and packages
- function registries
- local solve blocks
- richer observation and indexing models
- the right boundary between a backend-neutral core and backend-specific runtimes

Those questions are probably more important than any single implementation task, because they determine what `v2` is actually trying to prove.
