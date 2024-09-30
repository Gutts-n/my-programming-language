extern crate regex;

use self::regex::Regex;

#[derive(Clone)]
enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    Bang,
    BangOrEqual,
    Equal,
    Arrow,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    IF,
    NIL,
    OR,
    ECHO,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Clone)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: u32,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: u32) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

struct Scanner {
    source_code: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    fn new(source_code: String) -> Scanner {
        Scanner {
            source_code,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        return self.source_code.chars().nth(self.current as usize);
    }

    fn add_token_with_type(&mut self, token_type: TokenType) {
        self.add_token(token_type, None);
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = &self.source_code[self.start as usize..self.current as usize];
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line));
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.isAtEnd() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, String::new(), None, self.line));
        return self.tokens.clone();
    }

    fn isAtEnd(&self) -> bool {
        return self.current >= self.source_code.len() as u32;
    }

    fn scan_token(&self) {
        let c = self.advance();
        match c {
            Some('(') => self.add_token_with_type(TokenType::LEFT_PAREN),
            Some(')') => self.add_token_with_type(TokenType::RIGHT_PAREN),
            Some('{') => self.add_token_with_type(TokenType::LEFT_BRACE),
            Some('}') => self.add_token_with_type(TokenType::RIGHT_BRACE),
            Some(',') => self.add_token_with_type(TokenType::COMMA),
            Some('.') => self.add_token_with_type(TokenType::DOT),
            Some('-') => self.add_token_with_type(TokenType::MINUS),
            Some('+') => self.add_token_with_type(TokenType::PLUS),
            Some(';') => self.add_token_with_type(TokenType::SEMICOLON),
            Some('*') => self.add_token_with_type(TokenType::STAR),
            None => {}
        }
    }
}

pub fn generate_tokens(text: &str) {
    let string_regex = Regex::new(r#""[\w\s]*""#).unwrap();
    let number_regex = Regex::new(r"\d+\.?\d+").unwrap();
    let parentheses_regex = Regex::new(r"\(+\)+").unwrap();

    let strings_matched = string_regex.find_iter(text);
    let parentheses_matched = parentheses_regex.find_iter(text);
    let numbers_matched = number_regex.find_iter(text);

    for mat in parentheses_matched {
        println!("Parentheses matched: {}", mat.as_str());
    }

    for mat in strings_matched {
        println!("Strings matched: {}", mat.as_str());
    }

    for mat in numbers_matched {
        println!("Numbers matched: {}", mat.as_str());
    }

    for character in text.chars() {
        println!("{}", character);
    }
}
