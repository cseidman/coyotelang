//! Reads the AST and generates IR in SSA form
#![allow(dead_code, unused_variables)]

use crate::allocator::Registers;
use crate::ast::tree::{BinOp, Command, Node, UnaryOp, ValueType};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// These are the instructions that the IR will have
/// The IR will be in SSA form

pub struct IrGenerator {
    registers: Registers,
    instructions: Vec<String>,
    tmp_regs: Vec<usize>,

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
            tmp_regs: Vec::new(),
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

    pub fn generate(&mut self, node: &Node) {
        self.clear();
        self.generate_code(node);
    }

    fn generate_code(&mut self, node: &Node) {
        let data_type = node.data_type;
        let prefix = data_type.get_prefix();

        match &node.value_type {
            ValueType::Integer(value) => {
                let reg = self.registers.allocate();
                self.push_reg(reg);
                self.push(format!("iconst %r{}, {};", reg, value));
            }
            ValueType::Float(value) => {
                let reg = self.registers.allocate();
                self.push_reg(reg);
                self.push(format!("fconst %r{}, {};", reg, value));
            }
            ValueType::Text(value) => {
                let reg = self.registers.allocate();
                self.push_reg(reg);
                let loc = self.get_string_location(&*value);
                self.push(format!("sconst %r{}, {};", reg, loc));
            }
            ValueType::BinOperator(op) => {
                for child in &node.children {
                    self.generate_code(child);
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
                    self.generate_code(child);
                }
                // The unary operator will have a register for the child
                // and the result will be stored in the same register
                let reg = self.peek_reg();
                match op {
                    UnaryOp::Neg | UnaryOp::Not => {
                        self.push(format!("{prefix}neg %r{reg};"));
                    }
                }
            }
            ValueType::Statement(command) if *command == Command::Let => {
                let node = node.children[0].clone();

                // If it's NOT an identifier, panic
                let identifier_name = match &node.value_type {
                    ValueType::Identifier(name) => name,
                    _ => panic!("Expected identifier: found {:?}", node),
                };

                let data_type = node.data_type;
                let prefix = data_type.get_prefix();

                // Check of the next child in an assignment operator
                let next_node = &node.children[0];
                if next_node.value_type == ValueType::AssignmentOperator {
                    self.generate_code(&node.children[1]);

                    // Assign a register to the identifier and store it
                    let sreg = self.store_variable(identifier_name);
                    let reg = self.pop_reg();

                    self.push(format!("store %r{sreg}, %r{reg};",));
                }
            }
            ValueType::Statement(command) if *command == Command::Print => {
                for c in &node.children {
                    self.generate_code(c);
                }
                let reg = self.pop_reg();
                self.push(format!("{prefix}print %r{reg};"));
            }
            ValueType::Identifier(name) => {
                if let Some(var_reg) = self.get_variable(name) {
                    let reg = self.registers.allocate();
                    self.push_reg(reg);
                    self.push(format!("load %r{reg}, %r{var_reg}"));
                } else {
                    panic!("Variable {} not found", name);
                }
            }

            ValueType::Array => {
                // This is the register where the array is stored
                let reg = self.registers.allocate();
                // Save the array register for later
                self.push_reg(reg);
                // Get the count of elements
                let count = node.children.len();
                self.push(format!("a{prefix}const %r{reg}, {count}"));
                for child in &node.children {
                    self.generate_code(child);
                    let source_reg = self.pop_reg();
                    self.registers.free_register(source_reg);
                    self.push(format!("{prefix}mova %r{reg}, %r{source_reg}"));
                }
            }

            ValueType::Root => {
                for child in &node.children {
                    self.generate_code(child);
                }
            }

            _ => {}
        }
    }
}
