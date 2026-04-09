use std::collections::HashSet;

use crate::diagnostics::{Diagnostic, SourcePosition, SourceSpan};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelFile {
    pub name: String,
    pub items: Vec<Item>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    External(QuantityDecl),
    State(QuantityDecl),
    Node(QuantityDecl),
    Relation(BlockDecl),
    Slot(SlotDecl),
    Temporal(BlockDecl),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantityKind {
    External,
    State,
    Node,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantityDecl {
    pub kind: QuantityKind,
    pub name: String,
    pub ty: String,
    pub constraints: Vec<String>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockKind {
    Relation,
    Temporal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockDecl {
    pub kind: BlockKind,
    pub name: String,
    pub lines: Vec<String>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotDecl {
    pub name: String,
    pub provides: Vec<String>,
    pub inputs: Vec<String>,
    pub span: SourceSpan,
}

pub fn parse_model(source: &str) -> Result<ModelFile, Vec<Diagnostic>> {
    let parser = Parser::new(source);
    parser.parse()
}

pub fn parse_and_validate(source: &str) -> Result<ModelFile, Vec<Diagnostic>> {
    let model = parse_model(source)?;
    validate_model(&model)?;
    Ok(model)
}

pub fn validate_model(model: &ModelFile) -> Result<(), Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    let mut declared = HashSet::new();

    for item in &model.items {
        if let Item::External(quantity) | Item::State(quantity) | Item::Node(quantity) = item {
            if !declared.insert(quantity.name.clone()) {
                diagnostics.push(
                    Diagnostic::error(format!("duplicate declaration for '{}'", quantity.name))
                        .with_span(quantity.span),
                );
            }
        }
    }

    let declared_names: HashSet<&str> = model
        .items
        .iter()
        .filter_map(|item| match item {
            Item::External(quantity) | Item::State(quantity) | Item::Node(quantity) => {
                Some(quantity.name.as_str())
            }
            _ => None,
        })
        .collect();

    for item in &model.items {
        if let Item::Slot(slot) = item {
            for provided in &slot.provides {
                if !declared_names.contains(provided.as_str()) {
                    diagnostics.push(
                        Diagnostic::error(format!(
                            "slot '{}' provides unknown quantity '{}'",
                            slot.name, provided
                        ))
                        .with_span(slot.span),
                    );
                }
            }

            for input in &slot.inputs {
                if !declared_names.contains(input.as_str()) {
                    diagnostics.push(
                        Diagnostic::error(format!(
                            "slot '{}' references unknown input '{}'",
                            slot.name, input
                        ))
                        .with_span(slot.span),
                    );
                }
            }
        }
    }

    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

struct Parser<'a> {
    lines: Vec<Line<'a>>,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        let lines = source
            .lines()
            .enumerate()
            .map(|(idx, raw)| Line::new(idx + 1, raw))
            .collect();
        Self { lines }
    }

    fn parse(self) -> Result<ModelFile, Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();
        let mut cursor = 0;

        while cursor < self.lines.len() && self.lines[cursor].trimmed().is_empty() {
            cursor += 1;
        }

        let Some(header) = self.lines.get(cursor) else {
            return Err(vec![Diagnostic::error("empty .myco file")]);
        };

        let Some(name) = header.trimmed().strip_prefix("model ") else {
            return Err(vec![
                Diagnostic::error("first non-empty line must declare a model")
                    .with_span(header.span()),
            ]);
        };

        let model_name = name.trim();
        if model_name.is_empty() {
            return Err(vec![
                Diagnostic::error("model declaration must include a name").with_span(header.span()),
            ]);
        }

        let model_start = header.span().start;
        cursor += 1;

        let mut items = Vec::new();

        while cursor < self.lines.len() {
            let line = &self.lines[cursor];

            if line.trimmed().is_empty() {
                cursor += 1;
                continue;
            }

            if line.indent > 0 {
                diagnostics.push(
                    Diagnostic::error("top-level item may not be indented").with_span(line.span()),
                );
                cursor += 1;
                continue;
            }

            let trimmed = line.trimmed();

            if trimmed.starts_with("external ") {
                match self.parse_quantity(cursor, QuantityKind::External) {
                    Ok((item, next)) => {
                        items.push(Item::External(item));
                        cursor = next;
                    }
                    Err(diag) => {
                        diagnostics.push(diag);
                        cursor += 1;
                    }
                }
            } else if trimmed.starts_with("state ") {
                match self.parse_quantity(cursor, QuantityKind::State) {
                    Ok((item, next)) => {
                        items.push(Item::State(item));
                        cursor = next;
                    }
                    Err(diag) => {
                        diagnostics.push(diag);
                        cursor += 1;
                    }
                }
            } else if trimmed.starts_with("node ") {
                match self.parse_quantity(cursor, QuantityKind::Node) {
                    Ok((item, next)) => {
                        items.push(Item::Node(item));
                        cursor = next;
                    }
                    Err(diag) => {
                        diagnostics.push(diag);
                        cursor += 1;
                    }
                }
            } else if trimmed.starts_with("relation ") {
                match self.parse_block(cursor, BlockKind::Relation) {
                    Ok((block, next)) => {
                        items.push(Item::Relation(block));
                        cursor = next;
                    }
                    Err(diag) => {
                        diagnostics.push(diag);
                        cursor += 1;
                    }
                }
            } else if trimmed.starts_with("temporal ") {
                match self.parse_block(cursor, BlockKind::Temporal) {
                    Ok((block, next)) => {
                        items.push(Item::Temporal(block));
                        cursor = next;
                    }
                    Err(diag) => {
                        diagnostics.push(diag);
                        cursor += 1;
                    }
                }
            } else if trimmed.starts_with("slot ") {
                match self.parse_slot(cursor) {
                    Ok((slot, next)) => {
                        items.push(Item::Slot(slot));
                        cursor = next;
                    }
                    Err(diag) => {
                        diagnostics.push(diag);
                        cursor += 1;
                    }
                }
            } else {
                diagnostics.push(
                    Diagnostic::error(format!("unrecognized top-level item: {}", trimmed))
                        .with_span(line.span()),
                );
                cursor += 1;
            }
        }

        if diagnostics.is_empty() {
            let end = self
                .lines
                .last()
                .map(Line::span)
                .unwrap_or_else(|| header.span())
                .end;
            Ok(ModelFile {
                name: model_name.to_string(),
                items,
                span: SourceSpan::new(model_start, end),
            })
        } else {
            Err(diagnostics)
        }
    }

    fn parse_quantity(
        &self,
        start_idx: usize,
        kind: QuantityKind,
    ) -> Result<(QuantityDecl, usize), Diagnostic> {
        let line = &self.lines[start_idx];
        let keyword = match kind {
            QuantityKind::External => "external ",
            QuantityKind::State => "state ",
            QuantityKind::Node => "node ",
        };

        let rest = line
            .trimmed()
            .strip_prefix(keyword)
            .expect("caller must guard keyword");

        let (before_constraints, inline_constraints, multiline_open) =
            parse_constraint_shape(rest).map_err(|message| {
                Diagnostic::error(message).with_span(line.span())
            })?;
        let (name, ty) = parse_name_and_type(before_constraints).ok_or_else(|| {
            Diagnostic::error("expected `<name> : <type>`").with_span(line.span())
        })?;

        let mut constraints = inline_constraints;
        let mut cursor = start_idx + 1;
        let mut span_end = line.span().end;

        if multiline_open {
            let mut closed = false;
            while cursor < self.lines.len() {
                let current = &self.lines[cursor];
                if current.trimmed() == "}" {
                    span_end = current.span().end;
                    cursor += 1;
                    closed = true;
                    break;
                }
                if current.trimmed().is_empty() {
                    cursor += 1;
                    continue;
                }
                constraints.push(current.trimmed().to_string());
                span_end = current.span().end;
                cursor += 1;
            }

            if !closed {
                return Err(
                    Diagnostic::error("unterminated constraint block; expected '}'")
                        .with_span(line.span()),
                );
            }
        }

        Ok((
            QuantityDecl {
                kind,
                name: name.to_string(),
                ty: ty.to_string(),
                constraints,
                span: SourceSpan::new(line.span().start, span_end),
            },
            cursor,
        ))
    }

    fn parse_block(
        &self,
        start_idx: usize,
        kind: BlockKind,
    ) -> Result<(BlockDecl, usize), Diagnostic> {
        let line = &self.lines[start_idx];
        let keyword = match kind {
            BlockKind::Relation => "relation ",
            BlockKind::Temporal => "temporal ",
        };
        let rest = line
            .trimmed()
            .strip_prefix(keyword)
            .expect("caller must guard keyword");

        let Some(name) = rest.strip_suffix(':') else {
            return Err(
                Diagnostic::error("block declaration must end with ':'").with_span(line.span())
            );
        };

        let name = name.trim();
        if name.is_empty() {
            return Err(
                Diagnostic::error("block declaration must include a name").with_span(line.span())
            );
        }

        let mut lines = Vec::new();
        let mut cursor = start_idx + 1;
        let mut span_end = line.span().end;

        while cursor < self.lines.len() {
            let current = &self.lines[cursor];
            if current.trimmed().is_empty() {
                cursor += 1;
                continue;
            }
            if current.indent == 0 {
                break;
            }
            lines.push(current.trimmed().to_string());
            span_end = current.span().end;
            cursor += 1;
        }

        if lines.is_empty() {
            return Err(
                Diagnostic::error("block must contain at least one body line")
                    .with_span(line.span()),
            );
        }

        Ok((
            BlockDecl {
                kind,
                name: name.to_string(),
                lines,
                span: SourceSpan::new(line.span().start, span_end),
            },
            cursor,
        ))
    }

    fn parse_slot(&self, start_idx: usize) -> Result<(SlotDecl, usize), Diagnostic> {
        let line = &self.lines[start_idx];
        let rest = line
            .trimmed()
            .strip_prefix("slot ")
            .expect("caller must guard keyword");
        let (name_part, tail) = rest.split_once(" provides ").ok_or_else(|| {
            Diagnostic::error("slot declaration must include `provides`").with_span(line.span())
        })?;
        let Some(provides_part) = tail.strip_suffix(':') else {
            return Err(
                Diagnostic::error("slot declaration must end with ':'").with_span(line.span())
            );
        };

        let name = name_part.trim();
        if name.is_empty() {
            return Err(
                Diagnostic::error("slot declaration must include a name").with_span(line.span())
            );
        }

        let provides = parse_symbol_list(provides_part).ok_or_else(|| {
            Diagnostic::error("slot `provides` list must use `[a, b]` syntax")
                .with_span(line.span())
        })?;

        let mut inputs = None;
        let mut cursor = start_idx + 1;
        let mut span_end = line.span().end;

        while cursor < self.lines.len() {
            let current = &self.lines[cursor];
            if current.trimmed().is_empty() {
                cursor += 1;
                continue;
            }
            if current.indent == 0 {
                break;
            }

            if let Some((lhs, rhs)) = current.trimmed().split_once('=') {
                if lhs.trim() == "inputs" {
                    inputs = parse_symbol_list(rhs.trim());
                    if inputs.is_none() {
                        return Err(Diagnostic::error("slot inputs must use `[a, b]` syntax")
                            .with_span(current.span()));
                    }
                }
            }

            span_end = current.span().end;
            cursor += 1;
        }

        let inputs = inputs.ok_or_else(|| {
            Diagnostic::error("slot must declare `inputs = [...]`").with_span(line.span())
        })?;

        Ok((
            SlotDecl {
                name: name.to_string(),
                provides,
                inputs,
                span: SourceSpan::new(line.span().start, span_end),
            },
            cursor,
        ))
    }
}

fn parse_name_and_type(input: &str) -> Option<(&str, &str)> {
    let (name, ty) = input.split_once(':')?;
    let name = name.trim();
    let ty = ty.trim();
    if name.is_empty() || ty.is_empty() {
        return None;
    }
    Some((name, ty))
}

fn parse_constraint_shape(input: &str) -> Result<(&str, Vec<String>, bool), String> {
    let trimmed = input.trim_end();
    let Some(open_idx) = trimmed.find('{') else {
        return Ok((trimmed, Vec::new(), false));
    };

    let before = trimmed[..open_idx].trim_end();
    let after = trimmed[open_idx + 1..].trim();

    if let Some(close_idx) = after.find('}') {
        let inside = after[..close_idx].trim();
        let trailing = after[close_idx + 1..].trim();
        if !trailing.is_empty() {
            return Err("unexpected trailing content after inline constraint block".to_string());
        }
        let constraints = if inside.is_empty() {
            Vec::new()
        } else {
            vec![inside.to_string()]
        };
        Ok((before, constraints, false))
    } else {
        Ok((before, Vec::new(), true))
    }
}

fn parse_symbol_list(input: &str) -> Option<Vec<String>> {
    let trimmed = input.trim();
    let inner = trimmed.strip_prefix('[')?.strip_suffix(']')?.trim();
    if inner.is_empty() {
        return Some(Vec::new());
    }
    Some(
        inner
            .split(',')
            .map(|item| item.trim().to_string())
            .collect(),
    )
}

#[derive(Debug, Clone, Copy)]
struct Line<'a> {
    number: usize,
    raw: &'a str,
    indent: usize,
}

impl<'a> Line<'a> {
    fn new(number: usize, raw: &'a str) -> Self {
        let indent = raw.chars().take_while(|c| c.is_whitespace()).count();
        Self {
            number,
            raw,
            indent,
        }
    }

    fn trimmed(&self) -> &'a str {
        self.raw.trim()
    }

    fn span(&self) -> SourceSpan {
        let start = SourcePosition::new(self.number, 1);
        let end = SourcePosition::new(self.number, self.raw.chars().count() + 1);
        SourceSpan::new(start, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TINY_TREE: &str = include_str!("../tests/fixtures/tiny_tree.myco");

    #[test]
    fn parses_tiny_tree_fixture() {
        let model = parse_model(TINY_TREE).expect("tiny_tree should parse");
        assert_eq!(model.name, "TinyTree");
        assert_eq!(model.items.len(), 12);
    }

    #[test]
    fn validates_tiny_tree_fixture() {
        let model = parse_and_validate(TINY_TREE).expect("tiny_tree should validate");
        assert_eq!(model.name, "TinyTree");
    }

    #[test]
    fn rejects_unknown_slot_input() {
        let bad = r#"
model Bad

node x : scalar

slot provider provides [x]:
  inputs = [y]
"#;
        let diagnostics = parse_and_validate(bad).expect_err("validation should fail");
        assert!(
            diagnostics
                .iter()
                .any(|diag| diag.message.contains("unknown input 'y'"))
        );
    }

    #[test]
    fn rejects_missing_model_header() {
        let diagnostics = parse_model("node x : scalar").expect_err("parse should fail");
        assert!(diagnostics.iter().any(|diag| {
            diag.message
                .contains("first non-empty line must declare a model")
        }));
    }

    #[test]
    fn rejects_unterminated_constraint_block() {
        let source = r#"
model Bad

node x : scalar {
  self >= 0
"#;
        let diagnostics = parse_model(source).expect_err("parse should fail");
        assert!(diagnostics.iter().any(|diag| {
            diag.message
                .contains("unterminated constraint block; expected '}'")
        }));
    }

    #[test]
    fn rejects_trailing_content_after_inline_constraint_block() {
        let source = r#"
model Bad

node x : scalar { self >= 0 } trailing
"#;
        let diagnostics = parse_model(source).expect_err("parse should fail");
        assert!(diagnostics.iter().any(|diag| {
            diag.message
                .contains("unexpected trailing content after inline constraint block")
        }));
    }
}
