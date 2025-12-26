use std::ops::Range;

use crate::chunk::Chunk;
use crate::chunk::Op;

pub enum InterpretError { }

pub struct VM { }

impl VM {

	pub fn new() -> Self {
		Self { }
	}

	pub fn interpret(&mut self, chunk: &Chunk) -> Result<(), InterpretError> {
		// apparently dereferencing raw pointers is faster than indexing a vector, so setting up pointers
		// ip is modified a lot and so is kept as a local variable to keep it close / cacheable
		let Range { start: mut ip, end: end_ptr } = chunk.get_code_pointer_range();
		while ip < end_ptr {
			// Safety: ip is never beyond end_ptr at the start of the loop
			let opcode = unsafe { *ip };
			// since this is very hot code, transmute avoids the overhead of Op::try_from()
			// Safety: transmute could result in an unexpected/invalid op -> need to check if operands don't go beyond end_ptr
			let op: Op = unsafe { std::mem::transmute(opcode) };
			println!("{}", opcode);
			// Safety: potentially unsafe when ip is dereferenced oob, which the loop condition guards against
			unsafe { ip = ip.offset(op.size()); }
		}
		Ok(())
	}

}
