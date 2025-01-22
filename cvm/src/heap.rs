#![allow(unused_macros, dead_code, unused_imports)]
use crate::ctable::Table;
use crate::valuetypes::{Object, Value};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

#[derive(Clone)]
pub enum HeapValue {
    Text(String),
    Table(Table<Object>),
}

pub struct Heap {
    heap: Vec<Option<Box<HeapValue>>>,
    hp: usize,
    free_slots: Vec<usize>,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            heap: Vec::with_capacity(1_000_000),
            hp: 0,
            free_slots: Vec::with_capacity(1024),
        }
    }

    pub fn store(&mut self, data: HeapValue) -> usize {
        // Grab a free slot if there is one
        let position = if let Some(index) = self.free_slots.pop() {
            index
        } else {
            self.hp += 1;
            self.hp - 1
        };

        self.heap.push(Some(Box::new(data)));
        position
    }

    pub fn get(&self, index: usize) -> Option<&HeapValue> {
        self.heap.get(index)?.as_ref().map(|b| &**b)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut HeapValue> {
        self.heap.get_mut(index)?.as_mut().map(|b| &mut **b)
    }

    pub fn free_entry(&mut self, index: usize) {
        self.heap[index] = None;
        self.free_slots.push(index);
    }
}
