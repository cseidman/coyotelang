#![allow(dead_code)]

use crate::constants::*;
#[derive(Copy, Clone)]
union Value {
    i: i64,
    f: f64,
    bytes: [u8; 8],
}
impl Value {
    pub fn as_integer(&self) -> i64 {
        unsafe { self.i }
    }
    pub fn as_float(&self) -> f64 {
        unsafe { self.f }
    }

    pub fn as_bytes(&self) -> [u8; 8] {
        unsafe { self.bytes }
    }
}

type Register = Value;

struct Vm {
    // Registers
    registers: [Register; 64000],
    code: Vec<u8>,
    ip: usize,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            registers: [Value { i: 0 }; 64000],
            code: Vec::new(),
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

    pub fn run(&mut self) {
        macro_rules! ibinop {
            ($op:tt) => {
                let reg1 = self.get_register_location();
                let reg2 = self.get_register_location();

                let rval = self.registers[reg1].as_integer();
                let lval = self.registers[reg2].as_integer();
                self.registers[reg1].i = lval $op rval;
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

        loop {
            let b = self.get_instruction();

            match b {
                STORE => {
                    let reg = self.get_register_location();
                    let value = self.read_register_value();
                    self.registers[reg] = value;
                    //print!("store {}, {}\t", reg, value.as_integer());
                }
                ISTORE => {
                    let reg = self.get_register_location();
                    let value = self.read_register_value();
                    self.registers[reg] = value;
                    //print!("store {}, {}\t", reg, value.as_integer());
                }
                LOAD => {
                    let reg = self.get_register_location();
                    let value = self.read_register_value();
                    self.registers[reg] = value;
                    //print!("load {}, {}\t", reg, value.as_integer());
                }
                IMOV => {
                    let reg = self.get_register_location();
                    let value = self.get_data().as_integer();
                    self.registers[reg].i = value;
                    //print!("imov {}, {}\t", reg, value);
                }
                FMOV => {
                    let reg = self.get_register_location();
                    let value = self.get_data().as_float();
                    self.registers[reg].f = value;
                    //print!("imov {}, {}\t", reg, value);
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
                HALT => {
                    //println!("HALT\t");
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
        println!("{}", self.registers[0].as_integer());
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
            2, 0, IADD, 0, 0, 1, 0, 0,
        ];
        vm.run();
        //for i in 0..4 {
        //    println!("Register {}: {:?}", i, vm.registers[i].as_integer());
        //}
        println!("{}", vm.registers[0].as_integer());
        assert_eq!(vm.registers[0].as_integer(), 11);
    }
}
