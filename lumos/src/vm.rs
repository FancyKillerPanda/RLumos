use std::ops::{ Add, Sub, Mul, Div };
use super::chunk::{ Chunk, OpCode };
use super::disassembler::disassemble_instruction;
use super::value;
use super::compiler;
use super::DEBUG_TRACE_EXECUTION;

// Result from interpretation
pub enum InterpretResult {
	Ok,
	CompileError,
	RuntimeError,
}


// Max number of elements in stack
const STACK_MAX: usize = 256;


// Represents the virtual machine
pub struct VM {
	pub chunk: Chunk,
	pub stack: Vec<value::Value>,
}

impl VM {
	// Creates a new VM object
	pub fn new() -> VM {
		VM {
			chunk: Chunk::new(),
			stack: Vec::with_capacity(STACK_MAX),
		}
	}

	// Sets up the VM for interpreting
	pub fn interpret(&mut self, c: Chunk) -> InterpretResult {
		self.chunk = c;
		self.run()
	}

	// Pushes a value onto the stack
	fn push_stack(&mut self, val: value::Value) {
		self.stack.push(val);
	}

	// Pops off and retrieves to last value on the stack
	fn pop_stack(&mut self) -> value::Value {
		match self.stack.pop() {
			Some(t) => t,
			None => {
				println!("Tried to pop off empty stack.");
				0.0
			}
		}
	}

	// Handles the interpretation
	fn run(&mut self) -> InterpretResult {
		// Iterator through the chunk's code
		let chunk_code = self.chunk.code.clone();
		let mut ip = chunk_code.iter().enumerate();
		
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
				print!("          ");

				for val in &self.stack {
					print!("[ {} ]", val);
				}
				
				println!();
				disassemble_instruction(&self.chunk, offset, instruction);
			}

			// Closure for binary operators
			let mut binary_op = |op: fn(value::Value, value::Value) -> value::Value| {
				let b = self.pop_stack();
				let a = self.pop_stack();
				self.push_stack(op(a, b));
			};

			match instruction {
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

					// Pushes the constant onto the stack
					self.push_stack(constant);
				}
				// Binary Operators
				instruction if instruction == &(OpCode::Add as usize) => binary_op(Add::add),
				instruction if instruction == &(OpCode::Subtract as usize) => binary_op(Sub::sub),
				instruction if instruction == &(OpCode::Multiply as usize) => binary_op(Mul::mul),
				instruction if instruction == &(OpCode::Divide as usize) => binary_op(Div::div),

				// Negation
				instruction if instruction == &(OpCode::Negate as usize) => {
					let val = self.pop_stack();
					self.push_stack(-val);
				},
				// Return
				instruction if instruction == &(OpCode::Return as usize) => {
					println!("{}", self.pop_stack());
					return InterpretResult::Ok;
				},
				// Unknown (should be unreachable)
				_ => {
					return InterpretResult::CompileError;
				}
			}
		}
	}
}

// Interprets a source string
pub fn interpret(source: &String) -> InterpretResult {
	// Compiles the source
	compiler::compile(source);
	
	InterpretResult::Ok
}
