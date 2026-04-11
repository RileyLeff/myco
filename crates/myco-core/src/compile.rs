use std::collections::{HashMap, HashSet};

use crate::{
    diagnostics::Diagnostic,
    equality::{EqualityModel, EqualitySlot, Quantity, QuantityId},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompileMode {
    Simulate,
    Fit,
    Train,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileSpec {
    pub mode: CompileMode,
    pub horizon_steps: usize,
    pub consistency_policy: ConsistencyPolicy,
    pub assumptions: Vec<AssumptionSpec>,
    pub learned_slots: Vec<LearnedSlotSpec>,
    pub observations: Vec<ObservationSpec>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsistencyPolicy {
    Off,
    EquationOnly,
    All,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssumptionSpec {
    pub quantity: String,
    pub kind: AssumptionKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssumptionKind {
    DataSeries { steps: Vec<usize> },
    Constant,
    InitialState { source: InitialStateSource },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitialStateSource {
    Constant,
    Data,
    Learned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnedSlotSpec {
    pub slot: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationSpec {
    pub quantity: String,
    pub loss: LossKind,
    pub schedule: ObservationSchedule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LossKind {
    Mse,
    Huber,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObservationSchedule {
    DensePerStep,
    Sparse(Vec<usize>),
}

#[derive(Debug, Clone)]
pub struct BoundModel {
    pub mode: CompileMode,
    pub horizon_steps: usize,
    pub consistency_policy: ConsistencyPolicy,
    pub quantities: Vec<BoundQuantity>,
    pub core: std::sync::Arc<crate::egraph::EqualityCore>,
    pub assumptions: Vec<ResolvedAssumption>,
    pub learned_slots: Vec<ResolvedLearnedSlot>,
    pub observations: Vec<ResolvedObservation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundQuantity {
    pub quantity: Quantity,
    pub persistent: bool,
    pub assumption: Option<AssumptionKind>,
    pub slot_provider: Option<String>,
    pub observed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedAssumption {
    pub quantity: QuantityId,
    pub kind: AssumptionKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedLearnedSlot {
    pub slot: String,
    pub provides: Vec<QuantityId>,
    pub inputs: Vec<QuantityId>,
    pub input_arity: usize,
    pub output_arity: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedObservation {
    pub quantity: QuantityId,
    pub loss: LossKind,
    pub schedule: ObservationSchedule,
}

pub fn bind_compile_spec(
    model: &EqualityModel,
    spec: &CompileSpec,
) -> Result<BoundModel, Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();

    if spec.horizon_steps == 0 {
        diagnostics.push(Diagnostic::error(
            "compile horizon must be greater than zero",
        ));
    }

    validate_mode_requirements(spec, &mut diagnostics);

    let quantity_index: HashMap<&str, &Quantity> = model
        .quantities
        .iter()
        .map(|quantity| (quantity.name.as_str(), quantity))
        .collect();
    let inferred_persistent_quantities = model
        .persistent_quantities
        .iter()
        .copied()
        .collect::<HashSet<_>>();
    let slot_index: HashMap<&str, &EqualitySlot> = model
        .slots
        .iter()
        .map(|slot| (slot.name.as_str(), slot))
        .collect();

    let mut assumptions = Vec::new();
    let mut assumption_by_quantity = HashMap::<QuantityId, AssumptionKind>::new();

    for binding in &spec.assumptions {
        let Some(quantity) = quantity_index.get(binding.quantity.as_str()) else {
            diagnostics.push(Diagnostic::error(format!(
                "assumption references unknown quantity '{}'",
                binding.quantity
            )));
            continue;
        };

        validate_assumption(
            quantity,
            &binding.kind,
            spec.horizon_steps,
            &mut diagnostics,
        );

        if assumption_by_quantity
            .insert(quantity.id, binding.kind.clone())
            .is_some()
        {
            diagnostics.push(Diagnostic::error(format!(
                "quantity '{}' has multiple assumptions",
                quantity.name
            )));
            continue;
        }

        assumptions.push(ResolvedAssumption {
            quantity: quantity.id,
            kind: binding.kind.clone(),
        });
    }

    let mut learned_slots = Vec::new();
    let mut bound_slots = HashSet::new();
    let mut slot_provider_by_quantity = HashMap::<QuantityId, String>::new();

    for binding in &spec.learned_slots {
        let Some(slot) = slot_index.get(binding.slot.as_str()) else {
            diagnostics.push(Diagnostic::error(format!(
                "learned slot references unknown slot '{}'",
                binding.slot
            )));
            continue;
        };

        if !bound_slots.insert(slot.name.clone()) {
            diagnostics.push(Diagnostic::error(format!(
                "slot '{}' is bound more than once",
                slot.name
            )));
            continue;
        }

        for provided in &slot.provides {
            if assumption_by_quantity.contains_key(provided) {
                let quantity_name = &model.quantities[provided.0].name;
                diagnostics.push(Diagnostic::error(format!(
                    "quantity '{}' has both an assumption and learned slot '{}'",
                    quantity_name, slot.name
                )));
            }

            if let Some(existing_slot) =
                slot_provider_by_quantity.insert(*provided, slot.name.clone())
            {
                let quantity_name = &model.quantities[provided.0].name;
                diagnostics.push(Diagnostic::error(format!(
                    "quantity '{}' is provided by multiple slots ('{}' and '{}')",
                    quantity_name, existing_slot, slot.name
                )));
            }
        }

        learned_slots.push(ResolvedLearnedSlot {
            slot: slot.name.clone(),
            provides: slot.provides.clone(),
            inputs: slot.inputs.clone(),
            input_arity: slot.inputs.len(),
            output_arity: slot.provides.len(),
        });
    }

    let mut observations = Vec::new();
    let mut observed_quantities = HashSet::new();

    for observation in &spec.observations {
        let Some(quantity) = quantity_index.get(observation.quantity.as_str()) else {
            diagnostics.push(Diagnostic::error(format!(
                "observation references unknown quantity '{}'",
                observation.quantity
            )));
            continue;
        };

        validate_observation_schedule(
            &observation.schedule,
            spec.horizon_steps,
            &quantity.name,
            &mut diagnostics,
        );
        observed_quantities.insert(quantity.id);
        observations.push(ResolvedObservation {
            quantity: quantity.id,
            loss: observation.loss,
            schedule: observation.schedule.clone(),
        });
    }

    let workflow_persistent_quantities = model
        .persistent_quantities
        .iter()
        .copied()
        .chain(assumptions.iter().filter_map(|binding| match binding.kind {
            AssumptionKind::InitialState { .. } => Some(binding.quantity),
            _ => None,
        }))
        .collect::<HashSet<_>>();

    for quantity in &model.quantities {
        if inferred_persistent_quantities.contains(&quantity.id)
            && !matches!(
                assumption_by_quantity.get(&quantity.id),
                Some(AssumptionKind::InitialState { .. })
            )
        {
            diagnostics.push(Diagnostic::error(format!(
                "persistent quantity '{}' requires an explicit initial-state binding",
                quantity.name
            )));
        }
    }

    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }

    let quantities = model
        .quantities
        .iter()
        .cloned()
        .map(|quantity| BoundQuantity {
            persistent: workflow_persistent_quantities.contains(&quantity.id),
            assumption: assumption_by_quantity.get(&quantity.id).cloned(),
            slot_provider: slot_provider_by_quantity.get(&quantity.id).cloned(),
            observed: observed_quantities.contains(&quantity.id),
            quantity,
        })
        .collect();

    Ok(BoundModel {
        mode: spec.mode,
        horizon_steps: spec.horizon_steps,
        consistency_policy: spec.consistency_policy,
        quantities,
        core: std::sync::Arc::clone(&model.core),
        assumptions,
        learned_slots,
        observations,
    })
}

fn validate_mode_requirements(spec: &CompileSpec, diagnostics: &mut Vec<Diagnostic>) {
    match spec.mode {
        CompileMode::Simulate => {}
        CompileMode::Fit => {
            if spec.observations.is_empty() {
                diagnostics.push(Diagnostic::error(
                    "fit mode requires at least one observation",
                ));
            }
        }
        CompileMode::Train => {
            if spec.observations.is_empty() {
                diagnostics.push(Diagnostic::error(
                    "train mode requires at least one observation",
                ));
            }

            let has_learned_slot = !spec.learned_slots.is_empty();
            let has_learned_initial_state = spec.assumptions.iter().any(|binding| {
                matches!(
                    binding.kind,
                    AssumptionKind::InitialState {
                        source: InitialStateSource::Learned
                    }
                )
            });

            if !has_learned_slot && !has_learned_initial_state {
                diagnostics.push(Diagnostic::error(
                    "train mode requires at least one learned slot or learned initial state",
                ));
            }
        }
    }
}

fn validate_assumption(
    quantity: &Quantity,
    kind: &AssumptionKind,
    horizon_steps: usize,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match kind {
        AssumptionKind::DataSeries { steps } => {
            validate_step_list(
                steps,
                horizon_steps,
                &quantity.name,
                "assumption",
                diagnostics,
            );
            let expected = (0..horizon_steps).collect::<HashSet<_>>();
            let actual = steps.iter().copied().collect::<HashSet<_>>();
            if actual != expected {
                diagnostics.push(Diagnostic::error(format!(
                    "v1 data-series binding for '{}' must cover every step in the compile horizon",
                    quantity.name
                )));
            }
        }
        AssumptionKind::Constant => {}
        AssumptionKind::InitialState { .. } => {}
    }
}

fn validate_observation_schedule(
    schedule: &ObservationSchedule,
    horizon_steps: usize,
    quantity_name: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match schedule {
        ObservationSchedule::DensePerStep => {}
        ObservationSchedule::Sparse(steps) => {
            validate_step_list(
                steps,
                horizon_steps,
                quantity_name,
                "observation",
                diagnostics,
            );
        }
    }
}

fn validate_step_list(
    steps: &[usize],
    horizon_steps: usize,
    quantity_name: &str,
    context: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut seen = HashSet::new();
    for step in steps {
        if *step >= horizon_steps {
            diagnostics.push(Diagnostic::error(format!(
                "{} for '{}' references step {} outside horizon {}",
                context, quantity_name, step, horizon_steps
            )));
        }
        if !seen.insert(*step) {
            diagnostics.push(Diagnostic::error(format!(
                "{} for '{}' contains duplicate step {}",
                context, quantity_name, step
            )));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{equality, semantic, syntax::parse_and_validate};

    const TINY_TREE: &str = include_str!("../tests/fixtures/tiny_tree.myco");

    #[test]
    fn binds_valid_train_spec() {
        let model = tiny_tree_equality_model();
        let spec = CompileSpec {
            mode: CompileMode::Train,
            horizon_steps: 24,
            consistency_policy: ConsistencyPolicy::EquationOnly,
            assumptions: vec![
                AssumptionSpec {
                    quantity: "vpd_scale".to_string(),
                    kind: AssumptionKind::DataSeries {
                        steps: (0..24).collect(),
                    },
                },
                AssumptionSpec {
                    quantity: "soil_water".to_string(),
                    kind: AssumptionKind::DataSeries {
                        steps: (0..24).collect(),
                    },
                },
                AssumptionSpec {
                    quantity: "hydraulic_cond".to_string(),
                    kind: AssumptionKind::Constant,
                },
                AssumptionSpec {
                    quantity: "g_max".to_string(),
                    kind: AssumptionKind::Constant,
                },
                AssumptionSpec {
                    quantity: "water".to_string(),
                    kind: AssumptionKind::InitialState {
                        source: InitialStateSource::Constant,
                    },
                },
                AssumptionSpec {
                    quantity: "carbon".to_string(),
                    kind: AssumptionKind::InitialState {
                        source: InitialStateSource::Constant,
                    },
                },
            ],
            learned_slots: vec![LearnedSlotSpec {
                slot: "controller".to_string(),
            }],
            observations: vec![ObservationSpec {
                quantity: "transpiration".to_string(),
                loss: LossKind::Mse,
                schedule: ObservationSchedule::DensePerStep,
            }],
        };

        let bound = bind_compile_spec(&model, &spec).expect("binding should succeed");

        assert_eq!(bound.horizon_steps, 24);
        assert_eq!(bound.assumptions.len(), 6);
        assert_eq!(bound.learned_slots.len(), 1);
        assert_eq!(bound.learned_slots[0].input_arity, 6);
        assert_eq!(bound.learned_slots[0].output_arity, 1);
        assert_eq!(bound.observations.len(), 1);

        let stomata = bound
            .quantities
            .iter()
            .find(|quantity| quantity.quantity.name == "stomata")
            .expect("stomata should exist");
        assert_eq!(stomata.slot_provider.as_deref(), Some("controller"));
        assert!(stomata.assumption.is_none());
        assert!(!stomata.persistent);

        let transpiration = bound
            .quantities
            .iter()
            .find(|quantity| quantity.quantity.name == "transpiration")
            .expect("transpiration should exist");
        assert!(transpiration.observed);

        let water = bound
            .quantities
            .iter()
            .find(|quantity| quantity.quantity.name == "water")
            .expect("water should exist");
        assert!(water.persistent);
    }

    #[test]
    fn rejects_multiple_assumptions_for_same_quantity() {
        let model = tiny_tree_equality_model();
        let spec = CompileSpec {
            mode: CompileMode::Simulate,
            horizon_steps: 4,
            consistency_policy: ConsistencyPolicy::EquationOnly,
            assumptions: vec![
                AssumptionSpec {
                    quantity: "hydraulic_cond".to_string(),
                    kind: AssumptionKind::Constant,
                },
                AssumptionSpec {
                    quantity: "hydraulic_cond".to_string(),
                    kind: AssumptionKind::Constant,
                },
                initial_state("water"),
                initial_state("carbon"),
            ],
            learned_slots: vec![],
            observations: vec![],
        };

        let diagnostics = bind_compile_spec(&model, &spec).expect_err("binding should fail");
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.contains("multiple assumptions"))
        );
    }

    #[test]
    fn rejects_sparse_direct_data_series_in_v1() {
        let model = tiny_tree_equality_model();
        let spec = CompileSpec {
            mode: CompileMode::Simulate,
            horizon_steps: 4,
            consistency_policy: ConsistencyPolicy::EquationOnly,
            assumptions: vec![
                AssumptionSpec {
                    quantity: "vpd_scale".to_string(),
                    kind: AssumptionKind::DataSeries {
                        steps: vec![0, 2, 3],
                    },
                },
                AssumptionSpec {
                    quantity: "soil_water".to_string(),
                    kind: AssumptionKind::DataSeries {
                        steps: (0..4).collect(),
                    },
                },
                AssumptionSpec {
                    quantity: "hydraulic_cond".to_string(),
                    kind: AssumptionKind::Constant,
                },
                AssumptionSpec {
                    quantity: "g_max".to_string(),
                    kind: AssumptionKind::Constant,
                },
                initial_state("water"),
                initial_state("carbon"),
            ],
            learned_slots: vec![LearnedSlotSpec {
                slot: "controller".to_string(),
            }],
            observations: Vec::new(),
        };

        let diagnostics = bind_compile_spec(&model, &spec).expect_err("binding should fail");
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic
                .message
                .contains("must cover every step in the compile horizon")
        }));
    }

    #[test]
    fn rejects_missing_initial_state_bindings() {
        let model = tiny_tree_equality_model();
        let spec = CompileSpec {
            mode: CompileMode::Simulate,
            horizon_steps: 4,
            consistency_policy: ConsistencyPolicy::EquationOnly,
            assumptions: vec![],
            learned_slots: vec![],
            observations: vec![],
        };

        let diagnostics = bind_compile_spec(&model, &spec).expect_err("binding should fail");
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic
                .message
                .contains("persistent quantity 'water' requires an explicit initial-state binding")
        }));
    }

    #[test]
    fn requires_initial_state_for_temporal_node_even_without_state_keyword() {
        let source = r#"
model TemporalNode

quantity stock : scalar
quantity forcing : scalar

temporal stock_step:
  stock[t+1] = stock[t] + forcing[t]
"#;

        let syntax = parse_and_validate(source).expect("syntax should validate");
        let semantic = semantic::lower_model(&syntax).expect("semantic lowering should succeed");
        let equality = equality::lower_model(&semantic).expect("equality lowering should succeed");
        let spec = CompileSpec {
            mode: CompileMode::Simulate,
            horizon_steps: 4,
            consistency_policy: ConsistencyPolicy::EquationOnly,
            assumptions: vec![AssumptionSpec {
                quantity: "forcing".to_string(),
                kind: AssumptionKind::DataSeries {
                    steps: (0..4).collect(),
                },
            }],
            learned_slots: vec![],
            observations: vec![],
        };

        let diagnostics = bind_compile_spec(&equality, &spec).expect_err("binding should fail");
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic
                .message
                .contains("persistent quantity 'stock' requires an explicit initial-state binding")
        }));
    }

    #[test]
    fn initial_state_binding_makes_node_persistent_in_workflow() {
        let source = r#"
model WorkflowPersistent

quantity latent_store : scalar
quantity forcing : scalar

relation passthrough:
  forcing = forcing
"#;

        let syntax = parse_and_validate(source).expect("syntax should validate");
        let semantic = semantic::lower_model(&syntax).expect("semantic lowering should succeed");
        let equality = equality::lower_model(&semantic).expect("equality lowering should succeed");
        let spec = CompileSpec {
            mode: CompileMode::Simulate,
            horizon_steps: 4,
            consistency_policy: ConsistencyPolicy::EquationOnly,
            assumptions: vec![
                AssumptionSpec {
                    quantity: "forcing".to_string(),
                    kind: AssumptionKind::DataSeries {
                        steps: (0..4).collect(),
                    },
                },
                initial_state("latent_store"),
            ],
            learned_slots: vec![],
            observations: vec![],
        };

        let bound = bind_compile_spec(&equality, &spec).expect("binding should succeed");
        let latent_store = bound
            .quantities
            .iter()
            .find(|quantity| quantity.quantity.name == "latent_store")
            .expect("latent_store should exist");
        assert!(latent_store.persistent);
    }

    #[test]
    fn rejects_observation_steps_outside_horizon() {
        let model = tiny_tree_equality_model();
        let spec = CompileSpec {
            mode: CompileMode::Fit,
            horizon_steps: 3,
            consistency_policy: ConsistencyPolicy::EquationOnly,
            assumptions: vec![initial_state("water"), initial_state("carbon")],
            learned_slots: vec![],
            observations: vec![ObservationSpec {
                quantity: "transpiration".to_string(),
                loss: LossKind::Huber,
                schedule: ObservationSchedule::Sparse(vec![0, 3]),
            }],
        };

        let diagnostics = bind_compile_spec(&model, &spec).expect_err("binding should fail");
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic
                .message
                .contains("observation for 'transpiration' references step 3 outside horizon 3")
        }));
    }

    #[test]
    fn rejects_assumption_and_learned_slot_conflict() {
        let model = tiny_tree_equality_model();
        let spec = CompileSpec {
            mode: CompileMode::Train,
            horizon_steps: 6,
            consistency_policy: ConsistencyPolicy::EquationOnly,
            assumptions: vec![
                AssumptionSpec {
                    quantity: "stomata".to_string(),
                    kind: AssumptionKind::Constant,
                },
                initial_state("water"),
                initial_state("carbon"),
            ],
            learned_slots: vec![LearnedSlotSpec {
                slot: "controller".to_string(),
            }],
            observations: vec![],
        };

        let diagnostics = bind_compile_spec(&model, &spec).expect_err("binding should fail");
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic
                .message
                .contains("quantity 'stomata' has both an assumption and learned slot 'controller'")
        }));
    }

    #[test]
    fn fit_mode_requires_observation() {
        let model = tiny_tree_equality_model();
        let spec = CompileSpec {
            mode: CompileMode::Fit,
            horizon_steps: 6,
            consistency_policy: ConsistencyPolicy::EquationOnly,
            assumptions: vec![initial_state("water"), initial_state("carbon")],
            learned_slots: vec![],
            observations: vec![],
        };

        let diagnostics = bind_compile_spec(&model, &spec).expect_err("binding should fail");
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic
                .message
                .contains("fit mode requires at least one observation")
        }));
    }

    #[test]
    fn train_mode_requires_observation_and_learned_component() {
        let model = tiny_tree_equality_model();
        let no_observation = CompileSpec {
            mode: CompileMode::Train,
            horizon_steps: 6,
            consistency_policy: ConsistencyPolicy::EquationOnly,
            assumptions: vec![
                AssumptionSpec {
                    quantity: "g_max".to_string(),
                    kind: AssumptionKind::Constant,
                },
                initial_state("water"),
                initial_state("carbon"),
            ],
            learned_slots: vec![LearnedSlotSpec {
                slot: "controller".to_string(),
            }],
            observations: vec![],
        };

        let diagnostics =
            bind_compile_spec(&model, &no_observation).expect_err("binding should fail");
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic
                .message
                .contains("train mode requires at least one observation")
        }));

        let no_learned = CompileSpec {
            mode: CompileMode::Train,
            horizon_steps: 6,
            consistency_policy: ConsistencyPolicy::EquationOnly,
            assumptions: vec![
                AssumptionSpec {
                    quantity: "g_max".to_string(),
                    kind: AssumptionKind::Constant,
                },
                initial_state("water"),
                initial_state("carbon"),
            ],
            learned_slots: vec![],
            observations: vec![ObservationSpec {
                quantity: "transpiration".to_string(),
                loss: LossKind::Mse,
                schedule: ObservationSchedule::DensePerStep,
            }],
        };

        let diagnostics = bind_compile_spec(&model, &no_learned).expect_err("binding should fail");
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic
                .message
                .contains("train mode requires at least one learned slot or learned initial state")
        }));
    }

    fn tiny_tree_equality_model() -> EqualityModel {
        let syntax = parse_and_validate(TINY_TREE).expect("fixture should validate");
        let semantic = semantic::lower_model(&syntax).expect("semantic lowering should succeed");
        equality::lower_model(&semantic).expect("equality lowering should succeed")
    }

    fn initial_state(quantity: &str) -> AssumptionSpec {
        AssumptionSpec {
            quantity: quantity.to_string(),
            kind: AssumptionKind::InitialState {
                source: InitialStateSource::Constant,
            },
        }
    }
}
