use crate::tokens::{Token, TokenType};
use std::slice::{Iter};
use std::iter::Peekable;
use crate::ast::{BinOp, BinOperator, Float, Integer, Visitor};

const PREVIOUS: usize = 0;
const CURRENT: usize = 1;


struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens: tokens.iter().peekable(),
            current: 0,
        }
    }

    pub fn advance(&mut self) -> Option<&Token> {
        self.current+=1;
        self.tokens.next()
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek().cloned()
    }

    pub fn parse(&mut self) -> Option<Box<dyn Visitor>> {
        while let Some(token) = self.peek() {
            match token.token_type {
                /*
                TokenType::Let => {
                    self.parse_let()
                }
                TokenType::Identifier => {
                    self.parse_identifier()
                }
                */

                _ => {
                    return self.parse_expr();
                }
            }
        }
        None
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if let Some(t) = self.peek() {
            if t.token_type == token_type {
                self.advance();
                return true;
            }
        }
        false
    }
    fn expect_token(&mut self, token_type: TokenType) -> Option<&Token> {
        if let Some(t) = self.peek() {
            if t.token_type == token_type {
                return self.advance();
            }
        }
        None
    }

    fn parse_factor(&mut self) -> Option<Box<dyn Visitor>> {
        if self.match_token(TokenType::LParen) {
            let expr = self.parse_expr()?;
            self.expect_token(TokenType::LParen)?;
            return Some(expr);
        }

        let token = self.peek()?;

        macro_rules! make_factor {
            ($type:tt) => {{
                if let TokenType::$type(value) = token.token_type {
                    self.advance();
                    return Some(Box::new($type{value: value}));
                }
            }}
        }

        make_factor!(Integer);
        make_factor!(Float);

        None

    }

    fn parse_term(&mut self) -> Option<Box<dyn Visitor>> {
        // First check if the token is a number or grouping token
        let mut node = self.parse_factor()?;

        loop {
            let token = self.peek()?;
            let op = match token.token_type {
                TokenType::Star => BinOp::Mul,
                TokenType::Slash => BinOp::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_factor()?;
            node = Box::new(BinOperator::new(op, node, right));
        }

        Some(node)
    }

    fn parse_expr(&mut self) -> Option<Box<dyn Visitor>> {
        // First check if these is a higher precedence operator
        let mut node = self.parse_term()?;

        loop {
            let token = self.peek()?;
            let op = match token.token_type {
                TokenType::Plus => BinOp::Add,
                TokenType::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_term()?;
            let binoperator = Box::new(BinOperator::new(op, node, right));
            node = binoperator;
        }
        Some(node)
    }
/*
    fn parse_program(&mut self) -> Option<Block> {
        let mut statements = vec![];
        while self.pos < self.tokens.len() {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            } else {
                return None;
            }
        }
        Some(Block(statements))
    }

 */



}
pub fn parse(tokens: &Vec<Token>) -> Option<Box<dyn Visitor>> {
    let mut p = Parser::new(tokens);
    let ast = p.parse();
    ast
}