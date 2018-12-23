use super::chunk::{ Chunk, OpCode };
use super::disassembler::disassemble_instruction;
use super::DEBUG_TRACE_EXECUTION;

// Result from interpretation
pub enum InterpretResult {
	Ok,
	CompileError,
	RuntimeError,
}


// Represents the virtual machine
pub struct VM {
	pub chunk: Chunk,
}

impl VM {
	// Creates a new VM object
	pub fn new() -> VM {
		VM {
			chunk: Chunk::new(),
		}
	}

	// Sets up the VM for interpreting
	pub fn interpret(&mut self, c: Chunk) -> InterpretResult {
		self.chunk = c;
		self.run()
	}

	// Handles the interpretation
	fn run(&mut self) -> InterpretResult {
		// Iterator through the chunk's code
		let mut ip = self.chunk.code.iter().enumerate();
		
		loop {
			let instruction = ip.next();
			
			// Checks if next instruction exists
			let (offset, instruction) = match instruction {
				Some((i, t)) => (i, t),
				None => {
					println!("Could not find instruction.");
					return InterpretResult::CompileError;
				}
			};
			
			// Disassembles each instruction if debugging
			if DEBUG_TRACE_EXECUTION {
				disassemble_instruction(&self.chunk, offset, instruction);
			}

			match instruction {
				// Return
				instruction if instruction == &(OpCode::Return as usize) => return InterpretResult::Ok,
				// Constant
				instruction if instruction == &(OpCode::Constant as usize) => {
					// Tries to find the constant instruction
					let next = match ip.next() {
						Some((_i, t)) => t.clone(),
						None => {
							println!("Could not find constant instruction.");
							return InterpretResult::CompileError;
						}
					};

					// Retrieves the constant
					let constant = self.chunk.constants.values[next];

					// Temporarily prints the constant
					println!("{}", constant);
				}
				// Unknown (should be unreachable)
				_ => {
					return InterpretResult::CompileError;
				}
			}
		}
	}
}
