#![allow(dead_code)]

use crate::heap::Heap;
use crate::{constants::*, valuetypes::Value};
use std::collections::HashMap;

type Register = Value;

pub struct Vm {
    // Registers
    registers: [Register; 64000],
    heap: Heap,
    pub code: Vec<u8>,
    string_pool: Vec<String>,
    ip: usize,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            registers: [Value { i: 0 }; 64000],
            code: Vec::new(),
            heap: Heap::new(),
            string_pool: Vec::new(),
            ip: 0,
        }
    }

    fn get_instruction(&mut self) -> u8 {
        self.ip += 1;
        self.code[self.ip - 1]
    }

    fn read_register_value(&mut self) -> Register {
        let loc = u16::from_le_bytes(self.code[self.ip..self.ip + 1].try_into().unwrap());
        self.ip += 2;
        self.registers[loc as usize]
    }

    fn load_register(&mut self, reg: Register) {
        let loc = u16::from_le_bytes(self.code[self.ip..=self.ip + 2].try_into().unwrap());
        self.ip += 2;
        self.registers[loc as usize] = reg;
    }

    fn get_register_location(&mut self) -> usize {
        let loc = u16::from_le_bytes(self.code[self.ip..self.ip + 2].try_into().unwrap()) as usize;
        self.ip += 2;
        loc
    }

    fn get_data(&mut self) -> Value {
        let bytes: [u8; 8] = self.code[self.ip..self.ip + 8].try_into().unwrap();
        self.ip += 8;
        Value { bytes }
    }

    fn load_constants(&mut self) {
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

    pub fn run(&mut self) {
        println!("Loading constants");
        self.ip = 0;
        self.load_constants();
        println!("Executing code ..");

        macro_rules! ibinop {
            ($op:tt) => {
                let reg1 = self.get_register_location();
                let reg2 = self.get_register_location();

                let rval = self.registers[reg1].as_integer();
                let lval = self.registers[reg2].as_integer();
                self.registers[reg1].i = lval $op rval;
                println!("R{reg1}, R{reg2}; R{reg1}={lval}  R{reg2}={rval}");
            };
        }

        macro_rules! fbinop {
            ($op:tt) => {
                let reg1 = self.get_register_location();
                let reg2 = self.get_register_location();

                let rval = self.registers[reg1].as_float();
                let lval = self.registers[reg2].as_float();
                self.registers[reg1].f = lval $op rval;
            };
        }
        println!("\nVM Debug");
        println!("--------");
        loop {
            let b = self.get_instruction();
            print!("{} ", INSTRUCTIONS[b as usize]);
            match b {
                STORE => {
                    self.store();
                }

                LOAD => {
                    self.load();
                }

                ICONST => {
                    self.iconst();
                }

                FCONST => {
                    self.fconst();
                }

                SCONST => {
                    self.sconst();
                }
                AICONST => {
                    let reg = self.get_register_location();
                    let array_size = self.get_data().as_uint();
                    let mut array: HashMap<usize, Value> = HashMap::with_capacity(array_size);
                    for i in 0..array_size {
                        let value = self.get_data();
                        array.insert(i, value);
                    }
                    let ptr = self.heap.store(array);
                    self.registers[reg].ptr = ptr;
                }
                AFCONST => {
                    let reg = self.get_register_location();
                    let value = self.get_data().as_index();
                    self.registers[reg].index = value;
                }
                ASCONST => {
                    let reg = self.get_register_location();
                    let value = self.get_data().as_index();
                    self.registers[reg].index = value;
                }

                IADD => {
                    ibinop!(+);
                }
                FADD => {
                    fbinop!(+);
                }
                ISUB => {
                    ibinop!(-);
                }
                FSUB => {
                    fbinop!(-);
                }
                IMUL => {
                    ibinop!(*);
                }
                FMUL => {
                    fbinop!(*);
                }
                IDIV => {
                    ibinop!(/);
                }
                FDIV => {
                    fbinop!(/);
                }
                INEG => {
                    let reg = self.get_register_location();
                    self.registers[reg].i = -self.registers[reg].as_integer();
                }
                FNEG => {
                    let reg = self.get_register_location();
                    self.registers[reg].f = -self.registers[reg].as_float();
                }
                IPRINT => {
                    self.iprint();
                }
                IAPRINT => {
                    self.iaprint();
                }
                FPRINT => {
                    let reg = self.get_register_location();
                    let value = self.registers[reg].as_float();

                    println!("R{reg}");
                    println!("\t{value}");
                }
                SPRINT => {
                    let reg = self.get_register_location();
                    let index = self.registers[reg].as_index() as usize;
                    let value = self.string_pool[index].clone();

                    println!("R{reg}");
                    println!("\t{value}");
                }

                HALT => {
                    println!();
                    break;
                }
                _ => {
                    println!("Unknown instruction: {}", b);
                    break;
                }
            }

            //for i in 0..5 {
            //    print!("[{}]\t", self.registers[i].as_integer());
            //}
            //println!();
        }

        //println!("{}", self.registers[0].as_integer());
    }

    fn iprint(&mut self) {
        let reg = self.get_register_location();
        let value = self.registers[reg].as_integer();

        println!("R{reg}");
        println!("\t{value}");
    }

    fn iaprint(&mut self) {}

    fn sconst(&mut self) {
        // Get the target register
        let reg = self.get_register_location();
        // Get the index of the string in the string pool
        let index = self.get_data().as_index();
        // Get the string from the string pool

        self.registers[reg].index = index;
    }

    fn fconst(&mut self) {
        let reg = self.get_register_location();
        let value = self.get_data().as_float();
        self.registers[reg].f = value;
    }

    fn load(&mut self) {
        let reg = self.get_register_location();
        let value = self.get_register_location();
        self.registers[reg] = self.registers[value];
        println!("R{}, R{};", reg, value);
    }

    fn store(&mut self) {
        let reg = self.get_register_location();
        let value = self.get_register_location();
        self.registers[reg] = self.registers[value];
        println!("R{}, {};", reg, value);
    }
    fn iconst(&mut self) {
        let reg = self.get_register_location();
        let value = self.get_data().as_integer();
        self.registers[reg].i = value;
        println!("R{}, {};", reg, value);
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
    fn test_vm() {
        let mut vm = Vm::new();
        vm.code = vec![
            IMOV, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, IMOV, 1, 0, 3, 0, 0, 0, 0, 0, 0, 0, IMOV, 2, 0, 2,
            0, 0, 0, 0, 0, 0, 0, IMUL, 1, 0, 2, 0, IMOV, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, IADD, 1, 0,
            2, 0, IADD, 0, 0, 1, 0, HALT,
        ];
        vm.run();

        assert_eq!(vm.registers[0].as_integer(), 11);
    }

    #[test]
    fn test_vm_let() {
        let mut vm = Vm::new();
        vm.code = vec![
            STORE, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, STORE, 1, 0, 3, 0, 0, 0, 0, 0, 0, 0, IADD, 0, 0,
            1, 0, HALT,
        ];
        vm.run();

        assert_eq!(vm.registers[0].as_integer(), 7);
    }
}
