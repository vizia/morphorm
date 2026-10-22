use std::error::Error;
use std::fmt;

use morphorm::{LayoutType, LayoutWrap, Units};
use morphorm_ecs::{Entity, World};

#[derive(Debug)]
pub struct Fixture {
    root: NodeSpec,
}

#[derive(Debug)]
struct NodeSpec {
    properties: Vec<Property>,
    children: Vec<ChildSpec>,
}

#[derive(Debug)]
enum ChildSpec {
    Node(NodeSpec),
    Repeat { count: usize, children: Vec<ChildSpec> },
}

#[derive(Debug)]
enum Property {
    Width(Dimension),
    Height(Dimension),
    MinWidth(Units),
    MaxWidth(Units),
    MinHeight(Units),
    MaxHeight(Units),
    Padding(f32),
    Gap(f32),
    LayoutType(LayoutType),
    Wrap,
}

#[derive(Debug)]
enum Dimension {
    Units(Units),
    Ratio(f32),
}

#[derive(Debug, PartialEq)]
pub enum FixtureError {
    Unsupported { feature: String, line: usize, column: usize },
    Parse { message: String, line: usize, column: usize },
}

impl FixtureError {
    pub fn unsupported_feature(&self) -> Option<&str> {
        match self {
            Self::Unsupported { feature, .. } => Some(feature),
            Self::Parse { .. } => None,
        }
    }
}

impl fmt::Display for FixtureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported { feature, line, column } => {
                write!(formatter, "unsupported feature `{feature}` at {line}:{column}")
            }
            Self::Parse { message, line, column } => {
                write!(formatter, "{message} at {line}:{column}")
            }
        }
    }
}

impl Error for FixtureError {}

impl Fixture {
    pub fn parse(source: &str) -> Result<Self, FixtureError> {
        let tokens = Lexer::new(source).tokenize()?;
        let mut parser = Parser::new(tokens);
        let root = parser.parse_node()?;
        parser.expect_end()?;
        Ok(Self { root })
    }

    pub fn expanded_node_count(&self) -> usize {
        1 + count_children(&self.root.children)
    }

    pub fn build_world(&self) -> (World, Entity) {
        let mut world = World::default();
        let root = build_node(&mut world, None, &self.root);
        (world, root)
    }
}

fn count_children(children: &[ChildSpec]) -> usize {
    children
        .iter()
        .map(|child| match child {
            ChildSpec::Node(node) => 1 + count_children(&node.children),
            ChildSpec::Repeat { count, children } => count * count_children(children),
        })
        .sum()
}

fn build_node(world: &mut World, parent: Option<Entity>, node: &NodeSpec) -> Entity {
    let entity = world.add(parent);

    for property in &node.properties {
        match property {
            Property::Width(Dimension::Units(value)) => world.set_width(entity, *value),
            Property::Width(Dimension::Ratio(value)) => {
                world.set_width(entity, Units::Auto);
                world.set_aspect_ratio(entity, *value);
            }
            Property::Height(Dimension::Units(value)) => world.set_height(entity, *value),
            Property::Height(Dimension::Ratio(value)) => {
                world.set_height(entity, Units::Auto);
                world.set_aspect_ratio(entity, value.recip());
            }
            Property::MinWidth(value) => world.set_min_width(entity, *value),
            Property::MaxWidth(value) => world.set_max_width(entity, *value),
            Property::MinHeight(value) => world.set_min_height(entity, *value),
            Property::MaxHeight(value) => world.set_max_height(entity, *value),
            Property::Padding(value) => world.set_padding(entity, Units::Pixels(*value)),
            Property::Gap(value) => {
                world.set_horizontal_gap(entity, Units::Pixels(*value));
                world.set_vertical_gap(entity, Units::Pixels(*value));
            }
            Property::LayoutType(value) => world.set_layout_type(entity, *value),
            Property::Wrap => world.set_wrap(entity, LayoutWrap::Wrap),
        }
    }

    build_children(world, entity, &node.children);
    entity
}

fn build_children(world: &mut World, parent: Entity, children: &[ChildSpec]) {
    for child in children {
        match child {
            ChildSpec::Node(node) => {
                build_node(world, Some(parent), node);
            }
            ChildSpec::Repeat { count, children } => {
                for _ in 0..*count {
                    build_children(world, parent, children);
                }
            }
        }
    }
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    line: usize,
    column: usize,
}

#[derive(Debug)]
enum TokenKind {
    Ident(String),
    Number(String),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Dot,
    End,
}

struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self { chars: source.chars().peekable(), line: 1, column: 1 }
    }

    fn tokenize(mut self) -> Result<Vec<Token>, FixtureError> {
        let mut tokens = Vec::new();

        while let Some(character) = self.chars.peek().copied() {
            if character.is_whitespace() {
                self.advance();
                continue;
            }

            if character == '#' {
                while let Some(character) = self.advance() {
                    if character == '\n' {
                        break;
                    }
                }
                continue;
            }

            let line = self.line;
            let column = self.column;
            let kind = match character {
                '(' => {
                    self.advance();
                    TokenKind::LeftParen
                }
                ')' => {
                    self.advance();
                    TokenKind::RightParen
                }
                '{' => {
                    self.advance();
                    TokenKind::LeftBrace
                }
                '}' => {
                    self.advance();
                    TokenKind::RightBrace
                }
                '.' => {
                    self.advance();
                    TokenKind::Dot
                }
                character if character.is_ascii_alphabetic() || character == '_' => {
                    let mut identifier = String::new();
                    while let Some(character) = self.chars.peek().copied() {
                        if !character.is_ascii_alphanumeric() && character != '_' {
                            break;
                        }
                        identifier.push(character);
                        self.advance();
                    }
                    TokenKind::Ident(identifier)
                }
                character if character.is_ascii_digit() || character == '-' => {
                    let mut number = String::new();
                    while let Some(character) = self.chars.peek().copied() {
                        if !character.is_ascii_digit() && character != '.' && character != '-' {
                            break;
                        }
                        number.push(character);
                        self.advance();
                    }
                    TokenKind::Number(number)
                }
                _ => {
                    return Err(FixtureError::Parse {
                        message: format!("unexpected character `{character}`"),
                        line,
                        column,
                    });
                }
            };

            tokens.push(Token { kind, line, column });
        }

        tokens.push(Token { kind: TokenKind::End, line: self.line, column: self.column });
        Ok(tokens)
    }

    fn advance(&mut self) -> Option<char> {
        let character = self.chars.next()?;
        if character == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(character)
    }
}

struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    fn parse_node(&mut self) -> Result<NodeSpec, FixtureError> {
        let mut properties = vec![self.parse_property()?];
        while self.consume_dot() {
            properties.push(self.parse_property()?);
        }
        let children = self.parse_children()?;
        Ok(NodeSpec { properties, children })
    }

    fn parse_property(&mut self) -> Result<Property, FixtureError> {
        let (name, line, column) = self.expect_identifier()?;
        self.expect_left_paren()?;

        let property = match name.as_str() {
            "Width" => Property::Width(self.parse_dimension()?),
            "Height" => Property::Height(self.parse_dimension()?),
            "MinWidth" => Property::MinWidth(self.parse_constraint(&name, line, column)?),
            "MaxWidth" => Property::MaxWidth(self.parse_constraint(&name, line, column)?),
            "MinHeight" => Property::MinHeight(self.parse_constraint(&name, line, column)?),
            "MaxHeight" => Property::MaxHeight(self.parse_constraint(&name, line, column)?),
            "Padding" => Property::Padding(self.expect_number()?),
            "Gap" => Property::Gap(self.expect_number()?),
            "Horizontal" => Property::LayoutType(LayoutType::Row),
            "Vertical" => Property::LayoutType(LayoutType::Column),
            "Wrap" => {
                let (value, value_line, value_column) = self.expect_identifier()?;
                if value != "Auto" {
                    return Err(FixtureError::Unsupported {
                        feature: format!("Wrap({value})"),
                        line: value_line,
                        column: value_column,
                    });
                }
                Property::Wrap
            }
            _ => return Err(FixtureError::Unsupported { feature: name, line, column }),
        };

        self.expect_right_paren()?;
        Ok(property)
    }

    fn parse_constraint(&mut self, property: &str, line: usize, column: usize) -> Result<Units, FixtureError> {
        match self.parse_dimension()? {
            Dimension::Units(value) => Ok(value),
            Dimension::Ratio(_) => {
                Err(FixtureError::Unsupported { feature: format!("{property}(Ratio)"), line, column })
            }
        }
    }

    fn parse_dimension(&mut self) -> Result<Dimension, FixtureError> {
        if matches!(self.current().kind, TokenKind::Number(_)) {
            return Ok(Dimension::Units(Units::Pixels(self.expect_number()?)));
        }

        let (name, line, column) = self.expect_identifier()?;
        match name.as_str() {
            "Fit" => Ok(Dimension::Units(Units::Auto)),
            "Expand" if !matches!(self.current().kind, TokenKind::LeftParen) => {
                Ok(Dimension::Units(Units::Stretch(1.0)))
            }
            "Pixels" | "Percentage" | "Expand" | "Ratio" => {
                self.expect_left_paren()?;
                let value = self.expect_number()?;
                self.expect_right_paren()?;

                match name.as_str() {
                    "Pixels" => Ok(Dimension::Units(Units::Pixels(value))),
                    "Percentage" => Ok(Dimension::Units(Units::Percentage(value * 100.0))),
                    "Expand" => Ok(Dimension::Units(Units::Stretch(value))),
                    "Ratio" if value > 0.0 => Ok(Dimension::Ratio(value)),
                    "Ratio" => Err(FixtureError::Parse {
                        message: "ratio must be greater than zero".to_string(),
                        line,
                        column,
                    }),
                    _ => unreachable!(),
                }
            }
            _ => Err(FixtureError::Unsupported { feature: name, line, column }),
        }
    }

    fn parse_children(&mut self) -> Result<Vec<ChildSpec>, FixtureError> {
        self.expect_left_brace()?;
        let mut children = Vec::new();

        while !matches!(self.current().kind, TokenKind::RightBrace) {
            if matches!(self.current().kind, TokenKind::End) {
                return Err(self.parse_error("expected `}`"));
            }

            if matches!(&self.current().kind, TokenKind::Ident(name) if name == "Repeat") {
                children.push(self.parse_repeat()?);
            } else {
                children.push(ChildSpec::Node(self.parse_node()?));
            }
        }

        self.index += 1;
        Ok(children)
    }

    fn parse_repeat(&mut self) -> Result<ChildSpec, FixtureError> {
        self.index += 1;
        self.expect_left_paren()?;
        let token = self.current();
        let count = match &token.kind {
            TokenKind::Number(value) => value.parse::<usize>().map_err(|_| FixtureError::Parse {
                message: "repeat count must be a non-negative integer".to_string(),
                line: token.line,
                column: token.column,
            })?,
            _ => return Err(self.parse_error("expected repeat count")),
        };
        self.index += 1;
        self.expect_right_paren()?;
        let children = self.parse_children()?;
        Ok(ChildSpec::Repeat { count, children })
    }

    fn expect_identifier(&mut self) -> Result<(String, usize, usize), FixtureError> {
        let token = self.current();
        match &token.kind {
            TokenKind::Ident(value) => {
                let result = (value.clone(), token.line, token.column);
                self.index += 1;
                Ok(result)
            }
            _ => Err(self.parse_error("expected identifier")),
        }
    }

    fn expect_number(&mut self) -> Result<f32, FixtureError> {
        let token = self.current();
        match &token.kind {
            TokenKind::Number(value) => {
                let number = value.parse::<f32>().map_err(|_| FixtureError::Parse {
                    message: format!("invalid number `{value}`"),
                    line: token.line,
                    column: token.column,
                })?;
                self.index += 1;
                Ok(number)
            }
            _ => Err(self.parse_error("expected number")),
        }
    }

    fn expect_left_paren(&mut self) -> Result<(), FixtureError> {
        self.expect_token(|kind| matches!(kind, TokenKind::LeftParen), "expected `(`")
    }

    fn expect_right_paren(&mut self) -> Result<(), FixtureError> {
        self.expect_token(|kind| matches!(kind, TokenKind::RightParen), "expected `)`")
    }

    fn expect_left_brace(&mut self) -> Result<(), FixtureError> {
        self.expect_token(|kind| matches!(kind, TokenKind::LeftBrace), "expected `{`")
    }

    fn expect_token(&mut self, predicate: impl FnOnce(&TokenKind) -> bool, message: &str) -> Result<(), FixtureError> {
        if predicate(&self.current().kind) {
            self.index += 1;
            Ok(())
        } else {
            Err(self.parse_error(message))
        }
    }

    fn consume_dot(&mut self) -> bool {
        if matches!(self.current().kind, TokenKind::Dot) {
            self.index += 1;
            true
        } else {
            false
        }
    }

    fn expect_end(&self) -> Result<(), FixtureError> {
        if matches!(self.current().kind, TokenKind::End) {
            Ok(())
        } else {
            Err(self.parse_error("unexpected trailing input"))
        }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.index]
    }

    fn parse_error(&self, message: &str) -> FixtureError {
        let token = self.current();
        FixtureError::Parse { message: message.to_string(), line: token.line, column: token.column }
    }
}
