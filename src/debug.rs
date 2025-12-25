use crate::chunk::Chunk;
use crate::chunk::Op;
use crate::value::print_value;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
	println!("== {} ==", name);
	let mut offset: usize = 0;
	loop {
		if offset >= chunk.code.len() {
			break;
		}
		offset = disassemble_instruction(chunk, offset);
	}
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
	let byte = chunk.code[offset];

	print!("{:04} ", offset);
	let instruction_line = chunk.find_line(offset).unwrap();
	if offset > 0 && instruction_line == chunk.find_line(offset - 1).unwrap() {
		print!("   | ");
	} else {
		print!("{:04} ", instruction_line);
	}

	match Op::try_from(byte) {
		Ok(op) => match op {
			Op::Constant => constant_instruction("OP_CONSTANT", chunk, offset),
			Op::Return => simple_instruction("OP_RETURN", offset),
			Op::ConstantLong => constant_instruction("OP_CONSTANT_LONG", chunk, offset),
		},
		_ => {
			println!("Unknown opcode {}", byte);
			offset + 1
		}
	}
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
	let constant = if name == "OP_CONSTANT" {
		chunk.extract_constant_short(offset + 1)
	} else {
		chunk.extract_constant_long(offset + 1)
	};
	print!("{:<16} '", name);
	print_value(constant.0);
	println!("'");
	constant.1
}

fn simple_instruction(name: &str, offset: usize) -> usize {
	println!("{}", name);
	offset + 1
}
