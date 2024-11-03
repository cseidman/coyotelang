#![allow(dead_code)]
// This is a map of identifier names to a unique number. The names in here are not scoped
// nor do the numbers have any meaning other than being unique. This is a way to avoid carrying
// strings in emums and avoiding clones

use crate::datatypes::datatype::DataType;
use std::collections::HashMap;
#[derive(Clone)]
pub struct Item {
    pub data_type: DataType,
}
impl Item {
    pub fn new(data_type: DataType) -> Self {
        Self { data_type }
    }
}
#[derive(Clone)]
pub struct Symbol {
    pub symbols: HashMap<String, Item>,
    pub register: Option<usize>,
}

impl Symbol {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            register: None,
        }
    }

    fn add_symbol(&mut self, name: &str, data_type: DataType) {
        self.symbols.insert(name.to_string(), Item::new(data_type));
    }

    /// Get the item for a given identifier name. If the name is not in the map
    /// it creates a new entry in the symbol table and returns the unique number
    pub fn get(&mut self, name: &str) -> Option<Item> {
        self.symbols.get(name).cloned()
    }
}

/// A symbol table is a collection of symbol names. Each symbol name is a map of identifier
#[derive(Clone)]
pub struct SymbolTable {
    pub symbols: Vec<Symbol>,
    scope: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: vec![Symbol::new()],
            scope: 0,
        }
    }

    /// Get the unique number for a given identifier name. If the name is not in the map
    /// it creates a new entry in the symbol table and returns the unique number
    pub fn get(&mut self, name: &str) -> Option<Item> {
        self.symbols[self.scope].get(name)
    }

    pub fn add_symbol(&mut self, name: &str, data_type: DataType) {
        let scope = self.scope;
        self.symbols[scope].add_symbol(name, data_type);
    }

    /// Push a new scope onto the symbol table
    pub fn push_scope(&mut self) {
        self.symbols.push(Symbol::new());
        self.scope += 1;
    }

    /// Pop the current scope from the symbol table
    pub fn pop_scope(&mut self) {
        self.symbols.pop();
        self.scope -= 1;
    }
}
