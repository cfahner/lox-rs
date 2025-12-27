use crate::chunk::Chunk;
use crate::op::*;
use crate::value::Value;

/// Returns a string representation of the given opcode
pub fn op_to_string(op: u8) -> &'static str {
	match op {
		OP_CONSTANT => "OP_CONSTANT",
		OP_RETURN => "OP_RETURN",
		OP_CONSTANT_LONG => "OP_CONSTANT_LONG",
		_ => "OP_UNKNOWN"
	}
}

/// Prints a debug representation of a value
pub fn print_value(value: &Value) {
	print!("{}", value);
}

/// Disassembles the instruction at the given code offset in the chunk and returns the next code offset
pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
	print!("{offset:04} ");
	let instruction_line = chunk.find_line(offset).unwrap();
	if offset > 0 && instruction_line == chunk.find_line(offset - 1).unwrap() {
		print!("   | ");
	} else {
		print!("{instruction_line:04} ");
	}

	let opcode = chunk.code[offset];
	print!("{:<16} ", op_to_string(opcode));
	match opcode {
		OP_CONSTANT => constant_instruction("OP_CONSTANT", chunk, offset),
		OP_CONSTANT_LONG => constant_instruction("OP_CONSTANT_LONG", chunk, offset),
		_ => {}
	};
	println!();
	offset + op_size(opcode)
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) {
	let const_id_bytes: [u8;4] = if name == "OP_CONSTANT" {
		[ 0, 0, 0, chunk.code[offset + 1] ]
	} else {
		[ 0, chunk.code[offset + 1], chunk.code[offset + 2], chunk.code[offset + 3] ]
	};
	let const_value = chunk.get_constant(u32::from_be_bytes(const_id_bytes) as usize);
	print!("'");
	print_value(const_value);
	print!("'");
}
