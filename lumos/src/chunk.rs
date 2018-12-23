use super::value;

// All possible bytecode values
pub enum OpCode {
	Constant,
	Return,
}


// Represents a chunk of bytecode
pub struct Chunk {
	pub code: Vec<usize>,
	pub constants: value::ValueArray,
	pub lines: Vec<u32>,
}

impl Chunk {
	// Creates a new Chunk object
	pub fn new() -> Chunk {
		Chunk {
			code: vec![],
			constants: value::ValueArray::new(),
			lines: vec![],
		}
	}

	// Writes a byte to the chunk
	pub fn write_byte(&mut self, byte: usize, line: u32) {
		self.code.push(byte);
		self.lines.push(line);
	}

	// Writes a constant to the chunk
	pub fn add_constant(&mut self, val: value::Value) -> usize {
		self.constants.write_value(val);
		self.constants.values.len() - 1
	}
}
