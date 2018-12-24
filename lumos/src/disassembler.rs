use super::chunk;
use super::chunk::OpCode;

// Disassembles an entire chunk of bytecode
pub fn disassemble_chunk(c: &mut chunk::Chunk, name: &str) {
	// Outputs the name of the chunk to the console
	println!("== {} ==", name);

	let mut chunk_iter = c.code.iter().enumerate();
	
	// Disassembles one instruction at a time
	while let Some((offset, instruction)) = chunk_iter.next() {
		// Loops to add extra iterations to the chunk after some instructions
		// For example, OP_CONSTANT instructions need to skip the next
		// instruction (which is the location of the constant in the chunk).
		for _i in 0..disassemble_instruction(c, offset, instruction) {
			chunk_iter.next();
		}
	}
}

// Disassembles a single bytecode instruction
pub fn disassemble_instruction(c: &chunk::Chunk, offset: usize, instruction: &usize) -> u32
{
	// Prints the location (e.g. 0000 or 0001) of the instruction
	print!("{:04} ", offset);

	// Prints the line number (e.g. "0001" for a new line or "   |" for
	// the same line again)
	if offset > 0 && c.lines[offset] == c.lines[offset - 1] {
		print!("   | ");
	} else {
		print!("{:04} ", c.lines[offset]);
	}

	match instruction {
		// Return
		instruction if instruction == &(OpCode::Return as usize) => {
			println!("OP_RETURN");
		},
		// Constant
		instruction if instruction == &(OpCode::Constant as usize) => {
			let constant = c.code[offset + 1];
			println!("{:-16} {:04} '{}'", "OP_CONSTANT", constant, c.constants.values[constant]);
			return 1;
		},
		// Negation
		instruction if instruction == &(OpCode::Negate as usize) => {
			println!("OP_NEGATE");
		}
		// Unknown (should be unreachable)
		_ => {
			println!("Unknown OpCode {}", instruction);
		}
	}

	0
}
