extern crate lazy_static;
extern crate regex;
use self::lazy_static::lazy_static;
use self::regex::Regex;
use std::any::Any;
use std::collections::HashMap; // This line is crucial!
                               //
#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RigthParen,
    LeftBrace,
    RightBrace,
    Comma,
    Print,
    Dot,
    Minus,
    Plus,
    Semicolon,
    SLASH,
    Star,

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
    If,
    Nil,
    Or,
    ECHO,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
    EqualAndEqual,
}
#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    pub lexeme: String,
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

pub struct Scanner {
    source_code: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("if", TokenType::If);
        map.insert("else", TokenType::Else);
        map.insert("class", TokenType::Class);
        map.insert("false", TokenType::False);
        map.insert("for", TokenType::For);
        map.insert("fun", TokenType::Fun);
        map.insert("nil", TokenType::Nil);
        map.insert("while", TokenType::While);
        map.insert("true", TokenType::True);
        map.insert("or", TokenType::Or);
        map.insert("and", TokenType::And);
        map.insert("print", TokenType::Print);
        map.insert("return", TokenType::Return);
        map.insert("super", TokenType::Super);
        map.insert("this", TokenType::This);
        map.insert("var", TokenType::Var);
        map
    };
}

impl Scanner {
    pub fn new(source_code: String) -> Scanner {
        Scanner {
            source_code,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source_code.chars().nth(self.current as usize);

        self.current += 1;

        return c;
    }

    fn add_token_with_type(&mut self, token_type: TokenType) {
        self.add_token(token_type, None);
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Box<dyn Any>>) {
        let text = &self.source_code[self.start as usize..self.current as usize];
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line));
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
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
            Some('*') => self.add_token_with_type(TokenType::Star),
            Some('o') => {
                if self.validate_symbol('r') {
                    self.add_token_with_type(TokenType::Or);
                }
            }
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
                // This code is validating if there is a dual slash and ignoring anything after it
                // because it represents a commentary
                if self.validate_symbol('/') {
                    while self.peek().unwrap() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.validate_symbol('*') {
                    let mut is_multiple_line_comment = true;
                    while is_multiple_line_comment && !self.is_at_end() {
                        self.advance();
                        if self.peek_next().unwrap() == '*'
                            && self.peek_next_after_next().unwrap() == '/'
                        {
                            is_multiple_line_comment = false;
                            // advanced twice to ignore the end of the multiple line comment as
                            // tokens
                            self.advance();
                            self.advance();
                        }
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
            default if self.is_digit(default.unwrap()) => self.number(),
            default if self.is_alpha(default.unwrap()) => {
                self.identifier();
            }
            default => {
                panic!(
                    "Error token \"{}\"not recognized, at line {}",
                    default.unwrap_or('?'),
                    self.line
                );
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

    fn peek_next_after_next(&self) -> Option<char> {
        if self.current + 2 >= self.source_code.len() as u32 {
            return Some('\0');
        }

        return self.source_code.chars().nth((self.current + 2) as usize);
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
                self.current += 1;
                return true;
            }
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c <= 'a' && c <= 'Z') || (c == '_');
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        return self.is_alpha(c) && self.is_digit(c);
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek().unwrap()) {
            self.advance();
        }

        let text = &self.source_code[self.start as usize..self.current as usize];
        let token_type: TokenType = KEYWORDS.get(text).unwrap_or(&TokenType::Identifier).clone();
        self.add_token_with_type(token_type);
    }
}

pub fn generate_tokens(text: &str) {
    let string_regex = Regex::new(r#""[\w\s]*""#).unwrap();
    let number_regex = Regex::new(r"\d+\.?\d+").unwrap();
    let parentheses_regex = Regex::new(r"\(|\)").unwrap();
    let comment_regex = Regex::new(r"\/{2}.*").unwrap();

    let strings_matched = string_regex.find_iter(text);
    let parentheses_matched = parentheses_regex.find_iter(text);
    let numbers_matched = number_regex.find_iter(text);
    let comments_matched = comment_regex.find_iter(text);

    for mat in comments_matched {
        println!("Comments matched: {}", mat.as_str());
    }

    for mat in parentheses_matched {
        println!("Parentheses matched: {}", mat.as_str());
    }

    for mat in strings_matched {
        println!("Strings matched: {}", mat.as_str());
    }

    for mat in numbers_matched {
        println!("Numbers matched: {}", mat.as_str());
    }
}
