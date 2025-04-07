use crate::ast::{Binary, Expr, ExprVisitor, Grouping, Literal, LiteralValue, Unary};

pub struct RPNAstPrinter;

impl ExprVisitor for RPNAstPrinter {
    type Output = String;
    fn visit_binary_expr(&mut self, expr: &Binary) -> Self::Output {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Self::Output {
        self.parenthesize("group", &[&expr.expression])
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> Self::Output {
        match &expr.value {
            LiteralValue::Nil => String::from("nil"),
            LiteralValue::Boolean(b) => b.to_string(),
            LiteralValue::Float(n) => n.to_string(),
            LiteralValue::Integer(n) => n.to_string(),
            LiteralValue::String(s) => s.clone(),
        }
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> Self::Output {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }
}

impl RPNAstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Box<Expr>]) -> String {
        let mut builder = String::new();

        // builder.push('(');

        for expr in exprs {
            builder.push_str(&expr.accept(self));
            builder.push(' ');
        }

        builder.push_str(name);
        // builder.push(')');
        builder
    }
}
