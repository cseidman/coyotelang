use std::collections::HashMap;

pub struct Table<T> {
    array: Vec<T>,
    array_length: usize,
    hash: HashMap<usize, T>,
}

impl<T> Table<T> {
    pub fn new() -> Table<T> {
        Self {
            array: vec![],
            array_length: 0,
            hash: HashMap::new(),
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        // First check to see if the index length in bounds
        if index < self.array_length {
            return Some(&self.array[index]);
        }
        // If not, then it's in the hash
        self.hash.get(&index)
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
        self.hash.insert(index, value);
    }
}

impl<T> Default for Table<T> {
    fn default() -> Self {
        Self::new()
    }
}
