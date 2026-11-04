use std::fmt;

#[derive(Debug)]
pub enum Value {
	Nil,
	Bool(bool),
	Number(f64),
}

impl Value {

	pub fn clone(&self) -> Self {
		match self {
			Value::Nil => Value::Nil,
			Value::Bool(bool_val) => Value::Bool(*bool_val),
			Value::Number(num_val) => Value::Number(*num_val),
		}
	}

}

impl fmt::Display for Value {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Value::Nil => write!(f, "nil"),
			Value::Bool(bool_val) => write!(f, "{}", bool_val),
			Value::Number(num_val) => write!(f, "{}", num_val),
		}
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
