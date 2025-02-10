#![allow(dead_code)]

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
    pub arity: u8,
    pub slots: u8,
    pub code: Vec<u8>,
}

impl Func {
    pub fn new() -> Func {
        Func {
            arity: 0,
            slots: 0,
            code: Vec::new(),
        }
    }
}
