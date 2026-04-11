# Units And Dimensions Notes

This note captures the current `v2` direction for units and dimensions in the
node-first design.

It exists because units are no longer just a small implementation detail. In
the current design discussion they are part of the structural language:

- types depend on them
- constraints may mention them
- workflow data binding should validate against them
- proof and runtime verification should eventually use them

This note is still pre-spec.

## Why Units Need Their Own Note

The current `v1` implementation already has a simple dimensions layer in Rust,
but it is still basically:

- quantity-type strings
- a small hard-coded dimension table
- very limited unit support

The node-first `v2` direction is asking for much more:

- importable unit systems
- definable derived units
- clearer separation of dimensions versus units
- unit-aware constraints
- unit-aware data binding and coercion
- eventual proof obligations involving units

That is too important to leave implicit inside other notes.

## Current Design Direction

The current sketch assumes:

- dimensions and units are related but not the same thing
- units should be imported or defined, not ad hoc strings
- types should be able to reference units explicitly
- the workflow layer should be able to validate or coerce data against those
  units

The motivating syntax direction looks like:

```myco
use units::si::{
  megapascal as MPa,
  meter2 as m2,
  mole_per_second as mol_s,
  mole_per_square_meter_second as mol_m2_s,
  ratio,
}

unit mmol_m2_s = 1e-3 * mol_m2_s

type Potential : Scalar<MPa>
type Conductance : Scalar<mol_m2_s>
type Fraction : Scalar<ratio> {
  0 <= self <= 1
}
```

The important idea is:

- imported systems provide named units
- project-local files may define new units from those imports

## Separation We Probably Want

The likely separation is:

- **dimension**
  - pressure
  - conductance
  - ratio
  - area

- **unit**
  - MPa
  - Pa
  - mol m^-2 s^-1
  - m^2

Dimensions are about compatibility.

Units are about scale, naming, and conversion.

That separation matters because the system should be able to say things like:

- these two quantities have the same dimension
- but they use different units
- therefore conversion may be possible

instead of treating every unit spelling as a new semantic kind.

## What The First Practical Slice Should Probably Cover

The first useful `v2` unit system does not need to solve everything.

It probably only needs to support:

- imported standard unit systems such as SI
- project-local derived unit definitions
- type-level unit declarations
- dimension compatibility checks in relations
- unit compatibility checks at binding boundaries
- explicit conversion where scale is known
- unit-aware diagnostics

That is already enough to improve:

- model readability
- error messages
- data binding validation
- future observation operators

## What Should Remain Out Of Scope Initially

The first cut probably should **not** try to solve:

- every possible nonlinear unit system
- arbitrary symbolic unit algebra in the source language
- a huge built-in ontology of scientific dimensions
- full theorem-prover-level unit reasoning

Those can come later if the first real model genuinely forces them.

## How Units Interact With Data Binding

One important `v2` goal is better binding from structured data.

That means the binding layer should eventually be able to answer questions like:

- is this column compatible with `env.vpd_scale`?
- is it already in MPa?
- is it convertible from kPa?
- is this observation invalid because the supplied unit is wrong?

So unit handling is not only a parser/type issue. It is also part of the
workflow contract.

This is one reason units should be considered before the first real benchmark,
not after it.

## How Units Interact With Proof And Verification

Longer term, units should participate in both:

- **proof**
- **verification**

Examples:

- prove that an equation is dimensionally consistent
- prove that a path is unit-compatible under conversion
- reject a workflow that binds incompatible units
- verify at load time that concrete data carries allowed units

Units are one of the cleanest early examples of a fact that the compiler should
often be able to prove statically.

## Should Myco Vendor Or Borrow From `uom`?

The current best question is not:

- should Myco vendor `uom` wholesale?

It is:

- what concepts should Myco borrow from systems like `uom`?
- what has to remain Myco-native because it lives at the DSL/compiler/prover
  layer?

Reasons to be cautious about directly vendoring `uom`:

- `uom` is designed for Rust type-level quantities inside Rust programs
- Myco needs language-level unit syntax in `.myco`
- Myco also needs lowering into compiler IR
- Myco needs backend-neutral semantics
- Myco needs workflow data validation and future proof obligations

So there is probably a lot to learn from `uom`, but that does not automatically
mean it is the right implementation substrate.

## Good External Review Questions

When sending the codebase out for review, the units questions worth asking are:

- what should the first practical unit/dimension slice include?
- what should be deferred until after the first real model proof?
- which concepts from `uom` are worth borrowing?
- what parts of unit handling likely need to be Myco-native?
- should units live only on types, or can fields override them?
- how much conversion logic belongs in the language versus the workflow layer?

## Short Version

The current direction is:

- units and dimensions should become a first-class part of the node-first
  structural language
- imported and definable units are likely the right direction
- binding and validation should become unit-aware
- proof should eventually use unit information aggressively
- Myco should probably borrow ideas from `uom` without assuming that it should
  vendor `uom` directly
