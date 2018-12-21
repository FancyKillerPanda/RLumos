use lumos::chunk::*;
use lumos::disassembler;

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write_byte(OpCode::Constant as usize, 123);
    chunk.write_byte(constant, 123);

    chunk.write_byte(OpCode::Return as usize, 123);

    disassembler::disassemble_chunk(chunk, "Test Chunk");
}
