pub const HALT: u8 = 0;
pub const IMOV: u8 = 1;
pub const IADD: u8 = 2;
pub const ISUB: u8 = 3;
pub const IMUL: u8 = 4;
pub const IDIV: u8 = 5;
pub const IEQU: u8 = 6;
pub const FMOV: u8 = 7;
pub const FADD: u8 = 8;
pub const FSUB: u8 = 9;
pub const FMUL: u8 = 10;
pub const FDIV: u8 = 11;
pub const STORE: u8 = 12;
pub const LOAD: u8 = 13;
pub const IDEC: u8 = 14;
pub const CMP: u8 = 15;
pub const IINC: u8 = 16;
pub const ISTORE: u8 = 17;
pub const IPRINT: u8 = 18;
pub const INEG: u8 = 19;
pub const SPRINT: u8 = 20;
pub const SMOV: u8 = 21;
pub const SSTORE: u8 = 22;
pub const ICONST: u8 = 23;
pub const FCONST: u8 = 24;
pub const SCONST: u8 = 25;
pub const FPRINT: u8 = 26;

pub const INSTRUCTIONS: [&str; 27] = [
    "HALT", "IMOV", "IADD", "ISUB", "IMUL", "IDIV", "IEQU", "FMOV", "FADD", "FSUB", "FMUL", "FDIV",
    "STORE", "LOAD", "IDEC", "CMP", "IINC", "ISTORE", "IPRINT", "INEG", "SPRINT", "SMOV", "SSTORE",
    "ICONST", "FCONST", "SCONST", "FPRINT",
];
