//! Reads the AST and generates IR in SSA form
#![allow(dead_code, unused_variables)]

use crate::ast::node::{BinOp, NodeType, UnOp};
use crate::ast::tree::Node;
use crate::tokens::TokenType;
use std::fmt::{Display, Formatter};

const OPERATOR_LENGTH: usize = 1;
const OPERAND_LENGTH: usize = 8;

/// These are the instructions that the IR will have
/// The IR will be in SSA form

struct Symbols {
    list: Vec<String>,
    var_count: usize,
}
impl Symbols {
    fn new() -> Self {
        Self {
            list: vec![],
            var_count: 0,
        }
    }

    fn register_symbol(&mut self, symbol: String) -> usize {
        self.list.push(symbol.clone());
        self.var_count += 1;
        self.var_count - 1
    }
}

pub struct Instruction {
    start_location: usize,
    instruction_size: usize,
    code: String,
    jumped: bool,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.instruction_size > 0 {
            write!(f, "|{:06}| ", self.start_location)?;
        }
        write!(f, "{}", self.code)
    }
}

pub struct IrGenerator {
    instructions: Vec<Instruction>,
    current_location: usize,
    string_pool: Vec<String>,
    strings_index: usize,

    scope: usize,
    offset: usize,
    symbol_loc: Vec<Symbols>,

    jmp_counter: usize,
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
            current_location: 0,
            string_pool: Vec::new(),
            strings_index: 0,
            scope: 0,
            offset: 0,
            symbol_loc: vec![Symbols::new()],
            jmp_counter: 0,
        }
    }

    /// Clear the instructions. This is useful for REPLs where we're keeping a reference to the
    /// generator, but we need to clear the instructions before each run
    pub fn clear(&mut self) {
        self.instructions.clear()
    }

    fn next_jmp_counter(&mut self) -> usize {
        self.jmp_counter += 1;
        self.jmp_counter - 1
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
        let mut offset = self.offset;
        for i in (0..=scope).rev() {
            if let Some(index) = self.symbol_loc[i].list.iter().position(|x| x == name) {
                return index + offset;
            }
            if i > 0 {
                offset -= self.symbol_loc[i - 1].var_count;
            }
        }
        panic!("Variable '{name}' not found in symbol loc");
    }

    fn push_scope(&mut self) {
        let scope = self.scope;
        let offset = self.symbol_loc[scope].var_count;
        self.scope += 1;
        self.symbol_loc.push(Symbols::new());
        self.offset += offset;
        //self.push(format!("# push scope : offset={}", self.offset), 0);
    }

    fn pop_scope(&mut self) {
        self.scope -= 1;

        self.symbol_loc.pop();
        // Return the offset back to where it was
        let scope = self.scope;
        self.offset -= self.symbol_loc[scope].var_count;
        //self.push(format!("# pop scope : offset={}", self.offset), 0);
    }

    fn push<T: ToString>(&mut self, instruction: T, size: usize) {
        let instr = Instruction {
            start_location: self.current_location,
            instruction_size: size,
            code: instruction.to_string(),
            jumped: false,
        };

        self.current_location += size;
        self.instructions.push(instr);
    }

    pub fn generate(&mut self, node: &Node) {
        self.clear();
        self.generate_code(node);
    }

    fn generate_code(&mut self, node: &Node) {
        let data_type = &node.return_type;

        match node.clone().node_type {
            NodeType::Integer(value) => {
                self.push(format!("push {} ;", value), 1 + OPERAND_LENGTH + 1);
            }
            NodeType::Float(value) => {
                self.push(format!("push {} ;", value), 1 + OPERAND_LENGTH + 1);
            }
            NodeType::Text(value) => {
                let loc = self.get_string_location(&*value);
                self.push(format!("spool {} ;", loc), 1 + OPERAND_LENGTH + 1);
            }
            NodeType::Boolean(value) => {
                self.push(format!("push {} ;", value), 1 + OPERAND_LENGTH + 1);
            }

            NodeType::Block => {
                self.push_scope();
            }

            NodeType::EndBlock => {
                self.pop_scope();
            }

            NodeType::For => {
                let mut children = node.children.iter().peekable();
                self.push_scope();

                // Store the iteration variables
                let mut iter_var_location: usize = 0;
                if let Some(id_node) = children.next() {
                    if let NodeType::Ident(name) = &id_node.node_type {
                        iter_var_location = self.store_variable(name);
                    }
                }

                let mut iter2_var_location = self.store_variable("$2");

                let range = children.next().unwrap();
                // Push the two ends of the range
                for child in range.children.iter().rev() {
                    self.generate_code(&child);
                }

                // Store the beginning of the range
                self.push(
                    format!("store {iter_var_location} ; # from",),
                    1 + OPERAND_LENGTH + 1,
                );

                // Store the end of the range
                self.push(
                    format!("store {iter2_var_location} ; # to",),
                    1 + OPERAND_LENGTH + 1,
                );

                let code_block = children.next().unwrap();
                // Push the two ends of the range
                for child in code_block.children.iter().rev() {
                    self.generate_code(&child);
                }

                // Increment the iterator value
                self.push(
                    format!("load {iter_var_location} ;",),
                    1 + OPERAND_LENGTH + 1,
                );
                self.push(format!("push 1 ;",), 1 + OPERAND_LENGTH + 1);
                self.push(format!("add ;",), 1);
                self.push(
                    format!("store {iter_var_location} ;",),
                    1 + OPERAND_LENGTH + 1,
                );
                self.push(
                    format!("load {iter_var_location} ;",),
                    1 + OPERAND_LENGTH + 1,
                );
                self.push(
                    format!("load {iter2_var_location} ;",),
                    1 + OPERAND_LENGTH + 1,
                );
                self.push(format!("le ;",), 1 + OPERAND_LENGTH + 1);
                self.pop_scope();
            }

            NodeType::BinaryOp(op) => {
                for child in &node.children {
                    self.generate_code(child);
                }
                match op {
                    BinOp::Add => {
                        self.push("add ;", 1);
                    }
                    BinOp::Sub => {
                        self.push("sub ;", 1);
                    }
                    BinOp::Mul => {
                        self.push("mul ;", 1);
                    }
                    BinOp::Div => {
                        self.push("div ;", 1);
                    }
                    BinOp::Pow => {
                        self.push("pow ;", 1);
                    }
                    BinOp::Assign => {
                        //self.push(format!("set ;"));
                    }
                    BinOp::And => {
                        self.push("and ;", 1);
                    }
                    BinOp::Or => {
                        self.push("or ;", 1);
                    }
                    BinOp::GreaterThanEqual => {
                        self.push("ge ;", 1);
                    }
                    BinOp::GreaterThan => {
                        self.push("gt ;", 1);
                    }
                    BinOp::LessThanEqual => {
                        self.push("le ;", 1);
                    }
                    BinOp::LessThan => {
                        self.push("lt ;", 1);
                    }
                    BinOp::EqualEqual => {
                        self.push("eq ;", 1);
                    }
                    BinOp::NotEqual => {
                        self.push("neq ;", 1);
                    }
                }
            }
            NodeType::UnaryOp(op) => {
                for child in &node.children {
                    self.generate_code(child);
                }
                match op {
                    UnOp::Neg | UnOp::Not => {
                        self.push("neg ;", 1);
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

                let location = self.store_variable(&var_name);
                // Check the next child in an assignment operator
                if let Some(next_node) = node.children.get(0) {
                    // Generate the expression that gets assigned to the variable
                    self.generate_code(next_node);
                    // Generate the storage command
                    self.push(
                        format!("store {location} ; # name={var_name}",),
                        1 + OPERAND_LENGTH + 1,
                    );
                }
            }
            NodeType::Print => {
                for c in &node.children {
                    self.generate_code(c);
                }
                self.push("print ;", 1);
            }
            NodeType::Ident(name) => {
                let index = self.get_variable(&name);
                // Array element
                if node.children.len() == 1 {
                    self.generate_code(&node.children[0]);
                    if node.can_assign {
                        self.push(format!("store {index};"), 1 + OPERAND_LENGTH + 1);
                    } else {
                        self.push(format!("aload {index};"), 1 + OPERAND_LENGTH + 1);
                    }

                    return;
                }
                if node.can_assign {
                    self.push(
                        format!("store {index}; # name = {name}"),
                        1 + OPERAND_LENGTH + 1,
                    );
                } else {
                    self.push(
                        format!("load {index}; # name = {name}"),
                        1 + OPERAND_LENGTH + 1,
                    );
                }
            }
            // We don't need to capture the internal elements here because we're drilling
            // down into the elements
            NodeType::Array => {
                for child in node.children.iter().rev() {
                    self.generate_code(child);
                }
                let element_count = &node.children.len();
                self.push(
                    format!("newarray {element_count} ;"),
                    1 + OPERAND_LENGTH + 1,
                );
            }

            NodeType::If => {
                let mut jmp_false_loc: usize = 0;
                let mut jmp_false_byte_loc: usize = 0;

                let mut jmp_true_loc: usize = 0;
                let mut jmp_true_byte_loc: usize = 0;

                // Generate conditions
                for child in &node.children {
                    match child.node_type {
                        NodeType::Conditional => {
                            for c in &child.children {
                                self.generate_code(c);
                            }
                            self.push("jmpfalse 0", 1 + OPERAND_LENGTH + 1);
                            jmp_false_loc = self.instructions.len() - 1;
                            jmp_false_byte_loc = self.current_location;
                        }
                        NodeType::Block => {
                            self.push_scope();
                        }
                        NodeType::EndBlock => {
                            self.pop_scope();
                        }
                        NodeType::Else => {
                            self.push("# else", 0);

                            for c in &child.children {
                                self.generate_code(c);
                            }

                            self.instructions[jmp_true_loc].code =
                                format!("jmp {};", self.current_location - jmp_true_byte_loc);
                        }
                        NodeType::EndIf => {
                            self.push("# endif", 0);
                        }
                        NodeType::CodeBlock => {
                            // This is the body of the IF statement
                            for c in &child.children {
                                self.generate_code(c);
                            }
                            self.push("jmp 0;", 1 + OPERAND_LENGTH + 1);
                            jmp_true_loc = self.instructions.len() - 1;
                            jmp_true_byte_loc = self.current_location;

                            self.instructions[jmp_false_loc].code =
                                format!("jmpfalse {};", self.current_location - jmp_false_byte_loc);
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }

            NodeType::Root => {
                for child in &node.children {
                    self.generate_code(child);
                }
                self.push("Halt ;", 1);
            }
            _ => {
                println!(".end")
            }
        }
    }
}
