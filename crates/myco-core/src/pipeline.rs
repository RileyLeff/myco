use crate::{
    compile::{bind_compile_spec, BoundModel, CompileSpec},
    diagnostics::Diagnostic,
    equality::{self, EqualityModel},
    plan::{build_single_step_plan, SingleStepPlan},
    semantic::{self, SemanticModel},
    syntax::{self, ModelFile},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedModel {
    pub syntax: ModelFile,
    pub semantic: SemanticModel,
    pub equality: EqualityModel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedExperiment {
    pub model: LoadedModel,
    pub bound: BoundModel,
    pub plan: SingleStepPlan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelSummary {
    pub name: String,
    pub quantity_count: usize,
    pub relation_count: usize,
    pub slot_count: usize,
    pub external_count: usize,
    pub state_count: usize,
    pub node_count: usize,
    pub temporal_count: usize,
    pub quantity_names: Vec<String>,
    pub relation_names: Vec<String>,
    pub slot_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExperimentSummary {
    pub name: String,
    pub direct_binding_count: usize,
    pub slot_binding_count: usize,
    pub observation_count: usize,
    pub planned_slot_steps: usize,
    pub planned_equation_steps: usize,
    pub planned_temporal_steps: usize,
    pub alternative_path_count: usize,
    pub unresolved_current_count: usize,
}

pub fn load_model(source: &str) -> Result<LoadedModel, Vec<Diagnostic>> {
    let syntax = syntax::parse_and_validate(source)?;
    let semantic = semantic::lower_model(&syntax)?;
    let equality = equality::lower_model(&semantic)?;

    Ok(LoadedModel {
        syntax,
        semantic,
        equality,
    })
}

pub fn prepare_experiment(
    model: &LoadedModel,
    spec: &CompileSpec,
) -> Result<PreparedExperiment, Vec<Diagnostic>> {
    let bound = bind_compile_spec(&model.equality, spec)?;
    let plan = build_single_step_plan(&bound)?;

    Ok(PreparedExperiment {
        model: model.clone(),
        bound,
        plan,
    })
}

impl LoadedModel {
    pub fn summary(&self) -> ModelSummary {
        let mut quantity_names = self
            .equality
            .quantities
            .iter()
            .map(|quantity| quantity.name.clone())
            .collect::<Vec<_>>();
        quantity_names.sort();

        let mut relation_names = self
            .equality
            .equations
            .iter()
            .map(|equation| equation.block_name.clone())
            .collect::<Vec<_>>();
        relation_names.sort();
        relation_names.dedup();

        let mut slot_names = self
            .equality
            .slots
            .iter()
            .map(|slot| slot.name.clone())
            .collect::<Vec<_>>();
        slot_names.sort();

        let external_count = self
            .equality
            .quantities
            .iter()
            .filter(|quantity| matches!(quantity.kind, crate::syntax::QuantityKind::External))
            .count();
        let state_count = self
            .equality
            .quantities
            .iter()
            .filter(|quantity| matches!(quantity.kind, crate::syntax::QuantityKind::State))
            .count();
        let node_count = self
            .equality
            .quantities
            .iter()
            .filter(|quantity| matches!(quantity.kind, crate::syntax::QuantityKind::Node))
            .count();
        let temporal_count = self
            .equality
            .equations
            .iter()
            .filter(|equation| matches!(equation.kind, crate::syntax::BlockKind::Temporal))
            .count();

        ModelSummary {
            name: self.syntax.name.clone(),
            quantity_count: self.equality.quantities.len(),
            relation_count: relation_names.len(),
            slot_count: self.equality.slots.len(),
            external_count,
            state_count,
            node_count,
            temporal_count,
            quantity_names,
            relation_names,
            slot_names,
        }
    }
}

impl PreparedExperiment {
    pub fn summary(&self) -> ExperimentSummary {
        ExperimentSummary {
            name: self.model.syntax.name.clone(),
            direct_binding_count: self.bound.direct_bindings.len(),
            slot_binding_count: self.bound.slot_bindings.len(),
            observation_count: self.bound.observations.len(),
            planned_slot_steps: self.plan.slot_steps.len(),
            planned_equation_steps: self.plan.equation_steps.len(),
            planned_temporal_steps: self.plan.temporal_steps.len(),
            alternative_path_count: self.plan.alternatives.len(),
            unresolved_current_count: self.plan.unresolved_current.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile::{
        CompileMode, CompileSpec, DirectBindingKind, DirectBindingSpec, InitialStateSource,
        LossKind, ObservationSpec, ObservationSchedule, SlotBindingKind, SlotBindingSpec,
    };

    const TINY_TREE: &str = include_str!("../tests/fixtures/tiny_tree.myco");

    #[test]
    fn loads_model_pipeline() {
        let model = load_model(TINY_TREE).expect("model should load");
        let summary = model.summary();

        assert_eq!(summary.name, "TinyTree");
        assert_eq!(summary.quantity_count, 8);
        assert_eq!(summary.relation_count, 3);
        assert_eq!(summary.slot_count, 1);
        assert_eq!(summary.external_count, 3);
        assert_eq!(summary.state_count, 2);
        assert_eq!(summary.node_count, 3);
        assert_eq!(summary.temporal_count, 1);
        assert!(summary.quantity_names.iter().any(|name| name == "stomata"));
    }

    #[test]
    fn prepares_experiment_pipeline() {
        let model = load_model(TINY_TREE).expect("model should load");
        let experiment = prepare_experiment(
            &model,
            &CompileSpec {
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
            },
        )
        .expect("experiment should prepare");

        let summary = experiment.summary();
        assert_eq!(summary.name, "TinyTree");
        assert_eq!(summary.direct_binding_count, 6);
        assert_eq!(summary.slot_binding_count, 1);
        assert_eq!(summary.observation_count, 1);
        assert_eq!(summary.planned_slot_steps, 1);
        assert_eq!(summary.planned_equation_steps, 1);
        assert_eq!(summary.planned_temporal_steps, 1);
        assert!(summary.alternative_path_count >= 1);
        assert_eq!(summary.unresolved_current_count, 0);
    }
}
