mod ast;
mod ast_printer;
mod lexer;

use ast::{Binary, Expr};
use ast_printer::AstPrinter;
use lexer::Scanner;
use std::fs;
mod tools;
use std::fs::File;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    Ok(())
}

fn generate_ast() {
    let path = "../ast.rs";
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    tools::generate_structs::define_ast(
        &mut writer,
        "Expr",
        &[
            ("Binary", "Expr left, Token operator, Expr right"),
            ("Grouping", "Expr expression"),
            ("Literal", "Object value"),
            ("Unary", "Token operator, Expr right"),
        ],
    )?;
    println!("AST written to {}", path);
}

fn read_file() {
    let text = fs::read_to_string("example.tk");

    match text {
        // Ok(data) => generate_tokens(&data),
        Ok(data) => println!("{:?}", Scanner::new(data).scan_tokens()),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_ast() {}
