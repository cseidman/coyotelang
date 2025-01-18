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
    Identifier(String),
    Statement(Command),
    AssignmentOperator,
    ElementIndex,
    Array,
    List,
}

impl ValueType {
    pub(crate) fn text(text: Box<String>) -> ValueType {
        todo!()
    }
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
            ValueType::List => {
                write!(f, "List")
            }
        }
    }
}

#[cfg(test)]
mod test {}
