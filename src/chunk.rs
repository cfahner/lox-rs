use std::convert::TryFrom;

use crate::value::Value;
use crate::value::ValueArray;

const MAX_CONSTANTS: usize = 0xffff_ffff_ffff;

pub enum Op {
	Constant = 0x00,
	Return = 0x01,

	// opcodes that are not part of the default set go below this line

	ConstantLong = 0x02,
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

	pub lines: Vec<u32>,

	constants: ValueArray,

}

impl Chunk {

	pub fn new() -> Chunk {
		Chunk {
			code: Vec::with_capacity(8),
			lines: Vec::with_capacity(8),
			constants: ValueArray::new(),
		}
	}

	pub fn write(&mut self, byte: u8, line: u32) {
		self.code.push(byte);
		self.lines.push(line);
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
		for i in 0..3 {
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
