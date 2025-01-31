use crate::constants::Instruction;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Func {
    pub arity: usize,
    pub code: Vec<Instruction>,
    pub name: String,
}
