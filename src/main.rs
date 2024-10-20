mod lexer;
use lexer::generate_tokens;
use std::fs;

fn main() {
    let text = fs::read_to_string("example.tk");
    match text {
        Ok(data) => generate_tokens(&data),
        Err(e) => println!("Error: {}", e),
    }
}
