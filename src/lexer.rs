extern crate regex;
use self::regex::Regex;
use std::any::Any;

#[derive(Clone)]
enum TokenType {
    // Single-character tokens.
    LeftParen,
    RigthParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    SLASH,
    Start,

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
    EqualAndEqual,
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Box<dyn Any>>,
    line: u32,
}

impl Token {
    fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Box<dyn Any>>,
        line: u32,
    ) -> Token {
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

    fn add_token(&mut self, token_type: TokenType, literal: Option<Box<dyn Any>>) {
        let text = &self.source_code[self.start as usize..self.current as usize];
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line));
    }

    fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, String::new(), None, self.line));
        return &self.tokens;
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source_code.len() as u32;
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            Some('(') => self.add_token_with_type(TokenType::LeftParen),
            Some(')') => self.add_token_with_type(TokenType::RigthParen),
            Some('{') => self.add_token_with_type(TokenType::LeftBrace),
            Some('}') => self.add_token_with_type(TokenType::RightBrace),
            Some(',') => self.add_token_with_type(TokenType::Comma),
            Some('.') => self.add_token_with_type(TokenType::Dot),
            Some('-') => self.add_token_with_type(TokenType::Minus),
            Some('+') => self.add_token_with_type(TokenType::Plus),
            Some(';') => self.add_token_with_type(TokenType::Semicolon),
            Some('*') => self.add_token_with_type(TokenType::Start),
            // These next validations are comparing the next character after the current one and
            // validating if its a equal to return the symbol + equal combination
            Some('!') => {
                let token_type = if self.validate_symbol('=') {
                    TokenType::BangOrEqual
                } else {
                    TokenType::Bang
                };
                self.add_token_with_type(token_type)
            }
            Some('=') => {
                let token_type = if self.validate_symbol('=') {
                    TokenType::EqualAndEqual
                } else {
                    TokenType::Equal
                };
                self.add_token_with_type(token_type)
            }
            Some('<') => {
                let token_type = if self.validate_symbol('=') {
                    TokenType::LessOrEqual
                } else {
                    TokenType::Less
                };
                self.add_token_with_type(token_type)
            }
            Some('>') => {
                let token_type = if self.validate_symbol('=') {
                    TokenType::GreaterOrEqual
                } else {
                    TokenType::Greater
                };
                self.add_token_with_type(token_type)
            }
            Some('/') => {
                // This code is validating if there is a dual slash and ignoring anything inside of
                // this because it represents a commentary
                if self.validate_symbol('/') {
                    while self.peek().unwrap() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token_with_type(TokenType::SLASH)
                };
            }
            Some('"') => self.string(),
            Some('\n') => self.line = self.line + 1,
            Some(' ') => {}
            Some('\r') => {}
            Some('\t') => {}
            _ => {
                if (self.is_digit(_)) {
                    self.number()
                } else {
                    panic!("Error token \"{}\"not recognized", none.unwrap_or('?'));
                }
            }
            none => {
                // TODO change this line after
                panic!("Error token \"{}\"not recognized", none.unwrap_or('?'));
            }
        }
    }

    fn string(&mut self) {
        while self.peek().unwrap() != '"' && !self.is_at_end() {
            if self.peek().unwrap() == '\n' {
                self.line = self.line + 1
            }
            self.advance();
        }

        if self.is_at_end() {
            panic!("Line: {} Unterminated string", self.line)
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let end = self.current - 1;
        let start = self.start + 1;
        let value = &self.source_code[start as usize..end as usize];
        self.add_token(TokenType::String, Some(Box::new(value.to_string())));
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn number(&mut self) {
        while self.is_digit(self.peek().unwrap()) {
            self.advance();
        }

        if self.peek().unwrap() == '.' && self.is_digit(self.peek_next().unwrap()) {
            self.advance();

            while self.is_digit(self.peek().unwrap()) {
                self.advance();
            }
        }
        match self.source_code[self.start as usize..self.current as usize].parse::<f64>() {
            Ok(value) => self.add_token(TokenType::Number, Some(Box::new(value))),
            Err(e) => panic!("Failed to parse: {}", e),
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source_code.len() as u32 {
            return Some('\0');
        }

        return self.source_code.chars().nth((self.current + 1) as usize);
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return Some('\0');
        }

        return self.source_code.chars().nth(self.current as usize);
    }

    fn validate_symbol(&mut self, c: char) -> bool {
        match self.is_at_end() || self.source_code.chars().nth(self.current as usize).unwrap() != c
        {
            true => return false,
            false => {
                self.current = self.current + 1;
                return true;
            }
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
