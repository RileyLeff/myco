use std::{
    collections::{HashMap, HashSet},
    fmt,
    sync::Arc,
};

use crate::{
    constraints::QuantityConstraintSet,
    diagnostics::{Diagnostic, SourceSpan},
    dimensions::QuantityTypeInfo,
    egraph::{self, EqualityCore},
    semantic::{BinaryOp, Equation, Expr, SemanticModel},
    syntax::{BlockKind, QuantityKind},
};

#[derive(Debug, Clone)]
pub struct EqualityModel {
    pub name: String,
    pub quantities: Vec<Quantity>,
    pub persistent_quantities: Vec<QuantityId>,
    pub core: Arc<EqualityCore>,
    pub slots: Vec<EqualitySlot>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuantityId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EquationId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    pub span: SourceSpan,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quantity {
    pub id: QuantityId,
    pub kind: QuantityKind,
    pub name: String,
    pub ty: String,
    pub type_info: QuantityTypeInfo,
    pub dimension: crate::dimensions::Dimension,
    pub raw_constraints: Vec<String>,
    pub constraint_set: QuantityConstraintSet,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EqualityEquation {
    pub id: EquationId,
    pub kind: BlockKind,
    pub block_name: String,
    pub lhs: CoreExpr,
    pub rhs: CoreExpr,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EqualitySlot {
    pub name: String,
    pub provides: Vec<QuantityId>,
    pub inputs: Vec<QuantityId>,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreExpr {
    Quantity(QuantityRef),
    Special(SpecialRef),
    Number(String),
    Binary {
        op: BinaryOp,
        left: Box<CoreExpr>,
        right: Box<CoreExpr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantityRef {
    pub quantity: QuantityId,
    pub time: TimeReference,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeReference {
    Implicit,
    Relative(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialRef {
    Dt,
}

impl fmt::Display for CoreExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreExpr::Quantity(reference) => match reference.time {
                TimeReference::Implicit => write!(f, "q{}", reference.quantity.0),
                TimeReference::Relative(offset) => {
                    if offset == 0 {
                        write!(f, "q{}[t]", reference.quantity.0)
                    } else if offset > 0 {
                        write!(f, "q{}[t+{}]", reference.quantity.0, offset)
                    } else {
                        write!(f, "q{}[t{}]", reference.quantity.0, offset)
                    }
                }
            },
            CoreExpr::Special(SpecialRef::Dt) => write!(f, "dt"),
            CoreExpr::Number(number) => write!(f, "{number}"),
            CoreExpr::Binary { op, left, right } => {
                let op_str = match op {
                    BinaryOp::Add => "+",
                    BinaryOp::Sub => "-",
                    BinaryOp::Mul => "*",
                    BinaryOp::Div => "/",
                };
                write!(f, "({left} {op_str} {right})")
            }
        }
    }
}

pub fn lower_model(model: &SemanticModel) -> Result<EqualityModel, Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    let mut quantities = Vec::new();
    let mut quantity_ids = HashMap::new();

    for (index, declaration) in model.declarations.iter().enumerate() {
        let id = QuantityId(index);
        let type_info = crate::dimensions::parse_quantity_type(&declaration.ty);
        quantity_ids.insert(declaration.name.clone(), id);
        quantities.push(Quantity {
            id,
            kind: declaration.kind,
            name: declaration.name.clone(),
            ty: declaration.ty.clone(),
            dimension: type_info.dimension.clone(),
            type_info,
            raw_constraints: declaration.constraints.clone(),
            constraint_set: crate::constraints::parse_constraint_set(&declaration.constraints),
            provenance: Provenance {
                span: declaration.span,
                label: declaration.name.clone(),
            },
        });
    }

    let mut equations = Vec::new();
    let mut equation_index = 0usize;
    for block in &model.relations {
        for equation in &block.equations {
            match lower_equation(
                equation,
                EquationId(equation_index),
                block.kind,
                &block.name,
                &quantity_ids,
                block.span,
            ) {
                Ok(lowered) => equations.push(lowered),
                Err(mut errs) => diagnostics.append(&mut errs),
            }
            equation_index += 1;
        }
    }

    let mut slots = Vec::new();
    for slot in &model.slots {
        let mut provides = Vec::new();
        for provided in &slot.provides {
            match quantity_ids.get(provided) {
                Some(id) => provides.push(*id),
                None => diagnostics.push(
                    Diagnostic::error(format!(
                        "slot '{}' provides unknown quantity '{}'",
                        slot.name, provided
                    ))
                    .with_span(slot.span),
                ),
            }
        }

        let mut inputs = Vec::new();
        for input in &slot.inputs {
            match quantity_ids.get(input) {
                Some(id) => inputs.push(*id),
                None => diagnostics.push(
                    Diagnostic::error(format!(
                        "slot '{}' references unknown input '{}'",
                        slot.name, input
                    ))
                    .with_span(slot.span),
                ),
            }
        }

        slots.push(EqualitySlot {
            name: slot.name.clone(),
            provides,
            inputs,
            provenance: Provenance {
                span: slot.span,
                label: slot.name.clone(),
            },
        });
    }

    if diagnostics.is_empty() {
        let persistent_quantities = infer_persistent_quantities(&quantities, &equations);
        let core = Arc::new(egraph::build_core(&equations));
        Ok(EqualityModel {
            name: model.name.clone(),
            quantities,
            persistent_quantities,
            core,
            slots,
        })
    } else {
        Err(diagnostics)
    }
}

fn lower_equation(
    equation: &Equation,
    id: EquationId,
    kind: BlockKind,
    block_name: &str,
    quantity_ids: &HashMap<String, QuantityId>,
    block_span: SourceSpan,
) -> Result<EqualityEquation, Vec<Diagnostic>> {
    let lhs = lower_expr(&equation.lhs, quantity_ids, equation.span)?;
    let rhs = lower_expr(&equation.rhs, quantity_ids, equation.span)?;

    Ok(EqualityEquation {
        id,
        kind,
        block_name: block_name.to_string(),
        lhs,
        rhs,
        provenance: Provenance {
            span: block_span,
            label: block_name.to_string(),
        },
    })
}

fn lower_expr(
    expr: &Expr,
    quantity_ids: &HashMap<String, QuantityId>,
    span: SourceSpan,
) -> Result<CoreExpr, Vec<Diagnostic>> {
    match expr {
        Expr::Symbol(raw) => lower_symbol(raw, quantity_ids, span),
        Expr::Number(number) => Ok(CoreExpr::Number(number.clone())),
        Expr::Binary { op, left, right } => {
            let left = lower_expr(left, quantity_ids, span)?;
            let right = lower_expr(right, quantity_ids, span)?;
            Ok(CoreExpr::Binary {
                op: *op,
                left: Box::new(left),
                right: Box::new(right),
            })
        }
    }
}

fn lower_symbol(
    raw: &str,
    quantity_ids: &HashMap<String, QuantityId>,
    span: SourceSpan,
) -> Result<CoreExpr, Vec<Diagnostic>> {
    if raw == "dt" {
        return Ok(CoreExpr::Special(SpecialRef::Dt));
    }

    let (base, time) = parse_symbol_reference(raw)
        .map_err(|message| vec![Diagnostic::error(message).with_span(span)])?;

    let quantity = quantity_ids.get(base).copied().ok_or_else(|| {
        vec![Diagnostic::error(format!("reference to unknown quantity '{base}'")).with_span(span)]
    })?;

    Ok(CoreExpr::Quantity(QuantityRef { quantity, time }))
}

fn parse_symbol_reference(raw: &str) -> Result<(&str, TimeReference), String> {
    let Some((base, suffix)) = raw.split_once('[') else {
        return Ok((raw, TimeReference::Implicit));
    };

    let suffix = suffix
        .strip_suffix(']')
        .ok_or_else(|| format!("unterminated index expression in '{raw}'"))?;

    let time = parse_time_reference(suffix)?;
    Ok((base, time))
}

fn parse_time_reference(input: &str) -> Result<TimeReference, String> {
    let trimmed = input.trim();
    if trimmed == "t" {
        return Ok(TimeReference::Relative(0));
    }

    let rest = trimmed
        .strip_prefix('t')
        .ok_or_else(|| format!("unsupported index expression '{trimmed}'"))?;
    if rest.is_empty() {
        return Ok(TimeReference::Relative(0));
    }

    let offset = rest
        .parse::<i32>()
        .map_err(|_| format!("unsupported index expression '{trimmed}'"))?;
    Ok(TimeReference::Relative(offset))
}

fn infer_persistent_quantities(
    quantities: &[Quantity],
    equations: &[EqualityEquation],
) -> Vec<QuantityId> {
    let mut persistent = HashSet::new();
    for quantity in quantities {
        if quantity.kind == QuantityKind::State {
            persistent.insert(quantity.id);
        }
    }
    for equation in equations {
        if equation.kind != BlockKind::Temporal {
            continue;
        }
        if let Some(quantity) = persistent_lhs_quantity(&equation.lhs) {
            persistent.insert(quantity);
        }
    }

    let mut persistent = persistent.into_iter().collect::<Vec<_>>();
    persistent.sort_by_key(|quantity| quantity.0);
    persistent
}

fn persistent_lhs_quantity(expr: &CoreExpr) -> Option<QuantityId> {
    match expr {
        CoreExpr::Quantity(QuantityRef {
            quantity,
            time: TimeReference::Relative(_),
        }) => Some(*quantity),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{semantic, syntax::parse_and_validate};

    const TINY_TREE: &str = include_str!("../tests/fixtures/tiny_tree.myco");

    #[test]
    fn lowers_tiny_tree_into_equality_core() {
        let syntax = parse_and_validate(TINY_TREE).expect("fixture should validate");
        let semantic = semantic::lower_model(&syntax).expect("semantic lowering should succeed");
        let equality = lower_model(&semantic).expect("equality lowering should succeed");

        assert_eq!(equality.name, "TinyTree");
        assert_eq!(equality.quantities.len(), 8);
        assert_eq!(
            equality.persistent_quantities,
            vec![QuantityId(3), QuantityId(4)]
        );
        assert_eq!(equality.core.equations.len(), 3);
        assert_eq!(equality.slots.len(), 1);

        let supply = equality
            .core
            .equations
            .iter()
            .find(|registration| registration.equation.block_name == "supply_transpiration")
            .expect("supply equation should exist");
        assert!(matches!(
            supply.equation.lhs,
            CoreExpr::Quantity(QuantityRef {
                time: TimeReference::Implicit,
                ..
            })
        ));
    }

    #[test]
    fn resolves_temporal_and_special_references() {
        let syntax = parse_and_validate(TINY_TREE).expect("fixture should validate");
        let semantic = semantic::lower_model(&syntax).expect("semantic lowering should succeed");
        let equality = lower_model(&semantic).expect("equality lowering should succeed");

        let water_step = equality
            .core
            .equations
            .iter()
            .find(|registration| registration.equation.block_name == "water_step")
            .expect("water step should exist");

        assert_eq!(
            water_step.equation.lhs,
            CoreExpr::Quantity(QuantityRef {
                quantity: QuantityId(3),
                time: TimeReference::Relative(1),
            })
        );

        let rhs_text = water_step.equation.rhs.to_string();
        assert!(rhs_text.contains("dt"));
    }

    #[test]
    fn infers_persistent_quantities_from_temporal_lhs() {
        let source = r#"
model TemporalNode

node stock : scalar
node flow : scalar

temporal stock_step:
  stock[t+1] = stock[t] + flow[t]
"#;

        let syntax = parse_and_validate(source).expect("syntax should validate");
        let semantic = semantic::lower_model(&syntax).expect("semantic lowering should succeed");
        let equality = lower_model(&semantic).expect("equality lowering should succeed");

        assert_eq!(equality.persistent_quantities, vec![QuantityId(0)]);
    }

    #[test]
    fn rejects_unknown_symbol_references() {
        let source = r#"
model Broken

node y : scalar

relation bad:
  y = x + 1
"#;

        let syntax = parse_and_validate(source).expect("syntax should validate");
        let semantic = semantic::lower_model(&syntax).expect("semantic lowering should succeed");
        let diagnostics = lower_model(&semantic).expect_err("equality lowering should fail");

        assert_eq!(diagnostics.len(), 1);
        assert!(
            diagnostics[0]
                .message
                .contains("reference to unknown quantity 'x'")
        );
    }
}
