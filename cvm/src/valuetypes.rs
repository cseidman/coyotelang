#![allow(dead_code)]

use std::cmp::{Ordering, PartialEq};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

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

impl Object {
    pub fn new(tag: DataTag, data: Value) -> Object {
        Object { tag, data }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.tag, other.tag) {
            (DataTag::Nil, DataTag::Nil) => Some(Ordering::Equal),
            (DataTag::Integer, DataTag::Integer) | (DataTag::Float, _) | (_, DataTag::Float) => {
                self.data.as_float().partial_cmp(&other.data.as_float())
            }
            _ => panic!("cannot compare values of non-numeric objects"),
        }
    }

    fn lt(&self, other: &Self) -> bool {
        match (self.tag, other.tag) {
            (DataTag::Nil, DataTag::Nil) => false,
            (DataTag::Integer, DataTag::Integer) | (DataTag::Float, _) | (_, DataTag::Float) => {
                self.data.as_float() < other.data.as_float()
            }
            _ => panic!("cannot compare values of non-numeric objects"),
        }
    }

    fn le(&self, other: &Self) -> bool {
        match (self.tag, other.tag) {
            (DataTag::Nil, DataTag::Nil) => true,
            (DataTag::Integer, DataTag::Integer) | (DataTag::Float, _) | (_, DataTag::Float) => {
                self.data.as_float() <= other.data.as_float()
            }
            _ => panic!("cannot compare values of non-numeric objects"),
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match (self.tag, other.tag) {
            (DataTag::Nil, DataTag::Nil) => false,
            (DataTag::Integer, DataTag::Integer) | (DataTag::Float, _) | (_, DataTag::Float) => {
                self.data.as_float() > other.data.as_float()
            }
            _ => panic!("cannot compare values of non-numeric objects"),
        }
    }

    fn ge(&self, other: &Self) -> bool {
        match (self.tag, other.tag) {
            (DataTag::Nil, DataTag::Nil) => true,
            (DataTag::Integer, DataTag::Integer) | (DataTag::Float, _) | (_, DataTag::Float) => {
                self.data.as_float() >= other.data.as_float()
            }
            _ => panic!("cannot compare values of non-numeric objects"),
        }
    }
}

impl Add for Object {
    type Output = Object;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.tag, rhs.tag) {
            (DataTag::Nil, DataTag::Nil) => rhs,
            (DataTag::Float, DataTag::Float)
            | (DataTag::Integer, DataTag::Float)
            | (DataTag::Float, DataTag::Integer) => {
                let val = self.data.as_float() + rhs.data.as_float();
                Object::new(DataTag::Float, Value { f: val })
            }
            (DataTag::Integer, DataTag::Integer) => {
                let val = self.data.as_float() + rhs.data.as_float();
                Object::new(DataTag::Integer, Value { f: val })
            }
            _ => panic!("cannot add types {:?} and {:?}", self.tag, rhs.tag),
        }
    }
}

impl Sub for Object {
    type Output = Object;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.tag, rhs.tag) {
            (DataTag::Nil, DataTag::Nil) => rhs,
            (DataTag::Float, DataTag::Float)
            | (DataTag::Integer, DataTag::Float)
            | (DataTag::Float, DataTag::Integer) => {
                let val = self.data.as_float() - rhs.data.as_float();
                Object::new(DataTag::Float, Value { f: val })
            }
            (DataTag::Integer, DataTag::Integer) => {
                let val = self.data.as_float() - rhs.data.as_float();
                Object::new(DataTag::Integer, Value { f: val })
            }
            _ => panic!("cannot sub types {:?} and {:?}", self.tag, rhs.tag),
        }
    }
}

impl Mul for Object {
    type Output = Object;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.tag, rhs.tag) {
            (DataTag::Nil, DataTag::Nil) => rhs,
            (DataTag::Float, DataTag::Float)
            | (DataTag::Integer, DataTag::Float)
            | (DataTag::Float, DataTag::Integer) => {
                let val = self.data.as_float() * rhs.data.as_float();
                Object::new(DataTag::Float, Value { f: val })
            }
            (DataTag::Integer, DataTag::Integer) => {
                let val = self.data.as_float() * rhs.data.as_float();
                Object::new(DataTag::Integer, Value { f: val })
            }
            _ => panic!("cannot mul types {:?} and {:?}", self.tag, rhs.tag),
        }
    }
}

impl Div for Object {
    type Output = Object;

    fn div(self, rhs: Self) -> Self::Output {
        match (self.tag, rhs.tag) {
            (DataTag::Nil, DataTag::Nil) => rhs,
            (DataTag::Float, DataTag::Float)
            | (DataTag::Integer, DataTag::Float)
            | (DataTag::Float, DataTag::Integer) => {
                let val = self.data.as_float() / rhs.data.as_float();
                Object::new(DataTag::Float, Value { f: val })
            }
            (DataTag::Integer, DataTag::Integer) => {
                let val = self.data.as_float() / rhs.data.as_float();
                Object::new(DataTag::Integer, Value { f: val })
            }
            _ => panic!("cannot div types {:?} and {:?}", self.tag, rhs.tag),
        }
    }
}

impl Neg for Object {
    type Output = Object;

    fn neg(self) -> Self::Output {
        match self.tag {
            DataTag::Nil => self,
            DataTag::Float | DataTag::Integer => {
                let val = -self.data.as_float();
                Object::new(self.tag, Value { f: val })
            }
            _ => panic!("cannot negate types {:?}", self.tag),
        }
    }
}

impl PartialEq<Self> for Object {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
            && match self.tag {
                DataTag::Nil => true,
                DataTag::Float => self.data.as_float() == other.data.as_float(),
                DataTag::Bool => self.data.as_bool() == other.data.as_bool(),
                DataTag::Pointer => self.data.as_ptr() == other.data.as_ptr(),
                DataTag::Char => self.data.as_byte() == other.data.as_byte(),
                DataTag::Integer => self.data.as_integer() == other.data.as_integer(),
                DataTag::Byte => self.data.as_byte() == other.data.as_byte(),
                DataTag::UInt => self.data.as_uint() == other.data.as_uint(),
                DataTag::Text => self.data.as_text() == other.data.as_text(),
                DataTag::ConstText => self.data.as_text() == other.data.as_text(),
                DataTag::Array => {
                    // todo: Find a way to compare arrays
                    true
                }
            }
    }
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
    pub rptr: *mut Object,
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
