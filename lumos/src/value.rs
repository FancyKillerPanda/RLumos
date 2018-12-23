pub type Value = f64;

// An array of values
pub struct ValueArray {
	pub values: Vec<Value>,
}

impl ValueArray {
	// Creates a new ValueArray object
	pub fn new() -> ValueArray {
		ValueArray {
			values: vec![],
		}
	}

	// Writes a value to the array
	pub fn write_value(&mut self, value: Value) {
		self.values.push(value);
	}
}
