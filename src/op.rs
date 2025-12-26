pub const OP_CONSTANT: u8 = 0x00;
pub const OP_RETURN: u8 = 0x01;
pub const OP_CONSTANT_LONG: u8 = 0x02;

/// Returns the size of opcode + operands in bytes
pub fn op_size(op: u8) -> usize {
	match op {
		OP_CONSTANT => 2,
		OP_CONSTANT_LONG => 4,
		_ => 1
	}
}

/// Returns a string representation of the given opcode
pub fn op_to_string(op: u8) -> &'static str {
	match op {
		OP_CONSTANT => "OP_CONSTANT",
		OP_RETURN => "OP_RETURN",
		OP_CONSTANT_LONG => "OP_CONSTANT_LONG",
		_ => "OP_UNKNOWN"
	}
}
