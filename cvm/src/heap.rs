#![allow(unused_macros, dead_code)]
use std::collections::HashMap;

macro_rules! heap_type {
    ($name:ident, $value:expr, $val_type:tt) => {
        pub const $name: *mut u8 = $value.as_mut_ptr();
    };
}

pub struct Heap {
    pub heap: HashMap<*const u8, Vec<u8>>,
    pub next_id: usize,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            heap: HashMap::with_capacity(1000),
            next_id: 0,
        }
    }

    pub fn store(&mut self, heap_data: Vec<u8>) -> *const u8 {
        let ptr = heap_data.as_ptr();
        self.heap.insert(ptr, heap_data);
        ptr
    }

    pub fn get_val(&mut self, ptr: *const u8) -> Vec<u8> {
        self.heap.get(&ptr).unwrap().to_vec()
    }

    pub fn as_string(&mut self, ptr: *const u8) -> String {
        let bytes = self.get_val(ptr);
        String::from_utf8(bytes.to_vec()).unwrap()
    }
    pub fn as_integer(&mut self, ptr: *const u8) -> i64 {
        let bytes = self.get_val(ptr);
        i64::from_be_bytes(bytes.try_into().unwrap())
    }

    pub fn as_float(&mut self, ptr: *const u8) -> f64 {
        let bytes = self.get_val(ptr);
        f64::from_be_bytes(bytes.try_into().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::valuetypes::Value;

    #[test]
    fn test_heap() {
        let mut heap = Heap::new();
        let mut toast = "Toast".to_string();
        let val: Value = Value {
            ptr: toast.as_mut_ptr(),
        };
        // Stora a string on the heap
        let ptr = heap.store(unsafe { toast.as_bytes().to_vec() });

        let new_toast = heap.as_string(ptr);

        assert_eq!(new_toast, toast);

        let mut num = 42;
        let val: Value = Value { i: num };
        // Store an integer on the heap
        let ptr = heap.store(unsafe { num.to_be_bytes().to_vec() });
        // Retrieve the integer from the heap
        let new_num = heap.as_integer(ptr);
        assert_eq!(new_num, num);
    }
}
