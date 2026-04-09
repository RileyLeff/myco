use std::collections::{HashMap, HashSet};

use crate::{
    diagnostics::Diagnostic,
    equality::{EqualityEquation, EqualityModel, EqualitySlot, Quantity, QuantityId},
    syntax::QuantityKind,
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
    pub direct_bindings: Vec<DirectBindingSpec>,
    pub slot_bindings: Vec<SlotBindingSpec>,
    pub observations: Vec<ObservationSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectBindingSpec {
    pub quantity: String,
    pub kind: DirectBindingKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectBindingKind {
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
pub struct SlotBindingSpec {
    pub slot: String,
    pub kind: SlotBindingKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotBindingKind {
    DataSeries,
    Constant,
    Learned,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundModel {
    pub mode: CompileMode,
    pub horizon_steps: usize,
    pub quantities: Vec<BoundQuantity>,
    pub equations: Vec<EqualityEquation>,
    pub direct_bindings: Vec<ResolvedDirectBinding>,
    pub slot_bindings: Vec<ResolvedSlotBinding>,
    pub observations: Vec<ResolvedObservation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundQuantity {
    pub quantity: Quantity,
    pub direct_binding: Option<DirectBindingKind>,
    pub slot_provider: Option<String>,
    pub observed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedDirectBinding {
    pub quantity: QuantityId,
    pub kind: DirectBindingKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSlotBinding {
    pub slot: String,
    pub kind: SlotBindingKind,
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
        diagnostics.push(Diagnostic::error("compile horizon must be greater than zero"));
    }

    let quantity_index: HashMap<&str, &Quantity> = model
        .quantities
        .iter()
        .map(|quantity| (quantity.name.as_str(), quantity))
        .collect();
    let slot_index: HashMap<&str, &EqualitySlot> = model
        .slots
        .iter()
        .map(|slot| (slot.name.as_str(), slot))
        .collect();

    let mut direct_bindings = Vec::new();
    let mut direct_binding_by_quantity = HashMap::<QuantityId, DirectBindingKind>::new();

    for binding in &spec.direct_bindings {
        let Some(quantity) = quantity_index.get(binding.quantity.as_str()) else {
            diagnostics.push(Diagnostic::error(format!(
                "direct binding references unknown quantity '{}'",
                binding.quantity
            )));
            continue;
        };

        validate_direct_binding(quantity, &binding.kind, spec.horizon_steps, &mut diagnostics);

        if direct_binding_by_quantity
            .insert(quantity.id, binding.kind.clone())
            .is_some()
        {
            diagnostics.push(Diagnostic::error(format!(
                "quantity '{}' has multiple direct bindings",
                quantity.name
            )));
            continue;
        }

        direct_bindings.push(ResolvedDirectBinding {
            quantity: quantity.id,
            kind: binding.kind.clone(),
        });
    }

    let mut slot_bindings = Vec::new();
    let mut bound_slots = HashSet::new();
    let mut slot_provider_by_quantity = HashMap::<QuantityId, String>::new();

    for binding in &spec.slot_bindings {
        let Some(slot) = slot_index.get(binding.slot.as_str()) else {
            diagnostics.push(Diagnostic::error(format!(
                "slot binding references unknown slot '{}'",
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
            if direct_binding_by_quantity.contains_key(provided) {
                let quantity_name = &model.quantities[provided.0].name;
                diagnostics.push(Diagnostic::error(format!(
                    "quantity '{}' has both a direct binding and slot provider '{}'",
                    quantity_name, slot.name
                )));
            }

            if let Some(existing_slot) = slot_provider_by_quantity.insert(*provided, slot.name.clone()) {
                let quantity_name = &model.quantities[provided.0].name;
                diagnostics.push(Diagnostic::error(format!(
                    "quantity '{}' is provided by multiple slots ('{}' and '{}')",
                    quantity_name, existing_slot, slot.name
                )));
            }
        }

        slot_bindings.push(ResolvedSlotBinding {
            slot: slot.name.clone(),
            kind: binding.kind,
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

        validate_observation_schedule(&observation.schedule, spec.horizon_steps, &quantity.name, &mut diagnostics);
        observed_quantities.insert(quantity.id);
        observations.push(ResolvedObservation {
            quantity: quantity.id,
            loss: observation.loss,
            schedule: observation.schedule.clone(),
        });
    }

    for quantity in &model.quantities {
        if quantity.kind == QuantityKind::State
            && !matches!(
                direct_binding_by_quantity.get(&quantity.id),
                Some(DirectBindingKind::InitialState { .. })
            )
        {
            diagnostics.push(Diagnostic::error(format!(
                "state quantity '{}' requires an explicit initial-state binding",
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
            direct_binding: direct_binding_by_quantity.get(&quantity.id).cloned(),
            slot_provider: slot_provider_by_quantity.get(&quantity.id).cloned(),
            observed: observed_quantities.contains(&quantity.id),
            quantity,
        })
        .collect();

    Ok(BoundModel {
        mode: spec.mode,
        horizon_steps: spec.horizon_steps,
        quantities,
        equations: model.equations.clone(),
        direct_bindings,
        slot_bindings,
        observations,
    })
}

fn validate_direct_binding(
    quantity: &Quantity,
    kind: &DirectBindingKind,
    horizon_steps: usize,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match kind {
        DirectBindingKind::DataSeries { steps } => {
            validate_step_list(steps, horizon_steps, &quantity.name, "direct binding", diagnostics);
        }
        DirectBindingKind::Constant => {}
        DirectBindingKind::InitialState { .. } => {
            if quantity.kind != QuantityKind::State {
                diagnostics.push(Diagnostic::error(format!(
                    "quantity '{}' is not a state and cannot use an initial-state binding",
                    quantity.name
                )));
            }
        }
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
            validate_step_list(steps, horizon_steps, quantity_name, "observation", diagnostics);
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
            direct_bindings: vec![
                DirectBindingSpec {
                    quantity: "vpd_scale".to_string(),
                    kind: DirectBindingKind::DataSeries {
                        steps: (0..24).collect(),
                    },
                },
                DirectBindingSpec {
                    quantity: "soil_water".to_string(),
                    kind: DirectBindingKind::DataSeries {
                        steps: (0..24).collect(),
                    },
                },
                DirectBindingSpec {
                    quantity: "hydraulic_cond".to_string(),
                    kind: DirectBindingKind::Constant,
                },
                DirectBindingSpec {
                    quantity: "g_max".to_string(),
                    kind: DirectBindingKind::Constant,
                },
                DirectBindingSpec {
                    quantity: "water".to_string(),
                    kind: DirectBindingKind::InitialState {
                        source: InitialStateSource::Constant,
                    },
                },
                DirectBindingSpec {
                    quantity: "carbon".to_string(),
                    kind: DirectBindingKind::InitialState {
                        source: InitialStateSource::Constant,
                    },
                },
            ],
            slot_bindings: vec![SlotBindingSpec {
                slot: "controller".to_string(),
                kind: SlotBindingKind::Learned,
            }],
            observations: vec![ObservationSpec {
                quantity: "transpiration".to_string(),
                loss: LossKind::Mse,
                schedule: ObservationSchedule::DensePerStep,
            }],
        };

        let bound = bind_compile_spec(&model, &spec).expect("binding should succeed");

        assert_eq!(bound.horizon_steps, 24);
        assert_eq!(bound.direct_bindings.len(), 6);
        assert_eq!(bound.slot_bindings.len(), 1);
        assert_eq!(bound.slot_bindings[0].input_arity, 6);
        assert_eq!(bound.slot_bindings[0].output_arity, 1);
        assert_eq!(bound.observations.len(), 1);

        let stomata = bound
            .quantities
            .iter()
            .find(|quantity| quantity.quantity.name == "stomata")
            .expect("stomata should exist");
        assert_eq!(stomata.slot_provider.as_deref(), Some("controller"));
        assert!(stomata.direct_binding.is_none());

        let transpiration = bound
            .quantities
            .iter()
            .find(|quantity| quantity.quantity.name == "transpiration")
            .expect("transpiration should exist");
        assert!(transpiration.observed);
    }

    #[test]
    fn rejects_multiple_direct_bindings_for_same_quantity() {
        let model = tiny_tree_equality_model();
        let spec = CompileSpec {
            mode: CompileMode::Simulate,
            horizon_steps: 4,
            direct_bindings: vec![
                DirectBindingSpec {
                    quantity: "hydraulic_cond".to_string(),
                    kind: DirectBindingKind::Constant,
                },
                DirectBindingSpec {
                    quantity: "hydraulic_cond".to_string(),
                    kind: DirectBindingKind::Constant,
                },
                initial_state("water"),
                initial_state("carbon"),
            ],
            slot_bindings: vec![],
            observations: vec![],
        };

        let diagnostics = bind_compile_spec(&model, &spec).expect_err("binding should fail");
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("multiple direct bindings")));
    }

    #[test]
    fn rejects_missing_initial_state_bindings() {
        let model = tiny_tree_equality_model();
        let spec = CompileSpec {
            mode: CompileMode::Simulate,
            horizon_steps: 4,
            direct_bindings: vec![initial_state("water")],
            slot_bindings: vec![],
            observations: vec![],
        };

        let diagnostics = bind_compile_spec(&model, &spec).expect_err("binding should fail");
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic
                .message
                .contains("state quantity 'carbon' requires an explicit initial-state binding")
        }));
    }

    #[test]
    fn rejects_observation_steps_outside_horizon() {
        let model = tiny_tree_equality_model();
        let spec = CompileSpec {
            mode: CompileMode::Fit,
            horizon_steps: 3,
            direct_bindings: vec![initial_state("water"), initial_state("carbon")],
            slot_bindings: vec![],
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
    fn rejects_direct_binding_and_slot_provider_conflict() {
        let model = tiny_tree_equality_model();
        let spec = CompileSpec {
            mode: CompileMode::Train,
            horizon_steps: 6,
            direct_bindings: vec![
                DirectBindingSpec {
                    quantity: "stomata".to_string(),
                    kind: DirectBindingKind::Constant,
                },
                initial_state("water"),
                initial_state("carbon"),
            ],
            slot_bindings: vec![SlotBindingSpec {
                slot: "controller".to_string(),
                kind: SlotBindingKind::Learned,
            }],
            observations: vec![],
        };

        let diagnostics = bind_compile_spec(&model, &spec).expect_err("binding should fail");
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic
                .message
                .contains("quantity 'stomata' has both a direct binding and slot provider 'controller'")
        }));
    }

    fn tiny_tree_equality_model() -> EqualityModel {
        let syntax = parse_and_validate(TINY_TREE).expect("fixture should validate");
        let semantic = semantic::lower_model(&syntax).expect("semantic lowering should succeed");
        equality::lower_model(&semantic).expect("equality lowering should succeed")
    }

    fn initial_state(quantity: &str) -> DirectBindingSpec {
        DirectBindingSpec {
            quantity: quantity.to_string(),
            kind: DirectBindingKind::InitialState {
                source: InitialStateSource::Constant,
            },
        }
    }
}
