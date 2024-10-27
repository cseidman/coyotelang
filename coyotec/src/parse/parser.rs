#![allow(dead_code, unused_variables, unused_imports)]
use crate::ast::tree::ValueType::*;
use crate::ast::tree::{BinOp, DataType, Node, NodeType, UnaryOp, ValueType};
/// The parser takes a vector of tokens from the lexer and builds the AST
///
/// The parser is a recursive descent parser that builds the AST from the tokens
use crate::tokens::{Location, Token, TokenType};
use anyhow::{Error, Result};
use std::slice::Iter;

use crate::allocator::Registers;
use crate::symbols::SymbolTable;

const PREVIOUS: usize = 0;
const CURRENT: usize = 1;

pub struct Parser<'a> {
    tokens: Iter<'a, Token>,
    current: usize,            // The current token position being parsed
    symbol_table: SymbolTable, // A map of symbol names to location numbers
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
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
        self.current += 1;
        if let Some(token) = self.tokens.next() {
            return Some(token.clone());
        }
        None
    }
    /// Peek at the next token without advancing the iterator
    pub fn peek(&mut self) -> Option<Token> {
        // We need to clone the token because so as not to advance the token
        // iterator on the "real" vector of tokens
        if let Some(token) = self.tokens.clone().next() {
            return Some(token.clone());
        }
        None
    }
    /// Parse a `let` statement
    fn parse_let(&mut self) -> Option<Node> {
        // Expect a `let` token or send back an error
        if self.expect_token(TokenType::Let).is_err() {
            return None;
        }

        // Create a new node from the `let` token
        let mut node = Node::new(
            Let,
            self.peek()?.location,
            DataType::None,
            NodeType::Statement,
        );

        let init_identifier = self.new_identifier()?;
        node.add_child(init_identifier);

        // Parse the identifier and make another child node from it
        let id_node = self.parse_identifier()?;
        node.add_child(id_node);
        Some(node)
    }

    fn new_identifier(&mut self) -> Option<Node> {
        let token = self.advance()?;
        if let TokenType::Identifier(name) = token.token_type {
            let ident_id = self.symbol_table.get(&name);
            let node = Node::new(
                Identifier(ident_id),
                token.location,
                DataType::None,
                NodeType::Expr,
            );
            return Some(node);
        }
        None
    }

    /// Parse an identifier into a node
    ///
    fn parse_identifier(&mut self) -> Option<Node> {
        println!("Parsing identifier");
        let token = self.advance()?;
        if let TokenType::Identifier(name) = token.token_type {
            let ident_id = self.symbol_table.get(&name);

            let mut idnode = Node::new(
                Identifier(ident_id),
                token.location,
                DataType::None,
                NodeType::Expr,
            );
            let next_token = self.peek()?;

            // A colon indicates that the data type is specified
            if next_token.token_type == TokenType::Colon {
                self.advance();
                let data_def = self.advance()?;

                idnode.data_type = match data_def.token_type {
                    TokenType::DataType => match data_def.token_type {
                        TokenType::Integer(_) => DataType::Integer,
                        TokenType::Float(_) => DataType::Float,
                        TokenType::Text(_) => DataType::String,
                        TokenType::Boolean(_) => DataType::Boolean,
                        TokenType::Struct(name) => {
                            let ident = self.symbol_table.get(&name);
                            DataType::Struct(ident)
                        }
                        _ => DataType::None,
                    },
                    _ => DataType::None,
                };
            }

            // If the type wasn't specified then the data type MUST be inferred from the
            // assign
            println!("Parsing assignment");
            match next_token.token_type {
                TokenType::Assign => {
                    println!("Found assignment");
                    self.advance();
                    let expr = self.parse_expr()?;
                    // Edge case. If the data type was specified then the expression must match
                    if idnode.data_type != DataType::None && idnode.data_type != expr.data_type {
                        println!("Type mismatch {:?}, {:?}", idnode.data_type, expr.data_type);
                        return None;
                    }
                    // If not, the node infers the datatype from the expression
                    idnode.data_type = expr.data_type;
                    println!("Data type assigned to identifier {:?}", idnode.data_type);

                    let node = idnode;
                    idnode = Node::new(
                        ValueType::AssignmentOperator,
                        token.location,
                        expr.data_type,
                        NodeType::Statement,
                    );
                    idnode.add_child(node);
                    idnode.add_child(expr);
                }
                _ => {
                    idnode.data_type = DataType::None;
                }
            }

            // Final check: we must have a data type assigned to this node. If not, it's because
            // the data type was not specified and the expression was not parsed
            if idnode.data_type == DataType::None {
                println!("Data type not assigned to identifier");
                return None;
            }

            return Some(idnode);
        }
        None
    }

    pub fn parse(&mut self) -> Option<Node> {
        let mut node = Node::new(
            ValueType::Root,
            Location::new(),
            DataType::None,
            NodeType::Statement,
        );
        while let Some(token) = self.peek() {
            let n = match token.token_type {
                TokenType::Let => self.parse_let(),
                TokenType::Identifier(_) => self.parse_identifier(),
                _ => self.parse_expr(),
            };
            if let Some(n) = n {
                node.add_child(n);
            } else {
                break;
            }
        }
        Some(node)
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
    fn expect_token(&mut self, token_type: TokenType) -> Result<Token> {
        if let Some(t) = self.peek() {
            if t.token_type == token_type {
                return Ok(self.advance().unwrap());
            }
        }
        let msg = format!("Expected token {:?} not found", token_type);
        println!("{}", msg);
        Err(Error::msg(msg))
    }
    /// Parse a primary expression which is either a number, a unary operator, or a grouping
    fn parse_primary(&mut self) -> Option<Node> {
        if self.match_token(TokenType::LParen) {
            let expr = self.parse_expr()?;
            if self.expect_token(TokenType::RParen).is_err() {
                return None;
            }
            return Some(expr);
        }

        // Check for a unary operator
        let token = self.peek()?;
        let unary_node = self.parse_unary();

        let token = self.peek()?;

        let node = match token.token_type {
            TokenType::Integer(value) => {
                self.advance();
                Some(Node::new(
                    ValueType::Integer(value),
                    token.location,
                    DataType::Integer,
                    NodeType::Leaf,
                ))
            }
            TokenType::Float(value) => {
                self.advance();
                Some(Node::new(
                    ValueType::Float(value),
                    token.location,
                    DataType::Float,
                    NodeType::Expr,
                ))
            }
            TokenType::Identifier(_) => self.parse_identifier(),
            _ => None,
        };

        if let Some(mut unary) = unary_node {
            let node = node.unwrap();
            unary.add_child(node);
            return Some(unary);
        }
        node
    }

    fn parse_unary(&mut self) -> Option<Node> {
        let token = self.peek()?;

        match token.token_type {
            TokenType::Minus => {
                self.advance();
                Some(Node::new(
                    ValueType::UnaryOperator(UnaryOp::Neg),
                    token.location,
                    DataType::Integer,
                    NodeType::Op,
                ))
            }
            TokenType::Bang => {
                self.advance();
                Some(Node::new(
                    ValueType::UnaryOperator(UnaryOp::Not),
                    token.location,
                    DataType::Integer,
                    NodeType::Op,
                ))
            }
            _ => None,
        }
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
            node = Node::new(BinOperator(op), token.location, rdata_type, NodeType::Op);
            node.add_child(right);
            node.add_child(left);
        }

        Some(node)
    }

    fn parse_expr(&mut self) -> Option<Node> {
        // First check if there is a higher precedence operator
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
            let rdata_type = right.clone().data_type;
            let ldata_type = node.clone().data_type;

            if rdata_type != ldata_type {
                println!("Type mismatch {:?}, {:?}", rdata_type, ldata_type);
                return None;
            }

            let mut new_node =
                Node::new(BinOperator(op), token.location, rdata_type, NodeType::Leaf);
            new_node.add_child(right);
            new_node.add_child(node);
            node = new_node;
        }
        Some(node)
    }
}
pub fn parse(tokens: &[Token]) -> Option<Node> {
    Parser::new(tokens).parse()
}
