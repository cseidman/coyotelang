use std::any::Any;
use std::fmt::Display;
use crate::tokens::Location;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DataType {
    Integer,
    Float,
    Boolean,
    String,
    Array,
    Function,
    None,
}

#[derive(Clone, Copy)]
pub enum NodeType {
    Integer(i64),
    Float(f64),
    BinOperator(BinOp),
    UnaryOperator(UnaryOp),
}
#[derive(Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
    pub location: Location,
    pub data_type: DataType,
}

impl Node {
    pub fn new(node_type: NodeType, location: Location, return_type: DataType) -> Self {
        Self {
            node_type,
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

impl BinOp {
    pub fn op_from_type(&self, data_type: DataType) -> String {
        match (self, data_type) {
            (BinOp::Add, DataType::Integer) => "iadd",
            (BinOp::Sub, DataType::Integer) => "isub",
            (BinOp::Mul, DataType::Integer) => "imul",
            (BinOp::Div, DataType::Integer) => "idiv",

            (BinOp::Add, DataType::Float) => "fadd",
            (BinOp::Sub, DataType::Float) => "fsub",
            (BinOp::Mul, DataType::Float) => "fmul",
            (BinOp::Div, DataType::Float) => "fdiv",
            _ => {
                panic!("Invalid operation for data type");
            }
        }.to_string()

    }
}

#[derive(Debug, Clone, Copy)]
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



