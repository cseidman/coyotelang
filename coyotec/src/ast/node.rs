use crate::ast::tree::{BinOp, NodeType, UnaryOp, ValueType};
use crate::datatypes::datatype::DataType;
use crate::tokens::Location;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Node {
    pub value_type: ValueType,
    pub children: Vec<Node>,
    pub location: Location,
    pub data_type: DataType,
    pub node_type: NodeType,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value_type)
    }
}

impl Node {
    pub fn new(
        value_type: ValueType,
        location: Location,
        return_type: DataType,
        node_type: NodeType,
    ) -> Self {
        Self {
            value_type,
            children: vec![],
            location,
            data_type: return_type,
            node_type,
        }
    }
    pub fn add_child(&mut self, node: Node) {
        self.children.push(node);
    }
    pub fn display_tree(&self) {
        println!("{}", self);
        for child in &self.children {
            child.display_tree();
        }
    }
}

pub enum Term {
    Expr(ExprNode),
    Const(ConstNode),
}

struct ExprNode {
    pub expr: Box<Term>,
    pub location: Location,
}
struct BinopNode {
    pub left: Box<ExprNode>,
    pub right: Box<ExprNode>,
    pub binop: BinOp,
    pub location: Location,
}
struct UnopNode {
    pub value: Box<ExprNode>,
    pub unop: UnaryOp,
    pub location: Location,
}
struct ConstNode {
    pub value: Box<Node>,
    pub data_type: DataType,
    pub location: Location,
}
