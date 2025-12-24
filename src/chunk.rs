use std::convert::TryFrom;

use crate::value::Value;
use crate::value::ValueArray;

const LONG_CONSTANT_OFFSET_MASK: u32 = u32::MAX & (u32::MAX & (1 << 31));

pub enum Op {
	Constant = 0x00,
	Return = 0x01,
}

impl TryFrom<u8> for Op {
	type Error = ();

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0x00 => Ok(Op::Constant),
			0x01 => Ok(Op::Return),
			_ => Err(()),
		}
	}
}

pub struct Chunk {

	pub code: Vec<u8>,

	pub lines: Vec<u32>,

	pub constants: ValueArray,

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

	pub fn add_constant(&mut self, value: Value) -> usize {
		self.constants.write(value);
		self.constants.values.len() - 1
	}

	/// Assumes the given offset describes the location of a constant in the code and returns its value
	/// Returns a tuple containing the value and the next code offset
	pub fn extract_constant(&self, offset: usize) -> (Value, usize) {
		let const_offset = self.code[offset];
		if (0b_1000_0000 & const_offset) > 0 {
			let const_offset_bytes: [u8;4] = [
				self.code[offset],
				self.code[offset + 1],
				self.code[offset + 2],
				self.code[offset + 3],
			];
			let long_const_offset = u32::from_be_bytes(const_offset_bytes) & LONG_CONSTANT_OFFSET_MASK;
			return (self.constants.values[long_const_offset as usize], offset + 4);
		}
		(self.constants.values[const_offset as usize], offset + 1)
	}

}
