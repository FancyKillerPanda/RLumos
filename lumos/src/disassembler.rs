use super::chunk;
use super::chunk::OpCode;

pub fn disassemble_chunk(c: chunk::Chunk, name: &str) {
	println!("== {} ==", name);

	let mut chunk_iter = c.code.iter().enumerate();
	
	while let Some((offset, instruction)) = chunk_iter.next() {
		print!("{:04} ", offset);

		match instruction {
			OpCode::Return => println!("OP_RETURN"),
			_ => {
				println!("Unknown OpCode {}", instruction as *const OpCode as u32);
			}
		}
	}
}
