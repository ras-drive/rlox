use std::collections::HashMap;

use crate::{
    error::SyntaxError,
    token::{Literal, Token, TokenType},
};
use lazy_static::lazy_static;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = generate_keywords();
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            Literal::NilLiteral,
            self.line,
        ));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::LeftBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let b = self.match_char('=');
                self.add_token(if b {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                });
            }
            '=' => {
                let b = self.match_char('=');
                self.add_token(if b {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                });
            }
            '<' => {
                let b = self.match_char('=');
                self.add_token(if b {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                });
            }
            '>' => {
                let b = self.match_char('=');
                self.add_token(if b {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                });
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            '0'..='9' => {
                self.number();
            }
            'A'..='Z' | 'a'..='z' => {
                self.identifier();
            }
            _ => SyntaxError::report(
                self.line,
                "Unexpected character.".to_string(),
                "".to_string(),
            ),
        }
    }

    fn advance(&mut self) -> char {
        let next = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        next
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        };
        self.source.chars().nth(self.current).unwrap()
    }

    fn push_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line));
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.push_token(token_type, Literal::NilLiteral);
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            SyntaxError::report(self.line, "Unterminated String".to_string(), "".to_string())
        }

        self.advance();

        let str = &self.source[self.start + 1..=self.current - 1];
        self.push_token(TokenType::String, Literal::StringLiteral(String::from(str)));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let val: f64 = self.source[self.start..self.current].parse().unwrap();

        self.push_token(TokenType::Number, Literal::NumberLiteral(val));
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let mut token_type = KEYWORDS.get(text);

        if token_type.is_none() {
            token_type = Some(&TokenType::Identifier)
        }

        self.add_token(*token_type.unwrap());
    }
}

fn generate_keywords() -> HashMap<&'static str, TokenType> {
    let mut keywords = HashMap::new();

    keywords.insert("and", TokenType::And);
    keywords.insert("class", TokenType::Class);
    keywords.insert("else", TokenType::Else);
    keywords.insert("false", TokenType::False);
    keywords.insert("for", TokenType::For);
    keywords.insert("fun", TokenType::Fun);
    keywords.insert("if", TokenType::If);
    keywords.insert("nil", TokenType::Nil);
    keywords.insert("or", TokenType::Or);
    keywords.insert("print", TokenType::Print);
    keywords.insert("return", TokenType::Return);
    keywords.insert("super", TokenType::Super);
    keywords.insert("this", TokenType::This);
    keywords.insert("true", TokenType::True);
    keywords.insert("var", TokenType::Var);
    keywords.insert("while", TokenType::While);

    keywords
}
