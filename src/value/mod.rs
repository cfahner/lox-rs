use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub enum Value {
	Nil,
	Bool(bool),
	Number(f64),
	String(Rc<str>),
}

impl Value {

	pub fn new_concatenated_string(str1: &str, str2: &str) -> Self {
		let mut result = str1.to_owned();
		result.push_str(str2);
		Self::String(Rc::from(result))
	}

	pub fn clone(&self) -> Self {
		match self {
			Value::String(rc) => Value::String(rc.clone()),
			// none of the remaining variants have associated data that is heap allocated
			// Safety: cloning self into Self, no other way to interpret the bits
			_ => unsafe { std::mem::transmute_copy(self) }
		}
	}

	pub fn is_falsey(&self) -> bool {
		match self {
			Value::Nil => true,
			Value::Bool(bool_val) => !bool_val,
			_ => false
		}
	}

}

impl PartialEq for Value {

	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Value::Nil, Value::Nil) => true,
			(Value::Bool(bool1), Value::Bool(bool2)) => bool1 == bool2,
			(Value::Number(num1), Value::Number(num2)) => num1 == num2,
			(Value::String(str1), Value::String(str2)) => str1 == str2,
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
			Value::String(str_val) => write!(f, "{}", str_val),
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
		let cases = [
			(Value::Nil, true),
			(Value::Bool(true), false),
			(Value::Bool(false), true),
			(Value::Number(0.0), false),
			(Value::Number(-1.25), false),
		];

		for (sut, expected_result) in cases {
			let result = sut.is_falsey();

			assert_eq!(expected_result, result);
		}
	}

	#[test]
	fn eq_should_return_expected_result() {
		let cases = [
			(Value::Nil, Value::Nil, true),
			(Value::Bool(true), Value::Bool(true), true),
			(Value::Bool(false), Value::Bool(true), false),
			(Value::Nil, Value::Number(0.0), false),
			(Value::Number(1.0), Value::Number(-1.0), false),
			(Value::Number(0.0), Value::Number(0.0), true),
			(Value::Bool(false), Value::Number(0.0), false)
		];

		for (a, b, expected_result) in cases {
			let result = a == b;

			assert_eq!(expected_result, result);
		}
	}

}
