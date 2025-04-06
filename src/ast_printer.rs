use crate::ast::{Binary, Expr, ExprVisitor, Grouping, Literal, Unary};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &Binary) -> String {
        format!(
            "({} {} {})",
            expr.operator.lexeme,
            self.print(&expr.left),
            self.print(&expr.right)
        )
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> String {
        format!("(group {})", self.print(&expr.expression))
    }

    fn visit_literal_expr(&self, expr: &Literal) -> String {
        expr.value.to_string()
    }

    fn visit_unary_expr(&self, expr: &Unary) -> String {
        format!("({} {})", expr.operator.lexeme, self.print(&expr.right))
    }
}
