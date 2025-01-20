#![allow(dead_code)]

use crate::heap::Heap;
use crate::{
    constants::Instruction,
    constants::Instruction::*,
    valuetypes::{DataTag, Object, Value},
};
use rand::{random, random_range};
use std::{result, usize};

pub struct Vm {
    stack: [Object; 64000],
    sp: usize,
    heap: Heap,
    pub code: Vec<u8>,
    string_pool: Vec<String>,
    ip: usize,
}

impl Vm {
    pub fn new() -> Self {
        let obj = Object {
            tag: DataTag::Nil,
            data: Value { byte: 0 },
        };

        Self {
            stack: [obj; 64000],
            sp: 16,
            code: Vec::new(),
            heap: Heap::new(),
            string_pool: Vec::new(),
            ip: 0,
        }
    }

    fn get_instruction(&mut self) -> Instruction {
        let byte = self.code[self.ip];
        self.ip += 1;
        Instruction::from_u8(byte)
    }

    fn get_data(&mut self) -> Value {
        let bytes: [u8; 8] = self.code[self.ip..self.ip + 8].try_into().unwrap();
        self.ip += 8;
        Value { bytes }
    }

    fn get_integer(&mut self) -> usize {
        self.ip += 1;
        let bytes: [u8; 8] = self.code[self.ip..self.ip + 8].try_into().unwrap();

        self.ip += 8;
        f64::from_le_bytes(bytes) as usize
    }

    fn get_byte(&mut self) -> u8 {
        let byte = self.code[self.ip];
        self.ip += 8;
        byte
    }

    fn pop(&mut self) -> Object {
        self.sp -= 1;
        self.stack[self.sp]
    }

    fn push(&mut self, obj: Object) {
        self.stack[self.sp] = obj;
        self.sp += 1;
    }

    /// Gets the value that exists following the `const` instruction
    fn get_const(&mut self) -> Object {
        // The const has already been consumed so the next byte tells us the type
        let tag = DataTag::from(self.code[self.ip]);
        self.ip += 1;
        Object {
            tag,
            data: self.get_data(),
        }
    }

    /// Loads constants from the ASM file that need to go into the string pool
    fn load_string_pool(&mut self) {
        self.string_pool.clear();
        let num_constants = u32::from_le_bytes(self.code[0..4].try_into().unwrap());
        self.ip += 4;
        for _ in 0..num_constants {
            let s_len =
                u32::from_le_bytes(self.code[self.ip..self.ip + 4].try_into().unwrap()) as usize;
            self.ip += 4;
            let s_val = String::from_utf8(self.code[self.ip..self.ip + s_len].to_vec()).unwrap();
            self.string_pool.push(s_val);
            self.ip += s_len;
        }
    }

    /// Read the number of constants and use that as the basis for the starting position on the
    /// stack
    fn load_globals(&mut self) -> usize {
        let start = self.ip;
        let num_constants = u32::from_le_bytes(self.code[start..(start + 4)].try_into().unwrap());
        self.ip += 4;
        num_constants as usize
    }

    pub fn run(&mut self) {
        //println!("Loading constants");
        self.ip = 0;
        self.load_string_pool();

        let globals = self.load_globals();
        //println!("Globals: {globals}");

        //println!("Executing code ..");

        macro_rules! binop {
            ($op:tt) => {

                let left = self.pop();
                let right = self.pop();

                let left_tag = left.tag;
                let right_tag = right.tag;

                let ldata = left.data.as_float();
                let rdata = right.data.as_float();

                let result_value:f64 = ldata $op rdata;

                let result_tag = match (left_tag, right_tag) {
                    (DataTag::Integer, DataTag::Integer) => DataTag::Integer,
                    (DataTag::Float, _) | (_, DataTag::Float) => DataTag::Float,
                    _ => panic!("Tag combination {:?} and {:?} are not allowed", left_tag, right_tag),
                };

                let result = Object {
                    tag: result_tag,
                    data: Value { f: result_value },
                };

                self.push(result);
            };
        }

        //println!("\nVM Debug");
        //println!("--------");
        loop {
            let b = self.get_instruction();
            //println!("{} ", b.as_str());
            match b {
                Push => {
                    let obj = self.get_const();
                    self.push(obj);
                }

                Add => {
                    binop!(+);
                }

                Sub => {
                    binop!(-);
                }

                Mul => {
                    binop!(*);
                }

                Div => {
                    binop!(/);
                }

                Neg => {}

                Newarray => {}
                Store => {
                    let slot = self.get_integer();
                    self.stack[slot as usize] = self.pop();
                }
                Load => {
                    let slot = self.get_integer();
                    let obj = self.stack[slot as usize];
                    self.push(obj);
                }
                Pop => {}

                Print => {
                    self.print();
                }

                Halt => {
                    //println!("\n{}", self.pop());
                    break;
                }
                _ => {
                    println!("Unknown instruction: {}", b.as_str());
                    break;
                }
            }

            //for i in ((sp + 16)..(starting_sp + 26)) {
            //    print!("[{}] ", self.stack[i]);
            //}
            //println!();
        }

        //println!("{}", self.registers[0].as_integer());
    }

    fn print(&mut self) {
        let value = self.pop();
        println!("{}", value);
    }
}

pub fn execute(bytecode: Vec<u8>) {
    let mut vm = Vm::new();
    vm.code = bytecode;
    vm.run();
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vm::Vm;
    #[test]
    fn test_vm() {}

    #[test]
    fn test_vm_let() {}
}
