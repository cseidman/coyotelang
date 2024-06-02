use std::fmt::Write;
use crate::ast::Node;
use crate::ast::NodeType::*;
use std::fmt::{write, Error};


const REG_SIZE: usize = 64000  ;
type Register = [Option<u8>; 8];

pub struct Generator {
    registers: [Register; REG_SIZE],
    cur_reg: usize,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            registers: [[None;8];REG_SIZE],
            cur_reg: 0,
        }
    }

    pub fn generate(&mut self, node: &Node, asm: &mut String) {
        match node.node_type.clone() {
            BinOperator(op) => {
                let data_type = node.data_type;
                for child in &node.children {
                    self.generate(child, asm);
                }
                write(asm,format_args!("{} %r{}, %r{} ;", op.op_from_type(data_type), self.cur_reg -2, self.cur_reg -1)).expect("Error writing to asm");
                self.cur_reg -=1;
            },
            UnaryOperator(op) => {
                println!("Generating code for UnaryOperator");
            },
            Integer(value) => {
                let reg = self.cur_reg;
                write(asm,format_args!("imov %r{reg}, {value} ;")).expect("Error writing to asm");
                self.cur_reg +=1;
            },
            Float(value) => {
                println!("{value}");
            },
            _ => {
                println!("Code generation not implemented for this node");
            }
        }
    }
}

pub fn generate(node: &Node, asm: &mut String) {
    let mut generator = Generator::new();
    generator.generate(node, asm);
}