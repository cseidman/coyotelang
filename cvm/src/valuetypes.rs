#![allow(dead_code)]

const NONE: u8 = 0;
const FLOAT: u8 = 1;
const BOOLEAN: u8 = 2;
const POINTER: u8 = 3;
const INDEX: u8 = 4;
const BYTES: u8 = 5;
const INTEGER: u8 = 6;
const BYTE: u8 = 7;

const UINT: u8 = 8;

#[derive(Copy, Clone)]
pub(crate) union Value {
    pub i: i64,
    pub f: f64,
    pub b: bool,
    pub ptr: *const u8,
    pub index: u32,
    pub bytes: [u8; 8],
    pub uint: usize,
    pub byte: u8,
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

    pub fn as_ptr(&self) -> *const u8 {
        unsafe { self.ptr }
    }

    pub fn as_index(&self) -> u32 {
        unsafe { self.index }
    }

    pub fn as_uint(&self) -> usize {
        unsafe { self.uint }
    }

    pub fn as_byte(&self) -> u8 {
        unsafe { self.byte }
    }
}
#[repr(C)]
pub struct Array {
    pub data_type: u8,
    pub data: Vec<Value>,
}

impl Array {
    pub fn new(data_type: u8, size: usize) -> Self {
        Self {
            data_type,
            data: Vec::with_capacity(size),
        }
    }
}
