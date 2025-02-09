//! Reads the AST and generates IR in SSA form
#![allow(dead_code, unused_variables)]

use crate::allocator::Registers;
use crate::ast::node::{BinOp, NodeType, UnOp};
use crate::ast::tree::{Command, Node, ValueType};
use crate::datatypes::datatype::DataType;
use crate::tokens::{BaseType, TokenType};
use anyhow::anyhow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// These are the instructions that the IR will have
/// The IR will be in SSA form

pub struct IrGenerator {
    registers: Registers,
    instructions: Vec<String>,
    string_pool: Vec<String>,
    strings_index: usize,
    scope: usize,
    symbol_regs: Vec<HashMap<String, usize>>,
}

pub fn generate(node: &Node) -> String {
    let mut generator = IrGenerator::new(node);
    generator.generate_code(node);
    format!("{}", generator)
}

impl Display for IrGenerator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Write out the constants
        writeln!(f, ".constants")?;
        for s in self.string_pool.iter() {
            writeln!(f, "    {}", s)?;
        }
        writeln!(f, ".end")?;
        //writeln!(f, ".instructions")?;
        for line in &self.instructions {
            writeln!(f, "{line}")?;
        }
        //writeln!(f, ".end")?;
        writeln!(f, "")
    }
}

impl IrGenerator {
    pub fn new(node: &Node) -> Self {
        Self {
            registers: Registers::new(1024000),
            instructions: Vec::new(),
            string_pool: Vec::new(),
            strings_index: 0,
            scope: 0,
            symbol_regs: vec![HashMap::new()],
        }
    }

    /// Clear the instructions. This is useful for REPLs where we're keeping a reference to the
    /// generator, but we need to clear the instructions before each run
    pub fn clear(&mut self) {
        self.instructions.clear()
    }

    /// Get the location of a string in the string pool. If the string is not found, it will be added
    fn get_string_location(&mut self, string: &str) -> usize {
        for (i, s) in self.string_pool.iter().enumerate() {
            if s == string {
                return i;
            }
        }
        let idx = self.strings_index;
        self.string_pool.push(string.to_string());
        self.strings_index += 1;
        idx
    }

    fn store_variable(&mut self, name: &str) -> usize {
        let reg = self.registers.allocate();
        self.symbol_regs[self.scope].insert(name.to_string(), reg);
        reg
    }

    fn get_variable(&mut self, name: &str) -> Option<usize> {
        self.symbol_regs[self.scope].get(name).copied()
    }

    fn push(&mut self, instruction: String) {
        self.instructions.push(instruction);
    }

    pub fn generate(&mut self, node: &Node) {
        self.clear();
        self.generate_code(node);
    }

    fn generate_code(&mut self, node: &Node) {
        let data_type = &node.return_type;

        match node.clone().node_type {
            NodeType::Integer(value) => {
                self.push(format!("push {} ;", value));
            }
            NodeType::Float(value) => {
                self.push(format!("push {} ;", value));
            }
            NodeType::Text(value) => {
                let loc = self.get_string_location(&*value);
                self.push(format!("push {} ;", loc));
            }
            NodeType::Boolean(value) => {
                self.push(format!("push {} ;", value));
            }

            NodeType::BinaryOp(op) => {
                for child in &node.children {
                    self.generate_code(child);
                }
                match op {
                    BinOp::Add => {
                        self.push(format!("add ;"));
                    }
                    BinOp::Sub => {
                        self.push(format!("sub ;"));
                    }
                    BinOp::Mul => {
                        self.push(format!("mul ;"));
                    }
                    BinOp::Div => {
                        self.push(format!("div ;"));
                    }
                    BinOp::Pow => {
                        self.push(format!("pow ;"));
                    }

                    BinOp::And => {}
                    BinOp::Or => {}
                }
            }
            NodeType::UnaryOp(op) => {
                for child in &node.children {
                    self.generate_code(child);
                }
                match op {
                    UnOp::Neg | UnOp::Not => {
                        self.push(format!("neg ;"));
                    }
                }
            }
            NodeType::Let => {
                let node = node.children[0].clone();
                let token_type = node.token.unwrap().token_type;

                // There needs to be a variable name at this point
                let var_name = if let TokenType::Identifier(name) = token_type {
                    name
                } else {
                    panic!(
                        "There needs to be a variable name after `let`, found {:?}",
                        token_type
                    );
                };

                let data_type = node.return_type;

                // Check the next child in an assignment operator
                if let Some(next_node) = node.children.get(0) {
                    // Generate the expression that gets assigned to the variable
                    self.generate_code(next_node);

                    self.push(format!("store ;",));
                }
            }
            NodeType::Print => {
                for c in &node.children {
                    self.generate_code(c);
                }
                self.push(format!("print ;"));
            }
            NodeType::Ident(name, node_type) => {
                if let Some(var_reg) = self.get_variable(&name) {
                    // Allocate register to store the variable

                    // Load the contents of the location of the variable to the newly
                    // allocated register
                    self.push(format!("load {var_reg}"));

                    for child in &node.children {}
                } else {
                    panic!("Variable {} not found", name);
                }
            }
            // We don't need to capture the internal elements here because we're drilling
            // down into the elements
            NodeType::Array(_) => {}

            NodeType::Root => {
                for child in &node.children {
                    self.generate_code(child);
                }
            }

            _ => {
                println!(".end")
            }
        }
    }
}
