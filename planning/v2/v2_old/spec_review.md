# Mock Potkay Review — Paper Fidelity Check

**Paper**: Potkay & Feng (2023), "Do stomata optimize turgor-driven growth?", New Phytologist 238: 506–528
**Mock**: planning/v2/mock_potkay.myco

## Equation-by-Equation Comparison

### Eqn 1: GOH optimization objective (max integral of G(C, E) dt)

**Paper**: max_{g_c} integral of G(C, E) dt — the growth optimization hypothesis.
**Mock**: Lines 577–584. The mock handles this via a `slot stomatal_control provides [gas.g_s]` with `inputs = [*]`. The comment on line 578 correctly notes that "the GOH criterion becomes the training objective in the workflow layer." This is the correct Myco v2 idiom: the optimization objective is not expressed in the world model; it belongs in the workflow layer where the slot is trained to maximize cumulative growth.
**Verdict**: Correct structural representation. The objective function itself is not expressed in the .myco file (nor should it be).

### Eqn 2: NSC dynamics (dC/dt = a_t * A_n - R_M - R_G - G)

**Paper**: dC/dt = a_t * A_n - R_M - R_G - G, where a_t is total leaf area, A_n is leaf-area-specific net carbon assimilation.
**Mock**: Lines 593–598.
```
carbon.C[t+1] = carbon.C[t]
    + dt * (carbon.assimilation_total[t]
            - carbon.R_M[t]
            - carbon.R_G[t]
            - carbon.growth[t])
```
The `assimilation_total` is wired on line 571 as `structure.leaf_area * gas.photo.assimilation`, which correctly implements a_t * A_n.
**Verdict**: Matches paper exactly.

### Eqn 3: Growth with substrate limitation (G = sigma_g(C) * G_0)

**Paper**: G = sigma_g(C) * G_0
**Mock**: Lines 560–562.
```
carbon.growth = carbon.sigma_g * turgor.G_0
```
With `sigma_g` wired on line 542 via the `substrate_limitation` function.
**Verdict**: Matches paper exactly.

### Eqn 4: Maintenance respiration (R_M = sigma_r(C) * R_{M,0})

**Paper**: R_M = sigma_r(C) * R_{M,0}, where R_{M,0} is temperature-dependent.
**Mock**: Lines 564–567.
```
carbon.R_M = carbon.sigma_r
    * q10_response(carbon.R_M_25, carbon.q10_respiration, atm.temperature, 25)
```
This correctly applies the Q10 temperature scaling to R_{M,0} at 25C, then multiplies by the substrate limitation sigma_r.
**Verdict**: Matches paper. The Q10 formulation is consistent with Table 3 which lists separate Q10 values for stem and root maintenance respiration. The mock uses a single whole-tree Q10, which is a simplification the paper also makes at the whole-tree level.

### Eqn 5: sigma_g (growth substrate limitation)

**Paper**: sigma_g(C) = C / (C + gamma_g * C_struct)
**Mock**: Lines 113–123. The `substrate_limitation` function:
```
C / (C + gamma * C_struct)
```
Wired on line 542: `substrate_limitation(carbon.C, carbon.gamma_g, structure.C_struct)`.
**Verdict**: Matches paper exactly.

### Eqn 6: sigma_r (respiration substrate limitation)

**Paper**: sigma_r(C) = C / (C + gamma_r * C_struct)
**Mock**: Same `substrate_limitation` function, wired on line 545 with `carbon.gamma_r`.

**Issue**: The paper's Eqn 6 uses C_struct as the denominator scale, identical to Eqn 5. However, Table 3 says gamma_g = 0.26 and gamma_r = 0.38, and the paper text says "we include C_struct in Eqns 5 and 6 to signify tree size should influence potential NSC limitations and so sigma_g and sigma_r can be related to NSC concentrations rather than whole-tree NSC reserves." The mock correctly uses `structure.C_struct` for both.
**Verdict**: Matches paper exactly.

### Eqn 7: Lockhart growth integral (G_0 = phi_bar * (C_W / u_S) * integral of max(P_0(z_bar) - Gamma, 0) dz_bar)

**Paper**: G_0 = phi_bar * (C_W / u_S) * integral from 0 to 1 of max(P_0(z_bar) - Gamma, 0) dz_bar, where C_W is the stem biomass in carbon equivalents and u_S is the fraction of new growth allocated to stems.

**Mock**: Lines 403–409.
```
G_0 = phi * C_wood * mean_turgor_excess(turgor_base, turgor_apex, turgor_threshold)
```

**Issue — missing division by u_S**: The paper formula is G_0 = phi_bar * (C_W / u_S) * integral. The mock computes `phi * C_wood * mean_turgor_excess(...)` but does NOT divide by `u_S`. The field `u_s` is declared on line 407 (`u_s: Fraction`) but never used in the G_0 computation. The correct expression should be:
```
G_0 = phi * (C_wood / u_s) * mean_turgor_excess(turgor_base, turgor_apex, turgor_threshold)
```

**Verdict**: BUG. The u_S divisor is missing from the G_0 equation.

### Eqn 8: Marginal cost of water / optimality condition

**Paper**: Eqn 8(a) is the general marginal cost identity: chi_w = (1/(1-f_c)) * (1/a_t) * dG/dE. Eqn 8(b) specializes this using the Lockhart growth model. Eqn 8(c): chi_w = lambda.

These are the optimality conditions derived from the calculus of variations. They are not physics equations — they are the solution to the optimization problem (Eqn 1).

**Mock**: Not explicitly represented, which is correct. In Myco v2, the optimality condition is emergent from the slot training, not hardcoded. The slot `stomatal_control` is trained to maximize cumulative growth, and the optimal behavior should recover these conditions.
**Verdict**: Correctly omitted from the world model. This belongs in the workflow layer.

### Eqn 9: Lagrange multiplier dynamics (d_eta/dt)

**Paper**: d_eta/dt = eta * (d_sigma_r / dC) * R_{M,0} + (eta/(1-f_c) - 1) * (d_sigma_g/dC) * G_0

**Mock**: Not represented, which is correct for the same reason as Eqn 8. The Lagrange multiplier eta is part of the optimization solution, not the world model physics. It would emerge from the training dynamics of the stomatal slot.
**Verdict**: Correctly omitted.

### Osmotic potential relation (pi from phloem molality)

**Paper**: pi = -m_bar * rho * R * T_a * (c_{Pi,1} * m_p + c_{Pi,2} * m_p^2), where m_bar = 10^{-6} MPa Pa^{-1}.
**Mock**: Lines 381–382.
```
osmotic_potential = -1e-6 * water_density * gas_constant * temperature
    * (c_pi1 * phloem_molality + c_pi2 * phloem_molality ** 2)
```
**Verdict**: Matches paper. The `1e-6` corresponds to the m_bar unit conversion factor.

### Phloem molality relation (m_p from psi_S)

**Paper**: m_p = c_{m,1} - c_{m,2} * psi_S (Table 3 note, Paljakka et al. 2017)
**Mock**: Line 376.
```
phloem_molality = c_m1 - c_m2 * psi_stem
```
**Verdict**: Matches paper exactly.

### Hydraulic flow equations

**Paper** (from Fig 1 caption and text): Total transpiration E is conducted through soil, roots, stems, and leaves, given by their conductances k_R, k_S, and a_t * k_L. Specifically:
- E = k_R * (psi_soil - psi_R)
- E = k_S * (psi_R - psi_S - rho*g*H)
- E = a_t * k_L * (psi_S - psi_L)

**Mock**: Lines 458–477.
- root_flow (459–461): `transpiration = root_soil.conductance * (psi_soil - psi_root_junction)` — matches.
- stem_flow (465–467): `transpiration = stem.core.conductance * (psi_root_junction - turgor.psi_stem - structure.gravity_drop)` — matches.
- leaf_flow (470–473): `transpiration = structure.leaf_area * leaf.core.conductance * (turgor.psi_stem - leaf.core.water_potential)` — matches.
- transpiration_scaling (476–477): `transpiration = structure.leaf_area * gas.E_leaf` — matches.

**Verdict**: Matches paper.

### Energy balance

**Paper**: The paper references standard energy balance for leaf temperature (Notes S1, Supporting Information) but does not give an explicit equation in the main text.
**Mock**: Lines 512–520. Uses a linearized energy balance consistent with the Sperry mock. The radiation term uses `atm.insolation * leaf_props.emissivity` for absorbed shortwave, which differs slightly from the Sperry mock's `atm.r_abs` (absorbed radiation).

**Issue**: The mock multiplies insolation by emissivity for absorbed radiation (line 517: `atm.insolation * leaf_props.emissivity`). This is physically incorrect — shortwave absorptance is not the same as longwave emissivity. The paper's Table 3 lists epsilon = 0.97 as longwave emissivity, and I_s = 600 W/m^2 as incoming solar insolation. Absorbed shortwave should use an absorptance term (often ~ 0.5 for leaves), not emissivity. The paper likely absorbs a fraction of I_s determined by canopy structure and leaf optical properties, not by longwave emissivity.
**Verdict**: Likely incorrect absorptance term in energy balance. The insolation should be multiplied by a shortwave absorptance (or passed through a Beer-Lambert extinction), not by the longwave emissivity.

## Parameter Completeness (vs Table 3)

### Physical constants

| Symbol | Paper Value | Paper Units | Mock Status |
|--------|------------|-------------|-------------|
| C_p | 29.2 | J mol^{-1} K^{-1} | Imported as `specific_heat_air` from physics::constants — OK |
| g | 9.81 | m s^{-2} | Imported as `gravity` — OK |
| R | 8.314 | J mol^{-1} K^{-1} | Imported as `gas_constant` — OK |
| rho | 998 | kg m^{-3} | Imported as `water_density` — OK |
| sigma | 5.67e-8 | W m^{-2} K^{-4} | Imported as `stefan_boltzmann` — OK |

### Default environmental drivers

| Symbol | Paper Value | Paper Units | Mock Status |
|--------|------------|-------------|-------------|
| c_a | 4.10e-4 | mol mol^{-1} | Present as `atm.co2` (Pressure type in Pa, needs conversion) — field declared but value set in workflow |
| g_b | 2.4 | mol m^{-2} s^{-1} | **Issue**: paper treats g_b as a constant (2.4), but mock computes it from an empirical formula on line 530. See "boundary layer conductance" in Missing Physics. |
| I_s | 600 | W m^{-2} | Present as `atm.insolation` — OK |
| o_a | 2.07e-1 | mol mol^{-1} | Present as `atm.o2` — OK |
| P_atm | 101.325 | kPa | Present as `atm.pressure` — OK |
| RH | 0.4 | — | Present as `atm.rh` — OK |
| T_a | 25 | degC | Present as `atm.temperature` — OK |
| Phi (solar elevation) | 0 | rad | **MISSING** — not declared anywhere in mock. See Missing Physics. |
| psi_soil | 0 | MPa | Present as `atm.psi_soil` — OK |

### Structural metrics

| Symbol | Paper Value | Paper Units | Mock Status |
|--------|------------|-------------|-------------|
| a_L (leaf area) | 4.9 | m^2 | Present as `structure.leaf_area` — OK |
| C_struct | 1.13e3 | mol C | Present as `structure.C_struct` — OK |
| C_W | 8.3e2 | mol C | Present as `structure.C_wood` — OK |
| D (diameter) | 9.2 | cm | **MISSING** — not declared. Paper uses it for allometrics; mock treats structural metrics as direct inputs. Acceptable simplification. |
| H (height) | 14 | m | Present as `structure.height` — OK |
| W (canopy width) | 1.9 | m | Present as `structure.canopy_width` — OK (declared but unused in any equation) |
| Z (rooting depth) | 3 | m | Present as `structure.rooting_depth` — OK (declared but unused in any equation) |

### Light interception and photosynthesis

| Symbol | Paper Value | Paper Units | Mock Status |
|--------|------------|-------------|-------------|
| J_{max,17C} | 1.1e-4 | mol m^{-2} s^{-1} | Handled internally by FarquharC3 (j_max_25 + peaked Arrhenius) — OK, though the paper uses 17C as reference temperature while the mock uses 25C. This is fine since FarquharC3 uses its own temperature scaling. |
| K_c | 2.75e-4 | mol mol^{-1} | Handled by FarquharC3 — OK |
| K_o | 4.20e-1 | mol mol^{-1} | Handled by FarquharC3 — OK |
| V_{c,max,17C} | 6.1e-5 | mol m^{-2} s^{-1} | Handled by FarquharC3 — OK |
| Gamma_star | 3.60e-5 | mol mol^{-1} | Handled by FarquharC3 — OK |
| epsilon (emissivity) | 0.97 | — | Present as `leaf_props.emissivity` — OK |
| theta_c | 0.98 | — | Handled by FarquharC3 as `curvature_collab` — OK |
| theta_j | 0.90 | — | Handled by FarquharC3 as `curvature_light` — OK |
| kappa | 6.90e-7 | mol J^{-1} | **Partially present**: `leaf_props.kappa_L` is declared as a light extinction coefficient but the paper's kappa is a quantum yield conversion factor. The mock uses `kappa_L` to convert insolation to PAR on line 491: `gas.photo.par = leaf_props.kappa_L * atm.insolation`. This conflates the light extinction coefficient with the quantum yield. The paper actually uses kappa as the proportionality constant for J_l = kappa * I (Table 3). The naming is misleading but the physics intent is similar. |
| kappa_L (extinction coeff) | 0.5 | m^2 m^{-2} | See above — the mock declares `kappa_L` but uses it for the insolation-to-PAR conversion, not Beer-Lambert extinction. |

### Whole-tree carbon use

| Symbol | Paper Value | Paper Units | Mock Status |
|--------|------------|-------------|-------------|
| f_c | 0.28 | — | Present as `carbon.f_c` — OK |
| R_{M,0,25C} | 5.0e-5 | mol C s^{-1} | Present as `carbon.R_M_25` — OK |
| gamma_g | 0.26 | — | Present as `carbon.gamma_g` — OK |
| gamma_r | 0.38 | — | Present as `carbon.gamma_r` — OK |

### Turgor-limited growth

| Symbol | Paper Value | Paper Units | Mock Status |
|--------|------------|-------------|-------------|
| c_{m,1} | 0.48 | mol kg^{-1} | Present as `turgor.c_m1` — OK |
| c_{m,2} | 0.13 | mol kg^{-1} MPa^{-1} | Present as `turgor.c_m2` — OK |
| c_{Pi,1} | 0.998 | — | Present as `turgor.c_pi1` — OK |
| c_{Pi,2} | 0.089 | kg mol^{-1} | Present as `turgor.c_pi2` — OK |
| u_S | 0.25 | — | Declared as `turgor.u_s` on line 407 but **NOT USED** in the G_0 equation — BUG (see Eqn 7 above) |
| Gamma (turgor threshold) | 0.75 | MPa | Present as `turgor.turgor_threshold` — OK |
| phi_bar_25C | 4.6e-8 | MPa^{-1} s^{-1} | Present as `turgor.phi_25` — OK |

### Xylem hydraulics

| Symbol | Paper Value | Paper Units | Mock Status |
|--------|------------|-------------|-------------|
| k_L | 1.6e-2 | mol H2O m^{-2} s^{-1} MPa^{-1} | Present via `hydraulics.leaf` ConductingElement — OK |
| k_R | 2.30e-2 | mol s^{-1} MPa^{-1} | Present via `hydraulics.root_soil` ConductingElement — OK |
| k_S | 9.76e-2 | mol s^{-1} MPa^{-1} | Present via `hydraulics.stem` ConductingElement — OK |
| alpha_L | 1.5 | MPa^{-1} | Present via SigmoidVC slope parameter — OK |
| alpha_R | 3.5 | MPa^{-1} | Present via SigmoidVC slope parameter — OK |
| alpha_S | 0.8 | MPa^{-1} | Present via SigmoidVC slope parameter — OK |
| beta_L | -0.75 | MPa | Present via SigmoidVC p50 parameter — OK |
| beta_R | -1.1 | MPa | Present via SigmoidVC p50 parameter — OK |
| beta_S | -3.3 | MPa | Present via SigmoidVC p50 parameter — OK |

### Parameters for other models (comparison models, not GOSM)

| Symbol | Paper Value | Notes | Mock Status |
|--------|------------|-------|-------------|
| mu_w | 1e-3 | Cowan & Farquhar model | Not needed — correctly omitted |
| beta (Prentice) | 145 | Prentice et al. model | Not needed — correctly omitted |
| beta_1 | 4e-6 | Anderegg et al. model | Not needed — correctly omitted |
| beta_2, beta_3 | 0, 0 | Anderegg et al. model | Not needed — correctly omitted |

## Structural Issues

### Causal structure vs Fig. 1

Fig. 1 of the paper shows a conceptual diagram with:
- **Stomatal control** (top right) feeding carbon input and total transpiration
- **Whole-plant NSCs** (center) as the hub
- **Carbon output** processes: G + R_G + R_M (left)
- **Growth processes** affected by temperature, water potential, and NSC limitation (bottom)
- **Hydraulic catena**: Leaves -> Stem -> Roots -> Soil (right side)

The mock faithfully represents this structure:
- `GasExchange` (stomatal/photosynthesis) -> `CarbonBalance` (NSC) -> `TurgorGrowth` (growth)
- `HydraulicCatena` provides the water potential pathway
- Relations wire the carbon-water-turgor coupling

**Feedback loops properly represented:**
1. Carbon -> Water: NSC affects osmotic potential -> turgor -> growth demand. Present via `turgor.osmotic_potential` depending on `phloem_molality` which depends on `psi_stem`.
2. Water -> Carbon: Transpiration affects water potential, which affects photosynthesis (via stomatal conductance) and growth (via turgor). Present via hydraulic relations and turgor growth coupling.
3. Turgor feedback: Growth depends on turgor, turgor depends on water potential and osmotic potential, osmotic potential depends on NSC (indirectly through phloem molality from psi_S). **Partial issue**: the osmotic potential depends on psi_stem, not directly on C. In the paper, the phloem sap molality depends on psi_S (Eqn from Table 3: m_p = c_{m,1} - c_{m,2} * psi_S), which is an empirical relationship linking stem water status to phloem concentration. The carbon pool C affects growth and respiration demand but does NOT directly affect osmotic potential in this model — osmotic potential is driven by stem water potential. The mock correctly represents this.

**Structural concern — psi_soil placement**: `psi_soil` is placed on the `Atmosphere` node (line 210), which is semantically odd. Soil water potential is not an atmospheric quantity. However, for this single-soil-layer model, it serves as an environmental driver, and the paper treats it similarly (Table 3 lists it as an environmental driver). This is a modeling choice, not a bug.

## Undeclared / Phantom Fields

1. **`gas.leaf_water_potential`** (line 452): Referenced in `relation leaf_vc_pressure` but never declared as a field on `GasExchange`. The `GasExchange` node (lines 282–308) has no `leaf_water_potential` field. This is a bug — the leaf water potential needs to be either declared on GasExchange or the relation needs to reference the field from a different path (e.g., `hydraulics.leaf.core.water_potential` directly).

2. **`atm.min_psi_soil`** (lines 614–615): Referenced in `temporal soil_drought_memory` but never declared as a field on `Atmosphere`. The `Atmosphere` node (lines 202–211) has no `min_psi_soil` field. This field would need to be declared for the temporal block to work.

3. **`structure.canopy_width`** (line 226): Declared but never referenced in any relation or equation. Not harmful, but dead weight.

4. **`structure.rooting_depth`** (line 225): Declared but never referenced in any relation or equation. Not harmful, but dead weight.

## Spec Compliance Issues

1. **Inline relation syntax in nodes**: The mock uses bare `=` equations inside node bodies (e.g., line 233 `gravity_drop = 0.01 MPa_per_m * height`, line 297 `g_c = g_s * g_b / (g_s + g_b)`, line 347 `R_G = growth * f_c / (1 - f_c)`, etc.). Per the spec (section 6, line 784–798), both `constraint` and `relation` keywords can contain equalities, and bare equalities inside node bodies are also valid. The spec says "what matters is the form of the expression, not the keyword." **Verdict**: Compliant.

2. **Unit literal `MPa_per_m`** (line 233): The unit `MPa_per_m` is not imported in the `use units::si` block and is not a standard SI unit name. It would need to be either imported or defined. Minor issue — the intent is clear (0.01 MPa per meter of height = rho * g conversion), but the unit name is not declared.

3. **`if/then/else` in registered functions** (lines 181–194 in `mean_turgor_excess`): The spec (section 5.2, line 611–614) says conditional expressions produce piecewise expressions with "smooth approximation if needed for differentiability." The function declares `differentiability: subgradient`, which is appropriate for a piecewise function. **Verdict**: Compliant, and the differentiability annotation is correct.

4. **`temperature` field declared after use** (line 384): In `TurgorGrowth`, `temperature` is used on line 381 in the osmotic potential equation but declared on line 384. Per the spec, fields are not ordered — all fields are in scope within the node body. **Verdict**: Compliant (just a readability concern).

5. **Contract wiring pattern**: The mock wires photosynthesis inputs via individual relations (lines 487–500), which matches the spec's wiring pattern description (section 3.4, lines 397–412). **Verdict**: Compliant.

6. **Module-scope relations**: Lines 442–615 use module-scope relations referencing fields of the root node `PotkayTree`. Per the spec (section 2, lines 117–121), module-scope relations are implicitly scoped to the root node and can reference its fields without prefix. **Verdict**: Compliant.

## Missing Physics

### 1. R_G = G * f_c / (1 - f_c) relationship
**Status**: PRESENT. Line 347: `R_G = growth * f_c / (1 - f_c)`. This correctly captures the paper's statement that R_G is proportional to G such that their sum equals G/(1 - f_c), i.e., R_G = G * f_c / (1 - f_c).

### 2. Temperature dependence of extensibility (peaked Arrhenius with cold limit)
**Status**: PRESENT. The `extensibility_temperature` function (lines 139–165) implements a peaked Arrhenius with explicit cold limit at 5C (278.15 K). This matches the paper's description in Table 3: "phi_bar here continuously approaches zero as temperatures approach 5C."

### 3. Conductance hysteresis model
**Status**: PRESENT (partially). The mock includes irreversible cavitation tracking for stem (lines 601–604) and leaf (lines 606–609) via temporal min accumulators on `min_historical_pressure`. It also tracks `min_psi_soil` (lines 613–615) for drought legacy effects, though this field is undeclared (see Phantom Fields above). The `XylemSegment` nodes from the Sperry mock enforce `conductance <= max_conductance` based on historical minimum pressure.

**Missing detail**: The paper's conductance hysteresis (Figs 4d, 4h, 5) involves the interplay between `psi_soil^min` (minimum experienced soil water potential) and `psi_soil^rw` (post-rewatering soil potential). The mock tracks the minimum but does not explicitly model the rewatering/refilling dynamics. However, this may be emergent from the XylemSegment's irreversible cavitation constraint.

### 4. Boundary layer conductance formulation
**Status**: PRESENT but INCONSISTENT with paper. The mock computes g_b empirically from wind speed and leaf width (line 530: `g_b = 1.4 * 0.135 * sqrt(atm.wind_speed / leaf_props.width)`). However, the paper (Table 3) treats g_b as a constant parameter (g_b = 2.4 mol m^{-2} s^{-1}). The empirical computation is more physically realistic but departs from the paper's parameterization.

### 5. Solar elevation / radiation partitioning
**Status**: MISSING. The paper's Table 3 includes solar elevation angle Phi = 0 rad as a default parameter. The paper's Supporting Information (Notes S1) describes how absorbed radiation depends on solar elevation. The mock has no solar elevation field and no radiation partitioning — it uses raw `atm.insolation` multiplied by `kappa_L` for PAR, skipping the absorbed radiation computation entirely.

Specifically, the paper distinguishes between:
- Incoming solar insolation (I_s)
- Absorbed radiation (which depends on solar elevation, leaf area index, and optical properties)
- PAR (which is a fraction of total radiation)

The mock conflates these into a single `kappa_L * insolation` conversion, which loses the solar elevation dependence.

### 6. Explicit CO2 unit handling
**Minor issue**: The paper expresses CO2 as a mole fraction (mol mol^{-1}, e.g., c_a = 4.10e-4 mol mol^{-1}), while the mock types it as `Pressure` (Pa). The FarquharC3 contract from the Sperry mock uses `co2: Pressure` and `c_i: Pressure`, following the convention of partial pressure. This is internally consistent but differs from the paper's convention. The two are interconvertible via P_atm, and the FarquharC3 diffusion equation (line 215 of mock_sperry.myco) includes the atmospheric pressure division, so the physics works out.

### 7. Root-soil VC evaluation point
**Potential issue**: The paper describes root-soil conductance (k_R) evaluated at the soil water potential. The mock sets `hydraulics.root_soil.water_potential = atm.psi_soil` (line 444), which means the root-soil VC is evaluated at soil water potential. This is consistent with the paper's description of belowground conductance loss.

### 8. Turgor temperature
**Minor issue**: The mock wires `turgor.temperature = atm.temperature` (line 553), using air temperature for the osmotic potential and extensibility calculations. The paper uses T_a (air temperature) for this purpose (Table 3), so this is correct. However, in a more detailed model, stem temperature might differ from air temperature. The paper makes this same simplification.

## Import Consistency

The mock imports from two modules:

### `plant::hydraulics` (lines 29–34)
Imported items: `VulnerabilityCurve`, `SigmoidVC`, `ConductingElement`, `XylemSegment`

Cross-reference with mock_sperry.myco:
- `VulnerabilityCurve` — defined at line 85 of mock_sperry.myco as `pub contract` — OK
- `SigmoidVC` — defined at line 122 of mock_sperry.myco as `pub node` — OK
- `ConductingElement` — defined at line 331 of mock_sperry.myco as `pub node` — OK
- `XylemSegment` — defined at line 342 of mock_sperry.myco as `pub node` — OK

**Issue**: The mock_sperry.myco module path is `plant::sperry` (line 18), NOT `plant::hydraulics`. So importing from `plant::hydraulics` would fail — these items live in `plant::sperry`. For the import story to work, either:
(a) There would need to be a separate `plant::hydraulics` library module that re-exports these items, or
(b) The import path should be `plant::sperry`.

Similarly, `plant::photosynthesis` vs `plant::sperry`:

### `plant::photosynthesis` (lines 36–46)
Imported items: `Photosynthesis`, `FarquharC3`, `rubisco_limited`, `electron_transport_limited`, `electron_transport_rate`, `collatz_smooth_min`, `arrhenius`, `peaked_arrhenius`, `saturated_vapor_pressure`

Cross-reference with mock_sperry.myco:
- `Photosynthesis` — defined at line 96 as `pub contract` — OK
- `FarquharC3` — defined at line 155 as `pub node` — OK
- `rubisco_limited` — defined at line 224 as `pub fn` — OK
- `electron_transport_limited` — defined at line 240 as `pub fn` — OK
- `electron_transport_rate` — defined at line 253 as `pub fn` — OK
- `collatz_smooth_min` — defined at line 267 as `pub fn` — OK
- `arrhenius` — defined at line 280 as `pub fn` — OK
- `peaked_arrhenius` — defined at line 294 as `pub fn` — OK
- `saturated_vapor_pressure` — defined at line 313 as `pub fn` — OK

All items exist and are `pub`. But again, the module path is `plant::sperry`, not `plant::photosynthesis`.

**Note**: The mock's comments (lines 26–27) acknowledge this: "In a real setup, these come from library packages." The intent is that these would be factored into separate library modules. The import paths reflect the intended library architecture, not the current mock layout. This is a design aspiration, not a bug, but worth noting that the imports don't match the current single-module structure.

### Types not imported but used
The mock declares its own types (lines 85–103) but also references types from the Sperry mock (line 83 comment: "Reuse from library: WaterPotential, Conductance, HydraulicConductance, CarbonFlux, TranspirationRate, Temperature, Pressure, Fraction, PositiveScalar"). These are used throughout the mock but never imported. They would need to be imported from the library for this to compile. The comment acknowledges the intent but the actual `use` statements are missing.

## Summary

The mock is a generally faithful representation of the Potkay & Feng (2023) GOSM. The core carbon-water-turgor coupling (Eqns 2–7, 9), the substrate limitation functions (Eqns 5–6), the hydraulic catena, and the GOH slot structure are all correctly represented. The decision to express the optimization objective (Eqns 1, 8, 9) through the workflow layer rather than hardcoding it is a sound Myco v2 design choice.

**Critical issues:**
1. **Eqn 7 bug**: `u_s` (stem allocation fraction) is declared but not used in the G_0 equation. The division by `u_s` is missing.
2. **Phantom fields**: `gas.leaf_water_potential` and `atm.min_psi_soil` are referenced but never declared.

**Moderate issues:**
3. **Import paths** don't match the actual module structure (`plant::hydraulics` and `plant::photosynthesis` vs `plant::sperry`).
4. **Energy balance absorptance**: uses longwave emissivity where shortwave absorptance is needed.
5. **Solar elevation** is absent — the paper includes it as a parameter.
6. **Boundary layer conductance** is computed empirically rather than treated as a constant per Table 3.
7. **Type imports** from the Sperry library are commented but not actually imported.
8. **Undeclared unit** `MPa_per_m` on line 233.

**Minor issues:**
9. Unused declared fields (`canopy_width`, `rooting_depth`).
10. Light extinction coefficient naming confusion (`kappa_L` conflates extinction and quantum yield).

Overall, the mock captures roughly 85–90% of the paper's physics correctly and demonstrates the key Myco v2 patterns: contract reuse, slot-based optimization, temporal state, and carbon-water-turgor algebraic coupling. The two critical bugs (u_s omission and phantom fields) are straightforward to fix.
