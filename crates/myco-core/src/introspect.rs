use crate::{
    compile::DirectBindingKind,
    diagnostics::Diagnostic,
    equality::{CoreExpr, Provenance, QuantityId, TimeReference},
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
    pub expression: Option<String>,
    pub provenance_label: Option<String>,
    pub source_span: Option<SourceSpanExplanation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AlternativeExplanation {
    pub output: String,
    pub source: String,
    pub direction: String,
    pub cost: u32,
    pub expression: Option<String>,
    pub provenance_label: Option<String>,
    pub source_span: Option<SourceSpanExplanation>,
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
    pub expression: Option<String>,
    pub provenance_label: Option<String>,
    pub source_span: Option<SourceSpanExplanation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SourceSpanExplanation {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
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
        .map(|slot| explain_slot_path(experiment, slot, Some(quantity)))
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
    explain_slot_path(experiment, slot, None)
}

fn explain_slot_path(
    experiment: &PreparedExperiment,
    slot: &PlannedSlot,
    focused_output: Option<QuantityId>,
) -> PathExplanation {
    let output_names = focused_output
        .map(|output| vec![quantity_name(experiment, output)])
        .unwrap_or_else(|| quantity_name_list(experiment, &slot.outputs));
    let provenance = slot_provenance(experiment, &slot.slot);
    PathExplanation {
        output: output_names.join(", "),
        source: slot.slot.clone(),
        direction: "provider".to_string(),
        cost: slot.cost,
        dependencies: quantity_name_list(experiment, &slot.inputs),
        expression: Some(render_slot_expression(
            experiment,
            &slot.slot,
            slot.kind,
            &slot.inputs,
            focused_output,
        )),
        provenance_label: provenance.as_ref().map(|item| item.label.clone()),
        source_span: provenance
            .as_ref()
            .map(|item| source_span_explanation(&item.span)),
    }
}

fn explain_equation(
    experiment: &PreparedExperiment,
    equation: &PlannedEquation,
) -> PathExplanation {
    let provenance = equation_provenance(experiment, &equation.block_name);
    PathExplanation {
        output: quantity_name(experiment, equation.output),
        source: equation.block_name.clone(),
        direction: match equation.direction {
            EquationDirection::Forward => "forward".to_string(),
            EquationDirection::Inverted => "inverted".to_string(),
        },
        cost: equation.cost,
        dependencies: dependency_names(experiment, &equation.dependencies),
        expression: Some(render_expr(experiment, &equation.expression)),
        provenance_label: provenance.as_ref().map(|item| item.label.clone()),
        source_span: provenance
            .as_ref()
            .map(|item| source_span_explanation(&item.span)),
    }
}

fn explain_alternative(
    experiment: &PreparedExperiment,
    alternative: &AlternativePath,
) -> AlternativeExplanation {
    let provenance = source_provenance(experiment, &alternative.source);
    AlternativeExplanation {
        output: quantity_name(experiment, alternative.output),
        source: plan_source_name(&alternative.source),
        direction: candidate_direction_name(alternative.direction).to_string(),
        cost: alternative.cost,
        expression: alternative_expression(experiment, alternative),
        provenance_label: provenance.as_ref().map(|item| item.label.clone()),
        source_span: provenance
            .as_ref()
            .map(|item| source_span_explanation(&item.span)),
    }
}

fn explain_blocked_candidate(
    experiment: &PreparedExperiment,
    candidate: &BlockedCandidate,
    quantity: crate::equality::QuantityId,
) -> BlockedCandidateExplanation {
    let provenance = source_provenance(experiment, &candidate.source);
    BlockedCandidateExplanation {
        output: quantity_name(experiment, quantity),
        source: plan_source_name(&candidate.source),
        direction: candidate_direction_name(candidate.direction).to_string(),
        cost: candidate.cost,
        missing_dependencies: quantity_name_list(experiment, &candidate.missing_current),
        expression: blocked_candidate_expression(experiment, candidate, quantity),
        provenance_label: provenance.as_ref().map(|item| item.label.clone()),
        source_span: provenance
            .as_ref()
            .map(|item| source_span_explanation(&item.span)),
    }
}

fn render_expr(experiment: &PreparedExperiment, expr: &CoreExpr) -> String {
    match expr {
        CoreExpr::Quantity(reference) => match reference.time {
            TimeReference::Implicit => quantity_name(experiment, reference.quantity),
            TimeReference::Relative(0) => {
                format!("{}[t]", quantity_name(experiment, reference.quantity))
            }
            TimeReference::Relative(offset) if offset > 0 => {
                format!(
                    "{}[t+{}]",
                    quantity_name(experiment, reference.quantity),
                    offset
                )
            }
            TimeReference::Relative(offset) => {
                format!(
                    "{}[t{}]",
                    quantity_name(experiment, reference.quantity),
                    offset
                )
            }
        },
        CoreExpr::Special(crate::equality::SpecialRef::Dt) => "dt".to_string(),
        CoreExpr::Number(number) => number.clone(),
        CoreExpr::Binary { op, left, right } => {
            let op_str = match op {
                crate::semantic::BinaryOp::Add => "+",
                crate::semantic::BinaryOp::Sub => "-",
                crate::semantic::BinaryOp::Mul => "*",
                crate::semantic::BinaryOp::Div => "/",
            };
            format!(
                "({left} {op_str} {right})",
                left = render_expr(experiment, left),
                right = render_expr(experiment, right)
            )
        }
    }
}

fn render_slot_expression(
    experiment: &PreparedExperiment,
    slot_name: &str,
    kind: crate::compile::SlotBindingKind,
    inputs: &[QuantityId],
    focused_output: Option<QuantityId>,
) -> String {
    let output_suffix = focused_output
        .map(|output| format!(" -> {}", quantity_name(experiment, output)))
        .unwrap_or_default();
    match kind {
        crate::compile::SlotBindingKind::DataSeries => focused_output
            .map(|output| format!("data_series({})", quantity_name(experiment, output)))
            .unwrap_or_else(|| format!("data_series<{slot_name}>{output_suffix}")),
        crate::compile::SlotBindingKind::Constant => focused_output
            .map(|output| format!("constant({})", quantity_name(experiment, output)))
            .unwrap_or_else(|| format!("constant<{slot_name}>{output_suffix}")),
        crate::compile::SlotBindingKind::Learned => format!(
            "{slot_name}({inputs}){output_suffix}",
            inputs = quantity_name_list(experiment, inputs).join(", ")
        ),
    }
}

fn alternative_expression(
    experiment: &PreparedExperiment,
    alternative: &AlternativePath,
) -> Option<String> {
    match &alternative.payload {
        crate::plan::AlternativePayload::Equation { expression } => {
            Some(render_expr(experiment, expression))
        }
        crate::plan::AlternativePayload::Slot {
            kind,
            inputs,
            output_index: _,
        } => Some(render_slot_expression(
            experiment,
            &plan_source_name(&alternative.source),
            *kind,
            inputs,
            Some(alternative.output),
        )),
    }
}

fn blocked_candidate_expression(
    experiment: &PreparedExperiment,
    candidate: &BlockedCandidate,
    quantity: QuantityId,
) -> Option<String> {
    match &candidate.source {
        PlanSource::Slot(slot_name) => experiment
            .plan
            .slot_steps
            .iter()
            .find(|slot| slot.slot == *slot_name)
            .map(|slot| {
                render_slot_expression(
                    experiment,
                    slot_name,
                    slot.kind,
                    &slot.inputs,
                    Some(quantity),
                )
            })
            .or_else(|| {
                experiment
                    .bound
                    .slot_bindings
                    .iter()
                    .find(|slot| slot.slot == *slot_name)
                    .map(|slot| {
                        render_slot_expression(
                            experiment,
                            slot_name,
                            slot.kind,
                            &slot.inputs,
                            Some(quantity),
                        )
                    })
            }),
        PlanSource::Equation(block_name) => experiment
            .model
            .equality
            .core
            .directional
            .iter()
            .find(|registration| {
                registration.block_name == *block_name
                    && registration.output.quantity == quantity
                    && candidate_direction_matches(candidate.direction, registration.direction)
            })
            .map(|registration| render_expr(experiment, &registration.seed_expression)),
    }
}

fn source_provenance(experiment: &PreparedExperiment, source: &PlanSource) -> Option<Provenance> {
    match source {
        PlanSource::Slot(slot_name) => slot_provenance(experiment, slot_name),
        PlanSource::Equation(block_name) => equation_provenance(experiment, block_name),
    }
}

fn slot_provenance(experiment: &PreparedExperiment, slot_name: &str) -> Option<Provenance> {
    experiment
        .model
        .equality
        .slots
        .iter()
        .find(|slot| slot.name == slot_name)
        .map(|slot| slot.provenance.clone())
}

fn equation_provenance(experiment: &PreparedExperiment, block_name: &str) -> Option<Provenance> {
    experiment
        .model
        .equality
        .core
        .equations
        .iter()
        .find(|registration| registration.equation.block_name == block_name)
        .map(|registration| registration.equation.provenance.clone())
}

fn source_span_explanation(span: &crate::diagnostics::SourceSpan) -> SourceSpanExplanation {
    SourceSpanExplanation {
        start_line: span.start.line,
        start_column: span.start.column,
        end_line: span.end.line,
        end_column: span.end.column,
    }
}

fn candidate_direction_matches(
    candidate_direction: CandidateDirection,
    expression_direction: crate::egraph::ExpressionDirection,
) -> bool {
    matches!(
        (candidate_direction, expression_direction),
        (
            CandidateDirection::Forward,
            crate::egraph::ExpressionDirection::Forward
        ) | (
            CandidateDirection::Inverted,
            crate::egraph::ExpressionDirection::Inverted
        )
    )
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
        assert_eq!(
            transpiration
                .chosen_current
                .as_ref()
                .expect("chosen path")
                .expression
                .as_deref(),
            Some("(stomata * vpd_scale)")
        );
        assert_eq!(
            transpiration
                .chosen_current
                .as_ref()
                .expect("chosen path")
                .provenance_label
                .as_deref(),
            Some("demand_transpiration")
        );
        assert!(
            transpiration
                .alternatives
                .iter()
                .any(|alt| alt.source == "supply_transpiration")
        );
        assert!(transpiration.alternatives.iter().any(
            |alt| alt.expression.as_deref() == Some("(hydraulic_cond * (soil_water - water))")
        ));
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
