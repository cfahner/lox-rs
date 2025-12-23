use std::convert::TryFrom;

use crate::value::Value;
use crate::value::ValueArray;

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

}
