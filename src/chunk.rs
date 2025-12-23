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

	pub constants: ValueArray,

}

impl Chunk {

	pub fn new() -> Chunk {
		Chunk { code: vec![], constants: ValueArray::new() }
	}

	pub fn write(&mut self, byte: u8) {
		self.code.push(byte);
	}

	pub fn add_constant(&mut self, value: Value) -> usize {
		self.constants.write(value);
		self.constants.values.len() - 1
	}

}
