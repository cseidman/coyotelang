use crate::ast::tree::ValueType;
use crate::tokens::BaseType::Undefined;
use crate::tokens::{BaseType, Location, Token};
use std::fmt::{write, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    And,
    Or,
}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BinOp::Add => write!(f, "add"),
            BinOp::Sub => write!(f, "sub"),
            BinOp::Mul => write!(f, "mul"),
            BinOp::Div => write!(f, "div"),
            BinOp::Pow => write!(f, "pow"),
            BinOp::And => write!(f, "and"),
            BinOp::Or => write!(f, "or"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnOp {
    Neg,
    Not,
}
impl Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UnOp::Neg => write!(f, "neg"),
            UnOp::Not => write!(f, "not"),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Root,
    Undefined,
    Integer(f64),
    Float(f64),
    Boolean(bool),
    Text(Box<String>),
    Ident(Box<String>, Box<NodeType>),
    Array(Box<NodeType>),
    UnaryOp(UnOp),
    BinaryOp(BinOp),
    Function(Box<Vec<NodeType>>),
    Assignment,
    // Statements
    Let,
    Print,
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NodeType::Root => write!(f, "root"),
            NodeType::Undefined => write!(f, "undefined"),
            NodeType::Integer(i) => write!(f, "Int:{}", i),
            NodeType::Float(float) => write!(f, "Float:{}", float),
            NodeType::Boolean(b) => write!(f, "Boolean:{}", b),
            NodeType::Text(t) => write!(f, "{}", t),
            NodeType::Ident(t, _) => write!(f, "Ident:{}", t),
            NodeType::Array(t) => write!(f, "Array:{}", t),
            NodeType::UnaryOp(UnOp::Neg) => write!(f, "neg"),
            NodeType::UnaryOp(UnOp::Not) => write!(f, "not"),
            NodeType::BinaryOp(BinOp::Add) => write!(f, "add"),
            NodeType::BinaryOp(BinOp::Sub) => write!(f, "sub"),
            NodeType::BinaryOp(BinOp::Mul) => write!(f, "mul"),
            NodeType::BinaryOp(BinOp::Div) => write!(f, "div"),
            NodeType::BinaryOp(BinOp::Pow) => write!(f, "pow"),
            NodeType::BinaryOp(BinOp::And) => write!(f, "and"),
            NodeType::BinaryOp(BinOp::Or) => write!(f, "or"),
            NodeType::Function(_) => write!(f, "function"),
            NodeType::Assignment => write!(f, "assignment"),
            NodeType::Let => write!(f, "let"),
            NodeType::Print => write!(f, "print"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
    pub token: Option<Token>,
    // This gets filled in a subsequent pass
    pub return_type: BaseType,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.node_type)
    }
}

impl Node {
    pub fn new(node_type: NodeType, token: Option<Token>) -> Self {
        Self {
            node_type,
            children: vec![],
            token,
            return_type: Undefined,
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

/// A simple recursive function that displays the entire tree
/// starting from the given `node`.
pub fn display_tree(node: &Node) {
    // We'll use a helper (inner) function that carries a `depth` parameter
    // to control indentation.
    fn print_node(node: &Node, depth: usize) {
        let indent = "  ".repeat(depth); // 2 spaces per level
        println!("{}NodeType: {}", indent, node.node_type);

        // Recursively print each child, increasing the depth.
        for child in &node.children {
            print_node(child, depth + 1);
        }
    }

    // Start recursion at depth 0
    print_node(node, 0);
}
