use crate::tokens::BaseType::Undefined;
use crate::tokens::{BaseType, Token};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    And,
    Or,
    Assign,
    EqualEqual,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
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
            BinOp::Assign => write!(f, "or"),
            BinOp::EqualEqual => write!(f, "=="),
            BinOp::NotEqual => write!(f, "!="),
            BinOp::GreaterThan => write!(f, ">"),
            BinOp::GreaterThanEqual => write!(f, ">="),
            BinOp::LessThan => write!(f, "<"),
            BinOp::LessThanEqual => write!(f, "<="),
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
    Ident(Box<String>),
    Array,
    UnaryOp(UnOp),
    BinaryOp(BinOp),
    Function(Box<Vec<NodeType>>),
    Assignment,
    // IF can be an expression as well
    If,
    Else,
    ElseIf,
    EndIf,
    // Statements
    Let,
    Print,

    Block,
    EndBlock,
    Conditional,
    CodeBlock,
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
            NodeType::Ident(t) => write!(f, "Ident:{}", t),
            NodeType::Array => write!(f, "Array"),
            NodeType::UnaryOp(UnOp::Neg) => write!(f, "neg"),
            NodeType::UnaryOp(UnOp::Not) => write!(f, "not"),
            NodeType::BinaryOp(BinOp::Add) => write!(f, "add"),
            NodeType::BinaryOp(BinOp::Sub) => write!(f, "sub"),
            NodeType::BinaryOp(BinOp::Mul) => write!(f, "mul"),
            NodeType::BinaryOp(BinOp::Div) => write!(f, "div"),
            NodeType::BinaryOp(BinOp::Pow) => write!(f, "pow"),
            NodeType::BinaryOp(BinOp::And) => write!(f, "and"),
            NodeType::BinaryOp(BinOp::Or) => write!(f, "or"),
            NodeType::BinaryOp(BinOp::Assign) => write!(f, "assign"),

            NodeType::BinaryOp(BinOp::GreaterThan) => write!(f, "gt"),
            NodeType::BinaryOp(BinOp::GreaterThanEqual) => write!(f, "ge"),
            NodeType::BinaryOp(BinOp::LessThan) => write!(f, "lt"),
            NodeType::BinaryOp(BinOp::LessThanEqual) => write!(f, "le"),
            NodeType::BinaryOp(BinOp::EqualEqual) => write!(f, "eq"),
            NodeType::BinaryOp(BinOp::NotEqual) => write!(f, "neq"),

            NodeType::Function(_) => write!(f, "function"),
            NodeType::Assignment => write!(f, "assignment"),
            NodeType::Let => write!(f, "let"),
            NodeType::Print => write!(f, "print"),
            NodeType::Block => write!(f, "block"),
            NodeType::EndBlock => write!(f, "endblock"),
            NodeType::If => write!(f, "if"),
            NodeType::Else => write!(f, "else"),
            NodeType::ElseIf => write!(f, "elseif"),
            NodeType::EndIf => write!(f, "endif"),
            NodeType::Conditional => write!(f, "conditional"),
            NodeType::CodeBlock => write!(f, "codeblock"),
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
    pub can_assign: bool,
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
            can_assign: false,
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
        println!("{}{}", indent, node.node_type);

        // Recursively print each child, increasing the depth.
        for child in &node.children {
            print_node(child, depth + 1);
        }
    }

    // Start recursion at depth 0
    print_node(node, 0);
}
