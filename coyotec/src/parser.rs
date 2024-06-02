use crate::tokens::{Token, TokenType};
use std::slice::{Iter};
use std::iter::Peekable;
use crate::ast::{BinOp, UnaryOp, NodeType, Node, DataType};
use crate::ast::NodeType::*;

use std::borrow::Borrow;

const PREVIOUS: usize = 0;
const CURRENT: usize = 1;


struct Parser<'a> {
    tokens: Iter<'a, Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens: tokens.iter(),
            current: 0,
        }
    }

    pub fn advance(&mut self) -> Option<Token> {
        self.current+=1;
        if let Some(token) = self.tokens.next() {
            return Some(token.clone());
        }
        None
    }

    pub fn peek(&mut self) -> Option<Token> {
        if let Some(token ) = self.tokens.clone().next() {
            return Some(token.clone());
        }
        None
    }

    pub fn parse(&mut self) -> Option<Node> {
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
    fn expect_token(&mut self, token_type: TokenType) -> Option<Token> {
        if let Some(t) = self.peek() {
            if t.token_type == token_type {
                return self.advance();
            }
        }
        None
    }

    fn parse_primary(&mut self) -> Option<Node> {
        if self.match_token(TokenType::LParen) {
            let expr = self.parse_expr()?;
            self.expect_token(TokenType::RParen)?;
            return Some(expr);
        }

        let token = self.peek()?;

        match token.token_type {
            TokenType::Integer(value) => {
                self.advance();
                return Some(Node::new(NodeType::Integer(value), token.location, DataType::Integer));
            },
            TokenType::Float(value) => {
                self.advance();
                return Some(Node::new(NodeType::Float(value), token.location, DataType::Float));
            },
            _ => {
                return None;
            }
        }
    }

    fn parse_unary(&mut self) -> Option<Node> {
        let token = self.peek()?;
        let unop = match token.token_type {
            TokenType::Minus => UnaryOp::Neg,
            TokenType::Bang => UnaryOp::Not,
            _ => return self.parse_primary(),
        };
        self.advance();
        let expr = self.parse_expr()?;
        Some(expr)
    }

    fn parse_term(&mut self) -> Option<Node> {
        // First check if the token is a number or grouping token
        let mut node = self.parse_primary()?;

        loop {
            let token = self.peek()?;
            let op = match token.token_type {
                TokenType::Star => BinOp::Mul,
                TokenType::Slash => BinOp::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_primary()?;

            let rdata_type = right.data_type;
            let ldata_type = node.data_type;

            if rdata_type != ldata_type {
                println!("Type mismatch {:?}, {:?}", rdata_type, ldata_type);
                return None;
            }

            let left = node;
            node = Node::new(BinOperator(op), token.location, rdata_type);
            node.add_child(right);
            node.add_child(left);
        }

        Some(node)
    }

    fn parse_expr(&mut self) -> Option<Node> {
        // First check if these is a higher precedence operator
        let mut node = self.parse_term()?;

        loop {
            let token = self.peek()? ;

            let op = match token.token_type {
                TokenType::Plus => BinOp::Add,
                TokenType::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();

            let right = self.parse_term()?;
            let rdata_type = right.data_type;
            let ldata_type = node.data_type;

            if rdata_type != ldata_type {
                println!("Type mismatch {:?}, {:?}", rdata_type, ldata_type);
                return None;
            }

            let mut new_node = Node::new(BinOperator(op), token.location, rdata_type);
            new_node.add_child(right);
            new_node.add_child(node);
            node = new_node;
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
pub fn parse(tokens: &Vec<Token>) -> Option<Node> {
    let mut p = Parser::new(tokens);
    let ast = p.parse();
    ast
}