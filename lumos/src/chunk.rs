pub enum OpCode {
	Return,
}


pub struct Chunk {
	pub code: Vec<OpCode>,
}

impl Chunk {
	pub fn new() -> Chunk {
		Chunk {
			code: vec![]
		}
	}

	pub fn write_byte(&mut self, byte: OpCode) {
		self.code.push(byte);
	}
}
