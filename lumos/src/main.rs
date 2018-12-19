use lumos::chunk::*;
use lumos::disassembler;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write_byte(OpCode::Return);
    disassembler::disassemble_chunk(chunk, "Test Chunk");
}
