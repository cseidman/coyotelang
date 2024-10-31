#![allow(dead_code, unused_variables, unused_imports)]
use crate::tokens::{Location, Token, TokenType};
use anyhow::{anyhow, Context, Result};
use std::iter::Peekable;
use std::str::Chars;

pub enum SourceType {
    Interactive,
    Test,
    File(String),
}

struct Source<'a> {
    code: Peekable<Chars<'a>>,
    source_type: SourceType,
}

struct Lexer<'a> {
    source: Source<'a>,
    location: Location,
    error_mode: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source: Source<'a>) -> Self {
        Self {
            source,
            location: Location::new(),
            error_mode: false,
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        let value = self.source.code.next();
        self.location.increment(1);
        value
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.source.code.peek()
    }

    pub fn newline(&mut self) {
        self.location.newline();
    }

    pub fn get_number(&mut self) -> String {
        let mut snum = String::new();
        while let Some(&x) = self.peek() {
            if x.is_ascii_digit() {
                snum.push(x);
                self.advance();
            } else {
                break;
            }
        }
        snum
    }

    pub fn make_token(&mut self, token_type: TokenType) -> Token {
        Token {
            token_type,
            location: self.location,
        }
    }

    pub fn multi_line_comment(&mut self) {
        self.advance();
        while let Some(&x) = self.peek() {
            if x == '\n' {
                self.newline();
                self.advance();
                continue;
            }
            if x == '*' {
                self.advance();
                if let Some(&x) = self.peek() {
                    if x == '/' {
                        self.advance();
                        break;
                    }
                }
            }
            self.advance();
        }
    }

    pub fn single_line_comment(&mut self) {
        self.advance();
        while let Some(&x) = self.peek() {
            if x == '\n' {
                break;
            }
            self.advance();
        }
    }

    pub fn report_error(&mut self, err_msg: &str) {
        self.error_mode = true;
        println!(
            "{} at line {} position {}",
            err_msg, self.location.line, self.location.column
        );
    }

    fn make_error(&mut self, err_msg: &str) -> anyhow::Error {
        self.error_mode = true;
        anyhow!(
            "{} at line {} position {}",
            err_msg,
            self.location.line,
            self.location.column
        )
    }
}

pub fn lex(code: &str, source_type: SourceType) -> Result<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut lexer = Lexer::new(Source {
        code: code.chars().peekable(),
        source_type,
    });

    while let Some(&c) = lexer.peek() {
        // Get rid of whitespace characters
        if [' ', '\t'].contains(&c) {
            lexer.advance();
            continue;
        }

        if c.is_ascii_digit() {
            let mut snum = c.to_string();
            lexer.advance();
            snum.push_str(&lexer.get_number());

            let mut is_float = false;

            if let Some(&x) = lexer.peek() {
                if x == '.' {
                    lexer.advance();
                    is_float = true;
                    snum.push('.');

                    let num2: String = lexer.get_number();
                    snum.push_str(&num2);
                }
            }
            if is_float {
                let num: f64 = snum.parse().unwrap();
                tokens.push(lexer.make_token(TokenType::Float(num)));
                continue;
            } else {
                let num: i64 = snum.parse().unwrap();
                tokens.push(lexer.make_token(TokenType::Integer(num)));
                continue;
            }
        }

        if c.is_alphabetic() {
            let mut ident = c.to_string();
            lexer.advance();
            while let Some(&x) = lexer.peek() {
                if x.is_alphanumeric() {
                    ident.push(x);
                    lexer.advance();
                } else {
                    break;
                }
            }
            let tok = match ident.as_str() {
                "let" => lexer.make_token(TokenType::Let),
                "func" => lexer.make_token(TokenType::Func),
                "print" => lexer.make_token(TokenType::Print),
                _ => lexer.make_token(TokenType::Identifier(Box::new(ident))),
            };
            tokens.push(tok);
            lexer.advance();
            continue;
        }

        lexer.advance();
        let token_type = match c {
            '[' => TokenType::LBracket,
            ']' => TokenType::RBracket,
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            '!' => TokenType::Bang,
            '{' => TokenType::LBrace,
            '}' => TokenType::RBrace,
            '.' => TokenType::Dot,
            ',' => TokenType::Comma,
            ';' => TokenType::SemiColon,
            ':' => TokenType::Colon,
            '=' => {
                if *lexer.peek().unwrap_or(&'\0') == '=' {
                    lexer.advance();
                    TokenType::Equal
                } else {
                    TokenType::Assign
                }
            }
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Star,
            '/' => {
                if let Some(&x) = lexer.peek() {
                    match x {
                        '/' => {
                            lexer.single_line_comment();
                            continue;
                        }
                        '*' => {
                            lexer.multi_line_comment();
                            continue;
                        }
                        _ => {
                            lexer.make_token(TokenType::Slash);
                        }
                    }
                }
                TokenType::Slash
            }
            '%' => TokenType::Percent,
            '<' => TokenType::LessThan,
            '>' => TokenType::GreaterThan,
            '&' => TokenType::Ampersand,
            '|' => TokenType::Pipe,
            '^' => TokenType::Caret,
            '#' => TokenType::Hash,
            '@' => TokenType::At,
            '?' => TokenType::Question,
            '\n' => {
                lexer.newline();
                TokenType::Newline
            }
            '$' => TokenType::Dollar,
            '"' => {
                let mut s = String::new();
                while let Some(&x) = lexer.peek() {
                    if x == '"' {
                        break;
                    }
                    s.push(x);
                    lexer.advance();
                }
                lexer.advance();
                TokenType::Text(Box::new(s))
            }
            _ => {
                let err_msg = format!("Unexpected character: {c}");
                lexer.report_error(&err_msg);
                continue;
            }
        };
        tokens.push(lexer.make_token(token_type));
    }
    if lexer.error_mode {
        return Err(lexer.make_error("Lexer error"));
    }
    tokens.push(lexer.make_token(TokenType::EOF));
    Ok(tokens)
}
