use lumos::chunk::*;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write_byte(OpCode::Return);
}
