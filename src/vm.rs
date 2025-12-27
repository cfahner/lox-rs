use std::ops::Range;

use crate::chunk::Chunk;
use crate::op::*;

/// Possible error cases during chunk interpreting
#[derive(PartialEq)] #[derive(Debug)]
pub enum InterpretError {
	/// Occurs when a chunk was badly formatted (eg. the bytes don't match with opcodes + operands)
	BadChunk,
}

#[cfg(feature = "trace")]
fn trace_op(chunk: &Chunk, ptr: *const u8) {
	let start_ptr = chunk.code.as_ptr();
	// Safety: run() loop already checks if ip is safe relative to start_ptr
	crate::debug::disassemble_instruction(chunk, unsafe { ptr.offset_from(start_ptr) } as usize);
}

pub struct VM { }

impl VM {

	pub fn new() -> Self {
		Self { }
	}

	pub fn interpret(&mut self, chunk: &Chunk) -> Result<(), InterpretError> {
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
				trace_op(chunk, ip);
			}
			// Safety: update and check next ip first to prevent an unsafe ptr dereference
			unsafe { ip = ip.add(op_size(opcode)); }
			if ip > end_ptr {
				return Err(InterpretError::BadChunk); // ip went out of bounds
			}
			match opcode {
				OP_CONSTANT => self.op_constant(chunk, op_ptr),
				OP_CONSTANT_LONG => self.op_constant_long(chunk, op_ptr),
				OP_RETURN => return Ok(()),
				_ => return Err(InterpretError::BadChunk)
			}
			if ip >= end_ptr { // ip can't be greater than, but greater-check is added for safety
				break;
			}
		}
		Ok(())
	}

	#[inline]
	fn op_constant(&self, chunk: &Chunk, ptr: *const u8) {
		// Safety: run() loop has already checked safety of ptr
		let const_id = unsafe { *ptr.add(1) };
		println!("Constant: '{}'", chunk.get_constant(const_id as usize));
	}

	#[inline]
	fn op_constant_long(&self, chunk: &Chunk, ptr: *const u8) {
		// Safety: run() loop has already checked safety of ptr
		let const_id_bytes: [u8;4] = unsafe { [ 0, *ptr.add(1), *ptr.add(2), *ptr.add(3) ] };
		let const_id = u32::from_be_bytes(const_id_bytes);
		println!("Constant: '{}'", chunk.get_constant(const_id as usize));
	}

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn interpret_should_error_on_malformed_chunk() {
		let mut sut = VM::new();
		let mut chunk = Chunk::new();
		chunk.write(OP_CONSTANT, 1); // OP_CONSTANT is normally followed by one byte of constant id

		let result = sut.interpret(&chunk);

		assert_eq!(result, Result::Err(InterpretError::BadChunk));
	}
}
