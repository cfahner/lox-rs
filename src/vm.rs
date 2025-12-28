use std::ops::Range;

use crate::chunk::Chunk;
use crate::op::*;
use crate::value::Value;

/// Possible error cases during chunk interpreting
#[derive(PartialEq)] #[derive(Debug)]
pub enum InterpretError {
	/// Occurs when a chunk was badly formatted (eg. the bytes don't match with opcodes + operands)
	BadChunk,
}

pub struct VM<const N_STACK_SIZE: usize> {

	stack: [Value;N_STACK_SIZE],

	stack_top: *mut Value,

}

impl<const N_STACK_SIZE: usize> VM<N_STACK_SIZE> {

	pub fn new() -> Self {
		Self { stack: [Value::new(0.0);N_STACK_SIZE], stack_top: std::ptr::null_mut() }
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
			match opcode {
				OP_CONSTANT => self.op_constant(chunk, op_ptr),
				OP_ADD => self.op_add(),
				OP_SUBTRACT => self.op_subtract(),
				OP_MULTIPLY => self.op_multiply(),
				OP_DIVIDE => self.op_divide(),
				OP_NEGATE => self.op_negate(),
				OP_RETURN => self.op_return(),
				OP_CONSTANT_LONG => self.op_constant_long(chunk, op_ptr),
				_ => return Err(InterpretError::BadChunk)
			}
			if ip >= end_ptr { // ip can't be greater than, but greater-check is added for safety
				break;
			}
		}
		Ok(())
	}

	#[inline]
	fn op_constant(&mut self, chunk: &Chunk, ptr: *const u8) {
		// Safety: run() loop has already checked safety of ptr
		let const_id = unsafe { *ptr.add(1) };
		self.stack_push(chunk.get_constant(const_id as usize).clone());
	}

	#[inline]
	fn op_add(&mut self) {
		let b = self.stack_pop();
		unsafe { (*self.stack_top.offset(-1)).add(&b); }
	}

	#[inline]
	fn op_subtract(&mut self) {
		let b = self.stack_pop();
		unsafe { (*self.stack_top.offset(-1)).subtract(&b); }
	}

	#[inline]
	fn op_multiply(&mut self) {
		let b = self.stack_pop();
		unsafe { (*self.stack_top.offset(-1)).multiply(&b); }
	}

	#[inline]
	fn op_divide(&mut self) {
		let b = self.stack_pop();
		unsafe { (*self.stack_top.offset(-1)).divide(&b); }
	}

	#[inline]
	fn op_negate(&mut self) {
		unsafe { (*self.stack_top.offset(-1)).negate(); }
	}

	#[inline]
	fn op_return(&mut self) {
		println!("{}", self.stack_pop());
	}

	#[inline]
	fn op_constant_long(&mut self, chunk: &Chunk, ptr: *const u8) {
		// Safety: run() loop has already checked safety of ptr
		let const_id_bytes: [u8;4] = unsafe { [ 0, *ptr.add(1), *ptr.add(2), *ptr.add(3) ] };
		let const_id = u32::from_be_bytes(const_id_bytes);
		self.stack_push(chunk.get_constant(const_id as usize).clone());
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
	fn stack_pop(&mut self) -> Value {
		if self.stack_top == self.stack.as_mut_ptr() {
			panic!("Stack underflow");
		}
		unsafe {
			self.stack_top = self.stack_top.offset(-1);
			*self.stack_top
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
			chunk.write_constant(Value::new(1.0), 1);
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

}
