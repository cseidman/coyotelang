
#[derive(Copy, Clone)]
union Value {
    i: i64,
    f: f64,
    bytes: [u8;8],
}
impl Value {
    pub fn as_integer(&self) -> i64 {
        unsafe {
            self.i
        }
    }
    pub fn as_float(&self) -> f64 {
        unsafe {
            self.f
        }
    }

    pub fn as_bytes(&self) -> [u8;8] {
        unsafe {
            self.bytes
        }
    }
}


type Register = Value;

struct Vm {
    // Registers
    registers: [Register; 64000],
    code: Vec<u8>,
    ip: usize,
}

const HALT :u8 = 0;
const IMOV :u8 = 1;
const IADD :u8 = 2;
const ISUB :u8 = 3;
const IMUL :u8 = 4;
const IDIV :u8 = 5;
const IEQU :u8 = 6;

impl Vm {
    pub fn new() -> Self {
        Self {
            registers: [Value{i:0}; 64000],
            code: Vec::new(),
            ip: 0,
        }
    }

    fn get_instruction(&mut self) -> u8 {
        self.ip+=1;
        self.code[self.ip-1]
    }

    fn read_register_value(&mut self) -> Register {
        let loc = u16::from_le_bytes(self.code[self.ip..self.ip+1].try_into().unwrap());
        self.ip+=2;
        self.registers[loc as usize]
    }

    fn load_register(&mut self, reg: Register) {
        let loc = u16::from_le_bytes(self.code[self.ip..=self.ip+2].try_into().unwrap());
        self.ip+=2;
        self.registers[loc as usize] = reg;
    }

    fn get_register_location(&mut self) -> usize {
        let loc = u16::from_le_bytes(self.code[self.ip..self.ip+2].try_into().unwrap()) as usize;
        self.ip+=2;
        loc
    }

    fn get_data(&mut self) -> Value {
        let bytes:[u8;8] = self.code[self.ip..self.ip+8].try_into().unwrap();
        self.ip+=8;
        Value {
            bytes
        }
    }

    pub fn run(&mut self) {

        macro_rules! ibinop {
            ($op:tt) => {
                let reg1 = self.get_register_location();
                let reg2 = self.get_register_location();

                let lval = self.registers[reg1].as_integer();
                let rval = self.registers[reg2].as_integer();
                self.registers[reg1].i = lval $op rval;
            };
        }

        loop {
            let b = self.get_instruction();

            match b {
                IMOV => {
                    let reg = self.get_register_location();
                    let value = self.get_data().as_integer();
                    self.registers[reg].i = value;
                },
                IADD => {ibinop!(+);},
                ISUB => {ibinop!(-);},
                IMUL => {ibinop!(*);},
                IDIV => {ibinop!(/);},
                HALT => {
                    break;
                },
                _ => {
                    println!("Unknown instruction: {}", b);
                    break;
                }
            }
        }
    }
}

pub fn execute(bytecode: Vec<u8>) {
    let mut vm = Vm::new();
    vm.code = bytecode;
    vm.run();
    for i in 0..4 {
        println!("Register {}: {:?}", i, vm.registers[i].as_integer());
    }
}

#[cfg(test)]
mod test {
    use crate::vm::Vm;
    use super::*;
    #[test]
    fn test_vm() {
        let mut vm = Vm::new();
        vm.code = vec![
            IMOV,
            0,0,
            4,0,0,0,0,0,0,0,
            IMOV,
            1,0,
            3,0,0,0,0,0,0,0,
            IMOV,
            2,0,
            2,0,0,0,0,0,0,0,
            IMUL,
            1, 0, 2, 0,
            IMOV,
            2,0,
            1,0,0,0,0,0,0,0,
            IADD,
            1, 0, 2, 0,
            IADD,
            0, 0, 1, 0,
            0,
        ];
        vm.run();
        for i in 0..4 {
            println!("Register {}: {:?}", i, vm.registers[i].as_integer());
        }
        assert_eq!(vm.registers[0].as_integer(), 11);

    }
}