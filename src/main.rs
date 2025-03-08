mod lexer;
use lexer::Scanner;
use std::fs;

fn main() {
    let text = fs::read_to_string("example.tk");

    match text {
        // Ok(data) => generate_tokens(&data),
        Ok(data) => println!("{:?}", Scanner::new(data).scan_tokens()),
        Err(e) => println!("Error: {}", e),
    }
}
