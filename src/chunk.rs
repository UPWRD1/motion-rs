use std::collections::VecDeque;

use crate::{memory::*, value::*,};

#[derive(Clone, Copy, Debug, Default)]
pub enum OpCode {
    #[default]
    Return,
    Constant,
    Negate,
    Add,
    Sub,
    Mul,
    Div,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Return,
            1 => OpCode::Constant,
            2 => OpCode::Negate,
            3 => OpCode::Add,
            4 => OpCode::Sub,
            5 => OpCode::Mul,
            6 => OpCode::Div,
            // Add other mappings for different values
            _ => panic!("Invalid OpCode: {}", value),
        }
    }
}


#[derive(Clone)]
pub struct Chunk {
    pub count: usize,
    pub capacity: usize,
    pub code: VecDeque<u8>,
    pub lines: VecDeque<usize>,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            count: 0,
            capacity: 0,
            code: vec![].into(),
            lines: vec![].into(),
            constants: ValueArray::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        if self.capacity < (self.count + 1) {
            let old_capacity: usize = self.capacity;
            self.capacity = grow_capacity(old_capacity);
            self.lines = grow_array(&self.lines, old_capacity, self.capacity);
        }
        //self.code[self.count] = byte;
        self.code.push_back(byte);
        //self.lines[self.count] = line;
        self.lines.push_back(line);
        self.count += 1;
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        ValueArray::write(&mut self.constants, value);
        (self.constants.count - 1) as u8
    }

    pub fn free(&mut self) {
        free_array(&mut self.code, self.capacity);
        free_array(&mut self.lines, self.capacity);
        ValueArray::free(&mut self.constants);
        *self = Chunk::new();
    }
}
