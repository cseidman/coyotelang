#![allow(dead_code, unused_macros, unused_imports)]

const STACK_SIZE: usize = 1_000_000;
const GLOBAL_SIZE: usize = 1024;
const FRAMES_DEPTH: usize = 1024;

use crate::cfunction::Func;
use crate::ctable::Table;
use crate::{
    constants::Instruction,
    constants::Instruction::*,
    valuetypes::{DataTag, Object},
};
use colored::Colorize;
use std::ops::Neg;
use std::thread::sleep;
use std::time::Duration;
use std::usize;

#[derive(Debug, Clone)]
struct StackFrame {
    function: Func,
    ip: usize,
    start_sp: usize,
    sp: usize,
}

impl StackFrame {
    pub fn new(function: Func, ip: usize, sp: usize, start: usize) -> Self {
        Self {
            function,
            ip,
            start_sp: start,
            sp,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vm {
    stack: Vec<Object>,
    string_pool: Vec<String>,
    functions: Vec<Func>,
    stack_frame: Vec<StackFrame>,
}

impl Vm {
    pub fn new() -> Self {
        let obj = Object::Nil;

        let mut vm = Self {
            stack: vec![obj.clone(); STACK_SIZE],
            string_pool: Vec::new(),
            functions: Vec::new(),
            stack_frame: Vec::with_capacity(FRAMES_DEPTH),
        };

        let frame = StackFrame::new(Func::new(), 0, 0, 0);
        vm.stack_frame.push(frame);
        vm
    }

    pub fn current_stack(&mut self) -> &mut [Object] {
        let start = self.current_frame().start_sp;
        &mut self.stack[start..]
    }

    pub fn add_code(&mut self, code: Vec<u8>) {
        self.current_frame().function.code = code;
    }

    fn current_frame(&mut self) -> &mut StackFrame {
        self.stack_frame.last_mut().unwrap()
    }

    fn incr_ip(&mut self, by: usize) {
        self.current_frame().ip += by;
    }

    fn current_code(&mut self) -> &[u8] {
        &self.current_frame().function.code
    }

    fn get_tag(&mut self) -> DataTag {
        let ip = self.current_frame().ip;
        let code = self.current_code()[ip];
        let tag = DataTag::from(code);
        self.incr_ip(1);
        tag
    }

    fn get_data(&mut self) -> usize {
        let ip = self.current_frame().ip;
        let bytes: [u8; 8] = self.current_code()[ip..ip + 8].try_into().unwrap();
        self.incr_ip(8);
        f64::from_le_bytes(bytes) as usize
    }

    fn push_frame(&mut self, function: Func) {
        // Push the current ip and sp
        let mut ip = 0;
        let mut sp = 0;

        if self.stack_frame.len() > 1 {
            sp = self.current_frame().sp;
            //ip = self.current_frame().ip;
        }
        let start = sp - function.arity as usize;
        let frame = StackFrame::new(function, ip, sp, start);
        self.stack_frame.push(frame);
    }

    fn pop_frame(&mut self) {
        self.stack_frame.pop();
    }

    fn get_instruction(&mut self) -> Instruction {
        let ip = self.current_frame().ip;
        let byte = self.current_code()[ip];
        self.incr_ip(1);
        Instruction::from_u8(byte)
    }

    fn get_operand(&mut self) -> usize {
        self.incr_ip(1);
        let ip = self.current_frame().ip;
        let bytes: [u8; 8] = self.current_code()[ip..ip + 8].try_into().unwrap();

        self.incr_ip(8);
        f64::from_le_bytes(bytes) as usize
    }

    fn get_integer(&mut self) -> usize {
        self.incr_ip(1);
        let ip = self.current_frame().ip;
        let bytes: [u8; 8] = self.current_code()[ip..ip + 8].try_into().unwrap();

        self.incr_ip(8);
        f64::from_le_bytes(bytes) as usize
    }

    fn get_u16(&mut self) -> u16 {
        let ip = self.current_frame().ip;
        let bytes: [u8; 2] = self.current_code()[ip..ip + 2].try_into().unwrap();

        self.incr_ip(2);
        u16::from_le_bytes(bytes)
    }

    fn get_u32(&mut self) -> u32 {
        let ip = self.current_frame().ip;
        let bytes: [u8; 4] = self.current_code()[ip..ip + 4].try_into().unwrap();

        self.incr_ip(4);
        u32::from_le_bytes(bytes)
    }

    fn get_i32(&mut self) -> i32 {
        let ip = self.current_frame().ip;
        let bytes: [u8; 4] = self.current_code()[ip..ip + 4].try_into().unwrap();

        self.incr_ip(4);
        i32::from_le_bytes(bytes)
    }

    fn get_f64(&mut self) -> f64 {
        let ip = self.current_frame().ip;
        let bytes: [u8; 8] = self.current_code()[ip..ip + 8].try_into().unwrap();

        self.incr_ip(8);
        f64::from_le_bytes(bytes)
    }

    fn get_byte(&mut self) -> u8 {
        let ip = self.current_frame().ip;
        let byte = self.current_code()[ip];
        self.incr_ip(1);
        byte
    }

    fn pop(&mut self) -> Object {
        self.current_frame().sp -= 1;
        let sp = self.current_frame().sp;
        self.current_stack()[sp].clone()
    }

    fn push(&mut self, obj: Object) {
        let sp = self.current_frame().sp;
        self.current_stack()[sp] = obj;
        self.current_frame().sp += 1;
    }

    fn get_bool(&mut self) -> Object {
        let _tag = self.get_tag();
        let ip = self.current_frame().ip;
        let byte: u8 = self.current_code()[ip];
        self.incr_ip(1);
        Object::Bool(byte != 0)
    }

    fn get_const(&mut self) -> Object {
        let tag = self.get_tag();
        let ip = self.current_frame().ip;
        let bytes: [u8; 8] = self.current_code()[ip..ip + 8].try_into().unwrap();
        self.incr_ip(8);

        match tag {
            DataTag::Nil => Object::Nil,
            DataTag::Float => Object::Float(f64::from_le_bytes(bytes)),
            DataTag::Bool => Object::Bool(i64::from_le_bytes(bytes) != 0),
            DataTag::Integer => Object::Integer(f64::from_le_bytes(bytes) as i64),
            DataTag::Text => {
                let index = f64::from_le_bytes(bytes) as usize;
                let txt = self.string_pool[index].clone();
                Object::Str(txt)
            }
            _ => {
                panic!("Invalid constant tag: {:?}", tag);
            }
        }
    }

    fn get_string(&mut self) -> Object {
        self.get_tag();
        let index = self.get_u32() as usize;
        let value = self.string_pool[index].clone();
        Object::Str(value)
    }

    /// Loads constants from the ASM file that need to go into the string pool
    fn load_string_pool(&mut self) {
        self.string_pool.clear();
        let ip = self.current_frame().ip;
        let chunk = &self.current_code()[ip..ip + 4];
        let num_strings = u32::from_le_bytes(chunk.try_into().unwrap());

        self.incr_ip(4);
        for _ in 0..num_strings {
            let ip = self.current_frame().ip;
            let s_len =
                u32::from_le_bytes(self.current_code()[ip..ip + 4].try_into().unwrap()) as usize;
            self.incr_ip(4);
            let ip = self.current_frame().ip;
            let s_val = String::from_utf8(self.current_code()[ip..ip + s_len].to_vec()).unwrap();
            self.string_pool.push(s_val);

            self.incr_ip(s_len);
        }
    }

    fn load_subs(&mut self) {
        macro_rules! get_u32 {
            () => {{
                let start = self.current_frame().ip;
                let num =
                    u32::from_le_bytes(self.current_code()[start..(start + 4)].try_into().unwrap());
                self.incr_ip(4);
                num
            }};
        }

        let num_subs = get_u32!();

        for sub_count in 0..num_subs {
            // We don't really need this
            let _location = get_u32!();

            let ip = self.current_frame().ip;

            let arity = self.current_code()[ip] as u8;
            self.incr_ip(1);

            let ip = self.current_frame().ip;
            let slots = self.current_code()[ip] as u8;

            self.incr_ip(1);
            let code_length = get_u32!() as usize;

            let start = self.current_frame().ip;
            let code = self.current_code()[start..(start + code_length)].to_vec();
            self.incr_ip(code_length);

            let func = Func { arity, slots, code };
            self.functions.push(func);
        }
    }

    pub fn run(&mut self) {
        self.load_subs();
        self.load_string_pool();

        let ip = self.current_frame().ip;
        self.current_frame().function.code.drain(..ip);

        self.stack_frame.push(StackFrame {
            function: self.functions[0].clone(),
            ip: 0,
            start_sp: 0,
            sp: 0,
        });

        let offset = self.current_frame().function.slots as usize;

        // Clear out the bytes we already used and restart at 0

        self.current_frame().ip = 0;
        self.current_frame().start_sp += offset;
        self.current_frame().sp += offset;

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
                let mut ip = self.current_frame().ip;
                let b = self.current_code()[ip] as u8;
                let instr = Instruction::from_u8(b);
                print!("{:05}: {:<10} ", ip, instr.as_str().yellow());
                match instr {
                    Push => {
                        ip = ip + 2;
                        let bytes: [u8; 8] = self.current_code()[ip..ip + 8].try_into().unwrap();
                        //print!("bytes: {:?}", bytes);
                        let opd = f64::from_le_bytes(bytes) as usize;
                        print!("{:<6} |", opd.to_string().cyan());
                    }
                    SPush => {
                        ip = ip + 2;
                        let bytes: [u8; 4] = self.current_code()[ip..ip + 4].try_into().unwrap();

                        let opd = u32::from_le_bytes(bytes) as usize;
                        print!("{:<6} |", opd.to_string().cyan());
                    }
                    Store | Load | NewArray | Set | Index => {
                        ip = ip + 1;
                        let bytes: [u8; 2] = self.current_code()[ip..ip + 2].try_into().unwrap();

                        let opd = u16::from_le_bytes(bytes) as usize;
                        print!("{:<6} |", opd.to_string().cyan());
                    }
                    JmpFalse | Jmp => {
                        ip = ip + 1;
                        let bytes: [u8; 4] = self.current_code()[ip..ip + 4].try_into().unwrap();
                        let opd = i32::from_le_bytes(bytes) as usize;
                        print!("{:<6} |", opd.to_string().cyan());
                    }
                    _ => {
                        print!("{:<6} |", "");
                    }
                }
            };
        }

        macro_rules! display_stack {
            () => {
                let sp = self.current_frame().sp;
                print!("{}", "-> [ ".green());
                for idx in (offset..sp).rev() {
                    let obj = &self.current_stack()[idx];
                    if *obj != Object::Nil {
                        print!("{} ", obj.to_string().bright_yellow());
                    }
                }
                println!("{}", "]".green());
            };
        }

        loop {
            vm_debug!();
            let b = self.get_instruction();

            sleep(Duration::from_millis(100));
            match b {
                Push => {
                    let obj = self.get_const();
                    self.push(obj);
                }

                SPush => {
                    let obj = self.get_string();
                    self.push(obj)
                }

                BPush => {
                    let obj = self.get_bool();
                    self.push(obj)
                }

                Call => {
                    // Get the function object off the function registry
                    let index = self.get_u16() as usize;
                    let func = &self.functions[index];
                    self.push_frame(func.clone());
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

                Neg => {
                    let obj = self.pop();
                    self.push(obj.neg());
                }

                NewArray => {
                    let element_count = self.get_u16() as usize;
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
                    let idx = if let Object::Float(int) = self.pop() {
                        int as usize
                    } else {
                        panic!("not an integer");
                    };
                    // Get the new value
                    let value = self.pop().clone();

                    // Index array object
                    let array_location = self.get_u16() as usize;
                    let obj_array = self
                        .current_stack()
                        .get_mut(array_location as usize)
                        .unwrap();
                    let array: &mut Box<Table<Object>> =
                        if let Object::Array(ref mut table) = obj_array {
                            table
                        } else {
                            panic!("not an array");
                        };

                    array.set(idx, value);
                    //self.stack[array_location as usize]
                }

                Store => {
                    let slot = self.get_u16();
                    let obj = self.pop();
                    match obj {
                        _ => {
                            self.current_stack()[slot as usize] = obj;
                        }
                    }
                }

                Load => {
                    let slot = self.get_u16();
                    let obj = self.current_stack()[slot as usize].clone();
                    self.push(obj);
                }
                Jmp => {
                    let new_loc = self.get_i32() as usize;
                    self.current_frame().ip = new_loc;
                }
                JmpFalse => {
                    let new_loc = self.get_i32() as usize;
                    let obj = self.pop();
                    if let Object::Bool(b) = obj {
                        if !b {
                            self.current_frame().ip = new_loc;
                        }
                    }
                }

                // Get an element from an index
                Index => {
                    // Get the index expression
                    // index
                    let index = if let Object::Float(int) = self.pop() {
                        int as usize
                    } else {
                        panic!("not an integer");
                    };

                    // Get the array
                    let slot = self.get_u16() as usize;
                    let obj = self.current_stack()[slot].clone();

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
            /*
            println!("WHOLE STACK");
            for idx in 0..self.stack.len() - 1 {
                let obj = &self.stack[idx];
                if *obj != Object::Nil {
                    println!("{idx}: {obj} ");
                }
            }
            println!("------------");
            */
            display_stack!();
        }
        println!();
    }

    fn print(&mut self) {
        let value = self.pop();
        println!("{}", value);
    }
}

pub fn execute(bytecode: Vec<u8>) {
    let mut vm = Vm::new();
    vm.add_code(bytecode);
    vm.run();
}
