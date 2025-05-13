use ast::{Binary, Expr, Unary};
use lexer::{Token, TokenType};
use std::any::Any;
use std::fmt;

use crate::ast::{Grouping, Literal, LiteralValue};

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Box<Expr<'a>>> {
        match self.comma() {
            Ok(expr) => Some(expr),
            Err(_) => None,
        }
    }

    fn comparison(&mut self) -> Result<Box<Expr<'a>>, ParserError> {
        let mut expression = self.term()?;
        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterOrEqual,
            TokenType::Less,
            TokenType::LessOrEqual,
        ]) {
            let operator: &'a Token = self.previous();
            let right: Box<Expr<'a>> = self.term()?;
            expression = Box::new(Expr::Binary(Binary {
                left: expression,
                operator,
                right,
            }));
        }

        Ok(expression)
    }

    fn expression(&mut self) -> Result<Box<Expr<'a>>, ParserError> {
        self.ternary()
    }

    fn equality(&mut self) -> Result<Box<Expr<'a>>, ParserError> {
        let mut expression = self.comparison()?;

        while self.match_tokens(&[TokenType::BangAndEqual, TokenType::EqualAndEqual]) {
            let operator: &'a Token = self.previous();
            let right: Box<Expr<'a>> = self.comparison()?;
            expression = Box::new(Expr::Binary(Binary {
                left: expression,
                operator,
                right,
            }));
        }

        Ok(expression)
    }

    fn term(&mut self) -> Result<Box<Expr<'a>>, ParserError> {
        let mut expression = self.factor()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator: &'a Token = self.previous();
            let right: Box<Expr<'a>> = self.factor()?;
            expression = Box::new(Expr::Binary(Binary {
                left: expression,
                operator,
                right,
            }));
        }

        Ok(expression)
    }

    fn primary(&mut self) -> Result<Box<Expr<'a>>, ParserError> {
        if self.match_tokens(&[TokenType::Number]) {
            if let Some(int) = self.previous().get_literal::<i64>() {
                return Ok(Box::new(Expr::Literal(Literal {
                    value: LiteralValue::Integer(*int),
                })));
            } else if let Some(float) = self.previous().get_literal::<f64>() {
                return Ok(Box::new(Expr::Literal(Literal {
                    value: LiteralValue::Float(*float),
                })));
            } else {
                return Err(self.create_error(self.previous(), "Expected number literal"));
            }
        } else if self.match_tokens(&[TokenType::String]) {
            if let Some(s) = self.previous().get_literal::<String>() {
                return Ok(Box::new(Expr::Literal(Literal {
                    value: LiteralValue::String(s.clone()),
                })));
            } else {
                return Err(self.create_error(self.previous(), "Expected string literal"));
            }
        } else if self.match_tokens(&[TokenType::True, TokenType::False]) {
            if let Some(b) = self.previous().get_literal::<bool>() {
                return Ok(Box::new(Expr::Literal(Literal {
                    value: LiteralValue::Boolean(*b),
                })));
            } else {
                return Err(self.create_error(self.previous(), "Expected boolean literal"));
            }
        } else if self.match_tokens(&[TokenType::Nil]) {
            if self.previous().get_literal::<()>().is_some() {
                return Ok(Box::new(Expr::Literal(Literal {
                    value: LiteralValue::Nil,
                })));
            } else {
                return Err(self.create_error(self.previous(), "Expected nil"));
            }
        } else if self.match_tokens(&[TokenType::LeftParen]) {
            let expression = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Box::new(Expr::Grouping(Grouping { expression })));
        } else {
            Err(self.create_error(self.peek(), "Expect expression"))
        }
    }

    fn create_error(&mut self, token: &Token, message: &str) -> ParserError {
        self.report_error(token, message);
        ParserError()
    }

    fn report_error(&mut self, token: &Token, message: &str) {
        match token.token_type {
            TokenType::EOF => {
                self.report(token.line as usize, " at end", message);
            }
            _ => {
                self.report(
                    token.line as usize,
                    &format!(" at '{}'", token.lexeme),
                    message,
                );
            }
        }
    }

    fn consume(
        &mut self,
        token_type: TokenType,
        error_message: &str,
    ) -> Result<&Token, ParserError> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }

        Err(self.create_error(self.peek(), error_message))
    }

    fn report(&mut self, line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, location, message);
    }

    fn comma(&mut self) -> Result<Box<Expr<'a>>, ParserError> {
        let mut expression = self.expression()?;

        while self.match_tokens(&[TokenType::Comma]) {
            let operator: &'a Token = self.previous();
            let right: Box<Expr<'a>> = self.expression()?;
            expression = Box::new(Expr::Binary(Binary {
                left: expression,
                operator,
                right,
            }));
        }

        Ok(expression)
    }

    fn ternary(&mut self) -> Result<Box<Expr<'a>>, ParserError> {
        let mut expression = self.equality()?;

        while self.match_tokens(&[TokenType::Question, TokenType::Colon]) {
            let operator: &'a Token = self.previous();
            let right: Box<Expr<'a>> = self.expression()?;
            expression = Box::new(Expr::Binary(Binary {
                left: expression,
                operator,
                right,
            }));
        }

        Ok(expression)
    }

    fn unary(&mut self) -> Result<Box<Expr<'a>>, ParserError> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator: &'a Token = self.previous();
            let right: Box<Expr<'a>> = self.unary()?;
            return Ok(Box::new(Expr::Unary(Unary { operator, right })));
        }

        self.primary()
    }

    fn factor(&mut self) -> Result<Box<Expr<'a>>, ParserError> {
        let mut expression = self.unary()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator: &'a Token = self.previous();
            let right: Box<Expr<'a>> = self.unary()?;
            expression = Box::new(Expr::Binary(Binary {
                left: expression,
                operator,
                right,
            }));
        }

        Ok(expression)
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &'a Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        &self.peek().token_type == &TokenType::EOF
    }

    fn peek(&self) -> &'a Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &'a Token {
        &self.tokens[self.current - 1]
    }
}

#[derive(Debug)]
struct ParserError();

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParserError: {}", "Error happened")
    }
}

impl std::error::Error for ParserError {}
