use std::ops::Range;

use crate::rle::RunLengthEncoder;
use crate::op::*;
use crate::value::Value;
use crate::value::ValueArray;

const MAX_CONSTANTS: usize = 0xffff_ffff_ffff;

pub struct Chunk {

	pub code: Vec<u8>,

	lines: RunLengthEncoder<u32>,

	constants: ValueArray,

}

impl Chunk {

	pub fn new() -> Chunk {
		Chunk {
			code: Vec::with_capacity(8),
			lines: RunLengthEncoder::<u32>::new(),
			constants: ValueArray::new(),
		}
	}

	/// Returns pointers to the start and end of the code range
	pub fn get_code_pointer_range(&self) -> Range<*const u8> {
		let start_ptr = self.code.as_ptr();
		Range {
			start: start_ptr,
			// Safety: ptr cannot go beyond range assigned in self.code
			end: unsafe { start_ptr.add(self.code.len()) }
		}
	}

	pub fn find_line(&self, offset: usize) -> Option<u32> {
		match self.lines.find(offset) {
			Some(v) => Some(*v),
			None => None
		}
	}

	pub fn write(&mut self, byte: u8, line: u32) {
		self.code.push(byte);
		self.lines.add(line);
	}

	/// Writes a constant value into the chunk: either [Op::Constant] followed by a one byte index or
	/// [Op::ConstantLong] followed by a three byte index
	pub fn write_constant(&mut self, value: Value, line: u32) {
		self.constants.write(value);
		let const_index = self.constants.values.len() - 1;
		if const_index > MAX_CONSTANTS {
			panic!("Cannot write constant to chunk, maximum reached");
		}
		if const_index < u8::MAX as usize {
			self.write(OP_CONSTANT, line);
			self.write(const_index as u8, line);
			return;
		}
		self.write(OP_CONSTANT_LONG, line);
		let bytes = const_index.to_be_bytes();
		for i in 1..4 {
			self.write(bytes[bytes.len() - i], line);
		}
	}

	/// Returns the constant associated with the given "constant id", panics if it doesn't exist
	pub fn get_constant(&self, const_id: usize) -> Value {
		self.constants.values[const_id]
	}

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn write_constant_should_use_op_constant_long_when_exceeding_short_limit() {
		let short_limit = u8::MAX as usize;
		let mut chunk = Chunk::new();

		for i in 0..(short_limit + 1) {
			chunk.write_constant(1.0, i as u32);
		}

		// result should be 255 OP_CONSTANT's followed by 1 byte indices (510 bytes total)
		// then a single OP_CONSTANT_LONG followed by a 3 byte index (4 bytes addition)
		assert_eq!(chunk.code.len(), (short_limit * op_size(OP_CONSTANT)) + op_size(OP_CONSTANT_LONG));
		assert_eq!(chunk.constants.values.len(), short_limit + 1);
		assert_eq!(chunk.code[chunk.code.len() - 4], OP_CONSTANT_LONG);
	}

	#[test]
	fn find_line_should_return_line() {
		let mut chunk = Chunk::new();
		chunk.write(0u8, 1);
		chunk.write(0u8, 1);
		chunk.write(0u8, 1);
		chunk.write(0u8, 2);

		assert_eq!([
			chunk.find_line(0).unwrap(),
			chunk.find_line(1).unwrap(),
			chunk.find_line(2).unwrap(),
			chunk.find_line(3).unwrap(),
		], [ 1, 1, 1, 2 ]);
	}

	#[test]
	fn find_line_should_return_none_when_going_oob() {
		let chunk = Chunk::new();

		assert!(chunk.find_line(0).is_none());
	}

}
