# Myco V2 Planning Index

This folder is for the next real compiler milestone, not the long-term
platform vision.

If you are reviewing or resuming `v2` work, start here.

For higher-altitude material, see:

- [../v_long_term/README.md](../v_long_term/README.md)

## Recommended Reading Order

### 1. Goal And Scope

- [charter.md](./charter.md)
- [v2_do_this_first.md](./v2_do_this_first.md)

These two files say what `v2` is trying to prove and what boundary it must
protect.

### 2. Current Near-Term Directions

- [ideas.md](./ideas.md)
- [open_questions.md](./open_questions.md)

These are the main milestone-level direction and unresolved decision docs.

### 3. New Structural Direction

- [node_first_ownership_and_relationships.md](./node_first_ownership_and_relationships.md)
- [node_first_syntax_sketch.md](./node_first_syntax_sketch.md)
- [units_and_dimensions_notes.md](./units_and_dimensions_notes.md)

These notes capture the newer node-first and richer type/unit direction. They
are important because they may change what should be implemented before the
first real plant-model validation pass.

### 4. Validation Direction

- [sparse_multi_context_training_notes.md](./sparse_multi_context_training_notes.md)

This captures the strongest current validation story:

- one world
- many experiment bindings
- shared controller
- progressive data erasure

### 5. Historical Boundary Work

- [v2_do_this_first_implementation_plan.md](./v2_do_this_first_implementation_plan.md)

This is still useful context, but much of it is already landed in the codebase.

## What Is Resolved Enough To Build On

- the world/workflow split is a real design rule, not a tentative idea
- the public workflow vocabulary is now `assume`, `observe`, and `learn`
- `v2` should stay narrow and prove one real plant workflow
- JAX should remain the first-class backend for training workflows

## What Is Still Design Work

- the exact node-first structural language
- the first practical type/constraint/unit slice
- the right module and unit-import story
- how generics and repeated structure should work
- what should be borrowed from existing unit/type libraries such as `uom`
- the exact first real plant model family and validation benchmark

## What This Folder Is Not

This folder is not the place for:

- Myco/OzzyDB convergence
- long-term theorem proving or CAS ambitions
- the full scientific-platform vision

Those remain important, but they live in `planning/v_long_term/`.
