//! Reads the AST and generates IR in SSA form
#![allow(dead_code, unused_variables)]

use crate::ast::node::{BinOp, NodeType, UnOp};
use crate::ast::tree::Node;
use crate::tokens::TokenType;
use std::fmt::{Display, Formatter};

/// These are the instructions that the IR will have
/// The IR will be in SSA form

struct Symbols {
    list: Vec<String>,
    syp: usize,
}
impl Symbols {
    fn new() -> Self {
        Self {
            list: vec![],
            syp: 0,
        }
    }

    fn register_symbol(&mut self, symbol: String) -> usize {
        self.list.push(symbol.clone());
        self.syp += 1;
        self.syp - 1
    }
}

pub struct IrGenerator {
    instructions: Vec<String>,
    string_pool: Vec<String>,
    strings_index: usize,

    scope: usize,
    offset: usize,
    symbol_loc: Vec<Symbols>,
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
        writeln!(f, ".globals")?;
        writeln!(f, "    {}", self.symbol_loc.len())?;

        for line in &self.instructions {
            writeln!(f, "{line}")?;
        }
        writeln!(f, "")
    }
}

impl IrGenerator {
    pub fn new(node: &Node) -> Self {
        Self {
            instructions: Vec::new(),
            string_pool: Vec::new(),
            strings_index: 0,
            scope: 0,
            offset: 0,
            symbol_loc: vec![Symbols::new()],
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
        let scope = self.scope;
        self.symbol_loc[scope].register_symbol(name.to_string()) + self.offset
    }

    fn get_global(&mut self, name: &str) -> Option<usize> {
        if let Some(index) = self.symbol_loc[0].list.iter().position(|x| x == name) {
            return Some(index);
        }
        None
    }

    fn get_variable(&mut self, name: &str) -> usize {
        let scope = self.scope;
        let location = self.offset;
        if let Some(index) = self.symbol_loc[scope].list.iter().position(|x| x == name) {
            return index + location;
        }
        // The variable was not found, so we check in the global space (scope 0)
        if let Some(index) = self.get_global(name) {
            return index;
        }

        panic!("Variable '{name}' not found in symbol loc");
    }

    fn push_scope(&mut self) {
        let scope = self.scope;
        let offset = self.symbol_loc[scope].syp;
        self.scope += 1;
        self.symbol_loc.push(Symbols::new());
        self.offset += offset;
        self.push(format!("# push scope : offset={}", self.offset));
    }

    fn pop_scope(&mut self) {
        self.scope -= 1;

        self.symbol_loc.pop();
        // Return the offset back to where it was
        let scope = self.scope;
        self.offset -= self.symbol_loc[scope].syp;
        self.push(format!("# pop scope : offset={}", self.offset));
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
                self.push(format!("spool {} ;", loc));
            }
            NodeType::Boolean(value) => {
                self.push(format!("push {} ;", value));
            }

            NodeType::Block => {
                self.push_scope();
            }

            NodeType::EndBlock => {
                self.pop_scope();
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
                    BinOp::Assign => {
                        //self.push(format!("set ;"));
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
                let location = self.store_variable(&var_name);
                // Check the next child in an assignment operator
                if let Some(next_node) = node.children.get(0) {
                    // Generate the expression that gets assigned to the variable
                    self.generate_code(next_node);
                    // Generate the storage command
                    self.push(format!("store {location} ; # name={var_name}",));
                }
            }
            NodeType::Print => {
                for c in &node.children {
                    self.generate_code(c);
                }
                self.push(format!("print ;"));
            }
            NodeType::Ident(name) => {
                let index = self.get_variable(&name);
                // Array element
                if node.children.len() == 1 {
                    self.generate_code(&node.children[0]);
                    if node.can_assign {
                        self.push(format!("astore {index};"));
                    } else {
                        self.push(format!("aload {index};"));
                    }

                    return;
                }
                if node.can_assign {
                    self.push(format!("store {index}; # name = {name}"));
                } else {
                    self.push(format!("load {index}; # name = {name}"));
                }
            }
            // We don't need to capture the internal elements here because we're drilling
            // down into the elements
            NodeType::Array => {
                for child in node.children.iter().rev() {
                    self.generate_code(child);
                }
                let element_count = &node.children.len();
                self.push(format!("newarray {element_count} ;"));
            }

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
