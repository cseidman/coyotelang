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

#[derive(Debug, Clone)]
pub struct Instruction {
    start_location: usize,
    instruction_size: usize,
    code: String,
    jumped: bool,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.instruction_size > 0 {
            write!(f, "{:06} | ", self.start_location)?;
        }
        write!(f, "{}", self.code)
    }
}

/// Function template
#[derive(Debug, Clone, Default)]
struct Function {
    name: String,
    arity: usize,
    instructions: Vec<Instruction>,
    current_location: usize,
    slots: usize,
    constructed: bool,
}

impl Function {
    fn calculate_bytes(&self) -> usize {
        self.instructions.iter().map(|i| i.instruction_size).sum()
    }
}

/// Struct for loops
#[derive(Clone)]
struct LoopLocations {
    start_location: usize,
    exit_location: usize,
    continue_location: usize,
    breaks: Vec<usize>,
    continue_breaks: Vec<usize>,
}

impl LoopLocations {
    pub fn new() -> Self {
        Self {
            start_location: 0,
            exit_location: 0,
            continue_location: 0,
            breaks: Vec::new(),
            continue_breaks: Vec::new(),
        }
    }
}

pub struct IrGenerator {
    string_pool: Vec<String>,
    strings_index: usize,

    scope: usize,
    offset: usize,
    symbol_loc: Vec<Symbols>,

    loop_stack: Vec<LoopLocations>,
    loop_count: usize,

    functions: Vec<Function>,
    func_ptr: usize,
}

pub fn generate(node: &Node) -> String {
    let mut generator = IrGenerator::new(node);
    generator.generate_code(node);
    format!("{}", generator)
}

impl Display for IrGenerator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Write out the constants
        writeln!(f, ".strings {}", self.string_pool.len())?;
        for s in self.string_pool.iter() {
            writeln!(f, "    {}", s)?;
        }

        writeln!(f, ".subs {}", self.functions.len())?;
        for (i, func) in self.functions.iter().enumerate() {
            // Get the byte count of th
            let bytes = writeln!(
                f,
                ".sub {}:{i} arity:{} slots:{} lines:{} bytes:{}",
                func.name,
                func.arity,
                func.slots,
                func.instructions.len(),
                func.calculate_bytes(),
            )?;
            for line in func.instructions.iter() {
                writeln!(f, "     {line}")?;
            }
        }
        writeln!(f, ".start")?;
        writeln!(f, "     call 0")?;
        writeln!(f, "     halt")?;
        writeln!(f, "")
    }
}

impl IrGenerator {
    pub fn new(node: &Node) -> Self {
        let func = Function {
            name: "main".to_string(),
            arity: 0,
            instructions: vec![],
            current_location: 0,
            slots: 0,
            constructed: false,
        };

        Self {
            string_pool: Vec::new(),
            strings_index: 0,
            scope: 0,
            offset: 0,
            symbol_loc: vec![Symbols::new()],
            loop_stack: Vec::new(),
            loop_count: 0,
            functions: vec![func],
            func_ptr: 0,
        }
    }

    fn current_function(&mut self) -> &mut Function {
        let f_ptr = self.func_ptr;
        &mut self.functions[f_ptr]
    }

    fn current_location(&mut self) -> &mut usize {
        &mut self.current_function().current_location
    }

    fn add_slot(&mut self) {
        self.current_function().slots += 1;
    }

    /// Clear the instructions. This is useful for REPLs where we're keeping a reference to the
    /// generator, but we need to clear the instructions before each run
    pub fn clear(&mut self) {
        self.current_function().instructions.clear()
    }

    /// Get current loop location struct
    fn get_loop_locations(&mut self) -> &mut LoopLocations {
        let cur_loop_location = self.loop_count - 1;
        &mut self.loop_stack[cur_loop_location]
    }

    fn push_loop(&mut self, loop_loc: LoopLocations) {
        self.loop_stack.push(loop_loc);
        self.loop_count += 1;
    }

    fn pop_loop(&mut self) {
        self.loop_stack.pop();
        self.loop_count -= 1;
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
            start_location: *self.current_location(),
            instruction_size: size,
            code: instruction.to_string(),
            jumped: false,
        };

        let func = self.current_function();
        func.current_location += size;
        func.instructions.push(instr);
    }

    /// Pass in a function name. If the function exists in the function
    /// registry, then pass the index on. If it doesn't then that means that
    /// it hasn't been created yet, but it's being referred to, so in that
    /// case we make a temporary registration so that the call code can be
    /// generated. Later, in a subsequent pass, we'll be able to tell if
    /// calls are being made to non-existent functions
    fn get_function_index<T: ToString>(&mut self, function_name: T) -> usize {
        let f_name = function_name.to_string();
        if let Some(position) = self.functions.iter().position(|f| f.name == f_name) {
            position
        } else {
            let function = Function {
                name: f_name,
                arity: 0,
                instructions: vec![],
                current_location: 0,
                slots: 0,
                constructed: false,
            };
            self.functions.push(function);
            self.func_ptr += 1;
            self.func_ptr
        }
    }

    pub fn generate(&mut self, node: &Node) {
        self.clear();
        self.generate_code(node);
    }

    fn generate_code(&mut self, node: &Node) {
        macro_rules! instr {
            ($instr:expr) => {
                self.push(format!("{}", $instr), 1);
            };

            ($instr:expr, $operand:expr, $len:expr) => {
                self.push(format!("{} {}", $instr, $operand), 1 + $len);
            };

            ($instr:expr, $operand:expr, $len:expr, $comment:expr) => {
                self.push(format!("{} {} ; {}", $instr, $operand, $comment), 1 + $len);
            };
        }

        macro_rules! get_instr_loc {
            () => {
                self.current_function().instructions.len() - 1
            };
        }

        match node.clone().node_type {
            NodeType::Integer(value) => {
                instr!("push", value, 9);
            }
            NodeType::Float(value) => {
                instr!("push", value, 9);
            }
            NodeType::Text(value) => {
                let loc = self.get_string_location(&*value);
                instr!("spush", loc, 5);
            }
            NodeType::Boolean(value) => {
                instr!("bpush", value as u8, 2);
            }

            NodeType::Break => {
                // Are we in a loop?
                if self.loop_count > 0 {
                    instr!("jmp", 0, 4, "inner break");
                    let loc = get_instr_loc!();
                    self.get_loop_locations().breaks.push(loc);
                }
            }

            NodeType::Continue => {
                // Are we in a loop?
                if self.loop_count > 0 {
                    instr!("jmp", 0, 4, "inner continue");
                    let loc = get_instr_loc!();
                    self.get_loop_locations().continue_breaks.push(loc);
                }
            }

            NodeType::Block => {
                self.push_scope();
            }

            NodeType::EndBlock => {
                self.pop_scope();
            }

            NodeType::While => {
                self.push_loop(LoopLocations::new());

                // Generate conditions
                for child in &node.children {
                    match &child.node_type {
                        NodeType::Conditional => {
                            let loc = *self.current_location();
                            self.get_loop_locations().start_location = loc;

                            for c in &child.children {
                                self.generate_code(c);
                            }
                            instr!("jmpfalse", 0, 4);
                            self.get_loop_locations().exit_location = get_instr_loc!();
                        }

                        //NodeType::EndWhile => {}
                        NodeType::CodeBlock => {
                            // This is the body of the while statement
                            for c in &child.children {
                                self.generate_code(c);
                            }

                            let loc = self.get_loop_locations().start_location;
                            instr!("jmp", loc, 4);

                            let cur_loc = *self.current_location();
                            let instr_loc = self.get_loop_locations().exit_location;
                            self.current_function().instructions[instr_loc].code =
                                format!("jmpfalse {}", cur_loc);

                            let breaks = self.get_loop_locations().clone().breaks;
                            let continues = self.get_loop_locations().clone().continue_breaks;

                            for i in continues {
                                self.current_function().instructions[i].code =
                                    format!("jmp {} ; # continue", loc);
                            }

                            for i in breaks {
                                self.current_function().instructions[i].code =
                                    format!("jmp {} ; # break", cur_loc);
                            }
                        }
                        _ => {
                            continue;
                        }
                    }
                }
                self.pop_loop();
            }

            NodeType::Function(func_name) => {
                if self.scope > 0 {
                    // panic!("functions can only be created at the top level")
                }

                let index = self.get_function_index(func_name.clone());
                self.func_ptr = index;

                self.push_scope();
                for child in &node.children {
                    match &child.node_type {
                        NodeType::Ident(name) => {
                            self.current_function().name = *name.clone();
                        }
                        NodeType::Params => {
                            for param in child.children.clone() {
                                if let NodeType::Ident(var) = param.node_type {
                                    let loc = self.store_variable(&var);
                                    self.current_function().arity += 1;
                                }
                            }
                        }
                        NodeType::CodeBlock => {
                            for ch in &child.children {
                                self.generate_code(&ch);
                            }
                            if index == 0 {
                                instr!("halt");
                            } else {
                                instr!("return");
                            }
                        }
                        _ => {
                            panic!("Unexpected child  for a function");
                        }
                    }
                }
                self.current_function().constructed = true;
                self.func_ptr = 0;
                self.pop_scope();
            }

            NodeType::For => {
                self.push_loop(LoopLocations::new());

                let mut iter_var_location = 0;

                // Hidden variable containing the name of the target condition
                let mut iter2_var_location: usize = 0;

                for child in &node.children {
                    match &child.node_type {
                        NodeType::CodeBlock => {
                            let loc = *self.current_location();
                            self.get_loop_locations().start_location = loc;

                            instr!("load", iter_var_location, 2, "Load the start");
                            instr!("load", iter2_var_location, 2, "Load the target");

                            // This takes up 2 slots in the function
                            self.current_function().slots += 2;

                            instr!("ge");

                            instr!("jmpfalse", 0, 4);
                            self.get_loop_locations().exit_location = get_instr_loc!();
                            for ch in &child.children {
                                self.generate_code(&ch);
                            }

                            let continue_loc = *self.current_location();
                            self.get_loop_locations().continue_location = continue_loc;

                            instr!("load", iter_var_location, 2, "Start incr");
                            instr!("push", 1, 9);
                            instr!("add");
                            instr!("store", iter_var_location, 2);
                        }
                        NodeType::Range => {
                            for ch in &child.children {
                                self.generate_code(ch);
                            }
                            instr!("store", iter2_var_location, 2);
                            instr!("store", iter_var_location, 2);
                        }
                        NodeType::Ident(iter_name) => {
                            // Name of the iteration variable
                            iter_var_location = self.store_variable(&iter_name);
                            iter2_var_location = self.store_variable("$2");
                        }
                        NodeType::EndFor => {
                            let loc = self.get_loop_locations().start_location;
                            instr!("jmp", loc, 4);

                            let cur_loc = *self.current_location();
                            let instr_loc = self.get_loop_locations().exit_location;
                            self.current_function().instructions[instr_loc].code =
                                format!("jmpfalse {}", cur_loc);

                            let breaks = self.get_loop_locations().clone().breaks;
                            for i in breaks {
                                self.current_function().instructions[i].code =
                                    format!("jmp {} ; break", cur_loc);
                            }

                            let loc = self.get_loop_locations().continue_location;
                            let continues = self.get_loop_locations().clone().continue_breaks;
                            for i in continues {
                                self.current_function().instructions[i].code =
                                    format!("jmp {} ; continue", loc);
                            }
                        }
                        _ => {
                            continue;
                        }
                    }
                }
                self.pop_loop();
            }

            NodeType::BinaryOp(op) => {
                for child in &node.children {
                    self.generate_code(child);
                }
                if op == BinOp::Assign {
                    return;
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
                self.add_slot();
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
                    instr!("store", location, 2, format!("store to '{var_name}'"));
                }
            }
            NodeType::Print => {
                for c in &node.children {
                    self.generate_code(c);
                }
                instr!("print");
            }

            NodeType::Call(function_name) => {
                // Push the parameters on to the stack
                for child in &node.children {
                    self.generate_code(&child);
                }
                // get the function index
                let index = self.get_function_index(*function_name);
                instr!("call", index, 2);
            }

            NodeType::Ident(name) => {
                let index = self.get_variable(&name);
                let mut is_array = false;
                let can_assign = !node.can_assign;

                for child in &node.children {
                    is_array = true;
                    match child.node_type {
                        NodeType::ArrayElement => {
                            self.generate_code(child.children.first().unwrap());
                            if node.can_assign {
                                instr!("astore", index, 2);
                            } else {
                                instr!("index", index, 2);
                            }
                        }
                        _ => {}
                    }
                }
                if is_array {
                    return;
                }

                if node.can_assign {
                    instr!("store", index, 2);
                } else {
                    instr!("load", index, 2);
                }
            }
            // We don't need to capture the internal elements here because we're drilling
            // down into the elements
            NodeType::Array => {
                for child in node.children.iter().rev() {
                    self.generate_code(child);
                }
                let element_count = &node.children.len();
                instr!("newarray", element_count, 2);
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
                            instr!("jmpfalse", 0, 4);
                            jmp_false_loc = get_instr_loc!();
                        }

                        NodeType::Else => {
                            has_else = true;
                            instr!("jmp", 0, 4);
                            jmp_true_loc = self.current_function().instructions.len() - 1;
                            self.current_function().instructions[jmp_false_loc].code =
                                format!("jmpfalse {}", self.current_function().current_location);
                            for c in &child.children {
                                self.generate_code(c);
                            }
                        }
                        NodeType::EndIf => {
                            if has_else {
                                let loc = self.current_function().current_location;
                                self.current_function().instructions[jmp_true_loc].code =
                                    format!("jmp {loc}");
                            }
                        }
                        NodeType::CodeBlock => {
                            // This is the body of the IF statement
                            for c in &child.children {
                                self.generate_code(c);
                            }

                            self.current_function().instructions[jmp_false_loc].code =
                                format!("jmpfalse {}", self.current_function().current_location);
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
            }
            _ => {
                println!(".end")
            }
        }
    }
}
