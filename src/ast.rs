use crate::lexer::Token;

pub trait Expr {
    fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R;
}

pub trait ExprVisitor<R> {
    fn visit_binary_expr(&mut self, expr: &Binary) -> R;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> R;
    fn visit_literal_expr(&mut self, expr: &Literal) -> R;
    fn visit_unary_expr(&mut self, expr: &Unary) -> R;
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
pub struct AstPrinter;

impl Expr for Binary {
    fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
        visitor.visit_binary_expr(self)
    }
}

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

impl Expr for Grouping {
    fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
        visitor.visit_grouping_expr(self)
    }
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: Literal,
}

impl Expr for Literal {
    fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
        visitor.visit_literal_expr(self)
    }
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Expr for Unary {
    fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
        visitor.visit_unary_expr(self)
    }
}
