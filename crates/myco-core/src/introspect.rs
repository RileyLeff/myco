use crate::{
    compile::DirectBindingKind,
    diagnostics::Diagnostic,
    pipeline::PreparedExperiment,
    plan::{
        AlternativePath, BlockedCandidate, CandidateDirection, Dependency, DependencyTiming,
        EquationDirection, PlanSource, PlannedEquation, PlannedSlot,
    },
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlanExplanation {
    pub available_current: Vec<String>,
    pub chosen_current: Vec<PathExplanation>,
    pub chosen_temporal: Vec<PathExplanation>,
    pub alternatives: Vec<AlternativeExplanation>,
    pub unresolved: Vec<UnresolvedQuantityExplanation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct QuantityExplanation {
    pub quantity: String,
    pub direct_binding: Option<String>,
    pub slot_provider: Option<String>,
    pub observed: bool,
    pub chosen_current: Option<PathExplanation>,
    pub chosen_temporal: Option<PathExplanation>,
    pub alternatives: Vec<AlternativeExplanation>,
    pub blocked_candidates: Vec<BlockedCandidateExplanation>,
    pub unresolved: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PathExplanation {
    pub output: String,
    pub source: String,
    pub direction: String,
    pub cost: u32,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AlternativeExplanation {
    pub output: String,
    pub source: String,
    pub direction: String,
    pub cost: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UnresolvedQuantityExplanation {
    pub quantity: String,
    pub blocked_candidates: Vec<BlockedCandidateExplanation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BlockedCandidateExplanation {
    pub output: String,
    pub source: String,
    pub direction: String,
    pub cost: u32,
    pub missing_dependencies: Vec<String>,
}

pub fn explain_plan(experiment: &PreparedExperiment) -> PlanExplanation {
    let available_current = experiment
        .plan
        .available_current
        .iter()
        .map(|quantity| quantity_name(experiment, *quantity))
        .collect::<Vec<_>>();

    let chosen_current = experiment
        .plan
        .slot_steps
        .iter()
        .map(|slot| explain_slot(experiment, slot))
        .chain(
            experiment
                .plan
                .equation_steps
                .iter()
                .map(|equation| explain_equation(experiment, equation)),
        )
        .collect::<Vec<_>>();

    let chosen_temporal = experiment
        .plan
        .temporal_steps
        .iter()
        .map(|equation| explain_equation(experiment, equation))
        .collect::<Vec<_>>();

    let alternatives = experiment
        .plan
        .alternatives
        .iter()
        .map(|alternative| explain_alternative(experiment, alternative))
        .collect::<Vec<_>>();

    let unresolved = experiment
        .bound
        .quantities
        .iter()
        .map(|bound_quantity| bound_quantity.quantity.id)
        .filter(|quantity| is_unresolved(experiment, *quantity))
        .map(|quantity| UnresolvedQuantityExplanation {
            quantity: quantity_name(experiment, quantity),
            blocked_candidates: blocked_candidates_for(experiment, quantity),
        })
        .collect::<Vec<_>>();

    PlanExplanation {
        available_current,
        chosen_current,
        chosen_temporal,
        alternatives,
        unresolved,
    }
}

pub fn explain_quantity(
    experiment: &PreparedExperiment,
    quantity_name_to_find: &str,
) -> Result<QuantityExplanation, Vec<Diagnostic>> {
    let Some(bound_quantity) = experiment
        .bound
        .quantities
        .iter()
        .find(|quantity| quantity.quantity.name == quantity_name_to_find)
    else {
        return Err(vec![Diagnostic::error(format!(
            "unknown quantity '{quantity_name_to_find}'"
        ))]);
    };

    let quantity = bound_quantity.quantity.id;

    Ok(QuantityExplanation {
        quantity: quantity_name_to_find.to_string(),
        direct_binding: bound_quantity
            .direct_binding
            .as_ref()
            .map(direct_binding_name),
        slot_provider: bound_quantity.slot_provider.clone(),
        observed: bound_quantity.observed,
        chosen_current: chosen_current_for(experiment, quantity),
        chosen_temporal: chosen_temporal_for(experiment, quantity),
        alternatives: experiment
            .plan
            .alternatives
            .iter()
            .filter(|alternative| alternative.output == quantity)
            .map(|alternative| explain_alternative(experiment, alternative))
            .collect(),
        blocked_candidates: blocked_candidates_for(experiment, quantity),
        unresolved: is_unresolved(experiment, quantity),
    })
}

fn chosen_current_for(
    experiment: &PreparedExperiment,
    quantity: crate::equality::QuantityId,
) -> Option<PathExplanation> {
    experiment
        .plan
        .slot_steps
        .iter()
        .find(|slot| slot.outputs.contains(&quantity))
        .map(|slot| explain_slot(experiment, slot))
        .or_else(|| {
            experiment
                .plan
                .equation_steps
                .iter()
                .find(|equation| equation.output == quantity)
                .map(|equation| explain_equation(experiment, equation))
        })
}

fn chosen_temporal_for(
    experiment: &PreparedExperiment,
    quantity: crate::equality::QuantityId,
) -> Option<PathExplanation> {
    experiment
        .plan
        .temporal_steps
        .iter()
        .find(|equation| equation.output == quantity)
        .map(|equation| explain_equation(experiment, equation))
}

fn blocked_candidates_for(
    experiment: &PreparedExperiment,
    quantity: crate::equality::QuantityId,
) -> Vec<BlockedCandidateExplanation> {
    experiment
        .plan
        .blocked_current
        .iter()
        .filter(|candidate| candidate.outputs.contains(&quantity))
        .map(|candidate| explain_blocked_candidate(experiment, candidate, quantity))
        .collect()
}

fn is_unresolved(experiment: &PreparedExperiment, quantity: crate::equality::QuantityId) -> bool {
    !experiment.plan.available_current.contains(&quantity)
        && chosen_temporal_for(experiment, quantity).is_none()
}

fn explain_slot(experiment: &PreparedExperiment, slot: &PlannedSlot) -> PathExplanation {
    PathExplanation {
        output: quantity_name_list(experiment, &slot.outputs).join(", "),
        source: slot.slot.clone(),
        direction: "provider".to_string(),
        cost: slot.cost,
        dependencies: quantity_name_list(experiment, &slot.inputs),
    }
}

fn explain_equation(
    experiment: &PreparedExperiment,
    equation: &PlannedEquation,
) -> PathExplanation {
    PathExplanation {
        output: quantity_name(experiment, equation.output),
        source: equation.block_name.clone(),
        direction: match equation.direction {
            EquationDirection::Forward => "forward".to_string(),
            EquationDirection::Inverted => "inverted".to_string(),
        },
        cost: equation.cost,
        dependencies: dependency_names(experiment, &equation.dependencies),
    }
}

fn explain_alternative(
    experiment: &PreparedExperiment,
    alternative: &AlternativePath,
) -> AlternativeExplanation {
    AlternativeExplanation {
        output: quantity_name(experiment, alternative.output),
        source: plan_source_name(&alternative.source),
        direction: candidate_direction_name(alternative.direction).to_string(),
        cost: alternative.cost,
    }
}

fn explain_blocked_candidate(
    experiment: &PreparedExperiment,
    candidate: &BlockedCandidate,
    quantity: crate::equality::QuantityId,
) -> BlockedCandidateExplanation {
    BlockedCandidateExplanation {
        output: quantity_name(experiment, quantity),
        source: plan_source_name(&candidate.source),
        direction: candidate_direction_name(candidate.direction).to_string(),
        cost: candidate.cost,
        missing_dependencies: quantity_name_list(experiment, &candidate.missing_current),
    }
}

fn quantity_name(experiment: &PreparedExperiment, quantity: crate::equality::QuantityId) -> String {
    experiment.model.equality.quantities[quantity.0]
        .name
        .clone()
}

fn quantity_name_list(
    experiment: &PreparedExperiment,
    quantities: &[crate::equality::QuantityId],
) -> Vec<String> {
    quantities
        .iter()
        .map(|quantity| quantity_name(experiment, *quantity))
        .collect()
}

fn dependency_names(experiment: &PreparedExperiment, dependencies: &[Dependency]) -> Vec<String> {
    dependencies
        .iter()
        .map(|dependency| match dependency {
            Dependency {
                quantity: Some(quantity),
                timing: DependencyTiming::Current,
            } => quantity_name(experiment, *quantity),
            Dependency {
                quantity: Some(quantity),
                timing: DependencyTiming::Next,
            } => format!("{}[t+1]", quantity_name(experiment, *quantity)),
            Dependency {
                quantity: None,
                timing: DependencyTiming::SpecialDt,
            } => "dt".to_string(),
            Dependency {
                quantity: Some(quantity),
                timing: DependencyTiming::SpecialDt,
            } => format!("{}[dt]", quantity_name(experiment, *quantity)),
            Dependency { quantity: None, .. } => "<special>".to_string(),
        })
        .collect()
}

fn direct_binding_name(binding: &DirectBindingKind) -> String {
    match binding {
        DirectBindingKind::DataSeries { .. } => "data_series".to_string(),
        DirectBindingKind::Constant => "constant".to_string(),
        DirectBindingKind::InitialState { source } => match source {
            crate::compile::InitialStateSource::Constant => "initial_state:constant".to_string(),
            crate::compile::InitialStateSource::Data => "initial_state:data".to_string(),
            crate::compile::InitialStateSource::Learned => "initial_state:learned".to_string(),
        },
    }
}

fn plan_source_name(source: &PlanSource) -> String {
    match source {
        PlanSource::Slot(name) | PlanSource::Equation(name) => name.clone(),
    }
}

fn candidate_direction_name(direction: CandidateDirection) -> &'static str {
    match direction {
        CandidateDirection::Forward => "forward",
        CandidateDirection::Inverted => "inverted",
        CandidateDirection::Provider => "provider",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        compile::{
            CompileMode, CompileSpec, ConsistencyPolicy, DirectBindingKind, DirectBindingSpec,
            InitialStateSource, LossKind, ObservationSchedule, ObservationSpec, SlotBindingKind,
            SlotBindingSpec,
        },
        pipeline::{load_model, prepare_experiment},
    };

    const TINY_TREE: &str = include_str!("../tests/fixtures/tiny_tree.myco");

    #[test]
    fn explains_quantity_with_chosen_and_alternative_paths() {
        let model = load_model(TINY_TREE).expect("model should load");
        let experiment = prepare_experiment(&model, &tiny_tree_spec()).expect("experiment");

        let transpiration = explain_quantity(&experiment, "transpiration").expect("quantity");
        assert!(!transpiration.unresolved);
        assert_eq!(
            transpiration
                .chosen_current
                .as_ref()
                .expect("chosen path")
                .source,
            "demand_transpiration"
        );
        assert!(
            transpiration
                .alternatives
                .iter()
                .any(|alt| alt.source == "supply_transpiration")
        );
    }

    #[test]
    fn explains_unresolved_quantity_when_bindings_are_missing() {
        let model = load_model(TINY_TREE).expect("model should load");
        let experiment = prepare_experiment(&model, &spec_without_g_max()).expect("experiment");

        let g_max = explain_quantity(&experiment, "g_max").expect("quantity");
        assert!(g_max.unresolved);
        assert!(g_max.blocked_candidates.is_empty());
        assert!(g_max.chosen_current.is_none());
        assert!(g_max.direct_binding.is_none());

        let plan = explain_plan(&experiment);
        assert!(
            plan.unresolved
                .iter()
                .any(|entry| entry.quantity == "g_max")
        );
    }

    fn tiny_tree_spec() -> CompileSpec {
        CompileSpec {
            mode: CompileMode::Train,
            horizon_steps: 24,
            consistency_policy: ConsistencyPolicy::EquationOnly,
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
        }
    }

    fn spec_without_g_max() -> CompileSpec {
        CompileSpec {
            direct_bindings: tiny_tree_spec()
                .direct_bindings
                .into_iter()
                .filter(|binding| binding.quantity != "g_max")
                .collect(),
            ..tiny_tree_spec()
        }
    }
}
