use crate::constants::Instruction;

/// ## How the module system works
/// There are two ways to load modules:
/// 1. Load modules compiled along with the code in the project. This typically done when using custom modules
/// 2. Import pre-compiled modules from another location
///
/// ### Locating modules
/// A program can refer to Modules by pointing to files in the filesystem
pub struct Module {
    pub name: String,
    pub code: Vec<Func>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Func {
    pub arity: usize,
    pub code: Vec<u8>,
    pub name: String,
}

impl Func {
    pub fn main_func() -> Func {
        Func {
            arity: 0,
            code: Vec::new(),
            name: "$main".to_string(),
        }
    }
}
