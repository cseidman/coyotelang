//! Reads the AST and generates IR in SSA form
#![allow(dead_code, unused_variables)]
use crate::ast::{BinOp, UnaryOp, Node, ValueType};

pub fn generate_ir(node: &Node) {
    let reg: Option<usize> = None;

    match node.value_type.clone() {
        ValueType::BinOperator(op) => {
            let data_type = node.data_type.clone();
            for child in &node.children {
                generate_ir(child);
            }
            // self.instructions.push(match op {
            match op {
                BinOp::Add => {
                    // Instruction::Add(data_type, reg, reg)
                }
                BinOp::Sub => {
                    // Instruction::Sub(data_type, reg, reg)
                }
                BinOp::Mul => {
                    // Instruction::Mul(data_type, reg, reg)
                }
                BinOp::Div => {
                    // Instruction::Div(data_type, reg, reg)
                }
            }
        }
        ValueType::UnaryOperator(op) => {
            let data_type = node.data_type.clone();
            for child in &node.children {
                generate_ir(child);
            }
            // self.instructions.push(match op {
            match op {
                UnaryOp::Neg => {
                    // Instruction::Neg(data_type, reg)
                }
                UnaryOp::Not => {
                    // Instruction::Neg(data_type, reg)
                }
            }
        }
        _ => {}
    }
}
