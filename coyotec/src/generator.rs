//! Reads the AST and generates IR in SSA form
#![allow(dead_code, unused_variables)]

use crate::ast::node::{NodeType, UnOp};
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
        macro_rules! instr {
            ($instr:expr) => {
                self.push(format!("{} ;", $instr), 1);
            };

            ($instr:expr, $operand:expr) => {
                self.push(format!("{} {} ;", $instr, $operand), 1 + OPERAND_LENGTH + 1);
            };

            ($instr:expr, $operand:expr, $comment:expr) => {
                self.push(
                    format!("{} {} ; # {}", $instr, $operand, $comment),
                    1 + OPERAND_LENGTH + 1,
                );
            };
        }

        macro_rules! get_instr_loc {
            () => {
                self.instructions.len() - 1
            };
        }

        let data_type = &node.return_type;

        match node.clone().node_type {
            NodeType::Integer(value) => {
                instr!("push", value);
            }
            NodeType::Float(value) => {
                instr!("push", value);
            }
            NodeType::Text(value) => {
                let loc = self.get_string_location(&*value);
                instr!("spool", loc);
            }
            NodeType::Boolean(value) => {
                instr!("push", value);
            }

            NodeType::Block => {
                self.push_scope();
            }

            NodeType::EndBlock => {
                self.pop_scope();
            }

            NodeType::For => {
                let mut jmp_false_loc: usize = 0;
                let mut jmp_true_loc: usize = 0;

                let mut iter_var_location = 0;

                // Hidden variable containing the name of the target condition
                let mut iter2_var_location: usize = 0;

                for child in &node.children {
                    match &child.node_type {
                        NodeType::Block => {
                            self.push_scope();
                        }
                        NodeType::EndBlock => {
                            self.pop_scope();
                        }
                        NodeType::CodeBlock => {
                            jmp_true_loc = self.current_location;
                            instr!("load", iter_var_location, "Load the start");
                            instr!("load", iter2_var_location, "Load the target");
                            instr!("ge");
                            instr!("jmpfalse", 0);
                            jmp_false_loc = get_instr_loc!();
                            for ch in &child.children {
                                self.generate_code(&ch);
                            }
                            instr!("load", iter_var_location, "Start incr");
                            instr!("push", 1);
                            instr!("add");
                            instr!("store", iter_var_location);
                        }
                        NodeType::Range => {
                            for ch in &child.children {
                                self.generate_code(ch);
                            }
                            instr!("store", iter2_var_location);
                            instr!("store", iter_var_location);
                        }
                        NodeType::Ident(iter_name) => {
                            // Name of the iteration variable
                            iter_var_location = self.store_variable(&iter_name);
                            iter2_var_location = self.store_variable("$2");
                        }
                        NodeType::EndFor => {
                            instr!("jmp", jmp_true_loc);
                            self.instructions[jmp_false_loc].code =
                                format!("jmpfalse {}", self.current_location);
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }

            NodeType::BinaryOp(op) => {
                for child in &node.children {
                    self.generate_code(child);
                }

                let binop = format!("{}", op);
                instr!(binop);
            }

            NodeType::UnaryOp(op) => {
                for child in &node.children {
                    self.generate_code(child);
                }
                match op {
                    UnOp::Neg | UnOp::Not => {
                        instr!("neg");
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
                    instr!("store", location);
                }
            }
            NodeType::Print => {
                for c in &node.children {
                    self.generate_code(c);
                }
                instr!("print");
            }
            NodeType::Ident(name) => {
                let index = self.get_variable(&name);
                // Array element
                if node.children.len() == 1 {
                    self.generate_code(&node.children[0]);
                    if node.can_assign {
                        instr!("store", index);
                    } else {
                        instr!("aload", index);
                    }

                    return;
                }
                if node.can_assign {
                    instr!("store", index);
                } else {
                    instr!("load", index);
                }
            }
            // We don't need to capture the internal elements here because we're drilling
            // down into the elements
            NodeType::Array => {
                for child in node.children.iter().rev() {
                    self.generate_code(child);
                }
                let element_count = &node.children.len();
                instr!("newarray", element_count);
            }

            NodeType::If => {
                let mut jmp_false_loc: usize = 0;
                let mut jmp_true_loc: usize = 0;

                let mut has_else = false;

                // Generate conditions
                for child in &node.children {
                    match child.node_type {
                        NodeType::Conditional => {
                            for c in &child.children {
                                self.generate_code(c);
                            }
                            instr!("jmpfalse", 0);
                            jmp_false_loc = get_instr_loc!();
                        }

                        NodeType::Else => {
                            has_else = true;
                            self.push("# else", 0);
                            instr!("jmp", 0);
                            jmp_true_loc = self.instructions.len() - 1;
                            self.instructions[jmp_false_loc].code =
                                format!("jmpfalse {} ;", self.current_location);
                            for c in &child.children {
                                self.generate_code(c);
                            }
                        }
                        NodeType::EndIf => {
                            if has_else {
                                self.instructions[jmp_true_loc].code =
                                    format!("jmp {};", self.current_location);
                            }
                            self.push("# endif", 0);
                        }
                        NodeType::CodeBlock => {
                            // This is the body of the IF statement
                            for c in &child.children {
                                self.generate_code(c);
                            }

                            self.instructions[jmp_false_loc].code =
                                format!("jmpfalse {} ;", self.current_location);
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
                instr!("halt");
            }
            _ => {
                println!(".end")
            }
        }
    }
}
