use crate::token::{Token, TokenType};
use std::{error::Error, fmt, mem, str::Chars};

pub struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,

    char_iter: Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),

            start: 0,
            current: 0,
            line: 1,

            char_iter: source.chars(),
            current_char: None,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LexerError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), None, self.line));
        Ok(mem::take(&mut self.tokens))
    }

    fn scan_token(&mut self) -> Result<(), LexerError> {
        if let Some(c) = self.advance() {
            match c {
                // single char lexemes
                '(' => self.add_token(TokenType::LeftParen, None),
                ')' => self.add_token(TokenType::RightParen, None),
                '{' => self.add_token(TokenType::LeftBrace, None),
                '}' => self.add_token(TokenType::RightBrace, None),
                ',' => self.add_token(TokenType::Comma, None),
                '.' => self.add_token(TokenType::Dot, None),
                '-' => self.add_token(TokenType::Minus, None),
                '+' => self.add_token(TokenType::Plus, None),
                ';' => self.add_token(TokenType::Semicolon, None),
                '*' => self.add_token(TokenType::Star, None),

                // operators (single or double char lexemes)
                '!' => self.check_match('=', TokenType::BangEqual, TokenType::Bang),
                '=' => self.check_match('=', TokenType::EqualEqual, TokenType::Equal),
                '<' => self.check_match('=', TokenType::LessEqual, TokenType::Less),
                '>' => self.check_match('=', TokenType::GreaterEqual, TokenType::Greater),

                // longer lexemes
                '/' => {
                    // ignore comments
                    if self.match_char('/') {
                        while let Some(c) = self.advance() {
                            if c == '\n' {
                                break;
                            }
                        }
                    } else {
                        let _ = self.add_token(TokenType::Slash, None);
                    }
                    Ok(())
                }

                // ignore whitespace
                ' ' | '\r' | '\t' => Ok(()),

                // newlines
                '\n' => {
                    self.line += 1;
                    Ok(())
                }

                // string literals
                '"' => self.string(),

                // number literals
                c if c.is_digit(10) => self.number(),

                // reserved words and identifiers
                c if c.is_alphabetic() => self.identifier(),

                // default
                _ => Err(LexerError::UnexpectedCharacter {
                    line: self.line,
                    character: c,
                }),
            }
        } else {
            let _ = self.add_token(TokenType::EOF, None);
            Ok(())
        }
    }

    fn identifier(&mut self) -> Result<(), LexerError> {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() {
                self.advance();
            } else {
                break;
            }
        }
        let text = &self.source[self.start..self.current];
        let token_type = match text {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };
        self.add_token(token_type, None)?;

        Ok(())
    }

    fn number(&mut self) -> Result<(), LexerError> {
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }
        if let Some(c) = self.peek() {
            if c == '.' {
                self.advance();
                while let Some(c) = self.peek() {
                    if c.is_digit(10) {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
        }
        let literal = self.source[self.start..self.current].to_string();
        self.add_token(TokenType::Number, Some(literal))?;

        Ok(())
    }

    fn string(&mut self) -> Result<(), LexerError> {
        while let Some(c) = self.advance() {
            match c {
                '"' => break,
                '\n' => self.line += 1,
                _ => {}
            }

            if self.is_at_end() {
                return Err(LexerError::UnterminatedString { line: self.line });
            }
        }

        let literal = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, Some(literal))?;

        Ok(())
    }

    fn peek(&self) -> Option<char> {
        self.char_iter.clone().next()
    }

    fn check_match(
        &mut self,
        expected: char,
        on_match: TokenType,
        otherwise: TokenType,
    ) -> Result<(), LexerError> {
        let token_type = if self.match_char(expected) {
            on_match
        } else {
            otherwise
        };
        self.add_token(token_type, None)?;

        Ok(())
    }

    fn match_char(&mut self, expected: char) -> bool {
        match self.current_char {
            Some(c) if c == expected => {
                self.advance();
                true
            }
            _ => false,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let next_char = self.char_iter.next();
        self.current += 1;
        self.current_char = next_char;
        next_char
    }

    fn add_token(
        &mut self,
        token_type: TokenType,
        literal: Option<String>,
    ) -> Result<(), LexerError> {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line));

        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter { line: usize, character: char },
    UnterminatedString { line: usize },
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::UnexpectedCharacter { line, character } => {
                write!(f, "Unexpected character '{}' at line {}", character, line)
            }
            LexerError::UnterminatedString { line } => {
                write!(f, "Unterminated string at line {}", line)
            }
        }
    }
}

impl Error for LexerError {}
