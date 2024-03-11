use crate::ast::{Expr, Stmt};
use crate::error::ParserError;
use crate::token::{Token, TokenType};
use crate::value::Value;

pub struct Parser<'a> {
    pub tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statemtents = Vec::new();

        while !self.is_at_end() {
            statemtents.push(self.declaration()?);
        }

        Ok(statemtents)
    }

    pub fn declaration(&mut self) -> Result<Stmt, ParserError> {
        if self.match_token(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParserError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let initializer = if self.match_token(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var(name.lexeme, initializer))
    }

    fn statement(&mut self) -> Result<Stmt, ParserError> {
        if self.match_token(&[TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expression(value))
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
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
        self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;
        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;
        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;
        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        match self.peek().token_type {
            TokenType::False => {
                self.advance();
                Ok(Expr::Literal {
                    value: Value::Boolean(false),
                })
            }
            TokenType::True => {
                self.advance();
                Ok(Expr::Literal {
                    value: Value::Boolean(true),
                })
            }
            TokenType::Nil => {
                self.advance();
                Ok(Expr::Literal { value: Value::Nil })
            }
            TokenType::Number | TokenType::String => {
                let token = self.advance();
                match token.token_type {
                    TokenType::Number => {
                        let value = token
                            .literal
                            .as_ref()
                            .unwrap()
                            .parse()
                            .unwrap_or_else(|_| panic!("Failed to parse number."));
                        Ok(Expr::Literal {
                            value: Value::Number(value),
                        })
                    }
                    TokenType::String => {
                        let value = token.literal.as_ref().unwrap().clone();
                        Ok(Expr::Literal {
                            value: Value::String(value),
                        })
                    }
                    _ => unreachable!(),
                }
            }
            TokenType::Identifier => {
                let token = self.advance();
                Ok(Expr::Literal {
                    value: Value::String(token.lexeme().to_string()),
                })
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;

                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;

                Ok(Expr::Grouping(Box::new(expr)))
            }
            _ => Err(ParserError::new(
                format!("Unexpected token {:?}", self.peek().token_type),
                self.peek().line,
            )),
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParserError> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(ParserError::new(
                format!("{message}, found {:?}", self.peek().token_type),
                self.peek().line,
            ))
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }
            self.advance();
        }
    }
}
