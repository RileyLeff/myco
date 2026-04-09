use std::collections::{HashMap, HashSet};

use crate::{
    compile::{BoundModel, ResolvedSlotBinding, SlotBindingKind},
    diagnostics::Diagnostic,
    equality::{CoreExpr, EqualityEquation, QuantityId, QuantityRef, SpecialRef, TimeReference},
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
    pub cost: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedEquation {
    pub block_name: String,
    pub kind: BlockKind,
    pub output: QuantityId,
    pub dependencies: Vec<Dependency>,
    pub expression: CoreExpr,
    pub direction: EquationDirection,
    pub cost: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativePath {
    pub output: QuantityId,
    pub source: PlanSource,
    pub direction: CandidateDirection,
    pub cost: u32,
    pub payload: AlternativePayload,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanSource {
    Slot(String),
    Equation(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlternativePayload {
    Slot { inputs: Vec<QuantityId> },
    Equation { expression: CoreExpr },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateDirection {
    Forward,
    Inverted,
    Provider,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquationDirection {
    Forward,
    Inverted,
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

    let mut current_candidates = bound
        .slot_bindings
        .iter()
        .cloned()
        .map(Candidate::from_slot)
        .collect::<Vec<_>>();
    let mut temporal_candidates = Vec::new();

    for equation in &bound.equations {
        match Candidate::from_equation_directions(equation) {
            Ok(candidates) => {
                for candidate in candidates {
                    match candidate.timing {
                        CandidateTiming::Current => current_candidates.push(candidate),
                        CandidateTiming::Next => temporal_candidates.push(candidate),
                    }
                }
            }
            Err(mut errs) => diagnostics.append(&mut errs),
        }
    }

    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }

    let mut planned_slots = Vec::new();
    let mut planned_equations = Vec::new();
    let mut alternatives = Vec::new();

    loop {
        let mut ready = current_candidates
            .iter()
            .enumerate()
            .filter(|(_, candidate)| {
                !candidate.scheduled && candidate.dependencies_ready(&available_current)
            })
            .map(|(index, candidate)| ReadyCandidate {
                index,
                cost: candidate.cost,
                source_rank: candidate.source_rank(),
                name: candidate.name.clone(),
                output_rank: candidate.primary_output_rank(),
            })
            .collect::<Vec<_>>();

        if ready.is_empty() {
            break;
        }

        ready.sort();
        let entry = ready.remove(0);
        let candidate = &mut current_candidates[entry.index];

        candidate.scheduled = true;
        let unresolved_outputs = candidate
            .outputs
            .iter()
            .copied()
            .filter(|output| !available_current.contains(output))
            .collect::<Vec<_>>();

        if unresolved_outputs.is_empty() {
            for output in &candidate.outputs {
                alternatives.push(AlternativePath {
                    output: *output,
                    source: candidate.source.clone(),
                    direction: candidate.direction,
                    cost: candidate.cost,
                    payload: candidate.alternative_payload(),
                });
            }
            continue;
        }

        for output in &candidate.outputs {
            if available_current.contains(output) {
                alternatives.push(AlternativePath {
                    output: *output,
                    source: candidate.source.clone(),
                    direction: candidate.direction,
                    cost: candidate.cost,
                    payload: candidate.alternative_payload(),
                });
            }
        }

        for output in &unresolved_outputs {
            available_current.insert(*output);
        }

        match &candidate.payload {
            CandidatePayload::Slot { kind, inputs } => {
                planned_slots.push(PlannedSlot {
                    slot: candidate.name.clone(),
                    kind: *kind,
                    inputs: inputs.clone(),
                    outputs: candidate.outputs.clone(),
                    cost: candidate.cost,
                });
            }
            CandidatePayload::Equation {
                kind,
                expression,
                dependencies,
            } => {
                let output = unresolved_outputs[0];
                planned_equations.push(PlannedEquation {
                    block_name: candidate.name.clone(),
                    kind: *kind,
                    output,
                    dependencies: dependencies.clone(),
                    expression: expression.clone(),
                    direction: match candidate.direction {
                        CandidateDirection::Forward => EquationDirection::Forward,
                        CandidateDirection::Inverted => EquationDirection::Inverted,
                        CandidateDirection::Provider => {
                            unreachable!("equation candidates are not providers")
                        }
                    },
                    cost: candidate.cost,
                });
            }
        }
    }

    let unresolved_current_candidates = current_candidates
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
    for candidate in &temporal_candidates {
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

        match &candidate.payload {
            CandidatePayload::Equation {
                kind,
                expression,
                dependencies,
            } => {
                temporal_steps.push(PlannedEquation {
                    block_name: candidate.name.clone(),
                    kind: *kind,
                    output: candidate.outputs[0],
                    dependencies: dependencies.clone(),
                    expression: expression.clone(),
                    direction: EquationDirection::Forward,
                    cost: candidate.cost,
                });
            }
            CandidatePayload::Slot { .. } => {
                diagnostics.push(Diagnostic::error(format!(
                    "slot '{}' cannot produce next-step quantities in v1",
                    candidate.name
                )));
            }
        }
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Candidate {
    name: String,
    source: PlanSource,
    direction: CandidateDirection,
    timing: CandidateTiming,
    outputs: Vec<QuantityId>,
    payload: CandidatePayload,
    cost: u32,
    scheduled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CandidatePayload {
    Slot {
        kind: SlotBindingKind,
        inputs: Vec<QuantityId>,
    },
    Equation {
        kind: BlockKind,
        expression: CoreExpr,
        dependencies: Vec<Dependency>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CandidateTiming {
    Current,
    Next,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ReadyCandidate {
    cost: u32,
    source_rank: u8,
    name: String,
    output_rank: usize,
    index: usize,
}

impl Candidate {
    fn from_slot(slot: ResolvedSlotBinding) -> Self {
        Self {
            name: slot.slot.clone(),
            source: PlanSource::Slot(slot.slot),
            direction: CandidateDirection::Provider,
            timing: CandidateTiming::Current,
            outputs: slot.provides,
            payload: CandidatePayload::Slot {
                kind: slot.kind,
                inputs: slot.inputs,
            },
            cost: slot_cost(slot.kind),
            scheduled: false,
        }
    }

    fn from_equation_directions(equation: &EqualityEquation) -> Result<Vec<Self>, Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();
        let mut candidates = Vec::new();

        let Some(lhs_ref) = lhs_quantity_ref(equation) else {
            return Err(vec![Diagnostic::error(format!(
                "equation '{}' does not assign to a quantity reference on the lhs",
                equation.block_name
            ))]);
        };

        let forward_deps = collect_dependencies(&equation.rhs).map_err(|diag| vec![diag])?;
        candidates.push(Self {
            name: equation.block_name.clone(),
            source: PlanSource::Equation(equation.block_name.clone()),
            direction: CandidateDirection::Forward,
            timing: timing_from_reference(&lhs_ref, equation).map_err(|diag| vec![diag])?,
            outputs: vec![lhs_ref.quantity],
            payload: CandidatePayload::Equation {
                kind: equation.kind,
                expression: equation.rhs.clone(),
                dependencies: forward_deps,
            },
            cost: 10,
            scheduled: false,
        });

        if equation.kind == BlockKind::Relation {
            match inverted_candidates(equation, lhs_ref) {
                Ok(mut more) => candidates.append(&mut more),
                Err(err) => diagnostics.push(err),
            }
        }

        if diagnostics.is_empty() {
            Ok(candidates)
        } else {
            Err(diagnostics)
        }
    }

    fn dependencies_ready(&self, available_current: &HashSet<QuantityId>) -> bool {
        match &self.payload {
            CandidatePayload::Slot { inputs, .. } => inputs
                .iter()
                .all(|quantity| available_current.contains(quantity)),
            CandidatePayload::Equation { dependencies, .. } => {
                dependencies.iter().all(|dependency| match dependency {
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
        }
    }

    fn current_dependencies(&self) -> Vec<QuantityId> {
        match &self.payload {
            CandidatePayload::Slot { inputs, .. } => inputs.clone(),
            CandidatePayload::Equation { dependencies, .. } => dependencies
                .iter()
                .filter_map(|dependency| match dependency {
                    Dependency {
                        timing: DependencyTiming::Current,
                        quantity: Some(quantity),
                    } => Some(*quantity),
                    _ => None,
                })
                .collect(),
        }
    }

    fn missing_current_dependencies(
        &self,
        available_current: &HashSet<QuantityId>,
    ) -> Vec<QuantityId> {
        match &self.payload {
            CandidatePayload::Slot { inputs, .. } => inputs
                .iter()
                .copied()
                .filter(|quantity| !available_current.contains(quantity))
                .collect(),
            CandidatePayload::Equation { dependencies, .. } => dependencies
                .iter()
                .filter_map(|dependency| match dependency {
                    Dependency {
                        timing: DependencyTiming::Current,
                        quantity: Some(quantity),
                    } if !available_current.contains(quantity) => Some(*quantity),
                    _ => None,
                })
                .collect(),
        }
    }

    fn source_rank(&self) -> u8 {
        match self.source {
            PlanSource::Slot(_) => 0,
            PlanSource::Equation(_) => 1,
        }
    }

    fn primary_output_rank(&self) -> usize {
        self.outputs
            .first()
            .map(|quantity| quantity.0)
            .unwrap_or(usize::MAX)
    }

    fn alternative_payload(&self) -> AlternativePayload {
        match &self.payload {
            CandidatePayload::Slot { inputs, .. } => AlternativePayload::Slot {
                inputs: inputs.clone(),
            },
            CandidatePayload::Equation { expression, .. } => AlternativePayload::Equation {
                expression: expression.clone(),
            },
        }
    }
}

fn lhs_quantity_ref(equation: &EqualityEquation) -> Option<QuantityRef> {
    match &equation.lhs {
        CoreExpr::Quantity(reference) => Some(reference.clone()),
        _ => None,
    }
}

fn timing_from_reference(
    reference: &QuantityRef,
    equation: &EqualityEquation,
) -> Result<CandidateTiming, Diagnostic> {
    match reference.time {
        TimeReference::Implicit | TimeReference::Relative(0) => Ok(CandidateTiming::Current),
        TimeReference::Relative(1) => Ok(CandidateTiming::Next),
        other => Err(Diagnostic::error(format!(
            "equation '{}' uses unsupported lhs time reference {:?} in v1",
            equation.block_name, other
        ))),
    }
}

fn inverted_candidates(
    equation: &EqualityEquation,
    lhs_ref: QuantityRef,
) -> Result<Vec<Candidate>, Diagnostic> {
    if !matches!(
        lhs_ref.time,
        TimeReference::Implicit | TimeReference::Relative(0)
    ) {
        return Ok(Vec::new());
    }

    let lhs_expr = CoreExpr::Quantity(lhs_ref);
    let CoreExpr::Binary { op, left, right } = &equation.rhs else {
        return Ok(Vec::new());
    };

    let mut candidates = Vec::new();

    if let Some(target) = invertible_target(left) {
        let expression = invert_for_left(*op, lhs_expr.clone(), (**right).clone());
        let dependencies = collect_dependencies(&expression)?;
        candidates.push(Candidate {
            name: equation.block_name.clone(),
            source: PlanSource::Equation(equation.block_name.clone()),
            direction: CandidateDirection::Inverted,
            timing: CandidateTiming::Current,
            outputs: vec![target],
            payload: CandidatePayload::Equation {
                kind: equation.kind,
                expression,
                dependencies,
            },
            cost: 20,
            scheduled: false,
        });
    }

    if let Some(target) = invertible_target(right) {
        let expression = invert_for_right(*op, lhs_expr, (**left).clone(), (**right).clone());
        let dependencies = collect_dependencies(&expression)?;
        candidates.push(Candidate {
            name: equation.block_name.clone(),
            source: PlanSource::Equation(equation.block_name.clone()),
            direction: CandidateDirection::Inverted,
            timing: CandidateTiming::Current,
            outputs: vec![target],
            payload: CandidatePayload::Equation {
                kind: equation.kind,
                expression,
                dependencies,
            },
            cost: 20,
            scheduled: false,
        });
    }

    Ok(candidates)
}

fn invertible_target(expr: &CoreExpr) -> Option<QuantityId> {
    match expr {
        CoreExpr::Quantity(reference)
            if matches!(
                reference.time,
                TimeReference::Implicit | TimeReference::Relative(0)
            ) =>
        {
            Some(reference.quantity)
        }
        _ => None,
    }
}

fn invert_for_left(op: crate::semantic::BinaryOp, lhs: CoreExpr, right: CoreExpr) -> CoreExpr {
    use crate::semantic::BinaryOp;

    match op {
        BinaryOp::Add => CoreExpr::Binary {
            op: BinaryOp::Sub,
            left: Box::new(lhs),
            right: Box::new(right),
        },
        BinaryOp::Sub => CoreExpr::Binary {
            op: BinaryOp::Add,
            left: Box::new(lhs),
            right: Box::new(right),
        },
        BinaryOp::Mul => CoreExpr::Binary {
            op: BinaryOp::Div,
            left: Box::new(lhs),
            right: Box::new(right),
        },
        BinaryOp::Div => CoreExpr::Binary {
            op: BinaryOp::Mul,
            left: Box::new(lhs),
            right: Box::new(right),
        },
    }
}

fn invert_for_right(
    op: crate::semantic::BinaryOp,
    lhs: CoreExpr,
    left: CoreExpr,
    _right: CoreExpr,
) -> CoreExpr {
    use crate::semantic::BinaryOp;

    match op {
        BinaryOp::Add => CoreExpr::Binary {
            op: BinaryOp::Sub,
            left: Box::new(lhs),
            right: Box::new(left),
        },
        BinaryOp::Sub => CoreExpr::Binary {
            op: BinaryOp::Sub,
            left: Box::new(left),
            right: Box::new(lhs),
        },
        BinaryOp::Mul => CoreExpr::Binary {
            op: BinaryOp::Div,
            left: Box::new(lhs),
            right: Box::new(left),
        },
        BinaryOp::Div => CoreExpr::Binary {
            op: BinaryOp::Div,
            left: Box::new(left),
            right: Box::new(lhs),
        },
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
                    )));
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

fn slot_cost(kind: SlotBindingKind) -> u32 {
    match kind {
        SlotBindingKind::DataSeries | SlotBindingKind::Constant => 0,
        SlotBindingKind::Learned => 5,
    }
}

fn detect_cycle(candidates: &[&Candidate]) -> Option<Vec<QuantityId>> {
    let outputs = candidates
        .iter()
        .flat_map(|candidate| candidate.outputs.iter().copied())
        .collect::<HashSet<_>>();
    if outputs.is_empty() {
        return None;
    }

    let mut adjacency = HashMap::<QuantityId, Vec<QuantityId>>::new();
    for candidate in candidates {
        for output in &candidate.outputs {
            let deps = candidate
                .current_dependencies()
                .into_iter()
                .filter(|quantity| outputs.contains(quantity))
                .collect::<Vec<_>>();
            adjacency.insert(*output, deps);
        }
    }

    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    let mut in_stack = HashSet::new();

    for node in adjacency.keys().copied() {
        if let Some(cycle) = dfs_cycle(node, &adjacency, &mut visited, &mut stack, &mut in_stack) {
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
            CompileMode, CompileSpec, DirectBindingKind, DirectBindingSpec, InitialStateSource,
            LossKind, ObservationSchedule, ObservationSpec, SlotBindingKind, SlotBindingSpec,
            bind_compile_spec,
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
        assert!(plan.alternatives.len() >= 1);
        assert!(plan.unresolved_current.is_empty());

        assert_eq!(plan.slot_steps[0].slot, "controller");
        assert_eq!(plan.equation_steps[0].block_name, "demand_transpiration");
        assert_eq!(plan.equation_steps[0].direction, EquationDirection::Forward);
        assert_eq!(plan.temporal_steps[0].block_name, "water_step");
        assert!(plan.alternatives.iter().any(|alternative| {
            alternative.source == PlanSource::Equation("supply_transpiration".to_string())
                && alternative.direction == CandidateDirection::Forward
        }));
    }

    #[test]
    fn uses_inverted_direction_when_output_is_data_bound() {
        let source = r#"
model Recover

external vpd_scale : scalar
state water : scalar
node stomata : scalar
node transpiration : scalar

relation demand_transpiration:
  transpiration = stomata * vpd_scale

temporal water_step:
  water[t+1] = water[t] - transpiration[t]
"#;

        let syntax = parse_and_validate(source).expect("syntax should validate");
        let semantic = semantic::lower_model(&syntax).expect("semantic lowering should succeed");
        let equality = equality::lower_model(&semantic).expect("equality lowering should succeed");
        let bound = bind_compile_spec(
            &equality,
            &CompileSpec {
                mode: CompileMode::Fit,
                horizon_steps: 12,
                direct_bindings: vec![
                    DirectBindingSpec {
                        quantity: "vpd_scale".to_string(),
                        kind: DirectBindingKind::DataSeries {
                            steps: (0..12).collect(),
                        },
                    },
                    DirectBindingSpec {
                        quantity: "transpiration".to_string(),
                        kind: DirectBindingKind::DataSeries {
                            steps: (0..12).collect(),
                        },
                    },
                    DirectBindingSpec {
                        quantity: "water".to_string(),
                        kind: DirectBindingKind::InitialState {
                            source: InitialStateSource::Constant,
                        },
                    },
                ],
                slot_bindings: vec![],
                observations: vec![],
            },
        )
        .expect("binding should succeed");

        let plan = build_single_step_plan(&bound).expect("planning should succeed");
        let stomata_id = bound
            .quantities
            .iter()
            .find(|quantity| quantity.quantity.name == "stomata")
            .map(|quantity| quantity.quantity.id)
            .expect("stomata should exist");

        let equation = plan
            .equation_steps
            .iter()
            .find(|equation| equation.output == stomata_id)
            .expect("stomata should be solved by inversion");
        assert_eq!(equation.direction, EquationDirection::Inverted);
        assert_eq!(equation.block_name, "demand_transpiration");
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
                    loss: LossKind::Mse,
                    schedule: ObservationSchedule::DensePerStep,
                }],
            },
        )
        .expect("binding should succeed")
    }
}
