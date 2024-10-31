#![allow(dead_code)]
use cvm::constants::*;
use std::iter::Peekable;
use std::str::Chars;

struct Parser<'a> {
    asm: Peekable<Chars<'a>>,
    line: usize,
    bytecode: Vec<u8>,
}

impl<'a> Parser<'a> {
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

    pub fn assemble(&mut self) -> Vec<u8> {
        while let Some(&c) = self.asm.peek() {
            if c == '\n' {
                self.advance();
                self.line += 1;
                continue;
            }

            if c.is_whitespace() || c == ',' || c == ';' {
                self.advance();
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

                if let Some(byte) = self.match_keyword(s.clone()) {
                    self.emit_command(byte);
                    continue;
                }
            }

            if c.is_ascii_digit() {
                let s = self.make_number_string();

                let num = if s.contains('.') {
                    s.parse::<f64>().unwrap().to_le_bytes()
                } else {
                    s.parse::<i64>().unwrap().to_le_bytes()
                };
                self.emit_operand(num);
                continue;
            }

            match c {
                '%' => {
                    self.advance();
                    if let Some(&chr) = self.asm.peek() {
                        self.advance();
                        match chr {
                            'r' | 'm' => {
                                let s = self.make_number_string();
                                self.emit_register(s.parse::<u16>().unwrap());
                            }
                            _ => {
                                panic!("Invalid character: {}", chr);
                            }
                        }
                    }
                }
                _ => {
                    //println!("Parsing character: {}", c);
                }
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

    fn emit_register(&mut self, reg_number: u16) {
        let mut reg = reg_number.to_le_bytes().to_vec();
        self.bytecode.append(&mut reg);
    }

    fn match_keyword(&self, keyword: String) -> Option<u8> {
        match keyword.as_str() {
            "imov" => Some(IMOV),
            "iadd" => Some(IADD),
            "isub" => Some(ISUB),
            "imul" => Some(IMUL),
            "idiv" => Some(IDIV),
            "iequ" => Some(IEQU),
            "fmov" => Some(FMOV),
            "fadd" => Some(FADD),
            "fsub" => Some(FSUB),
            "fmul" => Some(FMUL),
            "fdiv" => Some(FDIV),
            "idec" => Some(IDEC),
            "cmp" => Some(CMP),
            "iinc" => Some(IINC),
            "store" => Some(STORE),
            "istore" => Some(ISTORE),
            "load" => Some(LOAD),
            "print" => Some(PRINT),
            "ineg" => Some(INEG),
            _ => None,
        }
    }
}

pub fn assemble(asm: &str) -> Vec<u8> {
    let mut parser = Parser::new(asm);
    parser.assemble()
}

/*
Language grammar:
    <instruction> <operand1>, <operand2> ;
    <operand> ::= <register> | <immediate> | <memory>
    <register> ::= %r[0-9]+
    <immediate> ::= [0-9]+
    <memory> ::= %m[0-9]+
    <instruction> ::= mov | add | sub | mul | div | equ | cmp | inc | dec
 */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_asm_parser() {
        let asm = r#"
        imov %r0, 4 ;
        imov %r1, 3 ;
        imov %r2, 2 ;
        imul %r1, %r2 ;
        imov %r2, 1 ;
        iadd %r1, %r2 ;
        iadd %r0, %r1 ;
        istore %r3, %r0 ;
        imov %r4, 10 ;
        iadd %r4, %r3 ;
        "#;
        let mut parser = Parser::new(asm);
        let byte_code = parser.assemble();
        let expected = vec![
            IMOV, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, IMOV, 1, 0, 3, 0, 0, 0, 0, 0, 0, 0, IMOV, 2, 0, 2,
            0, 0, 0, 0, 0, 0, 0, IMUL, 1, 0, 2, 0, IMOV, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, IADD, 1, 0,
            2, 0, IADD, 0, 0, 1, 0, ISTORE, 3, 0, 0, 0, IMOV, 4, 0, 10, 0, 0, 0, 0, 0, 0, 0, IADD,
            4, 0, 3, 0, 0,
        ];
        assert_eq!(byte_code, expected);
    }
}
