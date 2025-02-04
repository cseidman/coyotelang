#![allow(dead_code, unused_macros)]

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
use std::ops::Neg;
use std::usize;

#[derive(Debug, Clone)]
struct StackFrame {
    function: Func,
    ip: usize,
    start_sp: usize,
    sp: usize,
}

impl StackFrame {
    pub fn new(function: Func, ip: usize, sp: usize) -> Self {
        Self {
            function,
            ip,
            start_sp: sp,
            sp,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vm {
    stack: Vec<Object>,
    string_pool: Vec<String>,
    stack_frame: Vec<StackFrame>,
    fp: usize,
}

impl Vm {
    pub fn new() -> Self {
        let obj = Object::Nil;

        let mut s = Self {
            stack: vec![obj.clone(); STACK_SIZE],
            string_pool: Vec::new(),

            stack_frame: Vec::with_capacity(FRAMES_DEPTH),
            fp: 0,
        };
        s.push_frame(Func::main_func());
        s
    }

    pub fn current_stack(&mut self) -> &mut [Object] {
        let start_sp = self.current_frame().start_sp;
        &mut self.stack[start_sp..]
    }

    pub fn add_code(&mut self, code: Vec<u8>) {
        self.current_frame().function.code = code;
    }

    fn current_frame(&mut self) -> &mut StackFrame {
        let fp = self.fp;
        &mut self.stack_frame[fp]
    }

    fn incr_ip(&mut self, by: usize) {
        let fp = self.fp;
        self.stack_frame[fp].ip += by;
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
        let ip = 0;
        let sp = 0;

        if self.fp > 0 {
            let sp = self.current_frame().sp;
            let ip = self.current_frame().ip;
        }

        let mut frame = StackFrame::new(function, ip, sp);
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

    fn get_byte(&mut self) -> u8 {
        let ip = self.current_frame().ip;
        let byte = self.current_code()[ip];
        self.incr_ip(8);
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
        let index = self.get_data();
        let value = self.string_pool[index].clone();
        Object::Str(value)
    }

    /// Loads constants from the ASM file that need to go into the string pool
    fn load_string_pool(&mut self) {
        self.string_pool.clear();
        let num_constants = u32::from_le_bytes(self.current_code()[0..4].try_into().unwrap());
        self.incr_ip(4);
        for _ in 0..num_constants {
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

    /// Read the number of constants and use that as the basis for the starting position on the
    /// stack
    fn load_globals(&mut self) -> usize {
        let start = self.current_frame().ip;
        let num_constants =
            u32::from_le_bytes(self.current_code()[start..(start + 4)].try_into().unwrap());
        self.incr_ip(4);
        num_constants as usize
    }

    pub fn run(&mut self) {
        self.load_string_pool();
        self.load_globals();
        // Clear out the bytes we already used and restart at 0
        let ip = self.current_frame().ip;
        self.current_frame().function.code.drain(..ip);
        self.current_frame().ip = 0;
        self.current_frame().sp = 1024;

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
                print!("{:05}: {:<10} ", ip, instr.as_str());
                match instr {
                    Push | Store | Load | Jmp | Newarray | SPool | Set | Index => {
                        ip = ip + 2;
                        let bytes: [u8; 8] = self.current_code()[ip..ip + 8].try_into().unwrap();
                        //print!("bytes: {:?}", bytes);
                        let opd = f64::from_le_bytes(bytes) as usize;
                        print!("{:<6} |", opd);
                    }
                    JmpFalse => {
                        ip = ip + 2;
                        let bytes: [u8; 8] = self.current_code()[ip..ip + 8].try_into().unwrap();
                        let opd = f64::from_le_bytes(bytes) as usize;
                        print!("{:<6} |", opd);
                    }
                    _ => {
                        print!("{:<6} |", "");
                    }
                }
            };
        }

        macro_rules! display_stack {
            () => {
                print!(" -> [ ");
                for sp in 0..10 {
                    //let obj = self.stack[sp].clone();
                    let obj = self.current_stack()[sp].clone();
                    if obj != Object::Nil {
                        print!("{obj} ")
                    }
                }
                println!("]");
            };
        }

        loop {
            //vm_debug!();
            let b = self.get_instruction();

            //sleep(Duration::from_millis(500));
            match b {
                Push => {
                    let obj = self.get_const();
                    self.push(obj);
                }

                SPool => {
                    let obj = self.get_string();
                    self.push(obj)
                }

                Call => {
                    // Get the function object off the stack
                    let func_obj = self.pop();
                    let func = if let Object::Func(func_obj) = func_obj {
                        self.push_frame(*func_obj);
                    } else {
                        panic!("Not a function");
                    };

                    // Position the pointer where the parameters were left on the stack
                    let fp = self.fp;
                    //self.stack_frame[fp].sp -= func.arity;

                    self.pop_frame();
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
                    self.current_stack()[slot as usize] = self.pop();
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
                    let idx = if let Object::Integer(int) = self.pop() {
                        int as usize
                    } else {
                        panic!("not an integer");
                    };
                    // Get the new value
                    let value = self.pop().clone();

                    // Index array object
                    let array_location = self.get_operand();
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
                    let slot = self.get_integer();
                    let obj = self.pop();
                    match obj {
                        _ => {
                            self.current_stack()[slot as usize] = obj;
                        }
                    }
                }

                Load => {
                    let slot = self.get_integer();
                    let obj = self.current_stack()[slot].clone();
                    self.push(obj);
                }
                Jmp => {
                    let new_loc = self.get_operand();
                    self.current_frame().ip = new_loc;
                }
                JmpFalse => {
                    let new_loc = self.get_operand();
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
                    let index = if let Object::Integer(int) = self.pop() {
                        int as usize
                    } else {
                        panic!("not an integer");
                    };

                    // Get the array
                    let slot = self.get_operand();
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
            //display_stack!();
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
