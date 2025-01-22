#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Instruction {
    Halt = 0,
    Push = 1,
    Add = 2,
    Sub = 3,
    Mul = 4,
    Div = 5,
    Set = 6,
    Store = 7,
    Pop = 8,
    Cmp = 9,
    Print = 10,
    Neg = 11,
    Const = 12,
    Newarray = 13,
    Load = 14,
    SPool = 15,
    ALoad = 16,
    AStore = 17,
}

impl Instruction {
    pub const INSTRUCTIONS: [&'static str; 18] = [
        "halt",     // 0
        "push",     // 1
        "add",      // 2
        "sub",      // 3
        "mul",      // 4
        "div",      // 5
        "set",      // 6
        "store",    // 7
        "pop",      // 8
        "cmp",      // 9
        "print",    // 10
        "neg",      // 11
        "const",    // 12
        "newarray", // 13
        "load",     // 14
        "spush",    // 15
        "aload",    // 16
        "astore",   // 17
    ];

    /// Return the human-readable name of this instruction.
    pub fn as_str(&self) -> &'static str {
        Self::INSTRUCTIONS[*self as usize]
    }

    /// Convert a `u8` opcode into an `Instruction` (if it’s valid).
    pub fn from_u8(opcode: u8) -> Self {
        match opcode {
            0 => Instruction::Halt,
            1 => Instruction::Push,
            2 => Instruction::Add,
            3 => Instruction::Sub,
            4 => Instruction::Mul,
            5 => Instruction::Div,
            6 => Instruction::Set,
            7 => Instruction::Store,
            8 => Instruction::Pop,
            9 => Instruction::Cmp,
            10 => Instruction::Print,
            11 => Instruction::Neg,
            12 => Instruction::Const,
            13 => Instruction::Newarray,
            14 => Instruction::Load,
            15 => Instruction::SPool,
            16 => Instruction::ALoad,
            17 => Instruction::AStore,
            _ => {
                panic!("Unknown opcode {}", opcode);
            }
        }
    }

    pub fn match_instruction(s: &str) -> Option<Instruction> {
        match s {
            "halt" => Some(Instruction::Halt),
            "push" => Some(Instruction::Push),
            "add" => Some(Instruction::Add),
            "sub" => Some(Instruction::Sub),
            "mul" => Some(Instruction::Mul),
            "div" => Some(Instruction::Div),
            "set" => Some(Instruction::Set),
            "store" => Some(Instruction::Store),
            "pop" => Some(Instruction::Pop),
            "cmp" => Some(Instruction::Cmp),
            "print" => Some(Instruction::Print),
            "neg" => Some(Instruction::Neg),
            "const" => Some(Instruction::Const),
            "newarray" => Some(Instruction::Newarray),
            "load" => Some(Instruction::Load),
            "spool" => Some(Instruction::SPool),
            "aload" => Some(Instruction::ALoad),
            "astore" => Some(Instruction::AStore),
            _ => None,
        }
    }
}
