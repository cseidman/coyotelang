#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
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
    NewArray = 13,
    Load = 14,
    SPush = 15,
    Index = 16,
    AStore = 17,

    If = 18,
    Else = 19,
    ElseIf = 20,
    EndIf = 21,

    Eq = 22,
    Neq = 23,
    Gt = 24,
    Ge = 25,
    Lt = 26,
    Le = 27,
    And = 28,
    Or = 29,

    JmpFalse = 30,
    Jmp = 31,

    For = 32,
    While = 33,
    Nop = 34,

    Call = 35,
    Return = 36,
    BPush = 37,
}

impl Instruction {
    pub const INSTRUCTIONS: [&'static str; 38] = [
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
        "index",    // 16
        "astore",   // 17
        "if",       // 18
        "else",     // 19
        "elseif",   // 20
        "endif",    // 21
        "eq",       // 22
        "neq",      // 23
        "gt",       // 24
        "ge",       // 25
        "lt",       // 26
        "le",       // 27
        "and",      // 28
        "or",       // 29
        "jmpfalse", // 30
        "jmp",      // 31
        "for",      // 32
        "while",    // 33
        "nop",      // 34
        "call",     // 35
        "return",   // 36
        "bpush",    // 37
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
            13 => Instruction::NewArray,
            14 => Instruction::Load,
            15 => Instruction::SPush,
            16 => Instruction::Index,
            17 => Instruction::AStore,
            18 => Instruction::If,
            19 => Instruction::Else,
            20 => Instruction::ElseIf,
            21 => Instruction::EndIf,
            22 => Instruction::Eq,
            23 => Instruction::Neq,
            24 => Instruction::Gt,
            25 => Instruction::Ge,
            26 => Instruction::Lt,
            27 => Instruction::Le,
            28 => Instruction::And,
            29 => Instruction::Or,

            30 => Instruction::JmpFalse,
            31 => Instruction::Jmp,
            32 => Instruction::For,
            33 => Instruction::While,

            34 => Instruction::Nop,
            35 => Instruction::Call,
            36 => Instruction::Return,
            37 => Instruction::BPush,
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
            "newarray" => Some(Instruction::NewArray),
            "load" => Some(Instruction::Load),
            "spush" => Some(Instruction::SPush),
            "index" => Some(Instruction::Index),
            "astore" => Some(Instruction::AStore),
            "if" => Some(Instruction::If),
            "else" => Some(Instruction::Else),
            "elseif" => Some(Instruction::EndIf),
            "endif" => Some(Instruction::EndIf),
            "eq" => Some(Instruction::Eq),
            "neq" => Some(Instruction::Neq),
            "gt" => Some(Instruction::Gt),
            "ge" => Some(Instruction::Ge),
            "lt" => Some(Instruction::Lt),
            "le" => Some(Instruction::Le),
            "and" => Some(Instruction::And),
            "or" => Some(Instruction::Or),
            "jmpfalse" => Some(Instruction::JmpFalse),
            "jmp" => Some(Instruction::Jmp),
            "for" => Some(Instruction::For),
            "while" => Some(Instruction::While),
            "nop" => Some(Instruction::Nop),
            "call" => Some(Instruction::Call),
            "return" => Some(Instruction::Return),
            "bpush" => Some(Instruction::BPush),
            _ => None,
        }
    }
}
