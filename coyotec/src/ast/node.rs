use crate::ast::datatype::DataType;
use crate::ast::tree::{NodeType, ValueType};
use crate::tokens::Location;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
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
}
