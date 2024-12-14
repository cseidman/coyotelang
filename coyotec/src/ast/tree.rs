#![allow(dead_code, unused_variables)]

pub use crate::ast::Node;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Let,
    Print,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValueType {
    Root,
    Integer(i64),
    Float(f64),
    Text(Box<String>),
    BinOperator(BinOp),
    UnaryOperator(UnaryOp),
    Identifier(String),
    Statement(Command),
    AssignmentOperator,
    ElementIndex,
    Array,
}

impl ValueType {
    pub(crate) fn text(text: Box<String>) -> ValueType {
        todo!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeType {
    Leaf,
    Op,
    Expr,
    Statement,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Integer(value) => {
                write!(f, "{value}")
            }
            ValueType::Float(value) => {
                write!(f, "{value}")
            }
            ValueType::Text(value) => {
                write!(f, "{value}")
            }
            ValueType::BinOperator(value) => {
                write!(f, "{value}")
            }
            ValueType::UnaryOperator(value) => {
                write!(f, "{value}")
            }
            ValueType::Identifier(value) => {
                write!(f, "{value}")
            }
            ValueType::Statement(command) => match command {
                Command::Let => write!(f, "Let"),
                Command::Print => write!(f, "Print"),
            },
            ValueType::AssignmentOperator => {
                write!(f, "AssignmentOperator")
            }
            ValueType::Array => {
                write!(f, "Array")
            }
            ValueType::Root => {
                write!(f, "Root")
            }
            ValueType::ElementIndex => {
                write!(f, "Index")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BinOp::Add => write!(f, "add"),
            BinOp::Sub => write!(f, "sub"),
            BinOp::Mul => write!(f, "mul"),
            BinOp::Div => write!(f, "div"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}
impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "neg"),
            UnaryOp::Not => write!(f, "not"),
        }
    }
}

#[cfg(test)]
mod test {}
