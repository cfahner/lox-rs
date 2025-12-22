use std::convert::TryFrom;

pub enum Op {
	Return = 0x00,
}

impl TryFrom<u8> for Op {
	type Error = ();

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0x00 => Ok(Op::Return),
			_ => Err(()),
		}
	}
}

pub struct Chunk {
	pub code: Vec<u8>,
}

impl Chunk {
	pub fn new() -> Chunk {
		Chunk { code: vec![] }
	}

	pub fn write(&mut self, byte: u8) {
		self.code.push(byte);
	}
}
