use std::collections::HashSet;

use egg::{
    AstSize, CostFunction, EGraph, Extractor, Id, RecExpr, Rewrite, Runner, Symbol,
    define_language, rewrite as rw,
};

use crate::{
    diagnostics::Diagnostic,
    equality::{
        CoreExpr, EqualityEquation, EquationId, QuantityId, QuantityRef, SpecialRef, TimeReference,
    },
    semantic::BinaryOp,
    syntax::BlockKind,
};

const UNAVAILABLE_PENALTY: usize = 1_000_000;

#[derive(Debug, Clone)]
pub struct EqualityCore {
    pub egraph: EGraph<ArithmeticLang, ()>,
    pub equations: Vec<EquationRegistration>,
    pub directional: Vec<DirectionalRegistration>,
}

#[derive(Debug, Clone)]
pub struct EquationRegistration {
    pub equation: EqualityEquation,
    pub lhs_id: Id,
    pub rhs_id: Id,
    pub lhs_class: Id,
    pub rhs_class: Id,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectionalRegistration {
    pub equation_id: EquationId,
    pub block_name: String,
    pub kind: BlockKind,
    pub output: QuantityRef,
    pub seed_expression: CoreExpr,
    pub direction: ExpressionDirection,
    pub output_id: Id,
    pub expression_id: Id,
    pub output_class: Id,
    pub base_cost: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpressionDirection {
    Forward,
    Inverted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractedExpression {
    pub expression: CoreExpr,
    pub structural_cost: u32,
}

define_language! {
    pub enum ArithmeticLang {
        Symbol(Symbol),
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "*" = Mul([Id; 2]),
        "/" = Div([Id; 2]),
    }
}

pub fn build_core(equations: &[EqualityEquation]) -> EqualityCore {
    let mut egraph = EGraph::<ArithmeticLang, ()>::default();
    let mut registrations = Vec::new();
    let mut directional = Vec::new();

    for equation in equations {
        let lhs_id = add_expr(&mut egraph, &equation.lhs);
        let rhs_id = add_expr(&mut egraph, &equation.rhs);
        egraph.union(lhs_id, rhs_id);
        registrations.push(EquationRegistration {
            equation: equation.clone(),
            lhs_id,
            rhs_id,
            lhs_class: lhs_id,
            rhs_class: rhs_id,
        });

        for registration in directional_registrations(equation) {
            let output_expr = CoreExpr::Quantity(registration.output.clone());
            let output_id = add_expr(&mut egraph, &output_expr);
            let expression_id = add_expr(&mut egraph, &registration.seed_expression);
            egraph.union(output_id, expression_id);
            directional.push(DirectionalRegistration {
                equation_id: registration.equation_id,
                block_name: registration.block_name,
                kind: registration.kind,
                output: registration.output,
                seed_expression: registration.seed_expression,
                direction: registration.direction,
                output_id,
                expression_id,
                output_class: output_id,
                base_cost: registration.base_cost,
            });
        }
    }
    egraph.rebuild();

    let rewrites = arithmetic_rewrites();
    let runner = Runner::default().with_egraph(egraph).run(&rewrites);
    let egraph = runner.egraph;
    for registration in &mut registrations {
        registration.lhs_class = egraph.find(registration.lhs_id);
        registration.rhs_class = egraph.find(registration.rhs_id);
    }
    for registration in &mut directional {
        registration.output_class = egraph.find(registration.output_id);
    }

    EqualityCore {
        egraph,
        equations: registrations,
        directional,
    }
}

pub fn extract_best(expr: &CoreExpr, egraph: &EGraph<ArithmeticLang, ()>) -> String {
    let recexpr = to_recexpr(expr);
    let id = egraph
        .lookup_expr(&recexpr)
        .unwrap_or_else(|| panic!("expression not found in egraph"));
    let extractor = Extractor::new(egraph, AstSize);
    let (_, best) = extractor.find_best(id);
    best.to_string()
}

pub fn extract_available_expression(
    _core: &EqualityCore,
    registration: &DirectionalRegistration,
    available_current: &HashSet<QuantityId>,
    forbid_output_leaf: bool,
) -> Result<Option<ExtractedExpression>, Diagnostic> {
    let mut local_egraph = EGraph::<ArithmeticLang, ()>::default();
    let root_id = add_expr(&mut local_egraph, &registration.seed_expression);
    let rewrites = arithmetic_rewrites();
    let runner = Runner::default().with_egraph(local_egraph).run(&rewrites);
    let local_egraph = runner.egraph;
    let root_class = local_egraph.find(root_id);
    let extractor = Extractor::new(
        &local_egraph,
        AvailabilityCost {
            available_current: available_current.clone(),
            forbidden_output: forbid_output_leaf.then_some(registration.output.clone()),
        },
    );
    let (cost, best) = extractor.find_best(root_class);
    if cost >= UNAVAILABLE_PENALTY {
        return Ok(None);
    }

    let expression = recexpr_to_core(&best)?;
    Ok(Some(ExtractedExpression {
        expression,
        structural_cost: cost as u32,
    }))
}

fn arithmetic_rewrites() -> Vec<Rewrite<ArithmeticLang, ()>> {
    vec![
        rw!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        rw!("commute-mul"; "(* ?a ?b)" => "(* ?b ?a)"),
        rw!("sub-add-right"; "(- (+ ?a ?b) ?b)" => "?a"),
        rw!("sub-add-left"; "(- (+ ?a ?b) ?a)" => "?b"),
        rw!("add-sub-right"; "(+ (- ?a ?b) ?b)" => "?a"),
        rw!("sub-self-diff"; "(- ?a (- ?a ?b))" => "?b"),
        rw!("div-mul-right"; "(/ (* ?a ?b) ?b)" => "?a"),
        rw!("div-mul-left"; "(/ (* ?a ?b) ?a)" => "?b"),
        rw!("mul-div-right"; "(* (/ ?a ?b) ?b)" => "?a"),
        rw!("div-self-ratio"; "(/ ?a (/ ?a ?b))" => "?b"),
    ]
}

pub fn add_expr(egraph: &mut EGraph<ArithmeticLang, ()>, expr: &CoreExpr) -> Id {
    let recexpr = to_recexpr(expr);
    egraph.add_expr(&recexpr)
}

fn directional_registrations(equation: &EqualityEquation) -> Vec<DirectionalSeed> {
    let Some(lhs_ref) = lhs_quantity_ref(equation) else {
        return Vec::new();
    };

    let mut seeds = vec![DirectionalSeed {
        equation_id: equation.id,
        block_name: equation.block_name.clone(),
        kind: equation.kind,
        output: lhs_ref.clone(),
        seed_expression: equation.rhs.clone(),
        direction: ExpressionDirection::Forward,
        base_cost: 10,
    }];

    if equation.kind == BlockKind::Relation
        && matches!(
            lhs_ref.time,
            TimeReference::Implicit | TimeReference::Relative(0)
        )
    {
        let lhs_expr = CoreExpr::Quantity(lhs_ref);
        let CoreExpr::Binary { op, left, right } = &equation.rhs else {
            return seeds;
        };

        if let Some(target) = invertible_target(left) {
            seeds.push(DirectionalSeed {
                equation_id: equation.id,
                block_name: equation.block_name.clone(),
                kind: equation.kind,
                output: QuantityRef {
                    quantity: target,
                    time: TimeReference::Implicit,
                },
                seed_expression: invert_for_left(*op, lhs_expr.clone(), (**right).clone()),
                direction: ExpressionDirection::Inverted,
                base_cost: 20,
            });
        }

        if let Some(target) = invertible_target(right) {
            seeds.push(DirectionalSeed {
                equation_id: equation.id,
                block_name: equation.block_name.clone(),
                kind: equation.kind,
                output: QuantityRef {
                    quantity: target,
                    time: TimeReference::Implicit,
                },
                seed_expression: invert_for_right(*op, lhs_expr, (**left).clone()),
                direction: ExpressionDirection::Inverted,
                base_cost: 20,
            });
        }
    }

    seeds
}

fn lhs_quantity_ref(equation: &EqualityEquation) -> Option<QuantityRef> {
    match &equation.lhs {
        CoreExpr::Quantity(reference) => Some(reference.clone()),
        _ => None,
    }
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

fn invert_for_left(op: BinaryOp, lhs: CoreExpr, right: CoreExpr) -> CoreExpr {
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

fn invert_for_right(op: BinaryOp, lhs: CoreExpr, left: CoreExpr) -> CoreExpr {
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

fn to_recexpr(expr: &CoreExpr) -> RecExpr<ArithmeticLang> {
    let mut nodes = Vec::<ArithmeticLang>::new();
    build_recexpr(expr, &mut nodes);
    RecExpr::from(nodes)
}

fn build_recexpr(expr: &CoreExpr, nodes: &mut Vec<ArithmeticLang>) -> Id {
    match expr {
        CoreExpr::Quantity(QuantityRef { quantity, time }) => {
            nodes.push(ArithmeticLang::Symbol(
                quantity_symbol(*quantity, *time).into(),
            ));
        }
        CoreExpr::Special(SpecialRef::Dt) => {
            nodes.push(ArithmeticLang::Symbol("dt".into()));
        }
        CoreExpr::Number(number) => {
            nodes.push(ArithmeticLang::Symbol(number.clone().into()));
        }
        CoreExpr::Binary { op, left, right } => {
            let left = build_recexpr(left, nodes);
            let right = build_recexpr(right, nodes);
            let node = match op {
                BinaryOp::Add => ArithmeticLang::Add([left, right]),
                BinaryOp::Sub => ArithmeticLang::Sub([left, right]),
                BinaryOp::Mul => ArithmeticLang::Mul([left, right]),
                BinaryOp::Div => ArithmeticLang::Div([left, right]),
            };
            nodes.push(node);
        }
    }

    Id::from(nodes.len() - 1)
}

fn recexpr_to_core(expr: &RecExpr<ArithmeticLang>) -> Result<CoreExpr, Diagnostic> {
    let mut stack: Vec<CoreExpr> = Vec::with_capacity(expr.as_ref().len());
    for node in expr.as_ref() {
        let lowered = match node {
            ArithmeticLang::Symbol(symbol) => parse_symbol(symbol)?,
            ArithmeticLang::Add([left, right]) => CoreExpr::Binary {
                op: BinaryOp::Add,
                left: Box::new(stack[usize::from(*left)].clone()),
                right: Box::new(stack[usize::from(*right)].clone()),
            },
            ArithmeticLang::Sub([left, right]) => CoreExpr::Binary {
                op: BinaryOp::Sub,
                left: Box::new(stack[usize::from(*left)].clone()),
                right: Box::new(stack[usize::from(*right)].clone()),
            },
            ArithmeticLang::Mul([left, right]) => CoreExpr::Binary {
                op: BinaryOp::Mul,
                left: Box::new(stack[usize::from(*left)].clone()),
                right: Box::new(stack[usize::from(*right)].clone()),
            },
            ArithmeticLang::Div([left, right]) => CoreExpr::Binary {
                op: BinaryOp::Div,
                left: Box::new(stack[usize::from(*left)].clone()),
                right: Box::new(stack[usize::from(*right)].clone()),
            },
        };
        stack.push(lowered);
    }

    stack
        .last()
        .cloned()
        .ok_or_else(|| Diagnostic::error("failed to reconstruct extracted expression"))
}

fn parse_symbol(symbol: &Symbol) -> Result<CoreExpr, Diagnostic> {
    let raw = symbol.as_str();
    if raw == "dt" {
        return Ok(CoreExpr::Special(SpecialRef::Dt));
    }
    if raw
        .chars()
        .all(|ch| ch.is_ascii_digit() || matches!(ch, '.' | '-' | 'e' | 'E'))
    {
        return Ok(CoreExpr::Number(raw.to_string()));
    }

    let Some(quantity) = raw.strip_prefix('q') else {
        return Err(Diagnostic::error(format!(
            "failed to parse extracted symbol '{}'",
            raw
        )));
    };

    let (index, time) = match quantity.split_once('@') {
        Some((index, offset)) => {
            let quantity = index.parse::<usize>().map_err(|_| {
                Diagnostic::error(format!("failed to parse extracted quantity '{}'", raw))
            })?;
            let offset = offset.parse::<i32>().map_err(|_| {
                Diagnostic::error(format!(
                    "failed to parse extracted time reference '{}'",
                    raw
                ))
            })?;
            (quantity, TimeReference::Relative(offset))
        }
        None => {
            let quantity = quantity.parse::<usize>().map_err(|_| {
                Diagnostic::error(format!("failed to parse extracted quantity '{}'", raw))
            })?;
            (quantity, TimeReference::Implicit)
        }
    };

    Ok(CoreExpr::Quantity(QuantityRef {
        quantity: QuantityId(index),
        time,
    }))
}

fn quantity_symbol(quantity: QuantityId, time: TimeReference) -> String {
    match time {
        TimeReference::Implicit => format!("q{}", quantity.0),
        TimeReference::Relative(offset) => format!("q{}@{}", quantity.0, offset),
    }
}

#[derive(Debug, Clone)]
struct DirectionalSeed {
    equation_id: EquationId,
    block_name: String,
    kind: BlockKind,
    output: QuantityRef,
    seed_expression: CoreExpr,
    direction: ExpressionDirection,
    base_cost: u32,
}

#[derive(Debug, Clone)]
struct AvailabilityCost {
    available_current: HashSet<QuantityId>,
    forbidden_output: Option<QuantityRef>,
}

impl CostFunction<ArithmeticLang> for AvailabilityCost {
    type Cost = usize;

    fn cost<C>(&mut self, enode: &ArithmeticLang, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        match enode {
            ArithmeticLang::Symbol(symbol) => self.leaf_cost(symbol),
            ArithmeticLang::Add([left, right])
            | ArithmeticLang::Sub([left, right])
            | ArithmeticLang::Mul([left, right])
            | ArithmeticLang::Div([left, right]) => 1 + costs(*left) + costs(*right),
        }
    }
}

impl AvailabilityCost {
    fn leaf_cost(&self, symbol: &Symbol) -> usize {
        let raw = symbol.as_str();
        if raw == "dt" || raw.parse::<f64>().is_ok() {
            return 0;
        }

        let Some(quantity) = raw.strip_prefix('q') else {
            return UNAVAILABLE_PENALTY;
        };
        let (index, time) = match quantity.split_once('@') {
            Some((index, offset)) => {
                let quantity = match index.parse::<usize>() {
                    Ok(quantity) => quantity,
                    Err(_) => return UNAVAILABLE_PENALTY,
                };
                let offset = match offset.parse::<i32>() {
                    Ok(offset) => offset,
                    Err(_) => return UNAVAILABLE_PENALTY,
                };
                (QuantityId(quantity), TimeReference::Relative(offset))
            }
            None => {
                let quantity = match quantity.parse::<usize>() {
                    Ok(quantity) => quantity,
                    Err(_) => return UNAVAILABLE_PENALTY,
                };
                (QuantityId(quantity), TimeReference::Implicit)
            }
        };

        match time {
            TimeReference::Implicit | TimeReference::Relative(0) => {
                if self.forbidden_output
                    == Some(QuantityRef {
                        quantity: index,
                        time,
                    })
                {
                    return UNAVAILABLE_PENALTY;
                }
                if self.available_current.contains(&index) {
                    0
                } else {
                    UNAVAILABLE_PENALTY
                }
            }
            TimeReference::Relative(1) => UNAVAILABLE_PENALTY,
            TimeReference::Relative(_) => UNAVAILABLE_PENALTY,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{equality, semantic, syntax::parse_and_validate};

    const TINY_TREE: &str = include_str!("../tests/fixtures/tiny_tree.myco");

    #[test]
    fn unions_equation_lhs_and_rhs_into_same_class() {
        let syntax = parse_and_validate(TINY_TREE).expect("syntax");
        let semantic = semantic::lower_model(&syntax).expect("semantic");
        let equality = equality::lower_model(&semantic).expect("equality");
        let mut egraph = equality.core.egraph.clone();

        let equation = equality
            .core
            .equations
            .iter()
            .find(|registration| registration.equation.block_name == "demand_transpiration")
            .expect("equation");

        let lhs = add_expr(&mut egraph, &equation.equation.lhs);
        let rhs = add_expr(&mut egraph, &equation.equation.rhs);
        assert_eq!(egraph.find(lhs), egraph.find(rhs));
    }

    #[test]
    fn commutativity_rewrite_makes_equivalent_forms_extractable() {
        let source = r#"
model Commute

node x : scalar
node y : scalar
node z : scalar

relation pair:
  z = x + y
"#;

        let syntax = parse_and_validate(source).expect("syntax");
        let semantic = semantic::lower_model(&syntax).expect("semantic");
        let equality = equality::lower_model(&semantic).expect("equality");
        let mut egraph = equality.core.egraph.clone();

        let swapped = CoreExpr::Binary {
            op: BinaryOp::Add,
            left: Box::new(CoreExpr::Quantity(QuantityRef {
                quantity: QuantityId(1),
                time: TimeReference::Implicit,
            })),
            right: Box::new(CoreExpr::Quantity(QuantityRef {
                quantity: QuantityId(0),
                time: TimeReference::Implicit,
            })),
        };

        let swapped_id = add_expr(&mut egraph, &swapped);
        let canonical = equality
            .core
            .equations
            .iter()
            .find(|registration| registration.equation.block_name == "pair")
            .expect("equation");
        let rhs_id = add_expr(&mut egraph, &canonical.equation.rhs);
        assert_eq!(egraph.find(swapped_id), egraph.find(rhs_id));
    }

    #[test]
    fn seeds_inverted_directional_registrations() {
        let syntax = parse_and_validate(TINY_TREE).expect("syntax");
        let semantic = semantic::lower_model(&syntax).expect("semantic");
        let equality = equality::lower_model(&semantic).expect("equality");

        assert!(equality.core.directional.iter().any(|registration| {
            registration.block_name == "demand_transpiration"
                && registration.output.quantity == QuantityId(5)
                && registration.direction == ExpressionDirection::Inverted
        }));
    }

    #[test]
    fn extracts_available_inverted_expression_for_stomata() {
        let syntax = parse_and_validate(TINY_TREE).expect("syntax");
        let semantic = semantic::lower_model(&syntax).expect("semantic");
        let equality = equality::lower_model(&semantic).expect("equality");

        let registration = equality
            .core
            .directional
            .iter()
            .find(|registration| {
                registration.block_name == "demand_transpiration"
                    && registration.output.quantity == QuantityId(5)
            })
            .expect("registration");

        let available = HashSet::from([QuantityId(0), QuantityId(6)]);
        let extracted =
            extract_available_expression(&equality.core, registration, &available, true)
                .expect("extraction should succeed")
                .expect("expression should be available");

        assert_eq!(extracted.expression.to_string(), "(q6 / q0)");
    }

    #[test]
    fn candidate_extraction_does_not_steal_other_relation_expression() {
        let source = r#"
model NoLeak

external a : scalar
external b : scalar
external c : scalar
external d : scalar
node y : scalar

relation first:
  y = a + b

relation second:
  y = c + d
"#;

        let syntax = parse_and_validate(source).expect("syntax");
        let semantic = semantic::lower_model(&syntax).expect("semantic");
        let equality = equality::lower_model(&semantic).expect("equality");

        let first = equality
            .core
            .directional
            .iter()
            .find(|registration| {
                registration.block_name == "first"
                    && registration.output.quantity == QuantityId(4)
                    && registration.direction == ExpressionDirection::Forward
            })
            .expect("first registration");
        let second = equality
            .core
            .directional
            .iter()
            .find(|registration| {
                registration.block_name == "second"
                    && registration.output.quantity == QuantityId(4)
                    && registration.direction == ExpressionDirection::Forward
            })
            .expect("second registration");

        let only_second_available = HashSet::from([QuantityId(2), QuantityId(3)]);

        let first_extracted =
            extract_available_expression(&equality.core, first, &only_second_available, true)
                .expect("first extraction should succeed");
        let second_extracted =
            extract_available_expression(&equality.core, second, &only_second_available, true)
                .expect("second extraction should succeed");

        assert!(first_extracted.is_none());
        assert_eq!(
            second_extracted.expect("second path should be available").expression,
            CoreExpr::Binary {
                op: BinaryOp::Add,
                left: Box::new(CoreExpr::Quantity(QuantityRef {
                    quantity: QuantityId(2),
                    time: TimeReference::Implicit,
                })),
                right: Box::new(CoreExpr::Quantity(QuantityRef {
                    quantity: QuantityId(3),
                    time: TimeReference::Implicit,
                })),
            }
        );
    }
}
