use lumos::chunk::*;
use lumos::vm::VM;

fn main() {
    let mut vm = VM::new();

    // Creates a test chunk and writes to it
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write_byte(OpCode::Constant as usize, 123);
    chunk.write_byte(constant, 123);
    chunk.write_byte(OpCode::Return as usize, 123);

    // Interprets the bytecode
    vm.interpret(chunk);
}
