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

	/// Writes a constant value into the chunk: i.e. an [Op::Constant] followed by an index
	/// The index written is either 1 byte long (if <= 127) or 4 bytes long (if > 127)
	/// Indices > 127 are encoded as a big-endian u31 in 4 bytes where the first bit is always 1
	pub fn write_constant(&mut self, value: Value, line: u32) {
		self.constants.write(value);
		self.write(Op::Constant as u8, line);
		let const_index = self.constants.values.len() - 1;
		if const_index <= 127 {
			self.write(const_index as u8, line);
			return;
		}
		let bytes = const_index.to_be_bytes();
		for i in 0..4 {
			let mut byte = bytes[bytes.len() - i];
			if i == 3 {
				byte |= 1 << 7;
			}
			self.write(byte, line);
		}
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
