#![allow(unused_macros, dead_code)]
use crate::valuetypes::Value;
use std::collections::{HashMap, HashSet};
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
        #[repr(C)]
        struct Array {
            atype: u8,
            data: HashMap<u8, u8>,
        }

        let a = Array {
            atype: 8,
            data: HashMap::from([(1, 10), (2, 20)]),
        };

        let aptr = &a as *const Array as *const u8;
        let num_bytes = std::mem::size_of::<u8>(); // Number of bytes to read (for `atype` field in this case)
        let bytes = unsafe { std::slice::from_raw_parts(aptr, 1) };

        println!("Bytes: {:?}", bytes);

        let b: HashMap<u8, u8> = HashMap::from([(1, 10), (2, 20)]);

        let mut heap = Heap::new();
        let ptr = heap.store(a);
        let num = unsafe { &*(ptr as *const Array) };
        assert_eq!(b[&2], 20);
        heap.free_entry(ptr);
    }
}
