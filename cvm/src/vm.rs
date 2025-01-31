#![allow(dead_code, unused_macros)]

const STACK_SIZE: usize = 1_000_000;
const GLOBAL_SIZE: usize = 1024;
const FRAMES_DEPTH: usize = 1024;

use crate::ctable::Table;
use crate::{
    constants::Instruction,
    constants::Instruction::*,
    valuetypes::{DataTag, Object},
};
use std::cmp::{Ordering, PartialOrd};
use std::ops::Neg;
use std::usize;
#[derive(Debug, Clone)]
struct StackFrame<'a> {
    code: &'a [Object],
    ip: usize,
    sp: usize,
}

impl<'a> StackFrame<'a> {
    pub fn new(ip: usize, sp: usize) -> Self {
        Self { code: &[], ip, sp }
    }
}
#[derive(Debug, Clone)]
pub struct Vm<'a> {
    stack: Vec<Object>,
    sp: usize,
    pub code: Vec<u8>,
    string_pool: Vec<String>,
    ip: usize,

    stack_frame: Vec<StackFrame<'a>>,
    fp: usize,
}

impl<'a> Vm<'a> {
    pub fn new() -> Self {
        let obj = Object::Nil;

        Self {
            stack: vec![obj; STACK_SIZE],
            sp: GLOBAL_SIZE,
            code: Vec::new(),
            string_pool: Vec::new(),
            ip: 0,

            stack_frame: Vec::with_capacity(FRAMES_DEPTH),
            fp: 0,
        }
    }

    fn get_tag(&mut self) -> DataTag {
        let tag = DataTag::from(self.code[self.ip]);
        self.ip += 1;
        tag
    }

    fn get_data(&mut self) -> usize {
        let bytes: [u8; 8] = self.code[self.ip..self.ip + 8].try_into().unwrap();
        self.ip += 8;
        f64::from_le_bytes(bytes) as usize
    }

    fn push_frame(&mut self) {
        let frame = StackFrame::new(self.ip, self.sp);
        self.stack_frame.push(frame);
        self.ip = 0;
        self.sp += 1;
        self.fp += 1;
    }

    fn pop_frame(&mut self) {
        if let Some(frame) = self.stack_frame.pop() {
            self.ip = frame.ip;
            self.sp = frame.sp;
            self.sp -= 1;
        }
    }

    fn get_instruction(&mut self) -> Instruction {
        let ip = self.ip;
        let byte = self.code[ip];
        self.ip += 1;
        Instruction::from_u8(byte)
    }

    fn get_operand(&mut self) -> usize {
        self.ip += 1;
        let bytes: [u8; 8] = self.code[self.ip..self.ip + 8].try_into().unwrap();

        self.ip += 8;
        f64::from_le_bytes(bytes) as usize
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
        let sp = self.sp;
        self.stack[sp].clone()
    }

    fn push(&mut self, obj: Object) {
        self.stack[self.sp] = obj;
        self.sp += 1;
    }

    fn get_const(&mut self) -> Object {
        let tag = self.get_tag();
        let bytes: [u8; 8] = self.code[self.ip..self.ip + 8].try_into().unwrap();
        self.ip += 8;

        match tag {
            DataTag::Nil => Object::Nil,
            DataTag::Float => Object::Float(f64::from_le_bytes(bytes)),
            DataTag::Bool => Object::Bool(i64::from_le_bytes(bytes) != 0),
            DataTag::Integer => Object::Integer(f64::from_le_bytes(bytes) as i64),
            DataTag::Text => {
                let index = i64::from_le_bytes(bytes) as usize;
                let txt = self.string_pool[index].clone();
                Object::Str(txt)
            }
            _ => {
                panic!("Invalid constant tag: {:?}", tag);
            }
        }
    }

    fn get_string(&mut self) -> Object {
        let tag = self.get_tag();
        let index = self.get_data();
        let value = self.string_pool[index].clone();

        match tag {
            DataTag::Text => return Object::Str(value),
            _ => panic!("invalid constant tag"),
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
        self.ip = 0;

        self.load_string_pool();
        self.load_globals();
        // Clear out the bytes we already used and restart at 0
        self.code.drain(..self.ip);
        self.ip = 0;

        macro_rules! binop {
            ($op:tt) => {
                let left = self.pop();
                let right = self.pop();
                let obj = left $op right;

                self.push(obj);
            };
        }

        macro_rules! cmpop {
            ($op:tt) => {
                let left = self.pop();
                let right = self.pop();
                let val = left $op right;

                self.push(Object::Bool(val));
            };
        }

        macro_rules! boolop {
            ($op:tt) => {
                let right = if let Object::Bool(right_bool) = self.pop() {
                    right_bool
                } else {
                    panic!("not boolean")
                };
                let left = if let Object::Bool(left_bool) = self.pop() {
                    left_bool
                } else {
                    panic!("not boolean")
                };
                let val = left $op right;
                self.push(Object::Bool(val));
            };
        }

        macro_rules! vm_debug {
            () => {
                let ip = self.ip - 1;
                let b = self.code[ip] as u8;
                let instr = Instruction::from_u8(b);
                print!("{:05}: {} ", self.ip, instr.as_str());
                match instr {
                    Push | Store | Load | Jmp | JmpFalse | Newarray | SPool | Set => {
                        let bytes: [u8; 8] =
                            self.code[self.ip + 1..self.ip + 9].try_into().unwrap();
                        let opd = f64::from_le_bytes(bytes) as usize;
                        println!("{opd}");
                    }
                    _ => {
                        println!();
                    }
                }
            };
        }

        loop {
            let b = self.get_instruction();
            //vm_debug!();
            //sleep(Duration::from_millis(500));
            match b {
                Push => {
                    let obj = self.get_const();
                    self.push(obj);
                }

                SPool => {
                    let obj = self.get_const();
                    self.push(obj)
                }

                Call => {
                    self.push_frame();
                }

                Return => {
                    self.pop_frame();
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

                Eq => {
                    cmpop!(==);
                }

                Neq => {
                    cmpop!(!=);
                }

                Gt => {
                    cmpop!(>);
                }

                Ge => {
                    cmpop!(>=);
                }

                Lt => {
                    cmpop!(<);
                }

                Le => {
                    cmpop!(<=);
                }

                Set => {
                    let slot = self.get_operand();
                    self.stack[slot as usize] = self.pop();
                }

                Neg => {
                    let obj = self.pop();
                    self.push(obj.neg());
                }

                Newarray => {
                    let element_count = self.get_operand();
                    // Create the table as an array
                    let mut arr = Table::<Object>::new();

                    for _ in 0..element_count {
                        let obj = self.pop();
                        arr.push(obj)
                    }

                    let obj = Object::Array(Box::new(arr));

                    self.push(obj);
                }

                AStore => {
                    // Get the element index
                    let idx = if let Object::Integer(int) = &self.pop() {
                        *int as usize
                    } else {
                        panic!("not an integer");
                    };
                    // Get the new value
                    let value = self.pop().clone();

                    // Index array object
                    let array_location = self.get_operand();
                    let obj_array = self.stack.get_mut(array_location as usize).unwrap();
                    let array: &mut Box<Table<Object>> = if let Object::Array(table) = obj_array {
                        table
                    } else {
                        panic!("not an array");
                    };

                    array.set(idx, value);
                    //self.stack[array_location as usize]
                }

                Store => {
                    let slot = self.get_integer();
                    let obj = self.pop();
                    match obj {
                        _ => {
                            self.stack[slot as usize] = obj;
                        }
                    }
                }

                Load => {
                    let slot = self.get_operand();
                    let obj = self.stack[slot].clone();
                    self.push(obj);
                }
                Jmp => {
                    let new_loc = self.get_operand();
                    self.ip = new_loc;
                }
                JmpFalse => {
                    let new_loc = self.get_operand();
                    let obj = self.pop();
                    if let Object::Bool(b) = obj {
                        if !b {
                            self.ip = new_loc;
                        }
                    }
                }

                // Get an element from an index
                Index => {
                    // Get the index expression
                    // index
                    let index = if let Object::Integer(int) = self.pop() {
                        int as usize
                    } else {
                        panic!("not an integer");
                    };

                    // Get the array
                    let slot = self.get_operand();
                    let obj = self.stack[slot].clone();

                    if let Object::Array(table) = obj {
                        // Get the Object in the given location
                        if let Some(obj) = table.get(index as usize) {
                            self.push(obj.clone());
                        } else {
                            eprintln!("no value located at index {}", index);
                            return;
                        };
                    }
                }
                And => {
                    boolop!(&&);
                }
                Or => {
                    boolop!(||);
                }

                Nop => {}

                Print => {
                    self.print();
                }

                Halt => {
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
