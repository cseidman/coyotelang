#![allow(dead_code, unused_variables)]
use crate::ast::{BinOp, DataType, Node, UnaryOp, ValueType};
use crate::ast::ValueType::*;

type DReg = Option<usize>;
type SReg = Option<usize>;

enum Instruction {
    Mov(DataType, DReg, ValueType),
    Add(DataType, DReg, SReg),
    Sub(DataType, DReg, SReg),
    Mul(DataType, DReg, SReg),
    Div(DataType, DReg, SReg),
    Load(DataType, DReg, SReg),
    Store(DataType, DReg, ValueType),
    Neg(DataType, DReg),
    Halt,
}

const REG_SIZE: usize = 64000  ;
type Register = [Option<u8>; 8];

pub struct Generator {
    registers: [Register; REG_SIZE],
    instructions: Vec<Instruction>,
    cur_reg: Option<usize>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            registers: [[None;8];REG_SIZE],
            instructions: Vec::new(),
            cur_reg: None,
        }
    }

    fn to_asm(&self, asm: &mut String) {
        for instr in &self.instructions {
            match instr {
                Instruction::Mov(data_type, reg, value) => {
                    match value {
                        Integer(value) => {
                            asm.push_str(&format!("mov {}, {}\n", reg.unwrap_or(0), value));
                        },
                        Float(value) => {
                            asm.push_str(&format!("mov {}, {}\n", reg.unwrap_or(0), value));
                        },
                        _ => {}
                    }
                },
                Instruction::Add(data_type, reg1, reg2) => {
                    asm.push_str(&format!("add {}, {}\n", reg1.unwrap_or(0), reg2.unwrap_or(0)));
                },
                Instruction::Sub(data_type, reg1, reg2) => {
                    asm.push_str(&format!("sub {}, {}\n", reg1.unwrap_or(0), reg2.unwrap_or(0)));
                },
                Instruction::Mul(data_type, reg1, reg2) => {
                    asm.push_str(&format!("mul {}, {}\n", reg1.unwrap(), reg2.unwrap()));
                },
                Instruction::Div(data_type, reg1, reg2) => {
                    asm.push_str(&format!("div {}, {}\n", reg1.unwrap(), reg2.unwrap()));
                },
                Instruction::Load(data_type, reg1, reg2) => {
                    asm.push_str(&format!("load {}, {}\n", reg1.unwrap(), reg2.unwrap()));
                },
                Instruction::Store(data_type, reg1, value) => {
                    match value {
                        Identifier => {
                            asm.push_str(&format!("store {}, {}\n", reg1.unwrap(), value));
                        },
                        _ => {}
                    }
                },
                Instruction::Neg(data_type, reg) => {
                    asm.push_str(&format!("neg {}\n", reg.unwrap()));
                },
                Instruction::Halt => {
                    asm.push_str("halt\n");
                },
            }
        }
    }

    pub fn halt_instruction(&mut self) {
        self.instructions.push(Instruction::Halt);
    }

    pub fn generate(&mut self, node: &Node) {

        let reg:Option<usize> = None;
        let data_type = node.data_type.clone();

        match node.value_type.clone() {
            BinOperator(op) => {
                for child in &node.children {
                    self.generate(child);
                }
                self.instructions.push(match op {
                    BinOp::Add => {
                        Instruction::Add(data_type, reg, reg)
                    },
                    BinOp::Sub => {
                        Instruction::Sub(data_type, reg, reg)
                    },
                    BinOp::Mul => {
                        Instruction::Mul(data_type, reg, reg)
                    },
                    BinOp::Div => {
                        Instruction::Div(data_type, reg, reg)
                    },
                });
            },
            UnaryOperator(op) => {
                for child in &node.children {
                    self.generate(child);
                }
                if op == UnaryOp::Neg {
                    self.instructions.push(Instruction::Neg(data_type, reg));
                };
            },
            Integer(value) => {
                self.instructions.push(Instruction::Mov(data_type, reg, Integer(value)));
            },
            Float(value) => {
                self.instructions.push(Instruction::Mov(data_type, reg, Float(value)));
            },
            Identifier => {
                self.instructions.push(Instruction::Load(data_type, reg, reg));
            },
            Let => {
                self.instructions.push(Instruction::Store(data_type, reg, Identifier));
            },

        }
    }

}

pub fn generate(node: &Node, asm: &mut String) {
    let mut generator = Generator::new();
    generator.generate(node);
    generator.halt_instruction();
    generator.to_asm(asm);
}