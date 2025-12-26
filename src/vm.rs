use std::ops::Range;

use crate::chunk::Chunk;
use crate::chunk::Op;

/// Possible error cases during chunk interpreting
pub enum InterpretError {
	/// Occurs when a chunk was badly formatted (eg. the bytes don't match with opcodes + operands)
	BadChunk,
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
		if (ptr_range.is_empty()) {
			return Ok(());
		}
		self.run(ptr_range)
	}

	/// Runs the instructions present in the given range of pointers
	fn run(&mut self, ptr_range: Range<*const u8>) -> Result<(), InterpretError> {
		// ip is modified a lot and so is kept as a local variable to keep it close / cacheable
		let Range { start: mut ip, end: end_ptr } = ptr_range;
		loop {
			// Safety: ip is never beyond end_ptr at the start of the loop
			let opcode = unsafe { *ip };
			// since this is very hot code, transmute avoids the overhead of Op::try_from()
			// Safety: transmute could create invalid op -> check if operands don't go beyond end_ptr
			let op: Op = unsafe { std::mem::transmute(opcode) };
			// Safety: update and check next ip first to prevent an unsafe ptr dereference
			unsafe { ip = ip.offset(op.size()); }
			if ip > end_ptr {
				return Err(InterpretError::BadChunk); // ip went out of bounds
			}
			println!("{opcode:02x}");
			if ip >= end_ptr { // ip can't be greater than, but greater-check is added for safety
				break;
			}
		}
		Ok(())
	}

}
