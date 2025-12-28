use std::fmt;

#[derive(Clone)] #[derive(Copy)]
pub struct Value {

	pub value: f64,

}

impl Value {

	pub fn new(value: f64) -> Self {
		Self { value: value }
	}

}

impl fmt::Display for Value {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.value)
	}

}

pub struct ValueArray {

	pub values: Vec<Value>,

}

impl ValueArray {

	pub fn new() -> Self {
		ValueArray { values: Vec::with_capacity(8) }
	}

	pub fn write(&mut self, value: Value) {
		self.values.push(value);
	}

}
