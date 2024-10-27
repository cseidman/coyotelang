#![allow(dead_code, unused_variables)]

pub use crate::ast::{datatype::DataType, Node};

use std::fmt::{Display, Formatter};

#[derive(Clone, Copy)]
pub enum ValueType {
    Root,
    Integer(i64),
    Float(f64),
    BinOperator(BinOp),
    UnaryOperator(UnaryOp),
    Identifier(usize),
    Let,
    AssignmentOperator,
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
            ValueType::BinOperator(value) => {
                write!(f, "{value}")
            }
            ValueType::UnaryOperator(value) => {
                write!(f, "{value}")
            }
            ValueType::Identifier(value) => {
                write!(f, "{value}")
            }
            ValueType::Let => {
                write!(f, "Let")
            }
            ValueType::AssignmentOperator => {
                write!(f, "AssignmentOperator")
            }
            ValueType::Root => {
                write!(f, "Root")
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
