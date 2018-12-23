use super::chunk::{ Chunk, OpCode };
use super::disassembler::disassemble_instruction;
use super::DEBUG_TRACE_EXECUTION;

pub enum InterpretResult {
	Ok,
	CompileError,
	RuntimeError,
}


pub struct VM {
	pub chunk: Chunk,
	// pub ip: Option<Vec<usize>>,
}

impl VM {
	pub fn new() -> VM {
		VM {
			chunk: Chunk::new(),
			// ip: None,
		}
	}

	pub fn interpret(&mut self, c: Chunk) -> InterpretResult {
		self.chunk = c;
		// self.ip = Some(self.chunk.code);

		self.run()
	}

	fn run(&mut self) -> InterpretResult {
		let mut ip = self.chunk.code.iter().enumerate();
		
		loop {
			let instruction = ip.next();
			
			let (offset, instruction) = match instruction {
				Some((i, t)) => (i, t),
				None => {
					println!("Could not find instruction.");
					return InterpretResult::CompileError;
				}
			};
			
			if DEBUG_TRACE_EXECUTION {
				disassemble_instruction(&self.chunk, offset, instruction);
			}

			match instruction {
				instruction if instruction == &(OpCode::Return as usize) => return InterpretResult::Ok,
				instruction if instruction == &(OpCode::Constant as usize) => {
					let next = match ip.next() {
						Some((_i, t)) => t.clone(),
						None => {
							println!("Could not find constant instruction.");
							return InterpretResult::CompileError;
						}
					};

					let constant = self.chunk.constants.values[next];
					println!("{}", constant);
				}
				_ => {
					return InterpretResult::CompileError;
				}
			}
		}
	}
}
