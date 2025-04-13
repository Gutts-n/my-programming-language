use crate::lexer::Token;
use std::fmt::Debug;

// We'll use an enum approach instead of trait objects
#[derive(Debug)]
pub enum Expr<'a> {
    Binary(Binary<'a>),
    Grouping(Grouping<'a>),
    Literal(Literal),
    Unary(Unary<'a>),
}

pub trait ExprVisitor {
    type Output;
    fn visit_binary_expr(&mut self, expr: &Binary) -> Self::Output;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Self::Output;
    fn visit_literal_expr(&mut self, expr: &Literal) -> Self::Output;
    fn visit_unary_expr(&mut self, expr: &Unary) -> Self::Output;
}

impl Expr<'_> {
    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Expr::Binary(b) => visitor.visit_binary_expr(b),
            Expr::Grouping(g) => visitor.visit_grouping_expr(g),
            Expr::Literal(l) => visitor.visit_literal_expr(l),
            Expr::Unary(u) => visitor.visit_unary_expr(u),
        }
    }
}

#[derive(Debug)]
pub struct Binary<'a> {
    pub left: Box<Expr<'a>>,
    pub operator: &'a Token,
    pub right: Box<Expr<'a>>,
}

#[derive(Debug)]
pub struct Grouping<'a> {
    pub expression: Box<Expr<'a>>,
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: LiteralValue,
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub struct Unary<'a> {
    pub operator: &'a Token,
    pub right: Box<Expr<'a>>,
}
