#![allow(dead_code)]

use crate::cfunction::Func;
use crate::ctable::Table;
use std::cmp::PartialEq;
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
    FuncPtr = 11,
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
            11 => DataTag::FuncPtr,
            _ => {
                panic!("unknown tag")
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Object {
    Nil,
    Integer(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    Byte(u8),
    Str(String),
    Array(Box<Table<Object>>),
    Func(Box<Func>),
}

impl Add for Object {
    type Output = Object;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Nil, Object::Nil) => Object::Nil,
            (Object::Integer(lhs), Object::Integer(rhs)) => Object::Integer(lhs + rhs),
            (Object::Float(lhs), Object::Float(rhs)) => Object::Float(lhs + rhs),
            (Object::Float(lhs), Object::Integer(rhs)) => Object::Float(lhs + rhs as f64),
            (Object::Integer(lhs), Object::Float(rhs)) => Object::Float(lhs as f64 + rhs),
            _ => panic!("Incompatible types"),
        }
    }
}

impl Sub for Object {
    type Output = Object;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Nil, Object::Nil) => Object::Nil,
            (Object::Integer(lhs), Object::Integer(rhs)) => Object::Integer(lhs - rhs),
            (Object::Float(lhs), Object::Float(rhs)) => Object::Float(lhs - rhs),
            (Object::Float(lhs), Object::Integer(rhs)) => Object::Float(lhs - rhs as f64),
            (Object::Integer(lhs), Object::Float(rhs)) => Object::Float(lhs as f64 - rhs),
            _ => panic!("Incompatible types"),
        }
    }
}

impl Mul for Object {
    type Output = Object;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Nil, Object::Nil) => Object::Nil,
            (Object::Integer(lhs), Object::Integer(rhs)) => Object::Integer(lhs * rhs),
            (Object::Float(lhs), Object::Float(rhs)) => Object::Float(lhs * rhs),
            (Object::Float(lhs), Object::Integer(rhs)) => Object::Float(lhs * rhs as f64),
            (Object::Integer(lhs), Object::Float(rhs)) => Object::Float(lhs as f64 * rhs),
            _ => panic!("Incompatible types"),
        }
    }
}

impl Div for Object {
    type Output = Object;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Nil, Object::Nil) => Object::Nil,
            (Object::Integer(lhs), Object::Integer(rhs)) => Object::Integer(lhs / rhs),
            (Object::Float(lhs), Object::Float(rhs)) => Object::Float(lhs / rhs),
            (Object::Float(lhs), Object::Integer(rhs)) => Object::Float(lhs / rhs as f64),
            (Object::Integer(lhs), Object::Float(rhs)) => Object::Float(lhs as f64 / rhs),
            _ => panic!("Incompatible types"),
        }
    }
}

impl Neg for Object {
    type Output = Object;

    fn neg(self) -> Self::Output {
        match self {
            Object::Integer(i) => Object::Integer(-i),
            Object::Float(f) => Object::Float(-f),
            Object::Bool(b) => Object::Bool(!b),
            _ => panic!("Cannot negate type"),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Object::Nil => {
                write!(f, "nil")
            }
            Object::Float(val) => {
                write!(f, "{}", val)
            }
            Object::Bool(val) => {
                write!(f, "{}", val)
            }
            Object::Char(val) => {
                write!(f, "{}", val)
            }
            Object::Integer(val) => {
                write!(f, "{}", val)
            }
            Object::Byte(val) => {
                write!(f, "{}", val)
            }
            Object::Str(val) => {
                write!(f, "{}", val)
            }
            Object::Array(boxed_val) => {
                write!(f, "{}", boxed_val)
            }
            Object::Func(val) => {
                write!(f, "{}", val.name)
            }
        }
    }
}
