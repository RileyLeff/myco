use std::{fs, io, path::Path};

use serde::Serialize;

use crate::{
    compile::{BoundModel, CompileSpec, bind_compile_spec},
    diagnostics::Diagnostic,
    dimensions, emit,
    equality::{self, EqualityModel},
    introspect,
    plan::{SingleStepPlan, build_single_step_plan},
    semantic::{self, SemanticModel},
    syntax::{self, ModelFile},
};

#[derive(Debug, Clone)]
pub struct LoadedModel {
    pub syntax: ModelFile,
    pub semantic: SemanticModel,
    pub equality: EqualityModel,
}

#[derive(Debug, Clone)]
pub struct PreparedExperiment {
    pub model: LoadedModel,
    pub bound: BoundModel,
    pub plan: SingleStepPlan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendTarget {
    Python,
    Jax,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledArtifact {
    pub model_name: String,
    pub backend: BackendTarget,
    pub metadata: ArtifactMetadata,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ArtifactMetadata {
    pub compile_mode: String,
    pub consistency_policy: String,
    pub constraint_runtime_policy: String,
    pub loss_helpers_enabled: bool,
    pub persistent_quantities: Vec<String>,
    pub learned_initial_state: Vec<String>,
    pub learned_slots: Vec<String>,
    pub slot_interfaces: Vec<SlotInterfaceMetadata>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SlotInterfaceMetadata {
    pub slot: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub input_arity: usize,
    pub output_arity: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelSummary {
    pub name: String,
    pub quantity_count: usize,
    pub relation_count: usize,
    pub slot_count: usize,
    pub persistent_quantity_count: usize,
    pub temporal_count: usize,
    pub quantity_names: Vec<String>,
    pub relation_names: Vec<String>,
    pub slot_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExperimentSummary {
    pub name: String,
    pub assumption_count: usize,
    pub learning_count: usize,
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
    dimensions::validate_model_dimensions(&equality)?;

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

pub fn compile_experiment(
    experiment: &PreparedExperiment,
    backend: BackendTarget,
) -> CompiledArtifact {
    let metadata = artifact_metadata(experiment, backend);
    let source = match backend {
        BackendTarget::Python => emit::emit_python_module(experiment),
        BackendTarget::Jax => emit::emit_jax_module(experiment),
    };

    CompiledArtifact {
        model_name: experiment.model.syntax.name.clone(),
        backend,
        metadata,
        source,
    }
}

pub fn compile_model(
    model: &LoadedModel,
    spec: &CompileSpec,
    backend: BackendTarget,
) -> Result<CompiledArtifact, Vec<Diagnostic>> {
    let experiment = prepare_experiment(model, spec)?;
    Ok(compile_experiment(&experiment, backend))
}

pub fn compile_source(
    source: &str,
    spec: &CompileSpec,
    backend: BackendTarget,
) -> Result<CompiledArtifact, Vec<Diagnostic>> {
    let model = load_model(source)?;
    compile_model(&model, spec, backend)
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
            .core
            .equations
            .iter()
            .map(|registration| registration.equation.block_name.clone())
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

        let persistent_quantity_count = self.equality.persistent_quantities.len();
        let temporal_count = self
            .equality
            .core
            .equations
            .iter()
            .filter(|registration| {
                matches!(
                    registration.equation.kind,
                    crate::syntax::BlockKind::Temporal
                )
            })
            .count();

        ModelSummary {
            name: self.syntax.name.clone(),
            quantity_count: self.equality.quantities.len(),
            relation_count: relation_names.len(),
            slot_count: self.equality.slots.len(),
            persistent_quantity_count,
            temporal_count,
            quantity_names,
            relation_names,
            slot_names,
        }
    }
}

impl PreparedExperiment {
    pub fn summary(&self) -> ExperimentSummary {
        let learned_initial_count = self
            .bound
            .assumptions
            .iter()
            .filter(|binding| {
                matches!(
                    binding.kind,
                    crate::compile::AssumptionKind::InitialState {
                        source: crate::compile::InitialStateSource::Learned,
                    }
                )
            })
            .count();
        let learned_slot_count = self.bound.learned_slots.iter().count();
        let learning_count = learned_initial_count + learned_slot_count;

        ExperimentSummary {
            name: self.model.syntax.name.clone(),
            assumption_count: self.bound.assumptions.len() - learned_initial_count,
            learning_count,
            observation_count: self.bound.observations.len(),
            planned_slot_steps: self.plan.slot_steps.len(),
            planned_equation_steps: self.plan.equation_steps.len(),
            planned_temporal_steps: self.plan.temporal_steps.len(),
            alternative_path_count: self.plan.alternatives.len(),
            unresolved_current_count: self.plan.unresolved_current.len(),
        }
    }

    pub fn compile(&self, backend: BackendTarget) -> CompiledArtifact {
        compile_experiment(self, backend)
    }

    pub fn explain_plan(&self) -> introspect::PlanExplanation {
        introspect::explain_plan(self)
    }

    pub fn explain_quantity(
        &self,
        quantity_name: &str,
    ) -> Result<introspect::QuantityExplanation, Vec<Diagnostic>> {
        introspect::explain_quantity(self, quantity_name)
    }
}

impl CompiledArtifact {
    pub fn suggested_filename(&self) -> String {
        let stem = sanitize_module_name(&self.model_name);
        let backend_suffix = match self.backend {
            BackendTarget::Python => "python",
            BackendTarget::Jax => "jax",
        };
        format!("{stem}_{backend_suffix}.py")
    }

    pub fn write_to_path(&self, path: impl AsRef<Path>) -> io::Result<()> {
        fs::write(path, &self.source)
    }
}

fn sanitize_module_name(input: &str) -> String {
    let mut out = String::new();
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if !out.ends_with('_') {
            out.push('_');
        }
    }
    out.trim_matches('_').to_string()
}

fn artifact_metadata(experiment: &PreparedExperiment, backend: BackendTarget) -> ArtifactMetadata {
    let mut persistent_quantities = experiment
        .bound
        .quantities
        .iter()
        .filter(|quantity| quantity.persistent)
        .map(|quantity| quantity.quantity.name.clone())
        .collect::<Vec<_>>();
    persistent_quantities.sort();

    let slot_interfaces = experiment
        .bound
        .learned_slots
        .iter()
        .map(|slot| SlotInterfaceMetadata {
            slot: slot.slot.clone(),
            inputs: slot
                .inputs
                .iter()
                .map(|quantity| {
                    experiment.model.equality.quantities[quantity.0]
                        .name
                        .clone()
                })
                .collect(),
            outputs: slot
                .provides
                .iter()
                .map(|quantity| {
                    experiment.model.equality.quantities[quantity.0]
                        .name
                        .clone()
                })
                .collect(),
            input_arity: slot.input_arity,
            output_arity: slot.output_arity,
        })
        .collect();

    ArtifactMetadata {
        compile_mode: match experiment.bound.mode {
            crate::compile::CompileMode::Simulate => "simulate".to_string(),
            crate::compile::CompileMode::Fit => "fit".to_string(),
            crate::compile::CompileMode::Train => "train".to_string(),
        },
        consistency_policy: match experiment.bound.consistency_policy {
            crate::compile::ConsistencyPolicy::Off => "off".to_string(),
            crate::compile::ConsistencyPolicy::EquationOnly => "equation_only".to_string(),
            crate::compile::ConsistencyPolicy::All => "all".to_string(),
        },
        constraint_runtime_policy: match backend {
            BackendTarget::Python => "project_learned_raise_derived".to_string(),
            BackendTarget::Jax => "project_learned_penalize_derived".to_string(),
        },
        loss_helpers_enabled: !matches!(
            experiment.bound.mode,
            crate::compile::CompileMode::Simulate
        ),
        persistent_quantities,
        learned_initial_state: experiment
            .bound
            .assumptions
            .iter()
            .filter_map(|binding| match binding.kind {
                crate::compile::AssumptionKind::InitialState {
                    source: crate::compile::InitialStateSource::Learned,
                } => Some(
                    experiment.model.equality.quantities[binding.quantity.0]
                        .name
                        .clone(),
                ),
                _ => None,
            })
            .collect(),
        learned_slots: experiment
            .bound
            .learned_slots
            .iter()
            .map(|slot| slot.slot.clone())
            .collect(),
        slot_interfaces,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile::{
        AssumptionKind, AssumptionSpec, CompileMode, CompileSpec, ConsistencyPolicy,
        InitialStateSource, LearnedSlotSpec, LossKind, ObservationSchedule, ObservationSpec,
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
        assert_eq!(summary.persistent_quantity_count, 1);
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
            },
        )
        .expect("experiment should prepare");

        let summary = experiment.summary();
        assert_eq!(summary.name, "TinyTree");
        assert_eq!(summary.assumption_count, 6);
        assert_eq!(summary.learning_count, 1);
        assert_eq!(summary.observation_count, 1);
        assert_eq!(summary.planned_slot_steps, 1);
        assert_eq!(summary.planned_equation_steps, 1);
        assert_eq!(summary.planned_temporal_steps, 1);
        assert!(summary.alternative_path_count >= 1);
        assert_eq!(summary.unresolved_current_count, 0);
    }

    #[test]
    fn compiles_python_artifact_from_experiment() {
        let model = load_model(TINY_TREE).expect("model should load");
        let experiment =
            prepare_experiment(&model, &tiny_tree_spec()).expect("experiment should prepare");
        let artifact = experiment.compile(BackendTarget::Python);

        assert_eq!(artifact.backend, BackendTarget::Python);
        assert_eq!(artifact.model_name, "TinyTree");
        assert!(
            artifact
                .source
                .contains("def step(state, forcing, constants, slot_providers, dt):")
        );
        assert_eq!(artifact.metadata.compile_mode, "train");
        assert_eq!(artifact.metadata.consistency_policy, "equation_only");
        assert_eq!(
            artifact.metadata.constraint_runtime_policy,
            "project_learned_raise_derived"
        );
        assert!(artifact.metadata.loss_helpers_enabled);
        assert_eq!(
            artifact.metadata.persistent_quantities,
            vec!["carbon".to_string(), "water".to_string()]
        );
        assert_eq!(
            artifact.metadata.learned_slots,
            vec!["controller".to_string()]
        );
        assert_eq!(artifact.metadata.slot_interfaces.len(), 1);
        assert_eq!(artifact.metadata.slot_interfaces[0].slot, "controller");
        assert_eq!(artifact.metadata.slot_interfaces[0].input_arity, 6);
        assert_eq!(artifact.metadata.slot_interfaces[0].output_arity, 1);
        assert_eq!(artifact.suggested_filename(), "tinytree_python.py");
    }

    #[test]
    fn compiles_jax_artifact_from_source() {
        let artifact = compile_source(TINY_TREE, &tiny_tree_spec(), BackendTarget::Jax)
            .expect("compilation should succeed");

        assert_eq!(artifact.backend, BackendTarget::Jax);
        assert!(artifact.source.contains("import jax.numpy as jnp"));
        assert_eq!(artifact.metadata.compile_mode, "train");
        assert_eq!(
            artifact.metadata.constraint_runtime_policy,
            "project_learned_penalize_derived"
        );
        assert_eq!(
            artifact.metadata.persistent_quantities,
            vec!["carbon".to_string(), "water".to_string()]
        );
        assert_eq!(
            artifact.metadata.slot_interfaces[0].outputs,
            vec!["stomata".to_string()]
        );
        assert_eq!(artifact.suggested_filename(), "tinytree_jax.py");
    }

    fn tiny_tree_spec() -> CompileSpec {
        CompileSpec {
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
        }
    }
}
