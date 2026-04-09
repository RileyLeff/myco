use std::collections::{HashMap, HashSet};

use crate::{
    compile::{BoundModel, ResolvedSlotBinding, SlotBindingKind},
    diagnostics::Diagnostic,
    equality::{CoreExpr, EqualityEquation, QuantityId, SpecialRef, TimeReference},
    syntax::BlockKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleStepPlan {
    pub available_current: Vec<QuantityId>,
    pub slot_steps: Vec<PlannedSlot>,
    pub equation_steps: Vec<PlannedEquation>,
    pub temporal_steps: Vec<PlannedEquation>,
    pub alternatives: Vec<AlternativePath>,
    pub unresolved_current: Vec<QuantityId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedSlot {
    pub slot: String,
    pub kind: SlotBindingKind,
    pub inputs: Vec<QuantityId>,
    pub outputs: Vec<QuantityId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedEquation {
    pub block_name: String,
    pub kind: BlockKind,
    pub output: QuantityId,
    pub dependencies: Vec<Dependency>,
    pub expression: CoreExpr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativePath {
    pub output: QuantityId,
    pub source: PlanSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanSource {
    Slot(String),
    Equation(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dependency {
    pub quantity: Option<QuantityId>,
    pub timing: DependencyTiming,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyTiming {
    Current,
    Next,
    SpecialDt,
}

pub fn build_single_step_plan(bound: &BoundModel) -> Result<SingleStepPlan, Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    let quantity_names: HashMap<QuantityId, &str> = bound
        .quantities
        .iter()
        .map(|quantity| (quantity.quantity.id, quantity.quantity.name.as_str()))
        .collect();

    let mut available_current: HashSet<QuantityId> = bound
        .quantities
        .iter()
        .filter(|quantity| quantity.direct_binding.is_some())
        .map(|quantity| quantity.quantity.id)
        .collect();

    let mut slot_candidates = bound
        .slot_bindings
        .iter()
        .cloned()
        .map(Candidate::from_slot)
        .collect::<Vec<_>>();

    let mut current_equation_candidates = Vec::new();
    let mut temporal_equation_candidates = Vec::new();

    for equation in &bound.equations {
        match Candidate::from_equation(equation) {
            Ok(candidate) if candidate.timing == CandidateTiming::Current => {
                current_equation_candidates.push(candidate);
            }
            Ok(candidate) => temporal_equation_candidates.push(candidate),
            Err(diag) => diagnostics.push(diag),
        }
    }

    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }

    let mut planned_slots = Vec::new();
    let mut planned_equations = Vec::new();
    let mut alternatives = Vec::new();

    loop {
        let mut progress = false;

        for candidate in &mut slot_candidates {
            if candidate.scheduled {
                continue;
            }
            if !candidate.dependencies_ready(&available_current) {
                continue;
            }

            candidate.scheduled = true;
            progress = true;

            let already_available = candidate
                .outputs
                .iter()
                .all(|output| available_current.contains(output));
            if already_available {
                for output in &candidate.outputs {
                    alternatives.push(AlternativePath {
                        output: *output,
                        source: candidate.source.clone(),
                    });
                }
                continue;
            }

            for output in &candidate.outputs {
                available_current.insert(*output);
            }
            planned_slots.push(PlannedSlot {
                slot: candidate.name.clone(),
                kind: candidate.slot_kind.expect("slot candidates carry slot kind"),
                inputs: candidate.current_dependencies(),
                outputs: candidate.outputs.clone(),
            });
        }

        for candidate in &mut current_equation_candidates {
            if candidate.scheduled {
                continue;
            }
            if !candidate.dependencies_ready(&available_current) {
                continue;
            }

            candidate.scheduled = true;
            progress = true;
            let output = candidate
                .outputs
                .first()
                .copied()
                .expect("equation candidates always have one output");

            if available_current.contains(&output) {
                alternatives.push(AlternativePath {
                    output,
                    source: candidate.source.clone(),
                });
                continue;
            }

            available_current.insert(output);
            planned_equations.push(PlannedEquation {
                block_name: candidate.name.clone(),
                kind: candidate.block_kind.expect("equation candidates carry block kind"),
                output,
                dependencies: candidate.dependencies.clone(),
                expression: candidate.expression.clone().expect("equation candidates carry rhs"),
            });
        }

        if !progress {
            break;
        }
    }

    let unresolved_current_candidates = current_equation_candidates
        .iter()
        .filter(|candidate| !candidate.scheduled)
        .collect::<Vec<_>>();

    if let Some(cycle) = detect_cycle(&unresolved_current_candidates) {
        let labels = cycle
            .iter()
            .filter_map(|quantity| quantity_names.get(quantity).copied())
            .collect::<Vec<_>>()
            .join(", ");
        diagnostics.push(Diagnostic::error(format!(
            "intra-step algebraic loop detected among current-step quantities: {labels}"
        )));
    }

    let mut temporal_steps = Vec::new();
    for candidate in &temporal_equation_candidates {
        if !candidate.dependencies_ready(&available_current) {
            let missing = candidate
                .missing_current_dependencies(&available_current)
                .into_iter()
                .filter_map(|quantity| quantity_names.get(&quantity).copied())
                .collect::<Vec<_>>()
                .join(", ");
            diagnostics.push(Diagnostic::error(format!(
                "temporal equation '{}' could not be scheduled because current-step dependencies are unresolved: {}",
                candidate.name, missing
            )));
            continue;
        }

        let output = candidate
            .outputs
            .first()
            .copied()
            .expect("equation candidates always have one output");
        temporal_steps.push(PlannedEquation {
            block_name: candidate.name.clone(),
            kind: candidate.block_kind.expect("equation candidates carry block kind"),
            output,
            dependencies: candidate.dependencies.clone(),
            expression: candidate.expression.clone().expect("equation candidates carry rhs"),
        });
    }

    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }

    let unresolved_current = unresolved_current_candidates
        .into_iter()
        .flat_map(|candidate| candidate.outputs.iter().copied())
        .filter(|quantity| !available_current.contains(quantity))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let mut available_current = available_current.into_iter().collect::<Vec<_>>();
    available_current.sort_by_key(|quantity| quantity.0);

    Ok(SingleStepPlan {
        available_current,
        slot_steps: planned_slots,
        equation_steps: planned_equations,
        temporal_steps,
        alternatives,
        unresolved_current,
    })
}

#[derive(Debug, Clone)]
struct Candidate {
    name: String,
    source: PlanSource,
    timing: CandidateTiming,
    outputs: Vec<QuantityId>,
    dependencies: Vec<Dependency>,
    expression: Option<CoreExpr>,
    block_kind: Option<BlockKind>,
    slot_kind: Option<SlotBindingKind>,
    scheduled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CandidateTiming {
    Current,
    Next,
}

impl Candidate {
    fn from_slot(slot: ResolvedSlotBinding) -> Self {
        let dependencies = slot
            .inputs
            .iter()
            .copied()
            .map(|quantity| Dependency {
                quantity: Some(quantity),
                timing: DependencyTiming::Current,
            })
            .collect();
        Self {
            name: slot.slot.clone(),
            source: PlanSource::Slot(slot.slot),
            timing: CandidateTiming::Current,
            outputs: slot.provides,
            dependencies,
            expression: None,
            block_kind: None,
            slot_kind: Some(slot.kind),
            scheduled: false,
        }
    }

    fn from_equation(equation: &EqualityEquation) -> Result<Self, Diagnostic> {
        let (output, timing) = equation_output(equation)?;
        let dependencies = collect_dependencies(&equation.rhs)?;
        Ok(Self {
            name: equation.block_name.clone(),
            source: PlanSource::Equation(equation.block_name.clone()),
            timing,
            outputs: vec![output],
            dependencies,
            expression: Some(equation.rhs.clone()),
            block_kind: Some(equation.kind),
            slot_kind: None,
            scheduled: false,
        })
    }

    fn dependencies_ready(&self, available_current: &HashSet<QuantityId>) -> bool {
        self.dependencies.iter().all(|dependency| match dependency {
            Dependency {
                timing: DependencyTiming::Current,
                quantity: Some(quantity),
            } => available_current.contains(quantity),
            Dependency {
                timing: DependencyTiming::SpecialDt,
                ..
            } => true,
            Dependency {
                timing: DependencyTiming::Next,
                ..
            } => false,
            Dependency { quantity: None, .. } => true,
        })
    }

    fn current_dependencies(&self) -> Vec<QuantityId> {
        self.dependencies
            .iter()
            .filter_map(|dependency| match dependency {
                Dependency {
                    timing: DependencyTiming::Current,
                    quantity: Some(quantity),
                } => Some(*quantity),
                _ => None,
            })
            .collect()
    }

    fn missing_current_dependencies(&self, available_current: &HashSet<QuantityId>) -> Vec<QuantityId> {
        self.dependencies
            .iter()
            .filter_map(|dependency| match dependency {
                Dependency {
                    timing: DependencyTiming::Current,
                    quantity: Some(quantity),
                } if !available_current.contains(quantity) => Some(*quantity),
                _ => None,
            })
            .collect()
    }
}

fn equation_output(equation: &EqualityEquation) -> Result<(QuantityId, CandidateTiming), Diagnostic> {
    match &equation.lhs {
        CoreExpr::Quantity(reference) => match reference.time {
            TimeReference::Implicit | TimeReference::Relative(0) => {
                Ok((reference.quantity, CandidateTiming::Current))
            }
            TimeReference::Relative(1) => Ok((reference.quantity, CandidateTiming::Next)),
            other => Err(Diagnostic::error(format!(
                "equation '{}' uses unsupported lhs time reference {:?} in v1",
                equation.block_name, other
            ))),
        },
        _ => Err(Diagnostic::error(format!(
            "equation '{}' does not assign to a quantity reference on the lhs",
            equation.block_name
        ))),
    }
}

fn collect_dependencies(expr: &CoreExpr) -> Result<Vec<Dependency>, Diagnostic> {
    let mut dependencies = Vec::new();
    collect_dependencies_into(expr, &mut dependencies)?;
    Ok(dependencies)
}

fn collect_dependencies_into(
    expr: &CoreExpr,
    dependencies: &mut Vec<Dependency>,
) -> Result<(), Diagnostic> {
    match expr {
        CoreExpr::Quantity(reference) => {
            let timing = match reference.time {
                TimeReference::Implicit | TimeReference::Relative(0) => DependencyTiming::Current,
                TimeReference::Relative(1) => DependencyTiming::Next,
                other => {
                    return Err(Diagnostic::error(format!(
                        "unsupported rhs time reference {:?} in v1 single-step planning",
                        other
                    )))
                }
            };
            dependencies.push(Dependency {
                quantity: Some(reference.quantity),
                timing,
            });
        }
        CoreExpr::Special(SpecialRef::Dt) => dependencies.push(Dependency {
            quantity: None,
            timing: DependencyTiming::SpecialDt,
        }),
        CoreExpr::Number(_) => {}
        CoreExpr::Binary { left, right, .. } => {
            collect_dependencies_into(left, dependencies)?;
            collect_dependencies_into(right, dependencies)?;
        }
    }
    Ok(())
}

fn detect_cycle(candidates: &[&Candidate]) -> Option<Vec<QuantityId>> {
    let outputs = candidates
        .iter()
        .filter_map(|candidate| candidate.outputs.first().copied())
        .collect::<HashSet<_>>();
    if outputs.is_empty() {
        return None;
    }

    let mut adjacency = HashMap::<QuantityId, Vec<QuantityId>>::new();
    for candidate in candidates {
        let Some(output) = candidate.outputs.first().copied() else {
            continue;
        };
        let deps = candidate
            .dependencies
            .iter()
            .filter_map(|dependency| match dependency {
                Dependency {
                    quantity: Some(quantity),
                    timing: DependencyTiming::Current,
                } if outputs.contains(quantity) => Some(*quantity),
                _ => None,
            })
            .collect::<Vec<_>>();
        adjacency.insert(output, deps);
    }

    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    let mut in_stack = HashSet::new();

    for node in adjacency.keys().copied() {
        if let Some(cycle) = dfs_cycle(
            node,
            &adjacency,
            &mut visited,
            &mut stack,
            &mut in_stack,
        ) {
            return Some(cycle);
        }
    }

    None
}

fn dfs_cycle(
    node: QuantityId,
    adjacency: &HashMap<QuantityId, Vec<QuantityId>>,
    visited: &mut HashSet<QuantityId>,
    stack: &mut Vec<QuantityId>,
    in_stack: &mut HashSet<QuantityId>,
) -> Option<Vec<QuantityId>> {
    if in_stack.contains(&node) {
        let start = stack.iter().position(|entry| *entry == node).unwrap_or(0);
        return Some(stack[start..].to_vec());
    }
    if !visited.insert(node) {
        return None;
    }

    stack.push(node);
    in_stack.insert(node);

    if let Some(neighbors) = adjacency.get(&node) {
        for neighbor in neighbors {
            if let Some(cycle) = dfs_cycle(*neighbor, adjacency, visited, stack, in_stack) {
                return Some(cycle);
            }
        }
    }

    stack.pop();
    in_stack.remove(&node);
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        compile::{
            bind_compile_spec, CompileMode, CompileSpec, DirectBindingKind, DirectBindingSpec,
            InitialStateSource, ObservationSpec, ObservationSchedule, SlotBindingKind,
            SlotBindingSpec,
        },
        equality, semantic,
        syntax::parse_and_validate,
    };

    const TINY_TREE: &str = include_str!("../tests/fixtures/tiny_tree.myco");

    #[test]
    fn plans_tiny_tree_single_step() {
        let bound = tiny_tree_bound_model();
        let plan = build_single_step_plan(&bound).expect("planning should succeed");

        assert_eq!(plan.slot_steps.len(), 1);
        assert_eq!(plan.equation_steps.len(), 1);
        assert_eq!(plan.temporal_steps.len(), 1);
        assert_eq!(plan.alternatives.len(), 1);
        assert!(plan.unresolved_current.is_empty());

        assert_eq!(plan.slot_steps[0].slot, "controller");
        assert_eq!(plan.equation_steps[0].block_name, "demand_transpiration");
        assert_eq!(plan.temporal_steps[0].block_name, "water_step");
        assert_eq!(plan.alternatives[0].source, PlanSource::Equation("supply_transpiration".to_string()));
    }

    #[test]
    fn rejects_intra_step_cycle_that_blocks_temporal_update() {
        let source = r#"
model Loopy

state water : scalar
node a : scalar
node b : scalar

relation a_from_b:
  a = b + 1

relation b_from_a:
  b = a + 1

temporal water_step:
  water[t+1] = water[t] + a
"#;

        let syntax = parse_and_validate(source).expect("syntax should validate");
        let semantic = semantic::lower_model(&syntax).expect("semantic lowering should succeed");
        let equality = equality::lower_model(&semantic).expect("equality lowering should succeed");
        let bound = bind_compile_spec(
            &equality,
            &CompileSpec {
                mode: CompileMode::Simulate,
                horizon_steps: 4,
                direct_bindings: vec![DirectBindingSpec {
                    quantity: "water".to_string(),
                    kind: DirectBindingKind::InitialState {
                        source: InitialStateSource::Constant,
                    },
                }],
                slot_bindings: vec![],
                observations: vec![],
            },
        )
        .expect("binding should succeed");

        let diagnostics = build_single_step_plan(&bound).expect_err("planning should fail");
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic
                .message
                .contains("intra-step algebraic loop detected among current-step quantities")
        }));
    }

    fn tiny_tree_bound_model() -> BoundModel {
        let syntax = parse_and_validate(TINY_TREE).expect("fixture should validate");
        let semantic = semantic::lower_model(&syntax).expect("semantic lowering should succeed");
        let equality = equality::lower_model(&semantic).expect("equality lowering should succeed");
        bind_compile_spec(
            &equality,
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
                    loss: crate::compile::LossKind::Mse,
                    schedule: ObservationSchedule::DensePerStep,
                }],
            },
        )
        .expect("binding should succeed")
    }
}
