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
    let result = fs::read_to_string("example.tk");
    let code: String = match result {
        // Ok(string_value) => "1 + 2 / 4 - 2 * 45 - 2 + (55 * 12)".into(),
        Ok(string_value) => string_value,
        Err(err) => {
            panic!("Failed: {}", err)
        }
    };
    let mut scanner = Scanner::new(code);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    // let expression = parser.parse();
    let expression = match parser.parse() {
        Some(expr) => expr,
        none => {
            panic!("Failed: {}", 1)
        }
    };
    // if had_error {
    //     return;
    // }

    let mut printer = RPNAstPrinter {};
    println!("{}", printer.print(&expression));
}
