#![allow(dead_code)]

use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};

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
    Text = 8,
    ConstText = 9,
    Array = 10,
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
            8 => DataTag::Text,
            9 => DataTag::ConstText,
            10 => DataTag::Array,
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
            DataTag::Text => {
                write!(f, "{}", self.data.as_text())
            }
            DataTag::ConstText => {
                write!(f, "{}", self.data.as_text())
            }
            DataTag::Array => {
                write!(f, "{}", self.data.as_text())
            }
        }
    }
}

#[derive(Copy, Clone)]
pub union Value {
    pub i: i64,
    pub f: f64,
    pub b: bool,
    pub ptr: usize,
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

    pub fn as_ptr(&self) -> usize {
        unsafe { self.ptr }
    }

    pub fn as_uint(&self) -> usize {
        self.as_float() as usize
    }

    pub fn as_byte(&self) -> u8 {
        unsafe { self.byte }
    }
    pub fn as_text(&self) -> usize {
        self.as_uint()
    }
}
