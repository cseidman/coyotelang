#![allow(dead_code, unused_variables, unused_imports)]

use crate::ast::tree::ValueType;
use crate::ast::tree::ValueType::*;
use crate::datatypes::datatype::DataType;
/// The parser takes a vector of tokens from the lexer and builds the AST
///
/// The parser is a recursive descent parser that builds the AST from the tokens
use crate::tokens::{BaseType, Location, Token, TokenType};
use std::cmp::PartialEq;
use std::collections::HashMap;

use anyhow::{anyhow, bail, Error, Result};
use std::slice::Iter;

use crate::ast::node::NodeType::*;
use crate::ast::node::UnOp::Neg;
use crate::ast::node::{BinOp, Node, NodeType, UnOp};
use crate::symbols::{SymbolTable, Symbols};
use crate::tokens::{BaseType::*, TokenType::*};
use crate::{tokens, Deferable};

const PREVIOUS: usize = 0;
const CURRENT: usize = 1;
#[derive(Clone)]
pub struct Parser {
    pub source_code: String,
    pub tokens: Vec<Token>,
    current: usize,            // The current token position being parsed
    symbol_table: SymbolTable, // A map of symbol names to location numbers
    has_error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, source_code: String) -> Self {
        Self {
            // Iterators are used to avoid moving the vector of tokens
            tokens,
            source_code,
            current: 0,
            symbol_table: SymbolTable::new(),
            has_error: false,
        }
    }

    /// Register new variable in the symbol table
    fn register_variable(&mut self, variable_name: &str, data_type: DataType) {
        self.symbol_table.add_symbol(variable_name, data_type);
    }

    fn variable_exists(&mut self, variable_name: &str) -> bool {
        self.symbol_table.get(variable_name).is_some()
    }

    fn push_scope(&mut self) {
        self.symbol_table.push_scope();
    }

    fn pop_scope(&mut self) {
        self.symbol_table.pop_scope();
    }

    fn current_token(&self) -> Option<Token> {
        self.tokens.get(self.current).cloned()
    }

    fn raise_error(&mut self, msg: &str) {
        self.has_error = true;
        let current = self.current;
        let token = self.tokens[current].clone();
        let line = self
            .source_code
            .lines()
            .nth(token.location.line - 1 as usize)
            .unwrap_or("");
        let line_number = token.location.line;
        let arrow = format!("{: >1$}", "^", token.location.column + 1 as usize);
        println!("Line :{line_number} | {msg}");
        println!("|");
        println!("| {line}");
        println!("| {arrow}");
    }

    pub fn add_tokens(&mut self, tokens: Vec<Token>, source_code: String) {
        self.source_code = source_code;
        self.tokens = tokens;
        self.current = 0;
    }

    /// Advance the token iterator and return the next token. If there are no more tokens
    /// return `None`
    pub fn advance(&mut self) -> Option<Token> {
        let current = self.current;
        if current < self.tokens.len() {
            self.current += 1;
            return Some(self.tokens[current].clone());
        }
        None
    }
    /// Peek at the next token without advancing the iterator
    pub fn peek(&mut self) -> Option<Token> {
        let current = self.current;
        if current < self.tokens.len() {
            return Some(self.tokens[current].clone());
        }
        None
    }
    /// Parse a `let` statement
    fn parse_let(&mut self) -> Result<Node> {
        // Expect a `let` token or send back an error
        self.expect_token(TokenType::Let)?;

        // Create a new node from the `let` token
        let mut node = Node::new(NodeType::Let, self.current_token());

        // Tie the identifier to the variable
        let mut identifier = self.new_identifier()?;

        if self.match_token(TokenType::Assign) {
            let expr = self.parse_expr(0)?;
            identifier.add_child(expr);
        }

        node.add_child(identifier);
        Ok(node)
    }

    fn parse_datatype(&mut self) -> Result<BaseType> {
        if let Some(token) = self.peek() {
            let data_type = match token.token_type {
                TokenType::DataType(base_type) => base_type,
                _ => BaseType::NoType,
            };
            return Ok(data_type);
        }
        Err(anyhow!("Expected a data type"))
    }

    /// Parse an identifier into a node
    ///
    fn new_identifier(&mut self) -> Result<Node> {
        if let Some(token) = self.peek() {
            let node = if let TokenType::Identifier(name) = token.token_type {
                // Register the new variable in the symbol table
                self.register_variable(&name, DataType::None);

                // Create the identifier node
                Node::new(NodeType::Ident(Box::from(name)), self.current_token())
            } else {
                return Err(anyhow!("Expected identifier after `let`"));
            };
            self.advance();
            Ok(node)
        } else {
            Err(anyhow!("Expected identifier after `let`"))
        }
    }

    pub fn parse(&mut self) -> Result<Node> {
        // This is the starting point
        let mut node = Node::new(NodeType::Root, None);
        while let Some(token) = self.peek() {
            match token.token_type {
                TokenType::Let => {
                    if let Ok(n) = self.parse_let() {
                        node.add_child(n);
                    }
                }
                TokenType::Print => {
                    self.advance();
                    let expr = self.parse_expr(0)?;
                    let mut print_node = Node::new(NodeType::Print, Some(token));
                    print_node.add_child(expr);
                    node.add_child(print_node);
                }

                TokenType::Newline => {
                    self.advance();
                    continue;
                }
                _ => {
                    if let Ok(n) = self.parse_expr(0) {
                        node.add_child(n);
                    } else {
                        bail!("Unexpected token {:?}", token.token_type);
                    }
                }
            };
            self.advance();
        }
        Ok(node)
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
        self.raise_error(&msg);
        Err(Error::msg(msg))
    }

    /// Digs down to the base unit: a number, an identifier, or a parenthesized sub-expression
    /// We also start by handling unary operators
    fn parse_primary(&mut self) -> Result<Node> {
        let token = self.peek().expect("No primary token found");
        let token_type = token.clone().token_type;

        match token_type {
            // Value operands
            TokenType::Integer(value) => {
                self.advance();
                Ok(Node::new(NodeType::Integer(value), Some(token.clone())))
            }
            TokenType::Boolean(value) => {
                self.advance();
                Ok(Node::new(NodeType::Boolean(value), Some(token.clone())))
            }
            TokenType::Text(value) => {
                self.advance();
                Ok(Node::new(
                    NodeType::Text(Box::new(value)),
                    Some(token.clone()),
                ))
            }
            TokenType::Float(value) => {
                self.advance();
                Ok(Node::new(NodeType::Float(value), Some(token.clone())))
            }
            TokenType::Identifier(name) => {
                self.advance();
                // Check if the value is in the symbol table
                if !self.variable_exists(&name) {
                    panic!("Variable '{}' does not exist in this scope", name);
                }
                let varname = Box::new(name.to_string());
                let node = Node::new(Ident(varname), self.current_token());
                Ok(node)
            }
            TokenType::LParen => {
                self.advance();
                let expr = self.parse_expr(0)?;
                self.expect_token(TokenType::RParen)?;
                Ok(expr)
            }
            // Unary operators
            TokenType::Plus => {
                self.advance();
                // A plus has no effect as a unary operator, so just try and get the next one
                self.parse_primary()
            }
            TokenType::Minus => self.parse_unary(token, UnOp::Neg),
            TokenType::Bang => self.parse_unary(token, UnOp::Not),
            _ => Err(anyhow!(format!("Unexpected token {:?}", token.token_type))),
        }
    }

    fn parse_unary(&mut self, token: Token, unop: UnOp) -> Result<Node> {
        self.advance();
        // After the unary, we recursively call the function to get at the
        // value being negated
        let u_node = self.parse_primary()?;
        let mut node = Node::new(NodeType::UnaryOp(unop), Some(token));
        node.add_child(u_node);
        Ok(node)
    }

    fn parse_expr(&mut self, min_prec: u8) -> Result<Node> {
        // First, parse a primary expression (a number or parenthesized expr)
        let mut node = self.parse_primary()?;

        // Now, try to consume operators that have at least 'min_prec'
        loop {
            let mut is_right_associative = false;
            let token = self.peek().expect("No term token found");
            let token_type = token.clone().token_type;

            let (prec, op) = match token_type {
                Plus => (10, BinOp::Add),
                Minus => (10, BinOp::Sub),
                Star => (20, BinOp::Mul),
                Slash => (20, BinOp::Div),
                Caret => {
                    is_right_associative = true;
                    (30, BinOp::Pow)
                }
                _ => break, // no operator, stop
            };

            if prec < min_prec {
                break; // operator not strong enough to continue
            }

            // Consume the operator
            self.advance();

            // If operator is right-associative, we use the same precedence level,
            // else we use prec + 1 for the RHS to ensure correct associativity
            let next_min_prec = if is_right_associative { prec } else { prec + 1 };

            // Recursively parse the RHS with the updated minimum precedence
            let rhs = self.parse_expr(next_min_prec)?;
            let lhs = node.clone();

            node = Node::new(BinaryOp(op), Some(token));
            node.add_child(rhs);
            node.add_child(lhs);
        }

        Ok(node)
    }
}
pub fn parse(tokens: Vec<Token>, source_code: String) -> Result<Node> {
    Parser::new(tokens, source_code).parse()
}
