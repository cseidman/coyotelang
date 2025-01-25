#![allow(dead_code)]

const STACK_SIZE: usize = 1_000_000;
const GLOBAL_SIZE: usize = 1024;

use crate::ctable::Table;
use crate::heap::{Heap, HeapValue};
use crate::{
    constants::Instruction,
    constants::Instruction::*,
    valuetypes::{DataTag, Object, Value},
};
use std::ops::Neg;
use std::usize;

struct StackFrame {
    ip: usize,
    sp: usize,
}

impl StackFrame {
    pub fn new(ip: usize, sp: usize) -> Self {
        Self { ip, sp }
    }
}

pub struct Vm {
    stack: Vec<Object>,
    sp: usize,
    heap: Heap,
    pub code: Vec<u8>,
    string_pool: Vec<String>,
    ip: usize,

    stack_frame: Vec<StackFrame>,
    fp: usize,
}

impl Vm {
    pub fn new() -> Self {
        let obj = Object {
            tag: DataTag::Nil,
            data: Value { byte: 0 },
        };

        Self {
            stack: vec![obj; STACK_SIZE],
            sp: GLOBAL_SIZE,
            code: Vec::new(),
            heap: Heap::new(),
            string_pool: Vec::new(),
            ip: 0,

            stack_frame: vec![],
            fp: 0,
        }
    }

    fn get_tag(&mut self) -> DataTag {
        let tag = DataTag::from(self.code[self.ip]);
        self.ip += 1;
        tag
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

    fn get_data(&mut self) -> Value {
        let ip = self.ip;
        let bytes: [u8; 8] = self.code[ip..ip + 8].try_into().unwrap();
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
        Object {
            tag: self.get_tag(),
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
        self.ip = 0;
        self.load_string_pool();

        self.load_globals();

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

                self.push(Object::new(DataTag::Bool, Value { b: val }));
            };

        }

        macro_rules! boolop {
            ($op:tt) => {
                let right = self.pop();
                let left = self.pop();
                let val = left.data.as_bool() $op right.data.as_bool();
                self.push(Object::new(DataTag::Bool, Value { b: val }));
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

                SPool => {
                    // Get the index in the string pool
                    let pool_value = self.get_const();
                    // Create an object with the same index but with the correct data type
                    let obj = Object {
                        tag: DataTag::ConstText,
                        data: pool_value.data,
                    };
                    self.push(obj)
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
                    let slot = self.get_integer();
                    self.stack[slot as usize] = self.pop();
                }

                Neg => {
                    let obj = self.pop();
                    self.push(obj.neg());
                }

                Newarray => {
                    let element_count = self.get_integer();
                    // Create the table as an array
                    let mut arr = Table::<Object>::new();

                    for _ in 0..element_count {
                        let obj = self.pop();
                        arr.push(obj)
                    }

                    let heap_index = self.heap.store(HeapValue::Table(arr));
                    let value = Value { ptr: heap_index };

                    let object = Object {
                        tag: DataTag::Array,
                        data: value,
                    };
                    self.push(object);
                }
                Store => {
                    let slot = self.get_integer();
                    let obj = self.pop();
                    match obj.tag {
                        _ => {
                            self.stack[slot as usize] = obj;
                        }
                    }
                }

                Load => {
                    let slot = self.get_integer();
                    let obj = self.stack[slot as usize];
                    self.push(obj);
                }
                // Get an element from an index
                ALoad => {
                    // Get the index
                    let index = self.pop().data.as_integer() as usize;
                    // Get the array pointer
                    let slot = self.get_integer();
                    let obj = self.stack[slot as usize];
                    let ptr = obj.data.as_integer() as usize;
                    // Get the table itself
                    let Some(HeapValue::Table(table)) = self.heap.get(ptr) else {
                        eprintln!("no array located at {}", ptr);
                        return;
                    };

                    // Get the Object in the given location
                    let Some(&obj) = table.get(index) else {
                        eprintln!("no value located at index {}", index);
                        return;
                    };

                    self.push(obj);
                }
                And => {
                    boolop!(&&);
                }
                Or => {
                    boolop!(||);
                }

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
        match value.tag {
            DataTag::ConstText => {
                let index = value.data.as_text();
                let text = &self.string_pool[index];
                println!("{}", text);
            }
            DataTag::Array => {
                let ptr = value.data.as_ptr();
                if let Some(arr) = self.heap.get(ptr) {
                    if let HeapValue::Table(table) = arr {
                        println!("{}", table);
                    }
                }
            }
            _ => println!("{}", value),
        }
    }
}

pub fn execute(bytecode: Vec<u8>) {
    let mut vm = Vm::new();
    vm.code = bytecode;
    vm.run();
}
