use chunk::{Chunk, OpCode::*};
use debug::disassemble_chunk;
use value::*;
use vm::Vm;

mod chunk;
mod debug;
mod memory;
mod value;
mod vm;

pub const DEBUG_TRACE_EXEC: bool = true;
pub const STACK_MAX: usize = 256;


fn main() {
    let mut chunk: Chunk = Chunk::new();
    let mut vm: Vm = Vm::init();

    let constant: u8 = chunk.add_constant(new_value!(3.4));
    chunk.write(Constant as u8, 123);
    chunk.write(constant, 123);

    //chunk.write(OpAdd as u8, 123);

    //constant = chunk.add_constant(new_value!(5.6));
    //chunk.write(OpConstant as u8, 123);
    //chunk.write(constant, 123);

    //chunk.write(OpDiv as u8, 123);
    chunk.write(Negate as u8, 123);


    chunk.write(Return as u8, 123);
    disassemble_chunk(&chunk, "test chunk");
    vm.interpret(&chunk);
    vm.free();
    chunk.free();
}
