use std::collections::VecDeque;

use crate::memory::*;

#[derive(Clone, Copy, Default)]
pub struct Value {
    pub value: f64,
}

#[derive(Clone)]
pub struct ValueArray {
    pub capacity: usize,
    pub count: usize,
    pub values: VecDeque<Value>,
}

#[macro_export]
macro_rules! new_value {
    ($contents:expr) => {
        Value {
            value: $contents as f64
        }
    };
}

impl ValueArray {
    pub fn new() -> Self {
        Self {
            capacity: 0,
            count: 0,
            values: vec![].into(),

        }
    }

    pub fn write(&mut self, value: Value) {
        if self.capacity < self.count + 1 {
            let old_capacity = self.capacity;
            self.capacity = grow_capacity(old_capacity);
            self.values = grow_array(&self.values, old_capacity, self.capacity);
        }

        self.values[self.count] = value;
        self.count += 1;
    }

    pub fn free(&mut self) {
        free_array(&mut self.values, self.capacity);
        *self = ValueArray::new();
    }
}

impl Value {
    pub fn print(self) {
        print!("\'{}\'", self.value);
    }
}