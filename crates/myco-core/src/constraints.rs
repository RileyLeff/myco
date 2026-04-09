#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantityConstraintSet {
    pub lower: Option<ConstraintBound>,
    pub upper: Option<ConstraintBound>,
    pub penalties: Vec<PenaltySpec>,
    pub unparsed: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstraintBound {
    Number(String),
    Quantity(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PenaltySpec {
    Smooth { weight: String },
}

pub fn parse_constraint_set(raw_constraints: &[String]) -> QuantityConstraintSet {
    let mut lower = None;
    let mut upper = None;
    let mut penalties = Vec::new();
    let mut unparsed = Vec::new();

    for constraint in raw_constraints {
        if let Some(bound) = parse_bound_constraint(constraint, ">=") {
            lower = Some(bound);
        } else if let Some(bound) = parse_bound_constraint(constraint, "<=") {
            upper = Some(bound);
        } else if let Some(penalty) = parse_penalty_constraint(constraint) {
            penalties.push(penalty);
        } else {
            unparsed.push(constraint.clone());
        }
    }

    QuantityConstraintSet {
        lower,
        upper,
        penalties,
        unparsed,
    }
}

fn parse_bound_constraint(input: &str, op: &str) -> Option<ConstraintBound> {
    let (lhs, rhs) = input.split_once(op)?;
    if lhs.trim() != "self" {
        return None;
    }
    let rhs = rhs.trim();
    if rhs.is_empty() {
        return None;
    }
    if rhs.parse::<f64>().is_ok() {
        return Some(ConstraintBound::Number(rhs.to_string()));
    }
    if rhs.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Some(ConstraintBound::Quantity(rhs.to_string()));
    }
    None
}

fn parse_penalty_constraint(input: &str) -> Option<PenaltySpec> {
    let trimmed = input.trim();
    let rest = trimmed.strip_prefix("penalty ")?;
    let inside = rest.strip_prefix("smooth(")?.strip_suffix(')')?.trim();

    let (key, value) = inside.split_once('=')?;
    if key.trim() != "weight" {
        return None;
    }

    let weight = value.trim();
    if weight.is_empty() || weight.parse::<f64>().is_err() {
        return None;
    }

    Some(PenaltySpec::Smooth {
        weight: weight.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bounds_and_smooth_penalty() {
        let constraints = parse_constraint_set(&[
            "self >= 0".to_string(),
            "self <= g_max".to_string(),
            "penalty smooth(weight=1e-3)".to_string(),
        ]);

        assert_eq!(
            constraints.lower,
            Some(ConstraintBound::Number("0".to_string()))
        );
        assert_eq!(
            constraints.upper,
            Some(ConstraintBound::Quantity("g_max".to_string()))
        );
        assert_eq!(
            constraints.penalties,
            vec![PenaltySpec::Smooth {
                weight: "1e-3".to_string()
            }]
        );
        assert!(constraints.unparsed.is_empty());
    }

    #[test]
    fn leaves_unknown_constraints_unparsed() {
        let constraints = parse_constraint_set(&["penalty weird(weight=1.0)".to_string()]);
        assert!(constraints.penalties.is_empty());
        assert_eq!(
            constraints.unparsed,
            vec!["penalty weird(weight=1.0)".to_string()]
        );
    }
}
