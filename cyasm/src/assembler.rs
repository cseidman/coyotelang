#![allow(dead_code)]
use cvm::constants::*;
use cvm::valuetypes::DataTag;
use std::iter::Peekable;
use std::str::{Chars, FromStr};

struct AsmParser<'a> {
    asm: Peekable<Chars<'a>>,
    line: usize,
    bytecode: Vec<u8>,
}

impl<'a> AsmParser<'a> {
    pub fn new(asm: &'a str) -> Self {
        Self {
            asm: asm.chars().peekable(),
            line: 1,
            bytecode: Vec::new(),
        }
    }

    pub fn make_number_string(&mut self) -> String {
        let mut s = String::new();

        while let Some(&d) = self.asm.peek() {
            if d.is_numeric() || d == '.' {
                s.push(d);
                self.advance();
            } else {
                break;
            }
        }
        s
    }

    pub fn advance(&mut self) -> Option<char> {
        self.asm.next()
    }

    pub fn match_string(&mut self, value: &str) -> bool {
        let n = value.len();
        if value == self.asm.clone().take(n).collect::<String>() {
            (0..n).for_each(|_| {
                self.advance();
            });
            true
        } else {
            false
        }
    }

    fn get_line(&mut self) -> String {
        let mut line = String::new();
        while let Some(&c) = self.asm.peek() {
            if c == '\n' {
                self.advance();
                break;
            }
            line.push(c);
            self.advance();
        }
        line
    }

    fn write_global_slots(&mut self) {
        self.get_line(); // clear the .globals tag
        let line = self.get_line();
        let gcount = line.trim();
        let global_count = gcount.parse::<u32>().unwrap();
        self.bytecode
            .append(&mut global_count.to_le_bytes().to_vec());
    }
    fn write_constants(&mut self) {
        // Load all the constants after the .constants directive
        let mut const_count: u32 = 0;
        loop {
            let line = self.get_line();
            let line = line.trim();
            if line == ".end" {
                break;
            }
            if line == "" {
                continue;
            }

            // Add the length of the string as a 4 byte integer
            let constant = line;
            let mut const_bytes = constant.as_bytes().to_vec();
            let const_len: u32 = const_bytes.len() as u32;
            self.bytecode.append(&mut const_len.to_le_bytes().to_vec());

            // Add the string
            self.bytecode.append(&mut const_bytes);
            const_count += 1;
        }
        //println!("Constant count: {}", const_count);
        // Add the count of constants as the first 4 bytes
        for b in const_count.to_be_bytes().to_vec() {
            self.bytecode.insert(0, b);
        }
    }

    pub fn assemble(&mut self) -> Vec<u8> {
        while let Some(&c) = self.asm.peek() {
            if self.match_string(".constants") {
                self.write_constants();
                continue;
            }

            if self.match_string(".globals") {
                self.write_global_slots();
                continue;
            }

            if c == '\n' {
                self.advance();
                self.line += 1;
                continue;
            }

            if c.is_whitespace() || c == ',' || c == ';' {
                self.advance();
                continue;
            }

            if c == '"' {
                let mut str_val = String::new();
                self.advance();
                while let Some(&chr) = self.asm.peek() {
                    self.advance();
                    if chr == '"' {
                        break;
                    } else {
                        str_val.push(chr)
                    }
                }
                continue;
            }

            if c.is_alphabetic() {
                self.advance();
                let mut s = String::new();
                s.push(c);

                while let Some(&c) = self.asm.peek() {
                    if c.is_alphabetic() {
                        s.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }

                if let Some(byte) = self.match_keyword(&s) {
                    self.emit_command(byte as u8);
                }

                continue;
            }

            if c.is_ascii_digit() {
                let s = self.make_number_string();

                if s.contains('.') {
                    self.bytecode.push(DataTag::Float as u8);
                } else {
                    self.bytecode.push(DataTag::Integer as u8);
                };
                let num = s.parse::<f64>().unwrap().to_le_bytes();
                self.emit_operand(num);
                continue;
            }
        }
        self.bytecode.push(0x00);
        self.bytecode.clone()
    }

    fn emit_command(&mut self, code: u8) {
        self.bytecode.push(code);
    }

    fn emit_operand(&mut self, operand: [u8; 8]) {
        self.bytecode.append(&mut operand.to_vec());
    }

    fn emit_string_operand(&mut self, operand: String) {
        let mut bytes = operand.as_bytes().to_vec();
        self.bytecode.append(&mut bytes);
    }

    fn match_keyword(&self, keyword: &str) -> Option<Instruction> {
        Instruction::match_instruction(keyword)
    }
}

pub fn assemble(asm: &str) -> Vec<u8> {
    let mut parser = AsmParser::new(asm);
    parser.assemble()
}
