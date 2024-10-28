//! Reads the AST and generates IR in SSA form
#![allow(dead_code, unused_variables)]

use crate::allocator::Registers;
use crate::ast::tree::{BinOp, Node, UnaryOp, ValueType};

/// These are the instructions that the IR will have
/// The IR will be in SSA form

struct IrGenerator {
    registers: Registers,
    instructions: Vec<String>,
    tmp_regs: Vec<usize>,
}

pub fn generate(node: &Node) -> Vec<String> {
    let mut generator = IrGenerator::new(node);
    generator.generate(node);
    generator.instructions
}

impl IrGenerator {
    pub fn new(node: &Node) -> Self {
        Self {
            registers: Registers::new(1024000),
            instructions: Vec::new(),
            tmp_regs: Vec::new(),
        }
    }

    fn push(&mut self, instruction: String) {
        self.instructions.push(instruction);
    }

    fn pop_reg(&mut self) -> usize {
        if let Some(reg) = self.tmp_regs.pop() {
            return reg;
        }
        self.registers.allocate()
    }

    fn peek_reg(&mut self) -> usize {
        if let Some(reg) = self.tmp_regs.clone().pop() {
            return reg;
        }
        self.registers.allocate()
    }

    fn push_reg(&mut self, reg: usize) {
        self.tmp_regs.push(reg);
    }

    fn generate(&mut self, node: &Node) {
        let data_type = node.data_type;
        let prefix = data_type.get_prefix();

        match node.value_type {
            ValueType::Integer(value) => {
                let reg = self.registers.allocate();
                self.push_reg(reg);
                self.push(format!("imov %r{}, {};", reg, value));
            }
            ValueType::Float(value) => {
                let reg = self.registers.allocate();
                self.push_reg(reg);
                self.push(format!("fmov %r{}, {};", reg, value));
            }
            ValueType::BinOperator(op) => {
                for child in &node.children {
                    self.generate(child);
                }
                // The binary operator will have a register for each child
                // and the result will be stored in the first register
                // So we need to find two free registers to use
                // And the second register can be freed after the operation
                let rhr = self.pop_reg();
                let lhr = self.pop_reg();

                match op {
                    BinOp::Add => {
                        self.push(format!("{prefix}add %r{lhr}, %r{rhr} ;"));
                    }
                    BinOp::Sub => {
                        self.push(format!("{prefix}sub %r{lhr}, %r{rhr} ;"));
                    }
                    BinOp::Mul => {
                        self.push(format!("{prefix}mul %r{lhr}, %r{rhr} ;"));
                    }
                    BinOp::Div => {
                        self.push(format!("{prefix}div %r{lhr}, %r{rhr} ;"));
                    }
                }

                self.push_reg(lhr);
                self.registers.free_register(rhr);
            }
            ValueType::UnaryOperator(op) => {
                for child in &node.children {
                    self.generate(child);
                }
                // The unary operator will have a register for the child
                // and the result will be stored in the same register
                let reg = self.peek_reg();
                match op {
                    UnaryOp::Neg | UnaryOp::Not => {
                        self.push(format!("{prefix}neg %r{reg}; neg %r{reg}"));
                    }
                }
            }
            ValueType::Let => {
                println!("Generating let");
                for child in &node.children {
                    self.generate(child);
                }

                let reg = self.pop_reg();
                let sreg = self.pop_reg();

                self.push(format!(
                    "{prefix}store %r{sreg}, %r{reg}; store %r{sreg} in %r{reg}",
                ));
            }
            ValueType::Identifier(name) => {
                let reg = self.registers.allocate();
                self.push_reg(reg);
                self.push(format!("load %r{reg}, {name}"));
            }
            ValueType::AssignmentOperator => {
                for child in &node.children {
                    self.generate(child);
                }
                let reg = self.pop_reg();
                let sreg = self.pop_reg();
                self.push(format!(
                    "store %r{sreg}, %r{reg}; store %r{sreg} in %r{reg}"
                ));
            }
            ValueType::Root => {
                for child in &node.children {
                    self.generate(child);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::tree::{DataType, Node, NodeType, ValueType};
    use crate::tokens::Location;

    #[test]
    fn test_ir_generator() {
        let mut node = Node::new(
            ValueType::BinOperator(BinOp::Add),
            Location::new(),
            DataType::Integer,
            NodeType::Op,
        );
        let left = Node::new(
            ValueType::Integer(10),
            Location::new(),
            DataType::Integer,
            NodeType::Leaf,
        );
        let right = Node::new(
            ValueType::Integer(20),
            Location::new(),
            DataType::Integer,
            NodeType::Leaf,
        );
        node.add_child(left);
        node.add_child(right);

        let mut node3 = Node::new(
            ValueType::BinOperator(BinOp::Add),
            Location::new(),
            DataType::Integer,
            NodeType::Op,
        );
        let left3 = Node::new(
            ValueType::Integer(30),
            Location::new(),
            DataType::Integer,
            NodeType::Leaf,
        );
        let right3 = Node::new(
            ValueType::Integer(40),
            Location::new(),
            DataType::Integer,
            NodeType::Leaf,
        );
        node3.add_child(left3);
        node3.add_child(right3);

        let mut node2 = Node::new(
            ValueType::BinOperator(BinOp::Add),
            Location::new(),
            DataType::Integer,
            NodeType::Op,
        );

        node2.add_child(node3.clone());
        node2.add_child(node.clone());

        let instructions = generate(&node2);
        //assert_eq!(instructions.len(), 3);
        instructions.iter().for_each(|instruction| {
            println!("{}", instruction);
        });
    }
}
