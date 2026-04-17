# Node-First Ownership And Relationships

This note captures a specific design direction for the future type and
constraint system:

- everything is a node
- larger things are recursively structured nodes
- types declare what must be true about the things a node owns
- nodes can have mathematical or structural relationships to other nodes
- workflow assumptions remain separate from the world model

This is not an implementation plan yet. It is a conceptual note to keep the
future type/constraint work anchored to a simple core.

## Core Position

The cleanest foundation is:

- there is one kind of thing in the world model: **node**
- some nodes own only an atomic value
- some nodes own other nodes, and may also own atomic values
- relationships connect nodes across the graph

The important distinction is not "atomic type" versus "composite type."

The distinction is:

- some nodes happen to own only one atomic value
- some nodes own richer internal structure

Everything else should build on that.

## What A Type Does

A type is responsible for declaring what must be true about the things a node
owns.

That means:

- a node with one atomic value can still have a meaningful type
- a node with internal structure can also have a meaningful type
- the type owns the declarations of what must hold for the owned things

This is not two different categories of constraints.

It is one rule:

- a type declares facts about what its node owns

## Ownership

Ownership here means:

- this node includes, contains, or otherwise claims some thing as part of its
  structure

Important consequence:

- a thing can be owned by more than one node

That is not a bug. It is part of the model.

This is useful because different larger structures may legitimately constrain or
depend on the same owned thing. If those structures turn out to be mutually
inconsistent, that is good information. It means the representation needs to be
cleaned up.

## Relationships

Nodes do not just contain other nodes.

Nodes can also stand in mathematical or structural relationships to other
nodes.

Examples:

- equalities
- temporal updates
- conservation laws
- couplings
- grouped invariants

In practice, many of these relationships will eventually bottom out at atomic
values, even when they are written in terms of larger composite nodes.

So the graph has two important kinds of structure:

- **containment / ownership**
- **relationships / participation**

If you grab one node in a connected graph, you are effectively pulling on the
rest of the graph too.

## Constraints

Constraints are just facts that must hold about the things a node owns and the
relationships its owned things participate in.

Examples:

- an owned scalar must be nonnegative
- a grouped composition must sum to one
- a pressure must remain below zero
- a temporal update must preserve some invariant

The important simplification is:

- do not split this into separate fundamental kinds of constraints too early

Instead:

- a node owns some things
- the type declares what must be true about them
- the graph declares how they relate to other things

## Multiple Ownership Is Intentional

This direction explicitly allows the same thing to be owned by more than one
node.

That means the system must be able to:

- conjoin the resulting obligations
- detect inconsistency
- prove compatibility when possible
- verify compatibility at runtime when proof is not possible

This is a feature, not a defect.

The point is not to avoid representational tension. The point is to surface it
cleanly and early.

## Examples

### Example 1: An Atomic Node

```text
node stomata: Conductance
```

`stomata` owns one atomic value.

Its type might declare:

- `stomata >= 0`

That is enough for a meaningful type.

### Example 2: A Composite Node

```text
node dietary_composition: DietaryComposition {
  protein
  carbs
  fat
}
```

The node owns:

- `protein`
- `carbs`
- `fat`

The type might declare:

- `0 <= protein <= 1`
- `0 <= carbs <= 1`
- `0 <= fat <= 1`
- `protein + carbs + fat = 1`

This is not a different constraint system. It is the same basic rule applied to
owned internal structure.

### Example 3: Shared Ownership

Suppose a `leaf` node and a larger `tree_hydraulics` node both own or include a
common thing related to stomatal conductance or water state.

Then both nodes may impose facts involving that shared thing.

That is acceptable as long as the full set of constraints is treated seriously.
If the combined obligations are inconsistent, the graph or the workflow should
be rejected.

## Workflow Assumptions Stay Separate

The world model should still remain separate from workflow configuration.

So this note does **not** change the `v2_do_this_first` principle.

World model:

- nodes
- ownership
- relationships
- constraints
- temporal structure

Workflow/config:

- what is assumed
- what is observed
- what is learned
- which initial values are supplied or inferred

That separation remains important.

## Proof Versus Verification

This node-first direction makes the need for stronger checking clearer.

The system will eventually need to reason about:

- graph-level consistency
- workflow-on-graph compatibility
- runtime validity under concrete values

That naturally suggests a split between:

- **proof** when the compiler can establish something from structure and
  constraints
- **verification** when only concrete runtime values can settle it

Multiple ownership makes this more important, not less.

## Why This Direction Is Attractive

This model has a few advantages:

- it keeps the core object model simple
- it avoids premature ontology proliferation
- it allows rich recursive structure
- it allows shared ownership and representational tension
- it treats inconsistency as useful information
- it stays compatible with the world/workflow split

## Short Version

The intended future direction is:

- everything is a node
- nodes may own atomic values or richer internal structure
- a type declares what must be true about what its node owns
- nodes may also participate in relationships with other nodes
- the same thing may be owned by more than one node
- those obligations should compose seriously, with proof where possible and
  verification where necessary

That feels like a better base for the future type/constraint system than
starting from a flat catalog of separate object kinds.
