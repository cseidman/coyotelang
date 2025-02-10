#![allow(dead_code)]
use cvm::constants::Instruction;
use cvm::constants::Instruction::*;
use cvm::valuetypes::DataTag;
use regex::Regex;

#[repr(u8)]
enum HeaderType {
    StringPool = 0x00,
    Function = 0x01,
    Procedure = 0x02,
}

pub struct SubRoutine {
    name: String,
    location: u32,
    arity: u8,
    slots: u8,
    code: Vec<u8>,
    byte_size: usize,
}

pub struct StringEntry {
    string_bytes: Vec<u8>,
}

impl StringEntry {
    pub fn new<T: ToString>(string_bytes: T) -> Self {
        Self {
            string_bytes: string_bytes.to_string().into_bytes(),
        }
    }
}

pub struct Assembly {
    sub_count: u32,
    sub_routines: Vec<SubRoutine>,

    pool_count: u32,
    string_pool: Vec<StringEntry>,
}

impl Assembly {
    pub fn new() -> Self {
        Self {
            sub_count: 0,
            sub_routines: vec![],
            pool_count: 0,
            string_pool: vec![],
        }
    }

    pub fn to_bytecode(&self) -> Vec<u8> {
        let mut output = Vec::new();

        // 1) Write the subroutine count
        push_u32(&mut output, self.sub_count as u32);

        // 2) Write each subroutine (function/procedure)
        for sub in &self.sub_routines {
            write_subroutine(&mut output, sub);
        }

        // 3) Write the string pool count
        push_u32(&mut output, self.pool_count);

        // 4) Write each string entry
        for entry in &self.string_pool {
            write_string_entry(&mut output, entry);
        }

        // Done! Return the final byte array
        output
    }
}

fn push_u32(buf: &mut Vec<u8>, value: u32) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn push_u8(buf: &mut Vec<u8>, value: u8) {
    buf.push(value);
}

fn write_string_entry(buf: &mut Vec<u8>, entry: &StringEntry) {
    // First byte: type
    //push_u8(buf, HeaderType::StringPool as u8);

    // Then the length of the string
    let length = entry.string_bytes.len() as u32;
    push_u32(buf, length);

    // Then the actual string bytes
    buf.extend_from_slice(&entry.string_bytes);
}

fn write_subroutine(buf: &mut Vec<u8>, sub: &SubRoutine) {
    // If you eventually store sub.sub_type, you can push that instead
    //push_u8(buf, HeaderType::Function as u8);

    // Name
    //let name_len = sub.name.len() as u32;
    //push_u32(buf, name_len);
    //buf.extend_from_slice(sub.name.as_bytes());

    // location
    push_u32(buf, sub.location);

    // arity
    push_u8(buf, sub.arity);

    // slots
    push_u8(buf, sub.slots);

    // code
    let code_len = sub.code.len() as u32;
    push_u32(buf, code_len);
    buf.extend_from_slice(&sub.code);
}

pub fn assemble(source: &str) -> Vec<u8> {
    let mut assembly: Assembly = Assembly::new();
    // Iterator over the lines as we're going to evaluate the ASM code
    // line by line
    let mut lines = source.lines();

    while let Some(line) = lines.next() {
        if line.starts_with("#") {
            continue;
        }
        // Get the strings in the string pool in the header
        let rgx = Regex::new(r"^\.strings (?<strings_count>\d+)").unwrap();

        if let Some(number_of_strings) = rgx.captures(line) {
            let sub_list = number_of_strings["strings_count"].parse::<usize>().unwrap();
            for _ in 0..sub_list {
                let line_val = lines.next().unwrap();
                assembly.string_pool.push(StringEntry::new(line_val));
                assembly.pool_count += 1;
            }
            continue;
        }
        // Get the subroutines in the program
        let rgx = Regex::new(r"\.subs (?<sub_count>\d+)").unwrap();
        if let Some(number_of_subs) = rgx.captures(line) {
            // This tells us how many subroutines to expect
            let sub_list = number_of_subs["sub_count"].parse::<usize>().unwrap();
            assembly.sub_count = sub_list as u32;
            // Assemble each subroutine
            for _ in 0..sub_list {
                let line = lines.next().unwrap();
                let rgx = Regex::new(
                    r"\.sub (?<fn>\w+):(?<pos>\d+) arity:(?<arity>\d+) slots:(?<slots>\d+) lines:(?<lines>\d+) bytes:(?<bytes>\d+)",
                )
                .unwrap();
                let function_data = rgx.captures(line).unwrap();

                let mut sub = SubRoutine {
                    name: function_data["fn"].to_string(),
                    location: function_data["pos"].parse::<u32>().unwrap(),
                    arity: function_data["arity"].parse::<u8>().unwrap(),
                    slots: function_data["slots"].parse::<u8>().unwrap(),
                    code: vec![],
                    byte_size: function_data["bytes"].parse::<usize>().unwrap(),
                };

                let code_lines = function_data["lines"].parse::<usize>().unwrap();
                // Convert the code to instructions
                sub.code = (0..code_lines)
                    .map(|_| {
                        let line = lines.next().unwrap();
                        let re = Regex::new(r"(\d+ \|)").unwrap();
                        let line = re.replace_all(line, "");
                        assemble_to_code(line.trim())
                    })
                    .flatten()
                    .collect::<Vec<u8>>();

                assembly.sub_routines.push(sub);
            }

            continue;
        }
    }

    let bytecode = assembly.to_bytecode();
    bytecode
}

fn assemble_to_code(code: &str) -> Vec<u8> {
    let mut bytecode: Vec<u8> = Vec::new();

    let mut elements = code.split_whitespace();

    // Write the instruction
    if let Some(asm_instruction) = elements.next() {
        if let Some(byte) = Instruction::match_instruction(asm_instruction) {
            bytecode.push(byte as u8);

            if let Some(operand) = elements.next() {
                match Instruction::from_u8(byte as u8) {
                    Push => {
                        bytecode.push(DataTag::Float as u8);
                        let value = operand.parse::<f64>().unwrap();
                        bytecode.append(&mut value.to_le_bytes().to_vec());
                    }
                    BPush => {
                        bytecode.push(DataTag::Bool as u8);
                        let value = operand.parse::<u8>().unwrap();
                        bytecode.push(value);
                    }
                    SPush => {
                        bytecode.push(DataTag::Text as u8);
                        let value = operand.parse::<u32>().unwrap();
                        bytecode.append(&mut value.to_le_bytes().to_vec());
                    }
                    Load | Store | AStore | Call | NewArray | Index => {
                        let value = operand.parse::<u16>().unwrap();
                        bytecode.append(&mut value.to_le_bytes().to_vec());
                    }
                    JmpFalse | Jmp => {
                        let value = operand.parse::<i32>().unwrap();
                        bytecode.append(&mut value.to_le_bytes().to_vec());
                    }
                    _ => {
                        //bytecode.push(DataTag::Integer as u8);
                        //let value = operand.parse::<i64>().unwrap();
                        //bytecode.append(&mut value.to_le_bytes().to_vec());
                    }
                }
            }
        }
    }

    bytecode
}
