use std::fmt;

use crate::{
    diagnostics::{Diagnostic, SourceSpan},
    syntax::{BlockDecl, BlockKind, Item, ModelFile, QuantityDecl, QuantityKind, SlotDecl},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticModel {
    pub name: String,
    pub declarations: Vec<Declaration>,
    pub relations: Vec<EquationBlock>,
    pub slots: Vec<SemanticSlot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declaration {
    pub kind: QuantityKind,
    pub name: String,
    pub ty: String,
    pub constraints: Vec<String>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquationBlock {
    pub kind: BlockKind,
    pub name: String,
    pub equations: Vec<Equation>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Equation {
    pub lhs: Expr,
    pub rhs: Expr,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticSlot {
    pub name: String,
    pub provides: Vec<String>,
    pub inputs: Vec<String>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Symbol(String),
    Number(String),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Symbol(name) | Expr::Number(name) => write!(f, "{name}"),
            Expr::Binary { op, left, right } => {
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

pub fn lower_model(model: &ModelFile) -> Result<SemanticModel, Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    let mut declarations = Vec::new();
    let mut relations = Vec::new();
    let mut slots = Vec::new();

    for item in &model.items {
        match item {
            Item::External(quantity) | Item::State(quantity) | Item::Node(quantity) => {
                declarations.push(lower_declaration(quantity));
            }
            Item::Relation(block) | Item::Temporal(block) => match lower_block(block) {
                Ok(block) => relations.push(block),
                Err(mut errs) => diagnostics.append(&mut errs),
            },
            Item::Slot(slot) => slots.push(lower_slot(slot)),
        }
    }

    if diagnostics.is_empty() {
        Ok(SemanticModel {
            name: model.name.clone(),
            declarations,
            relations,
            slots,
        })
    } else {
        Err(diagnostics)
    }
}

fn lower_declaration(quantity: &QuantityDecl) -> Declaration {
    Declaration {
        kind: quantity.kind,
        name: quantity.name.clone(),
        ty: quantity.ty.clone(),
        constraints: quantity.constraints.clone(),
        span: quantity.span,
    }
}

fn lower_slot(slot: &SlotDecl) -> SemanticSlot {
    SemanticSlot {
        name: slot.name.clone(),
        provides: slot.provides.clone(),
        inputs: slot.inputs.clone(),
        span: slot.span,
    }
}

fn lower_block(block: &BlockDecl) -> Result<EquationBlock, Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    let mut equations = Vec::new();

    for line in &block.lines {
        match parse_equation(line, block.span) {
            Ok(eq) => equations.push(eq),
            Err(diag) => diagnostics.push(diag),
        }
    }

    if diagnostics.is_empty() {
        Ok(EquationBlock {
            kind: block.kind,
            name: block.name.clone(),
            equations,
            span: block.span,
        })
    } else {
        Err(diagnostics)
    }
}

fn parse_equation(input: &str, span: SourceSpan) -> Result<Equation, Diagnostic> {
    let (lhs, rhs) = input
        .split_once('=')
        .ok_or_else(|| Diagnostic::error("equation line must contain '='").with_span(span))?;

    let lhs = parse_expr(lhs.trim()).map_err(|msg| Diagnostic::error(msg).with_span(span))?;
    let rhs = parse_expr(rhs.trim()).map_err(|msg| Diagnostic::error(msg).with_span(span))?;

    Ok(Equation { lhs, rhs, span })
}

fn parse_expr(input: &str) -> Result<Expr, String> {
    let tokens = tokenize(input)?;
    let mut parser = ExprParser::new(tokens);
    let expr = parser.parse_expr()?;
    if parser.peek().is_some() {
        return Err("unexpected trailing tokens in expression".to_string());
    }
    Ok(expr)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Symbol(String),
    Number(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = input.chars().collect();
    let mut idx = 0;
    let mut tokens = Vec::new();

    while idx < chars.len() {
        match chars[idx] {
            c if c.is_whitespace() => idx += 1,
            '+' => {
                tokens.push(Token::Plus);
                idx += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                idx += 1;
            }
            '*' => {
                tokens.push(Token::Star);
                idx += 1;
            }
            '/' => {
                tokens.push(Token::Slash);
                idx += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                idx += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                idx += 1;
            }
            c if c.is_ascii_digit() => {
                let start = idx;
                idx += 1;
                while idx < chars.len() && (chars[idx].is_ascii_digit() || chars[idx] == '.') {
                    idx += 1;
                }
                tokens.push(Token::Number(chars[start..idx].iter().collect()));
            }
            c if is_symbol_start(c) => {
                let start = idx;
                idx += 1;
                while idx < chars.len() && is_symbol_continue(chars[idx]) {
                    idx += 1;
                }

                if idx < chars.len() && chars[idx] == '[' {
                    idx += 1;
                    while idx < chars.len() && chars[idx] != ']' {
                        idx += 1;
                    }
                    if idx >= chars.len() {
                        return Err("unterminated index expression in symbol".to_string());
                    }
                    idx += 1;
                }

                tokens.push(Token::Symbol(chars[start..idx].iter().collect()));
            }
            other => return Err(format!("unexpected character '{other}' in expression")),
        }
    }

    Ok(tokens)
}

fn is_symbol_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_symbol_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '.'
}

struct ExprParser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl ExprParser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cursor: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    fn next(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.cursor).cloned();
        if token.is_some() {
            self.cursor += 1;
        }
        token
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_mul_div()?;
        loop {
            match self.peek() {
                Some(Token::Plus) => {
                    self.next();
                    let rhs = self.parse_mul_div()?;
                    expr = Expr::Binary {
                        op: BinaryOp::Add,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    };
                }
                Some(Token::Minus) => {
                    self.next();
                    let rhs = self.parse_mul_div()?;
                    expr = Expr::Binary {
                        op: BinaryOp::Sub,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    };
                }
                _ => return Ok(expr),
            }
        }
    }

    fn parse_mul_div(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        loop {
            match self.peek() {
                Some(Token::Star) => {
                    self.next();
                    let rhs = self.parse_primary()?;
                    expr = Expr::Binary {
                        op: BinaryOp::Mul,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    };
                }
                Some(Token::Slash) => {
                    self.next();
                    let rhs = self.parse_primary()?;
                    expr = Expr::Binary {
                        op: BinaryOp::Div,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    };
                }
                _ => return Ok(expr),
            }
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.next() {
            Some(Token::Symbol(symbol)) => Ok(Expr::Symbol(symbol)),
            Some(Token::Number(number)) => Ok(Expr::Number(number)),
            Some(Token::LParen) => {
                let expr = self.parse_expr()?;
                match self.next() {
                    Some(Token::RParen) => Ok(expr),
                    _ => Err("expected ')'".to_string()),
                }
            }
            Some(Token::Minus) => {
                let rhs = self.parse_primary()?;
                Ok(Expr::Binary {
                    op: BinaryOp::Sub,
                    left: Box::new(Expr::Number("0".to_string())),
                    right: Box::new(rhs),
                })
            }
            _ => Err("expected expression".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::parse_and_validate;

    const TINY_TREE: &str = include_str!("../tests/fixtures/tiny_tree.myco");

    #[test]
    fn lowers_tiny_tree_fixture() {
        let syntax = parse_and_validate(TINY_TREE).expect("fixture should validate");
        let semantic = lower_model(&syntax).expect("lowering should succeed");

        assert_eq!(semantic.name, "TinyTree");
        assert_eq!(semantic.declarations.len(), 8);
        assert_eq!(semantic.relations.len(), 3);
        assert_eq!(semantic.slots.len(), 1);

        let demand = semantic
            .relations
            .iter()
            .find(|block| block.name == "demand_transpiration")
            .expect("demand relation should exist");
        assert_eq!(demand.equations.len(), 1);
        assert_eq!(demand.equations[0].lhs, Expr::Symbol("transpiration".to_string()));
    }

    #[test]
    fn parses_temporal_indexed_symbols() {
        let equation = parse_equation("water[t+1] = water[t] - dt * transpiration[t]", dummy_span())
            .expect("equation should parse");

        assert_eq!(equation.lhs, Expr::Symbol("water[t+1]".to_string()));
    }

    #[test]
    fn rejects_bad_expression() {
        let err = parse_expr("a * )").expect_err("expression should fail");
        assert!(err.contains("expected expression") || err.contains("expected ')'"));
    }

    fn dummy_span() -> SourceSpan {
        SourceSpan {
            start: crate::diagnostics::SourcePosition { line: 1, column: 1 },
            end: crate::diagnostics::SourcePosition { line: 1, column: 10 },
        }
    }
}
