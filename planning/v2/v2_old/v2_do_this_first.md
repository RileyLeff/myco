# V2 Do This First

This note is intentionally narrower than [ideas.md](./ideas.md) and [open_questions.md](./open_questions.md).

It captures one design decision that should probably be settled before building much more on top of the current language:

> the world model should stay about quantities, relationships, constraints, and temporal structure; workflow roles should move into binding/config instead of living in `.myco`

This is an implementation note, not a final spec.

For the concrete migration sequence, see [v2_do_this_first_implementation_plan.md](./v2_do_this_first_implementation_plan.md).

## Why This Matters

If this boundary stays blurry, Myco will accumulate debt in exactly the wrong place:

- the structural model will encode workflow assumptions
- the compile/binding layer will be less flexible than it should be
- model sharing will get harder because users will publish models tied to one usage pattern
- future package registries and semantic schemas will inherit the wrong abstraction boundary

If the boundary is cleaned up early, Myco gets a much stronger foundation for:

- reusable structural models
- multiple workflows over the same model
- package-based sharing
- stronger compile-time rejection of impossible model/config combinations

## The Core Principle

The `.myco` file should describe the world.

The binding/config layer should describe the workflow.

That means `.myco` should own things like:

- quantities
- dimensions and type-like metadata
- constraints
- temporal relations
- functions and relations
- semantic schemas later

And config/binding should own things like:

- directly provided values
- learned values
- observed values
- fixed values
- which quantities must be initialized
- which quantities are rollout-persistent in this workflow
- which outputs are required

The world should not need to know where the data came from.

## What This Means For Source Syntax

The old source split of:

- `external`
- `state`
- `node`

was useful in `v1`, but it mixed structural semantics with workflow semantics more than we want long term.

In particular:

- `external` is mostly a workflow role, not a world fact
- `state` may also be better treated as a binding-time or inferred property rather than a mandatory source-level distinction

The current direction is now the flatter:

- `quantity`

declaration form, with workflow meaning carried by binding/config instead.

The clean mental model is:

- the world says what relationships hold
- the config says how a particular run uses those relationships

For example, in a world containing:

```text
x = m * x + b
```

the world should not care whether:

- `m` is fixed
- `m` is learned
- `m` is supplied from data
- `m` is left latent

That is a workflow choice.

## Proposed Direction

The likely direction is:

1. minimize source-level quantity-role keywords in `.myco`
2. move provided/learned/observed/fixed/persistent decisions into config
3. let the compiler infer structural facts from temporal relations where possible
4. still allow explicit binding-time annotations whenever they help the compiler

That last point is important.

This note is **not** arguing for zero annotation.

It is arguing that annotations should live in the right layer.

If the compiler benefits from being told:

- this quantity is persistent in this workflow
- this quantity is directly supplied
- this quantity is observed
- this quantity is a fixed rollout-static input

that is fine.

The key is that those are binding-time annotations, not world-model syntax.

## Why This Is Better

### 1. The Same World Supports More Workflows

A single world model can then be reused for:

- simulation
- fitting
- control learning
- partial inference
- counterfactual analysis

without rewriting the model file to match each use case.

### 2. Model And Config Can Be Valid Separately

This also creates a cleaner compiler story:

- the `.myco` model can be valid on its own
- a config can be valid on its own
- the combination can still be invalid or impossible

That is a good thing.

It means Myco can reject:

- incompatible role assignments
- missing initialization obligations
- impossible requested workflows
- insufficiently constrained compilation targets

without pretending the world model itself is wrong.

### 3. Package Sharing Gets Cleaner

If published models do not bake in workflow assumptions, they become easier to share and reuse.

That matters a lot for the likely package direction:

- model bundles
- domain libraries
- reproducible versioned dependencies

The same package should not need multiple copies just because one user wants to learn a controller and another wants to fix it.

### 4. Semantic Layers Become Easier To Add

Semantic schemas, components, and trait-like contracts become much easier to reason about if they expand into a world model that remains workflow-neutral.

That supports the longer-term goal of things like:

- importable model packages
- reusable semantic components
- less-technical users asking an agent to import and adapt an existing model

## Likely Compiler Consequences

If this direction is adopted, the compiler will still need internal notions equivalent to:

- persistent quantities
- directly provided quantities
- observed quantities
- learned quantities

But those become properties of the prepared experiment / compiled workflow, not necessarily of the source language.

That means:

- some properties are inferred from temporal structure
- some are declared by binding/config
- the planner works over the merged result

So the compiler does not become less informed.

It just gets its information from the correct layer.

## Migration Direction

This does not require an immediate giant rewrite.

A reasonable sequence might be:

1. define the intended boundary clearly in the docs and spec
2. stop treating `external` as a long-term language commitment
3. explore how much of `state` can become inferred or config-driven
4. introduce binding-time annotations for persistence/provision/observation roles
5. later simplify the `.myco` surface once the new path is proven

The important thing is to avoid building many new `v2` features on top of assumptions that likely belong in config instead.

## Open Questions

This note does not settle everything.

Important remaining questions:

- Should persistence be fully inferred from temporal relations, or also explicitly annotatable in config?
- How should config express "rollout-persistent but not directly observed" versus "recomputed each step"?
- How much source-level sugar is still worth keeping for readability?
- How should this interact with future semantic schemas and package imports?
- What should the runtime contract look like once source-level role keywords shrink?

## Recommendation

Before substantial `v2` feature work, Myco should explicitly adopt this design principle:

> structural world description and workflow role assignment are separate concerns, and Myco should increasingly move role assignment into config/binding rather than source-level `.myco` declarations

That does not mean removing every annotation immediately.

It means making sure future features are built on the right conceptual boundary.
