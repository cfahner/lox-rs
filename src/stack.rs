use crate::value::Value;

pub struct Stack<const N_STACK_SIZE: usize> {
	values: [Value; N_STACK_SIZE],
	top: usize,
}

impl<const N_STACK_SIZE: usize> Stack<N_STACK_SIZE> {

	pub fn new() -> Self {
		Self { values: [const { Value::Nil }; N_STACK_SIZE], top: 0 }
	}

	pub fn push(&mut self, value: Value) {
		if self.top >= N_STACK_SIZE {
			panic!("Stack overflow");
		}

		self.values[self.top] = value;
		self.top += 1;
	}

	pub fn pop(&mut self) -> &Value {
		if self.top == 0 {
			panic!("Stack underflow");
		}

		self.top -= 1;
		&self.values[self.top]
	}

	pub fn replace_or_err<TErr>(&mut self, replacer: impl FnOnce(&Value) -> Result<Value, TErr>) -> Result<(), TErr> {
		if self.top == 0 {
			panic!("Stack underflow");
		}

		self.values[self.top - 1] = replacer(&self.values[self.top - 1])?;
		Ok(())
	}

	pub fn replace_two_or_err<TErr>(&mut self, replacer: impl FnOnce(&Value, &Value) -> Result<Value, TErr>) -> Result<(), TErr> {
		if self.top < 2 {
			panic!("Stack underflow");
		}

		let new_value = replacer(&self.values[self.top - 2], &self.values[self.top -1])?;
		self.top -= 1;
		self.values[self.top - 1] = new_value;
		Ok(())
	}

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test] #[should_panic]
	fn push_should_panic_when_stack_full() {
		let mut sut = Stack::<1>::new();

		sut.push(Value::Nil);
		sut.push(Value::Nil);
	}

	#[test]
	fn pop_should_return_last_pushed_item() {
		let expected_result = 84.0;
		let mut sut = Stack::<4>::new();
		sut.push(Value::Number(0.0));
		sut.push(Value::Number(expected_result));

		let result = sut.pop();

		assert_eq!(&expected_result, match result { Value::Number(num) => num, _ => unreachable!() });
	}

	#[test] #[should_panic]
	fn pop_should_panic_when_stack_empty() {
		let mut sut = Stack::<2>::new();

		let _ = sut.pop();
	}

	#[test]
	fn replace_should_modify_top_value() {
		let initial_number = 1.0;
		let mut sut = Stack::<4>::new();
		sut.push(Value::Number(0.0));
		sut.push(Value::Bool(true));
		sut.push(Value::Number(initial_number));

		let _ = sut.replace_or_err::<()>(|v| match v {
			Value::Number(num) => Ok(Value::Number(num + 1.0)),
			_ => unreachable!()
		});
		let result = sut.pop();

		assert_eq!(initial_number + 1.0, match result { Value::Number(num) => *num, _ => unreachable!() });
	}

	#[test] #[should_panic]
	fn replace_should_panic_when_stack_empty() {
		let mut sut = Stack::<4>::new();

		let _ = sut.replace_or_err::<()>(|_| Ok(Value::Nil));
	}

	#[test]
	fn replace_two_should_modify_top_two_values() {
		let mut sut = Stack::<4>::new();
		sut.push(Value::Number(1.0));
		sut.push(Value::Number(1.0));

		let _ = sut.replace_two_or_err::<()>(|a, b| match (a, b) {
			(Value::Number(num_a), Value::Number(num_b)) => Ok(Value::Number(*num_a + *num_b)),
			_ => unreachable!()
		});
		let result = sut.pop();

		assert_eq!(2.0, match result { Value::Number(num) => *num, _ => unreachable!() });
	}

	#[test] #[should_panic]
	fn replace_two_should_panic_when_stack_empty() {
		let mut sut = Stack::<4>::new();

		let _ = sut.replace_two_or_err::<()>(|_, _| Ok(Value::Nil));
	}

	#[test] #[should_panic]
	fn replace_two_should_panic_when_stack_contains_single_value() {
		let mut sut = Stack::<4>::new();
		sut.push(Value::Nil);

		let _ = sut.replace_two_or_err::<()>(|_, _| Ok(Value::Nil));
	}

}
