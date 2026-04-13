# Notes for Reviewers

These notes provide context that previous reviewers have found helpful. They
cover design philosophy, intentional constraints, and areas where the spec's
choices are motivated by the research use case in ways that aren't obvious from
the documents alone.

## 1. Slots replace heuristics — that's the whole point

The core research vision (Document 5) is to replace teleological stomatal
optimization heuristics (Sperry gain-risk, Potkay GOH) with neural controllers
trained on observational data. The `slot` mechanism is how this works: the world
model declares the physics, and the slot provides the control policy. The slot
is trained by minimizing observation losses against real data — the controller
learns to reproduce observed stomatal behavior without being told the
optimization criterion.

This means you will NOT find a way to express "maximize cumulative growth" or
"solve the gain-risk first-order condition" in the workflow API. That's
intentional. Those are the heuristics being replaced, not training objectives.
The GOH comment in the Potkay mock ("the GOH criterion becomes the training
objective") is aspirational shorthand — what it means is that the GOH's
*predictions* become the synthetic training data, not that the workflow layer
implements reward maximization.

Previous reviewers have suggested adding `maximize(quantity)` or
`slot_objective(reward=...)` to the workflow vocabulary. This would be a
fundamentally different training paradigm (reinforcement learning) that is out
of scope for v2. The supervised approach — train on observations, let the
controller implicitly learn the behavior — is sufficient for the stated
research plan and is simpler to implement correctly.

## 2. The `deriv` SCC feedback ban is intentional

Section 9.5 forbids `deriv(A, g_s)` from feeding back into the SCC that
determines A and g_s. A previous reviewer argued this prevents expressing the
Sperry gain-risk criterion natively:

```myco
// "You can't compile this without lifting the ban!"
relation gain_risk:
    deriv(total_assimilation, g_s) = theta * deriv(total_transpiration, g_s)
```

This is a misunderstanding of the workflow. You don't need this relation in the
`.myco` file because the stomatal controller is a **slot**, not a `deriv`-based
optimality condition. To generate synthetic data from a baseline Sperry model,
you provide a hand-coded Python function as the slot (implementing the
gain-risk criterion externally) and run in simulate mode. The `.myco` file
contains the physics; the heuristic lives outside it.

The ban exists because `deriv` feedback into an SCC requires computing the
Hessian of the SCC's equation system at every Newton iteration. While JAX can
compute Hessians, this is tractable only for tiny SCCs (1-2 equations). For the
general case — a 50-equation hydraulic network — it is genuinely
intractable. The spec prioritizes a rule that is correct everywhere over one
that works for special cases.

## 3. Shared controllers require structural identity (by design)

Section 7.1 requires that all experiments sharing a controller use the same
model instantiation. Previous reviewers have noted that this prevents sharing a
controller across sites with different `N_SOIL` or `N_CANOPY`, and suggested
element-wise (vectorized) slots where the controller operates per-element with
a fixed input dimension.

This is a good idea and may be added in a future version. The current strict
rule is intentionally conservative: it guarantees that the controller's input
vector has the same structure across experiments without requiring the compiler
to prove equivalence of resolved input sets. Relaxing it is an optimization,
not a correctness fix.

For the stated research plan, the workaround is straightforward: use explicit
`inputs = [list]` instead of `inputs = [*]`, choosing inputs that are
structurally invariant across sites (e.g., leaf-level quantities rather than
soil-layer arrays).

## 4. Dimensional analysis through the unit system

The unit system tracks dimension exponent vectors algebraically. A few points
that have tripped up reviewers:

- **Temperature differences are purely multiplicative.** Subtracting two
  affine-unit quantities (e.g., `T1 - T2` where both are `degC`) produces a
  temperature difference in the base unit (Kelvin). The affine offset cancels.
  This means `(temperature - T_ref) / 10 K` is a valid dimensionless
  expression — you do not need `value_in` for temperature differences.

- **`value_in` is for empirical equations calibrated to specific unit scales**
  (Buck equation, energy balance boundary layer conductance). It is the escape
  hatch, not the default. If the physics works in the dimension system
  directly, prefer that.

- **The compiler stores everything in base SI units internally** (section 4.5).
  When a quantity declared as `Scalar<MPa>` participates in an equation that
  produces a result in Pa, the compiler handles the conversion automatically.
  You will not find explicit conversion factors like `1e-6` in correctly
  written models.

- **`MPa_s_inv` in the Potkay mock means 1/(MPa*s)** — the Lockhart
  extensibility has units of reciprocal pressure per time. This is an
  admittedly confusing unit alias name; if it reads like MPa/s to you, that's
  a naming issue, not a dimensional error.

## 5. Mock simplifications

The mocks are stress tests for the language spec, not production-ready models.
Known simplifications:

- **Soil water balance (Sperry):** The temporal equation updates water
  potential directly from flow rates, which is dimensionally approximate. A
  production model would track volumetric water content and use the moisture
  retention curve. This is noted in the mock comments.

- **Single soil layer (Potkay):** The paper models a single soil layer. This
  is intentional — it tests a simpler hydraulic catena than Sperry while
  exercising different features (carbon balance, turgor growth).

- **Phloem osmotic coefficients (Potkay):** The empirical parameterization
  from Paljakka et al. 2017 uses `value_in(MPa)` to extract a dimensionless
  pressure value for the linear fit. This is the correct use of `value_in` —
  the fit was calibrated to specific units.
