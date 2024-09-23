#![allow(dead_code, unused_variables)]
use std::fmt::{Display, Formatter};
use crate::tokens::Location;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DataType {
    Integer,
    Float,
    Boolean,
    String,
    Array,
    Function,
    Struct(usize),
    None,
}

#[derive(Clone, Copy)]
pub enum ValueType {
    Integer(i64),
    Float(f64),
    BinOperator(BinOp),
    UnaryOperator(UnaryOp),
    Identifier,
    Let,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Integer(value) => {write!(f, "{value}")}
            ValueType::Float(value) => {write!(f, "{value}")}
            ValueType::BinOperator(value) => {write!(f, "{value}")}
            ValueType::UnaryOperator(value) => {write!(f, "{value}")}
            ValueType::Identifier => {write!(f, "Identifier")}
            ValueType::Let => {write!(f, "Let")}
        }
    }
}

#[derive(Clone)]
pub struct Node {
    pub value_type: ValueType,
    pub children: Vec<Node>,
    pub location: Location,
    pub data_type: DataType,
}

impl Node {
    pub fn new(node_type: ValueType, location: Location, return_type: DataType) -> Self {
        Self {
            value_type: node_type,
            children: vec![],
            location,
            data_type: return_type,
        }
    }
    pub fn add_child(&mut self, node: Node) {
        self.children.push(node);
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
mod test {

}



