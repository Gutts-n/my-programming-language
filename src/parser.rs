use lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Binary(Box<BinaryExpr>),
    // Other expression types can be added here
}

#[derive(Debug)]
pub struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl Expr {
    fn print(&self) {
        match self {
            Expr::Binary(binary) => {
                println!("({} {:?} {:?})", binary.operator.lexeme, binary.left, binary.right);
            }
        }
    }
}

