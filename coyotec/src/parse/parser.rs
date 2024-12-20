#![allow(dead_code, unused_variables, unused_imports)]
use crate::ast::tree::ValueType::*;
use crate::ast::tree::{BinOp, Command, Node, NodeType, UnaryOp, ValueType};
use crate::datatypes::datatype::DataType;

/// The parser takes a vector of tokens from the lexer and builds the AST
///
/// The parser is a recursive descent parser that builds the AST from the tokens
use crate::tokens::{Location, Token, TokenType};

use anyhow::{bail, Error, Result};
use std::slice::Iter;

use crate::allocator::Registers;
use crate::symbols::{Symbol, SymbolTable};
use crate::Deferable;

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
    fn parse_let(&mut self) -> Option<Node> {
        // Expect a `let` token or send back an error
        if self.expect_token(TokenType::Let).is_err() {
            return None;
        }

        // Create a new node from the `let` token
        let mut node = Node::new(
            Statement(Command::Let),
            self.peek()?.location,
            DataType::None,
            NodeType::Statement,
        );

        // Make a new identifier node and add it to the `let` node
        if let Ok(init_identifier) = self.new_identifier() {
            node.add_child(init_identifier.unwrap());
        }

        // Parse the identifier and make another child node from it
        //let id_node = self.parse_identifier()?;
        //node.add_child(id_node);
        Some(node)
    }

    /// Parse an identifier into a node
    ///
    fn new_identifier(&mut self) -> Result<Option<Node>> {
        //println!("Parsing identifier");
        let token = self.advance().expect("No token found");
        if let TokenType::Identifier(name) = token.token_type {
            let mut idnode = Node::new(
                Identifier(*name.clone()),
                token.location,
                DataType::None,
                NodeType::Expr,
            );
            let next_token = self.peek().expect("No token found");

            // A colon indicates that the data type is specified
            if next_token.token_type == TokenType::Colon {
                self.advance();
                let data_def = self.advance().expect("No data type found");

                idnode.data_type = match data_def.token_type {
                    TokenType::DataType => match data_def.token_type {
                        TokenType::Integer(_) => DataType::Integer,
                        TokenType::Float(_) => DataType::Float,
                        TokenType::Text(_) => DataType::String,
                        TokenType::Boolean(_) => DataType::Boolean,
                        TokenType::Struct(name) => {
                            //let ident = self.symbol_table.get(&name);
                            // todo: get the data type from the symbol table
                            DataType::Struct(0)
                        }
                        _ => DataType::None,
                    },
                    _ => DataType::None,
                };
            }

            // If the type wasn't specified then the data type MUST be inferred from the
            // assign
            //println!("Parsing assignment");
            match next_token.token_type {
                TokenType::Assign => {
                    //println!("Found assignment");
                    self.advance();
                    let expr = self.parse_expr()?.unwrap();
                    // Edge case. If the data type was specified then the expression must match
                    if idnode.data_type != DataType::None && idnode.data_type != expr.data_type {
                        let msg =
                            format!("Type mismatch {:?}, {:?}", idnode.data_type, expr.data_type);
                        bail!(msg);
                    }
                    // If not, the node infers the datatype from the expression
                    idnode.data_type = expr.data_type;
                    //println!("Data type assigned to identifier {:?}", idnode.data_type);

                    let asg_node = Node::new(
                        ValueType::AssignmentOperator,
                        token.location,
                        idnode.data_type,
                        NodeType::Statement,
                    );
                    idnode.add_child(asg_node);
                    idnode.add_child(expr);
                }
                _ => {
                    idnode.data_type = DataType::None;
                }
            }

            // Final check: we must have a data type assigned to this node. If not, it's because
            // the data type was not specified and the expression was not parsed
            if idnode.data_type == DataType::None {
                bail!("Data type not assigned to identifier");
            }
            self.symbol_table
                .add_symbol(&*name.clone(), idnode.clone().data_type);
            return Ok(Some(idnode));
        }
        bail!("Expected identifier not found");
    }

    pub fn parse(&mut self) -> Result<Node> {
        let mut node = Node::new(
            ValueType::Root,
            Location::new(),
            DataType::None,
            NodeType::Statement,
        );
        while let Some(token) = self.peek() {
            match token.token_type {
                TokenType::Let => {
                    if let Some(n) = self.parse_let() {
                        node.add_child(n);
                    }
                }
                TokenType::Print => {
                    self.advance();
                    let expr = self.parse_expr()?.unwrap();
                    let mut print_node = Node::new(
                        ValueType::Statement(Command::Print),
                        token.location,
                        expr.data_type,
                        NodeType::Statement,
                    );
                    print_node.add_child(expr);
                    node.add_child(print_node);
                }

                TokenType::Newline => {
                    self.advance();
                    continue;
                }
                _ => {
                    if let Some(n) = self.parse_expr()? {
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

    fn grouping(&mut self) -> Result<Option<Node>> {
        let expr = self.parse_expr()?;
        if self.expect_token(TokenType::RParen).is_err() {
            bail!("Expected closing parenthesis");
        }
        Ok(expr)
    }

    /// Array
    fn array(&mut self) -> Result<Option<Node>> {
        let current = self.current;
        let mut a_node = Node::new(
            ValueType::Array,
            self.tokens[current].location,
            DataType::None,
            NodeType::Expr,
        );
        let mut elem_count = 0;
        loop {
            if self.match_token(TokenType::RBracket) {
                break;
            }
            let expr = self.parse_expr()?;
            a_node.add_child(expr.unwrap());
            elem_count += 1;
            if self.match_token(TokenType::Comma) {
                continue;
            }
        }
        let data_type = if elem_count > 0 {
            a_node.children[0].data_type
        } else {
            DataType::None
        };
        a_node.data_type = data_type;
        Ok(Some(a_node))
    }

    /// Parse a primary expression which is either a number, a unary operator, or a grouping
    fn parse_primary(&mut self) -> Result<Option<Node>> {
        // Check for a unary operator
        if let Some(mut node) = self.parse_unary()? {
            let expr = self.parse_expr()?.unwrap();
            // Update the data type of the unary node based on the expression it's negating
            node.data_type = expr.data_type;
            node.add_child(expr);
            return Ok(Some(node));
        }

        let token = self.peek().expect("No primary token found");

        // Check for a grouping
        if self.match_token(TokenType::LParen) {
            return self.grouping();
        }

        if self.match_token(TokenType::LBracket) {
            return self.array();
        }

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
            TokenType::Text(value) => {
                self.advance();
                Some(Node::new(
                    ValueType::Text(value),
                    token.location,
                    DataType::String,
                    NodeType::Expr,
                ))
            }
            TokenType::Identifier(name) => {
                self.advance();
                let data_type = if let Some(item) = self.symbol_table.get(&name) {
                    item.data_type
                } else {
                    DataType::None
                };
                Some(Node::new(
                    Identifier(*name),
                    token.location,
                    data_type,
                    NodeType::Expr,
                ))
            }
            TokenType::EOF => None,
            _ => bail!("Unexpected token {:?}", token.token_type),
        };

        Ok(node)
    }

    fn parse_unary(&mut self) -> Result<Option<Node>> {
        let token = self.peek().expect("No unary token found");

        let node = match token.token_type {
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
        };
        Ok(node)
    }

    fn parse_term(&mut self) -> Result<Option<Node>> {
        // First check if the token is a number or grouping token
        let mut node = self.parse_primary()?;

        loop {
            let token = self.peek().expect("No term token found");
            let op = match token.token_type {
                TokenType::Star => BinOp::Mul,
                TokenType::Slash => BinOp::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_primary()?;

            let rdata_type = right.clone().unwrap().data_type;
            let ldata_type = node.clone().unwrap().data_type;

            if rdata_type != ldata_type {
                let msg = format!("Type mismatch {:?}, {:?}", rdata_type, ldata_type);
                bail!(msg);
            }

            let left = node.clone().unwrap();
            let right = right.unwrap();
            let mut t_node = Node::new(BinOperator(op), token.location, rdata_type, NodeType::Op);
            t_node.add_child(right);
            t_node.add_child(left);
            node = Some(t_node);
        }

        Ok(node)
    }

    /// Parses expressions (things that return a value)
    fn parse_expr(&mut self) -> Result<Option<Node>> {
        // First check if there is a higher precedence operator
        let mut node = self.parse_term()?;

        loop {
            // If the next peeked value is None, then we're done
            let token = self.peek().expect("No expression token found");

            let op = match token.token_type {
                TokenType::Plus => BinOp::Add,
                TokenType::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();

            let right = self.parse_term()?;

            if let None = right {
                self.raise_error("Missing right side of expression");
                bail!("parse error");
            }
            let right = right.unwrap();

            let rdata_type = right.clone().data_type;

            let ldata_type = node.clone().unwrap().data_type;

            if rdata_type != ldata_type {
                let msg = format!("Type mismatch {:?}, {:?}", rdata_type, ldata_type);
                self.raise_error(&msg);
                bail!(msg);
            }

            let mut new_node =
                Node::new(BinOperator(op), token.location, rdata_type, NodeType::Leaf);

            new_node.add_child(right);
            new_node.add_child(node.unwrap());
            node = Some(new_node);
        }
        Ok(node)
    }
}
pub fn parse(tokens: Vec<Token>, source_code: String) -> Result<Node> {
    Parser::new(tokens, source_code).parse()
}
