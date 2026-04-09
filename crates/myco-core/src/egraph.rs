use egg::{
    AstSize, EGraph, Extractor, Id, RecExpr, Runner, Symbol, define_language, rewrite as rw,
};

use crate::equality::{
    CoreExpr, EqualityEquation, EqualityModel, QuantityRef, SpecialRef, TimeReference,
};

define_language! {
    pub enum ArithmeticLang {
        Symbol(Symbol),
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "*" = Mul([Id; 2]),
        "/" = Div([Id; 2]),
    }
}

pub fn build_egraph(model: &EqualityModel) -> EGraph<ArithmeticLang, ()> {
    let mut egraph = EGraph::<ArithmeticLang, ()>::default();

    for equation in &model.equations {
        add_equation(&mut egraph, equation);
    }

    let rewrites = vec![
        rw!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        rw!("commute-mul"; "(* ?a ?b)" => "(* ?b ?a)"),
    ];

    let runner = Runner::default().with_egraph(egraph).run(&rewrites);
    runner.egraph
}

pub fn add_expr(egraph: &mut EGraph<ArithmeticLang, ()>, expr: &CoreExpr) -> Id {
    let recexpr = to_recexpr(expr);
    egraph.add_expr(&recexpr)
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

fn add_equation(egraph: &mut EGraph<ArithmeticLang, ()>, equation: &EqualityEquation) {
    let lhs = add_expr(egraph, &equation.lhs);
    let rhs = add_expr(egraph, &equation.rhs);
    egraph.union(lhs, rhs);
    egraph.rebuild();
}

fn to_recexpr(expr: &CoreExpr) -> RecExpr<ArithmeticLang> {
    let mut nodes = Vec::<ArithmeticLang>::new();
    build_recexpr(expr, &mut nodes);
    RecExpr::from(nodes)
}

fn build_recexpr(expr: &CoreExpr, nodes: &mut Vec<ArithmeticLang>) -> Id {
    match expr {
        CoreExpr::Quantity(QuantityRef { quantity, time }) => {
            let label = match time {
                TimeReference::Implicit => format!("q{}", quantity.0),
                TimeReference::Relative(offset) => format!("q{}@{}", quantity.0, offset),
            };
            nodes.push(ArithmeticLang::Symbol(label.into()));
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
                crate::semantic::BinaryOp::Add => ArithmeticLang::Add([left, right]),
                crate::semantic::BinaryOp::Sub => ArithmeticLang::Sub([left, right]),
                crate::semantic::BinaryOp::Mul => ArithmeticLang::Mul([left, right]),
                crate::semantic::BinaryOp::Div => ArithmeticLang::Div([left, right]),
            };
            nodes.push(node);
        }
    }

    Id::from(nodes.len() - 1)
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
        let mut egraph = build_egraph(&equality);

        let equation = equality
            .equations
            .iter()
            .find(|equation| equation.block_name == "demand_transpiration")
            .expect("equation");

        let lhs = add_expr(&mut egraph, &equation.lhs);
        let rhs = add_expr(&mut egraph, &equation.rhs);
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
        let mut egraph = build_egraph(&equality);

        let swapped = CoreExpr::Binary {
            op: crate::semantic::BinaryOp::Add,
            left: Box::new(CoreExpr::Quantity(crate::equality::QuantityRef {
                quantity: crate::equality::QuantityId(1),
                time: crate::equality::TimeReference::Implicit,
            })),
            right: Box::new(CoreExpr::Quantity(crate::equality::QuantityRef {
                quantity: crate::equality::QuantityId(0),
                time: crate::equality::TimeReference::Implicit,
            })),
        };

        let swapped_id = add_expr(&mut egraph, &swapped);
        let canonical = equality
            .equations
            .iter()
            .find(|equation| equation.block_name == "pair")
            .expect("equation");
        let rhs_id = add_expr(&mut egraph, &canonical.rhs);
        assert_eq!(egraph.find(swapped_id), egraph.find(rhs_id));
    }
}
