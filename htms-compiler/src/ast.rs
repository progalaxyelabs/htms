//! AST (Abstract Syntax Tree) node definitions for HTMS

use serde::{Deserialize, Serialize};
use crate::Location;

/// Root node of the AST
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub body: Vec<Declaration>,
    pub loc: Location,
}

/// Top-level declarations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Declaration {
    Component(ComponentDecl),
    Section(SectionDecl),
    Page(PageDecl),
}

/// Component declaration: `component NavBar { ... }`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDecl {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub attributes: Vec<Attribute>,
    pub body: Vec<Node>,
    pub loc: Location,
}

/// Section declaration: `section HeroSection { ... }`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionDecl {
    pub name: String,
    pub body: Vec<Node>,
    pub loc: Location,
}

/// Page declaration: `page home "/" { ... }`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageDecl {
    pub name: String,
    pub route: String,
    pub body: Vec<Node>,
    pub loc: Location,
}

/// Component parameter: `(item: user)`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub binding: String,
    pub loc: Location,
}

/// Any node that can appear in a body
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Node {
    Element(Element),
    ComponentRef(ComponentRef),
    Text(TextNode),
    If(IfStatement),
    Each(EachStatement),
    Slot(Slot),
}

/// HTML element: `div [class: "container"] { ... }`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    pub tag: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
    pub loc: Location,
}

/// Attribute: `class: "container"` or `onClick: submit`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub value: Expression,
    pub loc: Location,
}

/// Component reference: `NavBar` or `Card (item: ctx.user)`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentRef {
    pub name: String,
    pub parameters: Vec<ParameterBinding>,
    pub children: Vec<Node>,
    pub loc: Location,
}

/// Parameter binding: `item: ctx.user`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterBinding {
    pub name: String,
    pub value: Expression,
    pub loc: Location,
}

/// Text node: `{{ Hello "world" }}` or `ctx.user.name`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextNode {
    pub content: String,
    pub is_dynamic: bool,
    pub loc: Location,
}

/// Slot: `@slot`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slot {
    pub loc: Location,
}

/// If statement: `@if ctx.show { } @else { }`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub consequent: Vec<Node>,
    pub alternate: Option<Alternate>,
    pub loc: Location,
}

/// Else branch (either block or else-if)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Alternate {
    Block(Vec<Node>),
    ElseIf(Box<IfStatement>),
}

/// Each statement: `@each ctx.items as item, index { }`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EachStatement {
    pub iterable: Expression,
    pub item_name: String,
    pub index_name: Option<String>,
    pub body: Vec<Node>,
    pub loc: Location,
}

/// Expression (attribute values, conditions, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Expression {
    /// String literal: `"hello"`
    String(StringLiteral),
    /// Number literal: `42`
    Number(NumberLiteral),
    /// Boolean literal: `true` or `false`
    Boolean(BooleanLiteral),
    /// Context path: `ctx.user.name`
    ContextPath(ContextPathExpr),
    /// Identifier: `item` (loop variable) or `submit` (action)
    Identifier(IdentifierExpr),
    /// Member access: `item.name`, `user.profile.bio`
    MemberAccess(MemberAccessExpr),
    /// Binary expression: `a + b`, `a == b`
    Binary(BinaryExpr),
    /// Ternary expression: `a ? b : c`
    Ternary(TernaryExpr),
    /// Function call: `submit(item.id)`
    Call(CallExpr),
    /// Event handler: `onClick.prevent: submit`
    Event(EventExpr),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringLiteral {
    pub value: String,
    pub loc: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberLiteral {
    pub value: f64,
    pub loc: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BooleanLiteral {
    pub value: bool,
    pub loc: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextPathExpr {
    /// Full path including "ctx." prefix
    pub path: String,
    pub loc: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentifierExpr {
    pub name: String,
    pub loc: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberAccessExpr {
    /// Object expression (can be nested member access)
    pub object: Box<Expression>,
    /// Property name
    pub property: String,
    pub loc: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryExpr {
    pub operator: BinaryOp,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub loc: Location,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOp {
    // Comparison
    Eq,     // ==
    Ne,     // !=
    Lt,     // <
    Le,     // <=
    Gt,     // >
    Ge,     // >=
    // Logical
    And,    // &&
    Or,     // ||
    // Arithmetic
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TernaryExpr {
    pub condition: Box<Expression>,
    pub consequent: Box<Expression>,
    pub alternate: Box<Expression>,
    pub loc: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallExpr {
    pub callee: String,
    pub arguments: Vec<Expression>,
    pub loc: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventExpr {
    pub event: String,
    pub modifiers: Vec<String>,
    pub action: String,
    pub arguments: Vec<Expression>,
    pub loc: Location,
}
