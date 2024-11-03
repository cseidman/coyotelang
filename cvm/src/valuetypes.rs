#![allow(dead_code)]
#[derive(Copy, Clone)]
pub(crate) union Value {
    pub i: i64,
    pub f: f64,
    pub b: bool,
    pub ptr: *mut u8,
    pub index: u32,
    pub bytes: [u8; 8],
}
impl Value {
    pub fn as_integer(&self) -> i64 {
        unsafe { self.i }
    }
    pub fn as_float(&self) -> f64 {
        unsafe { self.f }
    }

    pub fn as_bytes(&self) -> [u8; 8] {
        unsafe { self.bytes }
    }

    pub fn as_bool(&self) -> bool {
        unsafe { self.b }
    }

    pub fn as_ptr(&self) -> *mut u8 {
        unsafe { self.ptr }
    }

    pub fn as_index(&self) -> u32 {
        unsafe { self.index }
    }
}
