use std::fmt;

#[derive(Debug)]
pub enum Value {
	Nil,
	Bool(bool),
	Number(f64),
}

impl Value {

	pub fn clone(&self) -> Self {
		unsafe { std::mem::transmute_copy(self) }
	}

	pub fn is_falsey(&self) -> bool {
		match self {
			Value::Nil => true,
			Value::Bool(bool_val) => !bool_val,
			_ => false
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn copy_should_create_new_value() {
		let number = 50.0;
		let sut = Value::Number(number);

		let result = sut.clone();

		assert!(!std::ptr::eq(std::ptr::from_ref(&sut), std::ptr::from_ref(&result)));
		match result {
			Value::Number(clone_number) => assert_eq!(number, clone_number),
			_ => unreachable!()
		}
	}

	#[test]
	fn is_falsey_should_return_expected_result() {
		let values = [
			(Value::Nil, true),
			(Value::Bool(true), false),
			(Value::Bool(false), true),
			(Value::Number(0.0), false),
			(Value::Number(-1.25), false),
		];

		for (sut, expected_result) in values {
			let result = sut.is_falsey();

			assert_eq!(result, expected_result);
		}
	}

}
