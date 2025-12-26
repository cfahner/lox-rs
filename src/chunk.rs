use std::convert::TryFrom;
use std::fmt;
use std::ops::Range;

use crate::rle::RunLengthEncoder;
use crate::value::Value;
use crate::value::ValueArray;

const MAX_CONSTANTS: usize = 0xffff_ffff_ffff;

// repr(u8) is required for std::mem::transmute(), otherwise Rust might encode the enum as another type
#[repr(u8)] #[derive(Debug)]
pub enum Op {
	Constant = 0x00,
	Return = 0x01,

	// opcodes that are not part of the default set go below this line

	ConstantLong = 0x02,
}

impl Op {

	/// Returns the size in bytes of the opcode + operands
	pub fn size(&self) -> isize {
		match self {
			Op::Constant => 2,
			Op::ConstantLong => 4,
			_ => 1
		}
	}

}

impl fmt::Display for Op {

	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", match self {
			Op::Constant => "OP_CONSTANT",
			Op::Return => "OP_RETURN",
			Op::ConstantLong => "OP_CONSTANT_LONG",
			_ => "OP_UNKNOWN"
		})
	}

}

impl TryFrom<u8> for Op {
	type Error = ();

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0x00 => Ok(Op::Constant),
			0x01 => Ok(Op::Return),
			0x02 => Ok(Op::ConstantLong),
			_ => Err(()),
		}
	}
}

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
		// Safety: its actually not unsafe until the end pointer is dereferenced
		Range {
			start: start_ptr,
			end: unsafe { start_ptr.add(self.code.len() * size_of::<u8>()) }
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
			self.write(Op::Constant as u8, line);
			self.write(const_index as u8, line);
			return;
		}
		self.write(Op::ConstantLong as u8, line);
		let bytes = const_index.to_be_bytes();
		for i in 1..4 {
			self.write(bytes[bytes.len() - i], line);
		}
	}

	/// Returns a tuple containing the decoded value at the given offset and the next code offset
	pub fn extract_constant_short(&self, offset: usize) -> (Value, usize) {
		(self.constants.values[self.code[offset] as usize], offset + 1)
	}

	/// Returns a tuple containing the decoded value at the given offset and the next code offset
	pub fn extract_constant_long(&self, offset: usize) -> (Value, usize) {
		let const_index_bytes: [u8;4] = [
			0, self.code[offset], self.code[offset + 1], self.code[offset + 2]
		];
		let const_index = u32::from_be_bytes(const_index_bytes);
		(self.constants.values[const_index as usize], offset + 3)
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
		assert_eq!(chunk.code.len(), (short_limit * 2) + 4);
		assert_eq!(chunk.constants.values.len(), short_limit + 1);
		assert_eq!(chunk.code[chunk.code.len() - 4], Op::ConstantLong as u8);
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
