use std::iter::Peekable;
use std::str::Chars;
use crate::tokens::{Location, Token, TokenType};

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
}

impl<'a> Lexer<'a> {
    pub fn new(source: Source<'a>) -> Self {
        Self {
            source,
            location: Location::new(),
        }
    }

    pub fn advance(&mut self) {
        self.source.code.next();
        self.location.increment(1);
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
            location: self.location.clone(),
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

}

pub fn lex(code: &str, source_type: SourceType) {

    let mut tokens:Vec<Token> = Vec::new();

    let mut lexer = Lexer::new(Source {
        code: code.chars().peekable(),
        source_type,
    });


    while let Some(&c) = lexer.peek() {

        // Get rid of whitespace characters
        while [' ', '\t'].contains(&c) {
            lexer.advance();
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

                    let num2:String = lexer.get_number();
                    snum.push_str(&num2);

                }
            }
            if is_float {
                let num:f64 = snum.parse().unwrap();
                println!("FLOAT IS: {num}");
                lexer.make_token(TokenType::Float(num));
                continue;
            } else {
                let num:i64 = snum.parse().unwrap();
                lexer.make_token(TokenType::Integer(num));
                println!("INTEGER IS: {num}");
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
            println!("IDENT IS: {ident}");
            lexer.make_token(TokenType::Text(Box::new(ident)));
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
            '=' => TokenType::Equal,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Star,
            '/' => {
                if let Some(&x) = lexer.peek() {
                    match x {
                        '/' => {
                            lexer.single_line_comment();
                            continue;
                        },
                        '*' => {
                            lexer.multi_line_comment();
                            continue;
                        },
                        _ => {},
                    }
                }
                TokenType::Slash
            },
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
            },
            '$' => TokenType::Dollar,
            '"' => TokenType::Quote,
            _ => {
                panic!("No goo");
            }
        };

        tokens.push(lexer.make_token(token_type));
    }
}

