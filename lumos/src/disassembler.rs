use super::chunk;
use super::chunk::OpCode;

pub fn disassemble_chunk(c: &mut chunk::Chunk, name: &str) {
	println!("== {} ==", name);

	let mut chunk_iter = c.code.iter().enumerate();
	
	while let Some((offset, instruction)) = chunk_iter.next() {
		for _i in 0..disassemble_instruction(c, offset, instruction) {
			chunk_iter.next();
		}
	}
}

pub fn disassemble_instruction(c: &chunk::Chunk, offset: usize, instruction: &usize) -> u32
{
	print!("{:04} ", offset);

	if offset > 0 && c.lines[offset] == c.lines[offset - 1] {
		print!("   | ");
	} else {
		print!("{:04} ", c.lines[offset]);
	}

	match instruction {
		instruction if instruction == &(OpCode::Return as usize) => {
			println!("OP_RETURN");
		},
		instruction if instruction == &(OpCode::Constant as usize) => {
			let constant = c.code[offset + 1];
			println!("{:-16} {:04} '{}'", "OP_CONSTANT", constant, c.constants.values[constant]);
			return 1;
		}
		_ => {
			println!("Unknown OpCode {}", instruction);
		}
	}

	0
}
