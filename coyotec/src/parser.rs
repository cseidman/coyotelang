#![allow(dead_code, unused_variables)]
/// The parser takes a vector of tokens from the lexer and builds the AST
///
/// The parser is a recursive descent parser that builds the AST from the tokens
use crate::tokens::{Token, TokenType};
use std::slice::{Iter};
use crate::ast::{BinOp, UnaryOp, ValueType, Node, DataType};
use crate::ast::ValueType::*;
use anyhow::Result;

use crate::symbols::{SymbolTable};

const PREVIOUS: usize = 0;
const CURRENT: usize = 1;

struct Parser<'a> {
    tokens: Iter<'a, Token>,
    current: usize, // The current token position being parsed
    symbol_table: SymbolTable // A map of symbol names to location numbers
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            // Iterators are used to avoid moving the vector of tokens
            tokens: tokens.iter(),
            current: 0,
            symbol_table: SymbolTable::new(),
        }
    }
    /// Advance the token iterator and return the next token. If there are no more tokens
    /// return `None`
    pub fn advance(&mut self) -> Option<Token> {
        self.current+=1;
        if let Some(token) = self.tokens.next() {
            return Some(token.clone());
        }
        None
    }
    /// Peek at the next token without advancing the iterator
    pub fn peek(&mut self) -> Option<Token> {
        // We need to clone the token because so as not to advance the token
        // iterator on the "real" vector of tokens
        if let Some(token ) = self.tokens.clone().next() {
            return Some(token.clone());
        }
        None
    }
    /// Parse a `let` statement
    fn parse_let(&mut self) -> Option<Node> {
        // Expect a `let` token or send back an error
        self.expect_token(TokenType::Let)?;

        // Create a new node from the `let` token
        let mut node = Node::new(Let, self.peek()?.location, DataType::None);

        // Parse the identifier and make another child node from it
        let id_node = self.parse_identifier()?;
        node.add_child(id_node);
        Some(node)
    }

    /// Parse an identifier into a node
    ///
    ///
    fn parse_identifier(&mut self) -> Option<Node> {
        let token = self.advance()?;
        if let TokenType::Identifier(name) = token.token_type {
            let ident_id = self.symbol_table.get(&name);
            let mut node = Node::new(Identifier, token.location, DataType::None);
            let next_token = self.peek()?;

            // A colon indicates that the data type is specified
            if next_token.token_type == TokenType::Colon {

                self.advance();
                let data_def = self.advance()?;

                node.data_type = match data_def.token_type {

                    TokenType::DataType => {
                        match data_def.token_type {
                            TokenType::Integer(_) => DataType::Integer,
                            TokenType::Float(_) => DataType::Float,
                            TokenType::Text(_) => DataType::String,
                            TokenType::Boolean(_) => DataType::Boolean,
                            TokenType::Struct(name) => {
                                let ident = self.symbol_table.get(&name);
                                DataType::Struct(ident)
                            },
                            _ => DataType::None,
                        }
                    },
                    _ => DataType::None,
                };
            }

            // If the type wasn't specified then the data type MUST be inferred from the
            // assign

            match next_token.token_type {

                TokenType::Equal => {
                    self.advance();
                    let expr = self.parse_expr()?;
                    // Edge case. If the data type was specified then the expression must match
                    if node.data_type != DataType::None && node.data_type != expr.data_type {
                        println!("Type mismatch {:?}, {:?}", node.data_type, expr.data_type);
                        return None;
                    }
                    // If not, the node infers the datatype from the expression
                    node.data_type = expr.data_type;
                    node.add_child(expr);
                },
                _ => {
                    node.data_type = DataType::None;
                }
            }

            // Final check: we must have a data type assigned to this node. If not, it's because
            // the data type was not specified and the expression was not parsed
            if node.data_type == DataType::None {
                println!("Data type not assigned to identifier");
                return None;
            }

            return Some(node);
        }
        None
    }

    pub fn parse(&mut self) -> Option<Node> {
        while let Some(token) = self.peek() {
            match token.token_type {

                TokenType::Let => {
                    return self.parse_let()
                }
                /*
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
                return Some(Node::new(ValueType::Integer(value), token.location, DataType::Integer));
            },
            TokenType::Float(value) => {
                self.advance();
                return Some(Node::new(ValueType::Float(value), token.location, DataType::Float));
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

            let rdata_type = right.clone().data_type;
            let ldata_type = node.clone().data_type;

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
        // First check if there is a higher precedence operator
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
            let rdata_type = right.clone().data_type;
            let ldata_type = node.clone().data_type;

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