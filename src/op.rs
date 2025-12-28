pub const OP_CONSTANT: u8 = 0x00;
pub const OP_ADD: u8 = 0x01;
pub const OP_SUBTRACT: u8 = 0x02;
pub const OP_MULTIPLY: u8 = 0x03;
pub const OP_DIVIDE: u8 = 0x04;
pub const OP_NEGATE: u8 = 0x05;
pub const OP_RETURN: u8 = 0x06;
pub const OP_CONSTANT_LONG: u8 = 0x07;

/// Returns the size of opcode + operands in bytes
pub fn op_size(op: u8) -> usize {
	match op {
		OP_CONSTANT => 2,
		OP_CONSTANT_LONG => 4,
		_ => 1
	}
}
