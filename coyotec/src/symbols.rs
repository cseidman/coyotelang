// This is a map of identifier names to a unique number. The names in here are not scoped
// nor do the numbers have any meaning other than being unique. This is a way to avoid carrying
// strings in emums and avoiding clones

use std::collections::HashMap;

pub struct SymbolNames {
    pub symbols: HashMap<String, usize>,
    pub next: usize,
}

impl SymbolNames {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            next: 0,
        }
    }

    pub fn get(&mut self, name: &str) -> usize {
        if let Some(&id) = self.symbols.get(name) {
            return id;
        }
        let id = self.next;
        self.next+=1;
        self.symbols.insert(name.to_string(), id);
        id
    }


}