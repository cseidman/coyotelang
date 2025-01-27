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
use crate::ast::node::{display_tree, BinOp, Node, NodeType, UnOp};
use crate::tokens::{BaseType::*, TokenType::*};
use crate::{tokens, Deferable};

const PREVIOUS: usize = 0;
const CURRENT: usize = 1;
#[derive(Clone)]
pub struct Parser {
    pub source_code: String,
    pub tokens: Vec<Token>,
    current: usize, // The current token position being parsed
    has_error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, source_code: String) -> Self {
        Self {
            // Iterators are used to avoid moving the vector of tokens
            tokens,
            source_code,
            current: 0,
            has_error: false,
        }
    }

    fn current_token(&self) -> Option<Token> {
        self.tokens.get(self.current).cloned()
    }

    fn raise_error(&mut self, msg: &str) {
        self.has_error = true;
        let current = self.current;
        let token = self.tokens[current - 1].clone();
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
        let node = Node::new(NodeType::Root, None);
        self.parse_to_node(node)
    }

    pub fn parse_to_node(&mut self, node: Node) -> Result<Node> {
        let mut node = node;

        // This is the starting point
        while let Some(token) = self.peek() {
            match token.token_type {
                TokenType::Let => {
                    let n = self.parse_let()?;
                    node.add_child(n);
                }
                TokenType::Print => {
                    self.advance();
                    let expr = self.parse_expr(0)?;
                    let mut print_node = Node::new(NodeType::Print, Some(token));
                    print_node.add_child(expr);
                    node.add_child(print_node);
                    continue;
                }

                Newline | EOF => {
                    self.advance();
                    continue;
                }
                LBrace => {
                    self.advance();
                    let n = Node::new(NodeType::Block, self.current_token());
                    node.add_child(n);
                    continue;
                }

                RBrace => {
                    self.advance();
                    let n = Node::new(NodeType::EndBlock, self.current_token());
                    node.add_child(n);
                    continue;
                }
                TokenType::Else | TokenType::EndIf => {
                    return Ok(node);
                }

                TokenType::If => {
                    self.advance();
                    // Root of the IF node
                    let mut if_node = Node::new(NodeType::If, self.current_token());

                    // This is the condition
                    let mut conditional = Node::new(Conditional, self.current_token());

                    // Get the condition  expression
                    let condition = self.parse_expr(0)?;
                    conditional.add_child(condition);
                    // Add the logical condition to the IF node
                    if_node.add_child(conditional);

                    // Start the scope block
                    let block = Node::new(NodeType::Block, self.current_token());
                    if_node.add_child(block);

                    // Parse all the statements inside the TRUE portion of the IF
                    let mut code_block = Node::new(NodeType::CodeBlock, self.current_token());
                    code_block = self.parse_to_node(code_block)?;
                    if_node.add_child(code_block);

                    // Close out the scope block
                    let end_block = Node::new(NodeType::EndBlock, self.current_token());
                    if_node.add_child(end_block);

                    while let Some(tok) = self.peek() {
                        match tok.token_type {
                            TokenType::Else => {
                                self.advance();
                                let mut else_node = Node::new(NodeType::Else, self.current_token());
                                let block = Node::new(NodeType::Block, self.current_token());
                                else_node.add_child(block);
                                else_node = self.parse_to_node(else_node)?;
                                let end_block = Node::new(NodeType::EndBlock, self.current_token());
                                else_node.add_child(end_block);
                                if_node.add_child(else_node);
                            }
                            TokenType::EndIf => {
                                self.advance();
                                let endif = Node::new(NodeType::EndIf, self.current_token());

                                // Add the ENDIF block
                                if_node.add_child(endif);
                                // Add the whole thing to the parent node
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    node.add_child(if_node);
                }
                _ => {
                    let n = self.parse_expr(0)?;
                    node.add_child(n);
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
                Ok(self.advance().unwrap())
            } else {
                let msg = format!(
                    "Expected token {:?} but found {:?}",
                    token_type, t.token_type
                );
                self.raise_error(&msg);
                Err(Error::msg(msg))
            }
        } else {
            let msg = "No more tokens left";
            self.raise_error(msg);
            Err(Error::msg(msg))
        }
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

                let varname = Box::new(name.to_string());
                let mut node = Node::new(Ident(varname), self.current_token());
                // Check if this is an array
                if self.match_token(TokenType::LBracket) {
                    if let Ok(index) = self.parse_expr(0) {
                        node.add_child(index);
                    }
                    self.expect_token(RBracket)?;
                }

                Ok(node)
            }
            LParen => {
                self.advance();
                let expr = self.parse_expr(0)?;
                self.expect_token(TokenType::RParen)?;
                Ok(expr)
            }
            LBracket => {
                self.advance();
                let mut node = Node::new(NodeType::Array, self.current_token());
                while !self.match_token(TokenType::RBracket) {
                    let expr = self.parse_expr(0)?;
                    self.match_token(TokenType::Comma);
                    node.add_child(expr);
                }
                Ok(node)
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
                Plus => (30, BinOp::Add),
                Minus => (30, BinOp::Sub),
                Star => (40, BinOp::Mul),
                Slash => (40, BinOp::Div),
                Caret => {
                    is_right_associative = true;
                    (50, BinOp::Pow)
                }
                GreaterThan => (20, BinOp::GreaterThan),
                LessThan => (20, BinOp::LessThan),
                EqualGreaterThan => (20, BinOp::GreaterThanEqual),
                EqualLessThan => (20, BinOp::LessThanEqual),
                EqualEqual => (20, BinOp::EqualEqual),
                NotEqual => (20, BinOp::NotEqual),
                And => (10, BinOp::And),
                Or => (10, BinOp::Or),

                Assign => {
                    node.can_assign = true;
                    (60, BinOp::Assign)
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
