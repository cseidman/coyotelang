#![allow(dead_code)]
use std::collections::BTreeMap;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Table<T: Display> {
    array: Vec<T>,
    array_length: usize,
    hash: BTreeMap<String, T>,
}

impl<T: Display> Display for Table<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut comma = "";
        for i in 0..self.array_length {
            if let Some(val) = self.get(i) {
                if i > 0 {
                    comma = ", ";
                }
                write!(f, "{}{}", comma, val)?;
            }
        }
        if !self.hash.is_empty() && self.array_length > 0 {
            write!(f, ", .. ")?;
        }
        for k in self.hash.keys() {
            write!(f, "{}{}", comma, self.hash[k])?;
        }
        write!(f, "]")
    }
}

impl<T: Display> Table<T> {
    pub fn new() -> Table<T> {
        Self {
            array: Vec::with_capacity(32),
            array_length: 0,
            hash: BTreeMap::new(),
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        // First check to see if the index length in bounds
        if index < self.array_length {
            return Some(&self.array[index]);
        }
        // If not, then it's in the hash
        self.hash.get(&index.to_string())
    }

    pub fn push(&mut self, value: T) {
        self.array.push(value);
        self.array_length += 1;
    }

    pub fn hset<S: ToString>(&mut self, key: S, value: T) {
        self.hash.insert(key.to_string(), value);
    }

    pub fn set(&mut self, index: usize, value: T) {
        // It within the array range, so update the value
        if index < self.array_length {
            self.array[index] = value;
            return;
        }

        // The index is the next value in the array, so tack it on to the end
        if index == self.array_length {
            self.array_length += 1;
            self.array.push(value);
            return;
        }

        // The index is out of range, so add it to the hash
        self.hset(index, value);
    }
}

impl<T: Display> Default for Table<T> {
    fn default() -> Self {
        Self::new()
    }
}
