use crate::chunk::*;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset: usize = 0;
    while offset < chunk.count {
        offset = disassemble_instruction(chunk, offset);
    }
}

#[allow(unreachable_patterns)]
pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("| ");
    } else {
        print!("{} ", chunk.lines[offset]);
    }

    let instruction: OpCode = chunk.code[offset].try_into().unwrap();
    match instruction {
        OpCode::Return => {
            simple_instruction("OP_RETURN", offset)
        }
        OpCode::Constant => {
            constant_instruction("OP_CONSTANT".to_string(), chunk, offset)
        }
        OpCode::Negate => {
            simple_instruction("OP_NEGATE", offset)
        }
        OpCode::Add => {
            simple_instruction("OP_ADD", offset)
        }
        OpCode::Sub => {
            simple_instruction("OP_SUB", offset)
        }
        OpCode::Mul => {
            simple_instruction("OP_MUL", offset)
        }
        OpCode::Div => {
            simple_instruction("OP_DIV", offset)
        }
        _ => {
            eprintln!("Unknown code!");
            offset + 1
        }
    }
}

pub fn simple_instruction(name: &str, offset: usize) -> usize{
    print!("{name}\n");
    offset + 1
}

pub fn constant_instruction(name: String, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    print!("{:<16} {:4} ", name, constant);
    chunk.constants.values[constant as usize].print();
    print!("\n");
    offset + 2
}
