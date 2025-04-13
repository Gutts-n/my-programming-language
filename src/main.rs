mod ast;
mod ast_printer;
mod lexer;
mod parser;
mod rpn_ast_printer;

use ast::{Binary, Expr, Grouping, Literal, LiteralValue, Unary};
use ast_printer::AstPrinter;
use lexer::{Scanner, Token, TokenType};
use parser::Parser;
use rpn_ast_printer::RPNAstPrinter;
use std::fs;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    read_ast();
}

fn read_file() {
    let text = fs::read_to_string("example.tk");

    match text {
        // Ok(data) => generate_tokens(&data),
        Ok(data) => println!("{:?}", Scanner::new(data).scan_tokens()),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_ast() {
    // let unary: Box<Expr> = Box::new(Expr::Unary(Unary {
    //     operator: Token::new(TokenType::Minus, "-".into(), None, 1),
    //     right: Box::new(Expr::Literal(Literal {
    //         value: LiteralValue::Integer(123),
    //     })),
    // }));
    //
    // let expr = Expr::Binary(Binary {
    //     left: unary,
    //     operator: Token::new(TokenType::Star, "*".into(), None, 1),
    //     right: Box::new(Expr::Grouping(Grouping {
    //         expression: Box::new(Expr::Literal(Literal {
    //             value: LiteralValue::Float(45.67),
    //         })),
    //     })),
    // });
    // let mut printer = AstPrinter {};
    let plus_op = &Token::new(TokenType::Plus, "+".into(), None, 1);
    let minus_op = &Token::new(TokenType::Minus, "-".into(), None, 1);
    let multiply_op = &Token::new(TokenType::Star, "*".into(), None, 1);
    let expr = Expr::Binary(Binary {
        left: Box::new(Expr::Binary(Binary {
            left: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Integer(1),
            })),
            operator: plus_op,
            right: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Integer(2),
            })),
        })),
        operator: multiply_op,
        right: Box::new(Expr::Binary(Binary {
            left: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Integer(4),
            })),
            operator: minus_op,
            right: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Integer(3),
            })),
        })),
    });
    let mut printer = RPNAstPrinter {};

    println!("{}", printer.print(&expr));
}
