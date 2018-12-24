use lumos::chunk::*;
use lumos::vm::VM;

fn main() {
    let mut vm = VM::new();

    // Creates a test chunk and writes to it
    let mut chunk = Chunk::new();
    let mut constant = chunk.add_constant(1.2);
    chunk.write_byte(OpCode::Constant as usize, 123);
    chunk.write_byte(constant, 123);

    constant = chunk.add_constant(3.4);
    chunk.write_byte(OpCode::Constant as usize, 123);
    chunk.write_byte(constant, 123);
    
    chunk.write_byte(OpCode::Add as usize, 123);

    constant = chunk.add_constant(5.6);    
    chunk.write_byte(OpCode::Constant as usize, 123);   
    chunk.write_byte(constant, 123);

    chunk.write_byte(OpCode::Divide as usize, 123);
    
    chunk.write_byte(OpCode::Negate as usize, 123);
    chunk.write_byte(OpCode::Return as usize, 123);

    // Interprets the bytecode
    vm.interpret(chunk);
}
