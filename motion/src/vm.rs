use crate::{chunk::OpCode::*, chunk::*, value::*, debug::*,DEBUG_TRACE_EXEC};

pub struct Vm {
    chunk: Chunk,
    ip: usize,
}

pub const VM: Vm = Vm::new();

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileErr,
    InterpretRuntimeErr,
}

impl Vm {
    pub const fn new() -> Self {
        Self {
            chunk: Chunk {
                count: 0,
                capacity: 0,
                code: vec![],
                lines: vec![],
                constants: ValueArray::new(),
            },
            ip: 0,
        }
    }

    pub fn init() -> Vm {
        Vm::new()
    }

    pub fn free(self: Self) {}

    fn read_byte(&mut self) -> Option<u8> {
        if self.ip < self.chunk.code.len() {
            let byte = self.chunk.code[self.ip];
            self.ip += 1;
            Some(byte)
        } else {
            None
        }
    }

    fn read_constant(&mut self) {
        self.chunk.constants.values[read_byte(self)]
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = self.chunk.code[self.ip] as usize;
        return Self::run(self);
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            if let Some(instruction) = self.read_byte() {
                if DEBUG_TRACE_EXEC {
                    disassemble_instruction(self.chunk, (self.ip - self.chunk.code))
                }
                match instruction {
                    OpReturn => {
                        return InterpretResult::INTERPRET_OK;
                    }
                    OpConstant => {
                        let constant: Value = self.read_constant();
                        Value::print(constant);
                        println!("");
                        break;
                    }
                    // Handle other opcodes as needed
                    _ => {
                        // Handle unknown opcode or implement other cases
                        unimplemented!("Opcode not implemented: {}", instruction);
                    }
                }
            } else {
                break; // End of bytecode
            }
        }
    }
}