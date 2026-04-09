use std::{collections::BTreeMap, fmt};

use crate::{
    constraints::ConstraintBound,
    diagnostics::Diagnostic,
    equality::{CoreExpr, EqualityModel, QuantityId, SpecialRef},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantityTypeInfo {
    pub base_type: String,
    pub unit: Option<String>,
    pub dimension: Dimension,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Dimension {
    exponents: BTreeMap<String, i32>,
}

impl Dimension {
    pub fn dimensionless() -> Self {
        Self::default()
    }

    pub fn atom(name: impl Into<String>) -> Self {
        let mut exponents = BTreeMap::new();
        exponents.insert(name.into(), 1);
        Self { exponents }
    }

    pub fn mul(&self, other: &Self) -> Self {
        self.combine(other, 1)
    }

    pub fn div(&self, other: &Self) -> Self {
        self.combine(other, -1)
    }

    pub fn is_dimensionless(&self) -> bool {
        self.exponents.is_empty()
    }

    fn combine(&self, other: &Self, sign: i32) -> Self {
        let mut exponents = self.exponents.clone();
        for (name, exp) in &other.exponents {
            let entry = exponents.entry(name.clone()).or_insert(0);
            *entry += sign * exp;
            if *entry == 0 {
                exponents.remove(name);
            }
        }
        Self { exponents }
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.exponents.is_empty() {
            return write!(f, "1");
        }

        let parts = self
            .exponents
            .iter()
            .map(|(name, exp)| {
                if *exp == 1 {
                    name.clone()
                } else {
                    format!("{name}^{exp}")
                }
            })
            .collect::<Vec<_>>();
        write!(f, "{}", parts.join(" * "))
    }
}

pub fn parse_quantity_type(raw: &str) -> QuantityTypeInfo {
    let (base_type, unit) = match raw.split_once('@') {
        Some((base_type, unit)) => {
            let unit = unit.trim();
            let unit = if unit.is_empty() {
                None
            } else {
                Some(unit.to_string())
            };
            (base_type.trim(), unit)
        }
        None => (raw.trim(), None),
    };

    QuantityTypeInfo {
        base_type: base_type.to_string(),
        unit,
        dimension: known_dimension(base_type),
    }
}

pub fn validate_model_dimensions(model: &EqualityModel) -> Result<(), Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();

    for equation in &model.equations {
        let lhs = infer_expr_dimension(
            model,
            &equation.lhs,
            &mut diagnostics,
            equation.block_name.as_str(),
        );
        let rhs = infer_expr_dimension(
            model,
            &equation.rhs,
            &mut diagnostics,
            equation.block_name.as_str(),
        );

        match (lhs, rhs) {
            (Some(lhs), Some(rhs)) if lhs != rhs => diagnostics.push(
                Diagnostic::error(format!(
                    "equation '{}' is dimensionally inconsistent: lhs has dimension {}, rhs has dimension {}",
                    equation.block_name, lhs, rhs
                ))
                .with_span(equation.provenance.span),
            ),
            _ => {}
        }
    }

    for quantity in &model.quantities {
        let constraint_set = &quantity.constraint_set;

        if let Some(lower) = &constraint_set.lower {
            validate_bound(
                model,
                quantity.id,
                &quantity.dimension,
                lower,
                "lower",
                &mut diagnostics,
                quantity.provenance.span,
            );
        }
        if let Some(upper) = &constraint_set.upper {
            validate_bound(
                model,
                quantity.id,
                &quantity.dimension,
                upper,
                "upper",
                &mut diagnostics,
                quantity.provenance.span,
            );
        }
    }

    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

fn infer_expr_dimension(
    model: &EqualityModel,
    expr: &CoreExpr,
    diagnostics: &mut Vec<Diagnostic>,
    context: &str,
) -> Option<Dimension> {
    match expr {
        CoreExpr::Quantity(reference) => {
            Some(model.quantities[reference.quantity.0].dimension.clone())
        }
        CoreExpr::Special(SpecialRef::Dt) => Some(Dimension::atom("time")),
        CoreExpr::Number(_) => Some(Dimension::dimensionless()),
        CoreExpr::Binary { op, left, right } => {
            let left_dim = infer_expr_dimension(model, left, diagnostics, context)?;
            let right_dim = infer_expr_dimension(model, right, diagnostics, context)?;
            match op {
                crate::semantic::BinaryOp::Add | crate::semantic::BinaryOp::Sub => {
                    if left_dim == right_dim {
                        Some(left_dim)
                    } else if is_zero_like(left) {
                        Some(right_dim)
                    } else if is_zero_like(right) {
                        Some(left_dim)
                    } else {
                        diagnostics.push(Diagnostic::error(format!(
                            "expression in '{}' combines incompatible dimensions with '{}': {} and {}",
                            context,
                            match op {
                                crate::semantic::BinaryOp::Add => "+",
                                crate::semantic::BinaryOp::Sub => "-",
                                _ => unreachable!(),
                            },
                            left_dim,
                            right_dim
                        )));
                        None
                    }
                }
                crate::semantic::BinaryOp::Mul => Some(left_dim.mul(&right_dim)),
                crate::semantic::BinaryOp::Div => Some(left_dim.div(&right_dim)),
            }
        }
    }
}

fn validate_bound(
    model: &EqualityModel,
    quantity_id: QuantityId,
    quantity_dimension: &Dimension,
    bound: &ConstraintBound,
    label: &str,
    diagnostics: &mut Vec<Diagnostic>,
    span: crate::diagnostics::SourceSpan,
) {
    match bound {
        ConstraintBound::Number(value) => {
            let is_zero = value
                .parse::<f64>()
                .map(|number| number == 0.0)
                .unwrap_or(false);
            if !is_zero && !quantity_dimension.is_dimensionless() {
                diagnostics.push(
                    Diagnostic::error(format!(
                        "{} bound on '{}' uses nonzero numeric literal '{}' for dimensioned quantity {}",
                        label,
                        model.quantities[quantity_id.0].name,
                        value,
                        quantity_dimension
                    ))
                    .with_span(span),
                );
            }
        }
        ConstraintBound::Quantity(name) => {
            let Some(other) = model
                .quantities
                .iter()
                .find(|candidate| candidate.name == *name)
            else {
                diagnostics.push(
                    Diagnostic::error(format!(
                        "{} bound on '{}' references unknown quantity '{}'",
                        label, model.quantities[quantity_id.0].name, name
                    ))
                    .with_span(span),
                );
                return;
            };

            if other.dimension != *quantity_dimension {
                diagnostics.push(
                    Diagnostic::error(format!(
                        "{} bound on '{}' is dimensionally inconsistent: {} versus {} on '{}'",
                        label,
                        model.quantities[quantity_id.0].name,
                        quantity_dimension,
                        other.dimension,
                        other.name
                    ))
                    .with_span(span),
                );
            }
        }
    }
}

fn is_zero_like(expr: &CoreExpr) -> bool {
    matches!(expr, CoreExpr::Number(value) if value.parse::<f64>().map(|number| number == 0.0).unwrap_or(false))
}

fn known_dimension(base_type: &str) -> Dimension {
    match base_type {
        "scalar" => Dimension::dimensionless(),
        "potential" => Dimension::atom("potential"),
        "conductance" => Dimension::dimensionless().div(&Dimension::atom("time")),
        "water_flux" => Dimension::atom("potential").div(&Dimension::atom("time")),
        "carbon_mass" => Dimension::atom("carbon"),
        "carbon_flux" => Dimension::atom("carbon").div(&Dimension::atom("time")),
        other => Dimension::atom(format!("opaque:{other}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{equality, semantic, syntax::parse_and_validate};

    #[test]
    fn parses_known_quantity_types() {
        let info = parse_quantity_type("water_flux");
        assert_eq!(info.base_type, "water_flux");
        assert_eq!(info.dimension.to_string(), "potential * time^-1");

        let opaque = parse_quantity_type("custom_type");
        assert_eq!(opaque.dimension.to_string(), "opaque:custom_type");
    }

    #[test]
    fn validates_dimensionally_consistent_model() {
        let source = r#"
model DimOk

external driver : potential
external rate : conductance
state water : potential
node flux : water_flux

relation flow:
  flux = rate * driver

temporal water_step:
  water[t+1] = water[t] - dt * flux[t]
"#;

        let syntax = parse_and_validate(source).expect("syntax");
        let semantic = semantic::lower_model(&syntax).expect("semantic");
        let equality = equality::lower_model(&semantic).expect("equality");
        validate_model_dimensions(&equality).expect("dimensions");
    }

    #[test]
    fn rejects_dimensionally_inconsistent_model() {
        let source = r#"
model DimBad

external rate : conductance
state water : potential

relation bad:
  water = water + rate
"#;

        let syntax = parse_and_validate(source).expect("syntax");
        let semantic = semantic::lower_model(&syntax).expect("semantic");
        let equality = equality::lower_model(&semantic).expect("equality");
        let diagnostics = validate_model_dimensions(&equality).expect_err("should fail");
        assert!(
            diagnostics[0]
                .message
                .contains("combines incompatible dimensions")
        );
    }

    #[test]
    fn rejects_dimensionally_incompatible_bounds() {
        let source = r#"
model BoundBad

node x : potential { self <= limit }
node limit : conductance
"#;

        let syntax = parse_and_validate(source).expect("syntax");
        let semantic = semantic::lower_model(&syntax).expect("semantic");
        let equality = equality::lower_model(&semantic).expect("equality");
        let diagnostics = validate_model_dimensions(&equality).expect_err("should fail");
        assert!(
            diagnostics[0]
                .message
                .contains("dimensionally inconsistent")
        );
    }
}
