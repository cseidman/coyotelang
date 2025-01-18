#![allow(dead_code)]

use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul};

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DataTag {
    Nil = 0,
    Float = 1,
    Bool = 2,
    Pointer = 3,
    Char = 4,
    Integer = 5,
    Byte = 6,
    UInt = 7,
}

impl From<u8> for DataTag {
    fn from(value: u8) -> Self {
        match value {
            0 => DataTag::Nil,
            1 => DataTag::Float,
            2 => DataTag::Bool,
            3 => DataTag::Pointer,
            4 => DataTag::Char,
            5 => DataTag::Integer,
            6 => DataTag::Byte,
            7 => DataTag::UInt,
            _ => {
                panic!("unknown tag")
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Object {
    pub tag: DataTag,
    pub data: Value,
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.tag {
            DataTag::Nil => {
                write!(f, "nil")
            }
            DataTag::Float => {
                write!(f, "{}", self.data.as_float())
            }
            DataTag::Bool => {
                write!(f, "{}", self.data.as_bool())
            }
            DataTag::Pointer => {
                write!(f, "*ptr")
            }
            DataTag::Char => {
                write!(f, "{}", unsafe { self.data.byte })
            }
            DataTag::Integer => {
                write!(f, "{}", self.data.as_float())
            }
            DataTag::Byte => {
                write!(f, "{}", unsafe { self.data.byte })
            }
            DataTag::UInt => {
                write!(f, "{}", self.data.as_uint())
            }
        }
    }
}

#[derive(Copy, Clone)]
pub union Value {
    pub i: i64,
    pub f: f64,
    pub b: bool,
    pub ptr: *const u8,
    pub bytes: [u8; 8],
    pub uint: usize,
    pub byte: u8,
}
impl Value {
    pub fn as_integer(&self) -> i64 {
        unsafe { self.f as i64 }
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
