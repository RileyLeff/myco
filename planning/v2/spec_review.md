# V2 Spec and Mock Sperry Review

Self-review of `spec.md` and `mock_sperry.myco` after initial drafting.
Issues are categorized by severity: **bug** (wrong), **gap** (missing),
**unclear** (underspecified).

---

## Issues in the Spec

### 1. Continuous vs discrete quantification in predicates [unclear]

**Location**: Section 3.3.1 (VulnerabilityCurve contract), Section 5

The VulnerabilityCurve monotonicity constraint quantifies over all pairs of
reals:

```myco
constraint monotonic:
    forall p1, p2 where p1 < p2:
        plc(p1) <= plc(p2)
```

Every other `forall` in the spec ranges over discrete index sets (`0..N`).
Continuous quantification is fundamentally different — the compiler can't check
it by expansion. It would require symbolic reasoning or trust.

**Options**:

- Restrict `forall` to discrete index ranges only. Express monotonicity as
  metadata on the contract (e.g., `monotone: increasing`) that the compiler
  trusts or verifies symbolically for simple expressions.
- Explicitly acknowledge two kinds of quantification (discrete and continuous)
  and define what the compiler does with each.
- Allow continuous quantification in constraints but document that it is
  checked symbolically where possible and trusted otherwise.

### 2. `=` vs `==` inconsistency in relations [unclear]

**Location**: Section 6, Appendix A

Relations in the spec body use `==` (equality predicate):

```myco
relation demand_transpiration:
    leaf.transpiration == leaf.stomata * env.vpd_scale
```

But the TinyTree example in Appendix A uses `=` (assignment-like):

```myco
relation demand_transpiration:
    transpiration = stomata * vpd_scale
```

The whole point of acausal modeling is that relations are equalities, not
directed computations. The spec should pick one syntax and be consistent. `==`
is more honest. `=` is more readable. Either way, the spec should state that
both sides are symmetric and the compiler may solve in either direction.

### 3. Type vs node distinction is unprincipled [unclear]

**Location**: Section 3.2

The spec says: "a type is a reusable structural pattern. A node is an instance
of a type." But in practice:

- `NscComposition` is a `type` with fields and constraints, used as a field
  type in nodes
- `WeibullVC` is a `node` that implements a contract, also used as a field type
  via generics

**Questions that need answers**:

- Can a type implement a contract?
- Can a node be used as a field type without generics?
- Is the distinction syntactic sugar (both lower to the same thing) or
  semantic (they have different compilation behavior)?
- Should there be only one keyword (`node`) with types being just nodes that
  are used as reusable patterns?

### 4. Conditioning-aware path blending is underspecified [unclear]

**Location**: Section 8.5

The section shows a code snippet with `conditioning_weight(...)` but doesn't
define:

- What `conditioning_weight` actually computes
- How the compiler decides when to use blending vs simple path selection
- Whether the user can control this (it's mentioned in compiler config 14.2
  but only as "on/off, blend sharpness")
- What the gradient behavior is at the blending boundary

This reads more like an idea than a spec section. Either flesh it out with a
concrete algorithm or mark it as deferred and remove the code snippet.

### 5. `inputs = [*]` interaction with SCCs [gap]

**Location**: Section 7.1, Section 12.5

If a slot has `inputs = [*]` (all available quantities), but some quantities
are inside an algebraic loop, the slot can't execute until the loop is solved.
But if the slot's output is needed by something inside the loop, you get a
chicken-and-egg problem.

The spec doesn't address this. The planner needs a rule for how slots interact
with SCCs:

- Option A: the slot executes before any SCC it doesn't participate in, and
  its output feeds into the SCC as a known value.
- Option B: if the slot's output is needed inside an SCC, the slot becomes
  part of the SCC (the learned function is inside the solver loop).
- Option C: `[*]` is resolved to "all quantities available *before* the SCC
  that needs this slot's output," not literally everything.

This matters for the Sperry model: the stomatal controller produces `stomata`,
which feeds into the demand transpiration relation, which is part of the
hydraulic SCC. Does the slot execute before the hydraulic solver, or is it
embedded in it?

### 6. Bound controller compilation is undefined [gap]

**Location**: Section 7.2

The spec shows a package directory with controller `.myco` files and a Python
API for `bind_slot`, but doesn't explain:

- What a controller `.myco` file looks like syntactically
- How the compiler loads and integrates it into the plan
- Whether the controller can have its own internal relations and constraints
- Whether the controller can create new algebraic loops

This needs at least a sketch of the controller file format and compilation
semantics.

### 7. No mention of `dt` semantics [gap]

**Location**: Section 6.3

Temporal relations use `dt`:

```myco
temporal water_step:
    water[t+1] = water[t] - dt * transpiration[t]
```

But `dt` is never defined. Is it:

- A global constant set at compile time?
- A per-experiment binding-time value?
- Available in the rollout function signature (like v1)?
- A quantity in the world model?

The v1 implementation passes it as a runtime argument to `rollout()`. The spec
should state this explicitly.

### 8. No error model or noise [gap]

**Location**: Section 15.2 (observations)

Observations contribute to loss, but the spec doesn't discuss observation
error models. Real data has measurement noise, and the loss function should
account for it. At minimum:

- Should observation declarations support a noise/uncertainty parameter?
- Should the loss be weighted by observation uncertainty?
- Is this a v2 concern or deferred?

Likely deferred, but worth noting.

---

## Issues in the Mock Sperry

### 9. Rhizosphere and root xylem use the same conductance [bug]

**Location**: `rhizosphere_flow` and `root_flow` relations

Both use `pathway.root[j].conductance`, but they represent different physical
segments:

- Rhizosphere flow (soil to rhizosphere boundary): conductance from van
  Genuchten soil properties
- Root xylem flow (rhizosphere to root junction): conductance from the Weibull
  vulnerability curve

The mock should either have separate rhizosphere segments with their own
conductance (computed from van Genuchten), or at minimum use different
conductance values for the two relations.

### 10. All canopy layers share one `stomata` value [bug]

**Location**: `stomatal_to_conductance` relation, `SperryTree` node

```myco
relation stomatal_to_conductance[i in 0..N_CANOPY]:
    canopy_layers[i].g_w == stomata
```

This gives sun and shade leaves identical stomatal conductance, which defeats
the purpose of sun/shade decomposition. In Sperry's model, the optimization
produces different `g_w` for sun vs shade (same hydraulic risk curve, different
gain curves due to different PAR).

Fix: `stomata` should be per-layer, or the slot should provide per-layer
outputs:

```myco
slot stomatal_control provides [canopy_layers[*].g_w]:
    inputs = [*]
```

### 11. Temperature-adjusted params use only layer 0's temperature [bug]

**Location**: `adjust_vmax`, `adjust_jmax`, etc.

```myco
relation adjust_vmax:
    params.v_max == peaked_arrhenius(
        params.v_max_25, 65330.0, 200000.0, 650.0, canopy_layers[0].leaf_temperature,
    )
```

Sun and shade leaves have different temperatures and therefore different
effective Vmax, Jmax, etc. Having one shared `params.v_max` applied to all
layers is physically wrong.

Fix: either make the adjusted params per-layer fields inside
`LeafGasExchange`, or compute them inline in the per-layer photosynthesis
relations.

### 12. VC invocation syntax is inconsistent [bug]

**Location**: `XylemSegment` node

Two different patterns for using the VC:

```myco
// Pattern 1: field access (implicit current pressure)
constraint conductance_from_vc:
    conductance == k_max * (1.0 - vc.plc)

// Pattern 2: function call (explicit pressure argument)
constraint max_conductance_from_history:
    max_conductance == k_max * (1.0 - vc(min_historical_pressure).plc)
```

The spec doesn't define whether a contract implementation is:

- A node with state (field access, bound to current context)
- A callable (function-like, can be evaluated at any input)
- Both (default input from context, but overridable)

This needs a language-level decision. The third option is probably right but
needs explicit spec language.

### 13. Missing day respiration [gap]

The Farquhar model subtracts dark respiration (`R_day`) from gross assimilation
to get net assimilation:

```
A_net = A_gross - R_day
```

The mock only has gross assimilation. For the training workflow, net vs gross
matters for the gain function.

### 14. Missing soil redistribution [gap]

The temporal step only handles transpiration extraction:

```myco
temporal soil_water_step[j in 0..N_SOIL]:
    soil.layers[j].water_potential[t+1] =
        soil.layers[j].water_potential[t]
        - dt * pathway.layer_flow[j] / soil.layers[j].thickness
```

The real model has water flow between adjacent soil layers (driven by pressure
gradients), rain infiltration, and potentially groundwater exchange. The mock
is missing all inter-layer dynamics.

### 15. Energy balance has hardcoded constants [gap]

The `energy_balance` relation has magic numbers:

```myco
let emissivity = 0.97
let latent_heat = 44100.0
let g_ha = 1.4 * 0.135 * sqrt(atm.wind_speed / 0.0072)
```

These should either be:

- Named constants in a node (e.g., `leaf_properties.emissivity`)
- Parameters that can be assumed or calibrated
- At minimum, commented with units

The 0.0072 is leaf width * 0.72 — a plant parameter, not a universal constant.

### 16. Leaf VPD relation is oversimplified [gap]

```myco
relation leaf_vpd[i in 0..N_CANOPY]:
    canopy_layers[i].leaf_vpd ==
        atm.vpd + 0.0511 * (canopy_layers[i].leaf_temperature - atm.temperature)
```

The real relationship involves saturated vapor pressure (exponential in
temperature), not a linear offset. This linearization is fine for a mock but
should be flagged as a placeholder.

---

## Summary

| #  | Type    | Severity | Component | Summary |
|----|---------|----------|-----------|---------|
| 1  | unclear | medium   | spec      | Continuous vs discrete quantification |
| 2  | unclear | low      | spec      | `=` vs `==` in relations |
| 3  | unclear | medium   | spec      | Type vs node distinction |
| 4  | unclear | low      | spec      | Path blending underspecified |
| 5  | gap     | high     | spec      | Slot inputs vs SCC interaction |
| 6  | gap     | medium   | spec      | Bound controller compilation |
| 7  | gap     | medium   | spec      | `dt` semantics undefined |
| 8  | gap     | low      | spec      | No observation error model |
| 9  | bug     | high     | mock      | Rhizosphere/root conductance conflated |
| 10 | bug     | high     | mock      | Sun/shade share one stomata value |
| 11 | bug     | high     | mock      | Temp-adjusted params use only layer 0 |
| 12 | bug     | medium   | mock      | VC invocation syntax inconsistent |
| 13 | gap     | low      | mock      | Missing day respiration |
| 14 | gap     | medium   | mock      | Missing soil redistribution |
| 15 | gap     | low      | mock      | Hardcoded energy balance constants |
| 16 | gap     | low      | mock      | Oversimplified leaf VPD |
