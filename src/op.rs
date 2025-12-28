pub const OP_CONSTANT: u8 = 0x00;
pub const OP_NEGATE: u8 = 0x01;
pub const OP_RETURN: u8 = 0x02;
pub const OP_CONSTANT_LONG: u8 = 0x03;

/// Returns the size of opcode + operands in bytes
pub fn op_size(op: u8) -> usize {
	match op {
		OP_CONSTANT => 2,
		OP_CONSTANT_LONG => 4,
		_ => 1
	}
}
