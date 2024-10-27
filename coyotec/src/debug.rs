#![allow(dead_code)]
use crate::ast::tree::*;
/// Display the ast
pub fn display_ast(node: &Node, level: usize) {
    println!("{}", node);
    for child in &node.children {
        display_ast(child, level + 1);
    }
}
