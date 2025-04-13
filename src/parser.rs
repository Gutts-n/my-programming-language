use ast::{Binary, Expr, Unary};
use lexer::{Token, TokenType};
use std::any::Any;

use crate::ast::{Grouping, Literal, LiteralValue};

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, current: 0 }
    }

    fn comparison(&mut self) -> Box<Expr<'a>> {
        let mut expression = self.term();
        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterOrEqual,
            TokenType::Less,
            TokenType::LessOrEqual,
        ]) {
            let operator: &'a Token = self.previous();
            let right: Box<Expr<'a>> = self.term();
            expression = Box::new(Expr::Binary(Binary {
                left: expression,
                operator,
                right,
            }));
        }

        expression
    }

    fn expression(&mut self) -> Box<Expr<'a>> {
        self.equality()
    }

    fn equality(&mut self) -> Box<Expr<'a>> {
        let mut expression = self.comparison();

        while self.match_tokens(&[TokenType::BangOrEqual, TokenType::EqualAndEqual]) {
            let operator: &'a Token = self.previous();
            let right: Box<Expr<'a>> = self.comparison();
            expression = Box::new(Expr::Binary(Binary {
                left: expression,
                operator,
                right,
            }));
        }

        expression
    }

    fn term(&mut self) -> Box<Expr<'a>> {
        let mut expression = self.factor();

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator: &'a Token = self.previous();
            let right: Box<Expr<'a>> = self.factor();
            expression = Box::new(Expr::Binary(Binary {
                left: expression,
                operator,
                right,
            }));
        }

        expression
    }

    fn primary(&mut self) -> Box<Expr<'a>> {
        if self.match_tokens(&[TokenType::Number]) {
            if let Some(int) = self.previous().get_literal::<i64>() {
                return Box::new(Expr::Literal(Literal {
                    value: LiteralValue::Integer(*int),
                }));
            } else if let Some(float) = self.previous().get_literal::<f64>() {
                return Box::new(Expr::Literal(Literal {
                    value: LiteralValue::Float(*float),
                }));
            } else {
                panic!("Expected number literal");
            }
        } else if self.match_tokens(&[TokenType::String]) {
            if let Some(s) = self.previous().get_literal::<String>() {
                return Box::new(Expr::Literal(Literal {
                    value: LiteralValue::String(s.clone()),
                }));
            } else {
                panic!("Expected string literal");
            }
        } else if self.match_tokens(&[TokenType::True, TokenType::False]) {
            if let Some(b) = self.previous().get_literal::<bool>() {
                return Box::new(Expr::Literal(Literal {
                    value: LiteralValue::Boolean(*b),
                }));
            } else {
                panic!("Expected boolean literal");
            }
        } else if self.match_tokens(&[TokenType::Nil]) {
            if self.previous().get_literal::<()>().is_some() {
                return Box::new(Expr::Literal(Literal {
                    value: LiteralValue::Nil,
                }));
            } else {
                panic!("Expected nil");
            }
        } else if self.match_tokens(&[TokenType::LeftParen]) {
            let expression = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Box::new(Expr::Grouping(Grouping { expression }));
        } else {
            panic!("Expected expression");
        }
    }

    fn consume(&mut self, token_type: TokenType, error_message: &str) {}

    fn unary(&mut self) -> Box<Expr<'a>> {
        while self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator: &'a Token = self.previous();
            let right: Box<Expr<'a>> = self.unary();
            return Box::new(Expr::Unary(Unary { operator, right }));
        }

        self.primary()
    }

    fn factor(&mut self) -> Box<Expr<'a>> {
        let mut expression = self.unary();

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator: &'a Token = self.previous();
            let right: Box<Expr<'a>> = self.unary();
            expression = Box::new(Expr::Binary(Binary {
                left: expression,
                operator,
                right,
            }));
        }

        expression
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
