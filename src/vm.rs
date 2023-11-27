use std::collections::VecDeque;

use super::DEBUG_TRACE_EXEC;
use crate::{chunk::OpCode::*, chunk::*, debug::*, new_value, value::*};

#[derive(Clone)]
pub struct Vm {
    chunk: Chunk,
    ip: usize,
    stack: VecDeque<Value>,
}

pub enum InterpretResult {
    Ok,
    CompileErr,
    RuntimeErr,
}

macro_rules! binary_op {
    ($op: tt, $vm: ident) => {
        loop {
            let b = $vm.pop();
            let a = $vm.pop();
            let res = a.value $op b.value;
            $vm.push(new_value!(res));
            if !false {
                break;
            }
        }
    };
}

impl Vm {
    pub fn new() -> Self {
        Self {
            chunk: Chunk {
                count: 0,
                capacity: 0,
                code: vec![].into(),
                lines: vec![].into(),
                constants: ValueArray::new(),
            },
            ip: 0,
            stack: vec![].into(),
        }
    }

    pub fn init() -> Vm {
        let mut nvm = Vm::new();
        nvm.reset_stack();
        nvm
    }

    pub fn free(self) {}

    pub fn push(&mut self, value: Value) {
        self.stack.push_front(value);
    }

    pub fn pop(&mut self) -> Value {
        //self.stack_top -= 1;
        match self.stack.pop_front() {
            Some(val) => {
                val
            }
            None => {
                new_value!(0)
            }
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        if self.ip < self.chunk.code.len() {
            let byte = self.chunk.code[self.ip];
            self.ip += 1;
            Some(byte)
        } else {
            None
        }
    }

    fn read_constant(&mut self) -> Value {
        let mut s2 = self.clone();
        self.chunk.constants.values[Self::read_byte(&mut s2).unwrap() as usize]
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        self.chunk = chunk.clone();
        self.ip = self.chunk.code[self.ip] as usize;
        Self::run(self)
    }

    pub fn reset_stack(&mut self) {
        self.stack = vec![].into();
    }

    #[allow(unreachable_patterns)]
    pub fn run(&mut self) -> InterpretResult {
        loop {
            if let Some(instruction) = self.read_byte() {
                if DEBUG_TRACE_EXEC {
                    disassemble_instruction(
                        &self.chunk,
                        ((self.ip as u8) - self.chunk.code.front().unwrap()).into(),
                    );
                }
                match instruction.try_into().unwrap() {
                    Return => {
                        self.pop().print();
                        println!();
                        return InterpretResult::Ok;
                    }
                    Constant => {
                        let constant: Value = self.read_constant();
                        self.push(constant);
                        return InterpretResult::Ok;
                    }
                    Negate => {
                        let popped = self.pop().value;
                        self.push(Value {
                            value: popped * -1_f64,
                        });
                        //break;
                    }
                    Add => {
                        binary_op!(+, self);
                        //return InterpretResult::InterpretOk;
                    }
                    Sub => {
                        binary_op!(-, self);
                        //return InterpretResult::InterpretOk;
                    }
                    Mul => {
                        binary_op!(*, self);
                        //return InterpretResult::InterpretOk;
                    }
                    Div => {
                        binary_op!(/, self);
                        //return InterpretResult::InterpretOk;
                    }
                    // Handle other opcodes as needed
                    _ => {
                        // Handle unknown opcode or implement other cases
                        unimplemented!("Opcode not implemented: {}", instruction);
                    }
                }
            } else {
                return InterpretResult::RuntimeErr;
                // End of bytecode
            }
        }
    }
}
