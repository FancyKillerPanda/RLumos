use super::value;

pub enum OpCode {
	Constant,
	Return,
}


pub struct Chunk {
	pub code: Vec<usize>,
	pub constants: value::ValueArray,
	pub lines: Vec<u32>,
}

impl Chunk {
	pub fn new() -> Chunk {
		Chunk {
			code: vec![],
			constants: value::ValueArray::new(),
			lines: vec![],
		}
	}

	pub fn write_byte(&mut self, byte: usize, line: u32) {
		self.code.push(byte);
		self.lines.push(line);
	}

	pub fn add_constant(&mut self, val: value::Value) -> usize {
		self.constants.write_value(val);
		self.constants.values.len() - 1
	}
}
