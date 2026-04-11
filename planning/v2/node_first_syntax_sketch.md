# Node-First Syntax Sketch

This note captures one concrete syntax sketch for the node-first `v2`
direction.

It is not the final spec.

The goal is to make the design discussion concrete enough to iterate on:

- importable and definable units
- node-first world structure
- recursive containment
- group constraints
- const generics
- generic physiology implementations
- path-based binding from workflow/config

This note should be read alongside:

- [v2_do_this_first.md](./v2_do_this_first.md)
- [node_first_ownership_and_relationships.md](./node_first_ownership_and_relationships.md)
- [sparse_multi_context_training_notes.md](./sparse_multi_context_training_notes.md)

## Design Direction

The current direction behind this sketch is:

- the world model should be structural and workflow-neutral
- the world model should be node-first rather than flat-quantity-first
- units should be imported or defined, not treated as magic text labels
- the model should support recursive structure and generics
- generators for concrete instances should stay outside the world model
- workflow assumptions and observation bindings should still live in config

The point is to make the structural language rich enough to describe the world
cleanly before using a real plant model as validation.

## Worked Example

```myco
module plant::tiny_tree

use units::si::{
  megapascal as MPa,
  meter2 as m2,
  mole_per_second as mol_s,
  mole_per_square_meter_second as mol_m2_s,
  gram_carbon as gC,
  ratio,
}

unit mmol_m2_s = 1e-3 * mol_m2_s

type Fraction : Scalar<ratio> {
  0 <= self <= 1
}

type Potential : Scalar<MPa>
type Conductance : Scalar<mol_m2_s> {
  self >= 0
}
type WaterFlux : Scalar<mol_s> {
  self >= 0
}
type CarbonMass : Scalar<gC> {
  self >= 0
}
type Area : Scalar<m2> {
  self > 0
}

node NscComposition {
  sugar: Fraction
  starch: Fraction

  constraint normalized:
    sugar + starch = 1
}

contract Photosynthesis {
  input ci: Potential
  input par: Scalar<ratio>
  input temperature: Scalar<ratio>
  input jmax: Conductance
  input vmax: Conductance

  output assimilation: CarbonMass
}

node Leaf<P: Photosynthesis> {
  water: Potential {
    self <= 0 MPa
  }

  stomata: Conductance
  g_max: Conductance
  jmax: Conductance
  vmax: Conductance
  area: Area
  nsc: NscComposition
  photo: P
  transpiration: WaterFlux
}

node Environment {
  vpd_scale: Potential
  soil_water: Potential
  par: Scalar<ratio>
  temperature: Scalar<ratio>
  hydraulic_cond: Conductance
}

node Canopy<const N: usize, P: Photosynthesis> {
  leaves: [Leaf<P>; N]

  constraint positive_total_area:
    sum(leaves[i].area for i in 0..N) > 0 m2
}

node Tree<const N: usize, P: Photosynthesis> {
  canopy: Canopy<N, P>
  env: Environment
}

relation demand_transpiration[i in 0..N]:
  canopy.leaves[i].transpiration =
    canopy.leaves[i].stomata * env.vpd_scale

relation supply_transpiration[i in 0..N]:
  canopy.leaves[i].transpiration =
    env.hydraulic_cond * (env.soil_water - canopy.leaves[i].water)

relation photosynthesis_inputs[i in 0..N]:
  canopy.leaves[i].photo.ci = canopy.leaves[i].water

relation photosynthesis_env[i in 0..N]:
  canopy.leaves[i].photo.par = env.par

relation photosynthesis_temp[i in 0..N]:
  canopy.leaves[i].photo.temperature = env.temperature

relation photosynthesis_traits[i in 0..N]:
  canopy.leaves[i].photo.jmax = canopy.leaves[i].jmax

relation photosynthesis_capacity[i in 0..N]:
  canopy.leaves[i].photo.vmax = canopy.leaves[i].vmax

slot controller provides [canopy.leaves[*].stomata]:
  inputs = [
    canopy.leaves[*].water,
    canopy.leaves[*].nsc.sugar,
    canopy.leaves[*].nsc.starch,
    canopy.leaves[*].jmax,
    canopy.leaves[*].vmax,
    env.vpd_scale,
    env.soil_water,
  ]

temporal water_step[i in 0..N]:
  canopy.leaves[i].water[t+1] =
    canopy.leaves[i].water[t] - dt * canopy.leaves[i].transpiration[t]
```

## Why This Example Exists

This example is trying to force several design questions into the open at once.

### 1. Units Are Imported Or Defined

The sketch assumes that units are not raw strings.

Instead:

- standard systems such as SI are importable
- project-local units can be defined in terms of imported units

This is closer to how a serious unit system should behave. It also lines up with
the instinct to follow something more like Rust `uom` than ad hoc textual unit
tags.

### 2. Module Identity Beats A Free-Floating Model Name

The sketch uses:

```myco
module plant::tiny_tree
```

instead of a separate:

```myco
model TinyTree
```

The point is to make identity and modularization come from the module system
rather than from an extra top-level name that can drift out of sync.

This does not commit Myco to Rust syntax exactly. It just expresses the design
direction:

- support modularization directly
- avoid redundant naming surfaces

### 3. Nodes Are Recursive Structural Objects

`Leaf`, `Environment`, `Canopy`, and `Tree` are all nodes.

Some own atomic values.
Some own richer internal structure.

This is the node-first model from
[node_first_ownership_and_relationships.md](./node_first_ownership_and_relationships.md)
made concrete.

### 4. Types Say What Must Be True

The sketch keeps types narrow but meaningful:

- units
- scalar carrier
- simple constraints

Then nodes can add more specific constraints or grouped constraints over owned
structure.

This is enough to express both:

- an atomic fact such as `self >= 0`
- a nontrivial group fact such as `sugar + starch = 1`

### 5. Group Constraints Need To Be First-Class

`NscComposition` exists in the sketch because some important scientific facts do
not emerge from atomic constraints on their parts.

The sugar/starch example is analogous to dietary composition:

- each component is bounded
- but the important constraint is on the group

That is one reason a recursive node model is attractive.

### 6. Const Generics Matter

The sketch uses:

```myco
node Canopy<const N: usize, P: Photosynthesis>
```

because some scientific structures need:

- repeated internal structure
- but not a fixed count hard-coded in the model definition

The important idea is that the world model can say:

- a canopy contains `N` leaves

without also embedding the generator that creates a concrete population of
leaves.

### 7. Generic Implementations Matter

The `Photosynthesis` contract is intended to represent the need for swappable
subsystems such as:

- `C3`
- `C4`
- `CAM`

The point is not that the exact `contract` syntax is final.

The point is that Myco likely needs a way to say:

- this node contains some implementor of a known interface
- different physiological implementations can be swapped in later

That is a strong test for generic support.

### 8. Generators Stay Out Of The World Model

The sketch intentionally does **not** try to say how to generate 1000 leaves
with correlated `jmax`, `vmax`, area, and other traits.

That is a config/runtime concern.

The world model should express:

- the canopy contains `N` leaves
- each leaf has its own traits

The workflow/config side should decide:

- how many leaves there are in one run
- how their traits are sampled or loaded
- what distributions or empirical values are used

This keeps the world structural and workflow-neutral.

## What Binding Would Look Like

The intended workflow side still looks separate from the world:

```python
experiment.assume_series("env.vpd_scale", steps)
experiment.assume_series("env.soil_water", steps)
experiment.assume_constant("env.hydraulic_cond")
experiment.assume_constant("canopy.leaves[*].jmax")
experiment.assume_constant("canopy.leaves[*].vmax")
experiment.assume_constant("canopy.leaves[*].area")
experiment.assume_initial("canopy.leaves[*].water")
experiment.learn_slot("controller")
experiment.observe_dense("canopy.leaves[*].transpiration")
```

This is still only a sketch, but it shows the intended binding direction:

- path-based binding
- wildcard support for repeated structure
- the generator for concrete trait arrays or correlated samples lives outside
  `.myco`

## Things This Sketch Is Trying To Settle

This sketch is meant to put pressure on these design questions:

- units should be importable and definable
- module identity should likely replace a separate free-form model name
- path syntax should be a first-class part of the world model
- grouped constraints should be expressible directly
- const generics and generic implementations should be possible
- repeated structure and concrete instance generation should stay separate

## Things This Sketch Does Not Yet Settle

This note is intentionally not pretending the details are finished.

It does **not** settle:

- the exact final module/import/export syntax
- the exact syntax for contract implementations such as `C3`, `C4`, and `CAM`
- how shared ownership is written when the same thing belongs to more than one
  node
- whether types themselves can own structure or only nodes can
- whether unit annotations belong only on types or can be overridden per field
- the precise flattening strategy from recursive nodes into the current lower
  compiler substrate

## Practical Use

The point of a sketch like this is not to commit too early.

The point is to create a concrete target that can be criticized:

- does this feel like the right structural language?
- is the module/unit story moving in the right direction?
- are generics carrying their weight?
- is the boundary between world structure and workflow generation still clean?

Those are the questions this note is meant to support.
