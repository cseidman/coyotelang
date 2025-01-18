#![allow(unused_macros, dead_code, unused_imports)]
use crate::valuetypes::Value;
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Clone)]
pub struct Heap {
    pub heap: HashSet<*const u8>,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            heap: HashSet::with_capacity(1000),
        }
    }

    pub fn store<T>(&mut self, data: T) -> *const u8 {
        let ptr = Box::into_raw(Box::new(data)) as *const u8;
        self.heap.insert(ptr);
        ptr
    }

    pub fn free_entry(&mut self, index: *const u8) {
        if self.heap.contains(&index) {
            unsafe {
                // Convert the raw pointer back into a Box to safely deallocate it
                let _ = Box::from_raw(index as *mut u8);
            }
            // Remove the pointer from the vector to avoid a dangling pointer
            self.heap.remove(&index);
        } else {
            println!("Value not found in heap");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::valuetypes::Array;

    #[test]
    fn test_heap() {
        let mut heap = Heap::new();
        let data = 10;
        let ptr = heap.store(data);
        let num = unsafe { *(ptr as *const i32) };
        assert_eq!(num, data);
        heap.free_entry(ptr);
    }

    #[test]
    fn test_heap_array() {
        let a = Array {
            data_type: 6,
            data: vec![Value { i: 10 }, Value { i: 20 }],
        };

        let mut heap = Heap::new();
        let new_ptr = heap.store(a);
        let bytes = unsafe { std::slice::from_raw_parts(new_ptr, 1) };
        assert_eq!(bytes, &[6]);
        let num = unsafe { &*(new_ptr as *const Array) };
        assert_eq!(num.data[0].as_integer(), 10);
        heap.free_entry(new_ptr);
    }
}
