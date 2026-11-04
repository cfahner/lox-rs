use std::ops::Range;

use sysexits::ExitCode;

use crate::chunk::Chunk;
use crate::op::*;
use crate::value::Value;

/// Possible error cases during chunk interpreting
#[derive(Debug, PartialEq)]
pub enum InterpretError {

	/// Occurs when a chunk was badly formatted (eg. the bytes don't match with opcodes + operands)
	BadChunk,

	/// Occurs when an unknown opcode was encountered
	UnknownOpcode(u8),

	Compile,

	/// Occurs for invalid operations at runtime (eg. trying to negate a value that is not a number)
	InvalidOperation {
		message: String,
		line: u32,
	},

}

impl InterpretError {

	pub fn to_exit_code(&self) -> ExitCode {
		match self {
			Self::BadChunk | Self::UnknownOpcode(_) => ExitCode::Software,
			Self::Compile => ExitCode::DataErr,
			Self::InvalidOperation { message: _, line: _ } => ExitCode::Software,
		}
	}

}

pub struct VM<const N_STACK_SIZE: usize> {

	stack: [Value;N_STACK_SIZE],

	stack_top: *mut Value,

}

impl<const N_STACK_SIZE: usize> VM<N_STACK_SIZE> {

	pub fn new() -> Self {
		Self { stack: [const { Value::Nil };N_STACK_SIZE], stack_top: std::ptr::null_mut() }
	}

	pub fn interpret(&mut self, chunk: &Chunk) -> Result<(), InterpretError> {
		if self.stack_top.is_null() {
			self.stack_top = self.stack.as_mut_ptr();
		}
		// apparently dereferencing raw pointers is faster than indexing a vector, so setting up pointers
		let ptr_range = chunk.get_code_pointer_range();
		// if range is empty, the first iteration of the loop would trigger undefined behavior
		if ptr_range.is_empty() {
			return Ok(());
		}
		self.run(chunk, ptr_range)
	}

	/// Runs the instructions in the given range of pointers
	fn run(&mut self, chunk: &Chunk, ptr_range: Range<*const u8>) -> Result<(), InterpretError> {
		// ip is modified a lot and so is kept as a local variable to keep it close / cacheable
		let Range { start: mut ip, end: end_ptr } = ptr_range;

		loop {
			// create a copy of the pointer to the opcode with operands
			let op_ptr = ip;
			// Safety: ip is never beyond end_ptr at the start of the loop
			let opcode = unsafe { *ip };
			#[cfg(feature = "trace")] {
				self.trace_op(chunk, ip);
			}
			// Safety: update and check next ip first to prevent an unsafe ptr dereference
			unsafe { ip = ip.add(op_size(opcode)); }
			if ip > end_ptr {
				return Err(InterpretError::BadChunk); // ip went out of bounds
			}

			let op_result = match opcode {
				OP_CONSTANT => self.op_constant(chunk, op_ptr),
				OP_NIL => self.op_push_literal(Value::Nil),
				OP_TRUE => self.op_push_literal(Value::Bool(true)),
				OP_FALSE => self.op_push_literal(Value::Bool(false)),
				OP_EQUAL => self.op_equal(),
				OP_NOT_EQUAL => self.op_not_equal(),
				OP_GREATER => self.op_binary_numeric(|l, r| Value::Bool(l > r)),
				OP_GREATER_EQUAL => self.op_binary_numeric(|l, r| Value::Bool(l >= r)),
				OP_LESS => self.op_binary_numeric(|l, r| Value::Bool(l < r)),
				OP_LESS_EQUAL => self.op_binary_numeric(|l, r| Value::Bool(l <= r)),
				OP_ADD => self.op_binary_numeric(|l, r| Value::Number(l + r)),
				OP_SUBTRACT => self.op_binary_numeric(|l, r| Value::Number(l - r)),
				OP_MULTIPLY => self.op_binary_numeric(|l, r| Value::Number(l * r)),
				OP_DIVIDE => self.op_binary_numeric(|l, r| Value::Number(l / r)),
				OP_NOT => self.op_not(),
				OP_NEGATE => self.op_negate(),
				OP_RETURN => self.op_return(),
				OP_CONSTANT_LONG => self.op_constant_long(chunk, op_ptr),
				_ => return Err(InterpretError::UnknownOpcode(opcode))
			};

			op_result.map_err(|msg| {
				// Safety: offset_from() is called within the same allocation range
				let offset: usize = unsafe { ip.offset_from(ptr_range.start).try_into().unwrap() };
				InterpretError::InvalidOperation {
					message: msg.to_owned(),
					line: chunk.find_line(offset).unwrap()
				}
			})?;

			if ip >= end_ptr { // ip can't be greater than, but greater-check is added for safety
				break;
			}
		}
		Ok(())
	}

	#[inline]
	fn op_constant(&mut self, chunk: &Chunk, ptr: *const u8) -> Result<(), &str> {
		// Safety: run() loop has already checked safety of ptr
		let const_id = unsafe { *ptr.add(1) };
		self.stack_push(chunk.get_constant(const_id as usize).clone());
		Ok(())
	}

	#[inline]
	fn op_push_literal(&mut self, value: Value) -> Result<(), &str> {
		self.stack_push(value);
		Ok(())
	}

	#[inline]
	fn op_equal(&mut self) -> Result<(), &str> {
		self.stack_replace_top_2::<&str>(|l, r| {
			Ok(Value::Bool(l.equals(r)))
		})
	}

	#[inline]
	fn op_not_equal(&mut self) -> Result<(), &str> {
		self.stack_replace_top_2::<&str>(|l, r| {
			Ok(Value::Bool(!l.equals(r)))
		})
	}

	#[inline]
	fn op_binary_numeric(&mut self, operate: impl FnOnce(f64, f64) -> Value) -> Result<(), &str> {
		self.stack_replace_top_2(|l, r| {
			match (l, r) {
				(Value::Number(l_num), Value::Number(r_num)) => {
					Ok(operate(*l_num, *r_num))
				},
				_ => Err("Operands must be numbers.")
			}
		})
	}

	#[inline]
	fn op_not(&mut self) -> Result<(), &str> {
		self.stack_replace_top::<&str>(|v| Ok(Value::Bool(v.is_falsey())))
	}

	#[inline]
	fn op_negate(&mut self) -> Result<(), &str> {
		self.stack_replace_top(|v| {
			match v {
				Value::Number(num) => Ok(Value::Number(-num)),
				_ => Err("Operand must be a number.")
			}
		})
	}

	#[inline]
	fn op_return(&mut self) -> Result<(), &str> {
		println!("{}", self.stack_pop());
		Ok(())
	}

	#[inline]
	fn op_constant_long(&mut self, chunk: &Chunk, ptr: *const u8) -> Result<(), &str> {
		// Safety: run() loop has already checked safety of ptr
		let const_id_bytes: [u8;4] = unsafe { [ 0, *ptr.add(1), *ptr.add(2), *ptr.add(3) ] };
		let const_id = u32::from_be_bytes(const_id_bytes);
		self.stack_push(chunk.get_constant(const_id as usize).clone());
		Ok(())
	}

	#[inline]
	fn stack_push(&mut self, value: Value) {
		if self.stack_top.cast_const() >= self.stack.as_ptr_range().end {
			panic!("Stack overflow");
		}
		unsafe {
			*self.stack_top = value;
			self.stack_top = self.stack_top.add(1);
		}
	}

	#[inline]
	fn stack_pop(&mut self) -> &mut Value {
		self.ensure_stack_filled();

		// Safety: already checked against the floor position above
		unsafe {
			self.stack_top = self.stack_top.offset(-1);
			&mut *self.stack_top
		}
	}

	#[inline]
	fn stack_replace_top_2<TErr>(&mut self, replacer: impl FnOnce(&Value, &Value) -> Result<Value, TErr>) -> Result<(), TErr> {
		self.ensure_stack_filled();

		// Safety: ensure_stack_filled() does the safety check
		unsafe {
			let new_value = replacer(&*self.stack_top.offset(-2), &*self.stack_top.offset(-1))?;
			self.stack_top = self.stack_top.offset(-1);
			*self.stack_top.offset(-1) = new_value;
			Ok(())
		}
	}

	#[inline]
	fn stack_replace_top<TErr>(&mut self, replacer: impl FnOnce(&Value) -> Result<Value, TErr>) -> Result<(), TErr> {
		self.ensure_stack_filled();

		// Safety: ensure_stack_filled() does the safety check
		unsafe {
			*self.stack_top.offset(-1) = replacer(&*self.stack_top.offset(-1))?;
			Ok(())
		}
	}

	#[inline]
	fn ensure_stack_filled(&self) {
		if self.stack_top as *const Value == self.stack.as_ptr() {
			panic!("Stack underflow");
		}
	}

	#[cfg(feature = "trace")]
	fn trace_op(&self, chunk: &Chunk, ptr: *const u8) {
		print!("          ");
		let mut stack_ptr = self.stack.as_ptr();
		while stack_ptr < self.stack_top {
			unsafe {
				print!("[ {} ]", *stack_ptr);
				stack_ptr = stack_ptr.add(1);
			}
		}
		println!();
		let start_ptr = chunk.code.as_ptr();
		// Safety: run() loop already checks if ip is safe relative to start_ptr
		crate::debug::disassemble_instruction(chunk, unsafe { ptr.offset_from(start_ptr) } as usize);
	}

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn interpret_should_error_on_malformed_chunk() {
		let mut sut = VM::<8>::new();
		let mut chunk = Chunk::new();
		chunk.write(OP_CONSTANT, 1); // OP_CONSTANT is normally followed by one byte of constant id

		let result = sut.interpret(&chunk);

		assert_eq!(result, Result::Err(InterpretError::BadChunk));
	}

	#[test] #[should_panic]
	fn interpret_should_panic_on_full_stack() {
		let mut chunk = Chunk::new();
		for i in 0..9 {
			print!("{i:}");
			chunk.write_constant(Value::Number(1.0), 1);
		}
		let mut sut = VM::<8>::new();

		let _ = sut.interpret(&chunk);
	}

	#[test] #[should_panic]
	fn interpret_should_panic_when_popping_empty_stack() {
		let mut chunk = Chunk::new();
		chunk.write(OP_ADD, 1);
		let mut sut = VM::<8>::new();

		let _ = sut.interpret(&chunk);
	}

	#[test] #[should_panic]
	fn interpret_should_panic_when_negating_empty_stack() {
		let mut chunk = Chunk::new();
		chunk.write(OP_NEGATE, 1);
		let mut sut = VM::<8>::new();

		let _ = sut.interpret(&chunk);
	}

}
