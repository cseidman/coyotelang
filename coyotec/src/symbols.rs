// This is a map of identifier names to a unique number. The names in here are not scoped
// nor do the numbers have any meaning other than being unique. This is a way to avoid carrying
// strings in emums and avoiding clones

use std::collections::HashMap;

pub struct Symbol {
    pub symbols: HashMap<String, usize>,
    pub next: usize,
}

impl Symbol {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            next: 0,
        }
    }

    /// Get the unique number for a given identifier name. If the name is not in the map
    /// it creates a new entry in the symbol table and returns the unique number
    pub fn get(&mut self, name: &str) -> usize {
        self.symbols.get(name).copied().unwrap_or_else(|| {
            let id = self.next;
            self.next+=1;
            self.symbols.insert(name.to_string(), id);
            id
        })
    }
}

/// A symbol table is a collection of symbol names. Each symbol name is a map of identifier
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
    pub fn get(&mut self, name: &str) -> usize {
        self.symbols[self.scope].get(name)
    }

    /// Push a new scope onto the symbol table
    pub fn push_scope(&mut self) {
        self.symbols.push(Symbol::new());
        self.scope+=1;
    }

    /// Pop the current scope from the symbol table
    pub fn pop_scope(&mut self) {
        self.symbols.pop();
        self.scope-=1;
    }
}

/// Tests for the symbol table
///
#[cfg(test)]
mod test {
    use crate::symbols::{Symbol, SymbolTable};

    #[test]
    fn test_symbol() {
        let mut sym = Symbol::new();
        assert_eq!(sym.get("foo"), 0);
        assert_eq!(sym.get("bar"), 1);
        assert_eq!(sym.get("foo"), 0);
    }

    #[test]
    fn test_symbol_table() {
        let mut sym_table = SymbolTable::new();
        assert_eq!(sym_table.get("foo"), 0);
        assert_eq!(sym_table.get("bar"), 1);
        sym_table.push_scope();

        assert_eq!(sym_table.get("foo"), 0);
        sym_table.pop_scope();

        assert_eq!(sym_table.get("foo"), 0);
        assert_eq!(sym_table.get("bar"), 1);
    }
}
