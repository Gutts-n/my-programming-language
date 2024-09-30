mod lexer;
use lexer::{generate_tokens};

fn main() {
    let text = "myNewLanguage = \"my string\" myOtherLanguage = 123.44 anotherOne = 5566 (x1)";
    generate_tokens(text);
}
