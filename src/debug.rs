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
	if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
		print!("   | ");
	} else {
		print!("{:04} ", chunk.lines[offset]);
	}

	match Op::try_from(byte) {
		Ok(op) => match op {
			Op::Constant => constant_instruction(chunk, offset),
			Op::Return => simple_instruction("OP_RETURN", offset),
		},
		_ => {
			println!("Unknown opcode {}", byte);
			offset + 1
		}
	}
}

fn constant_instruction(chunk: &Chunk, offset: usize) -> usize {
	let constant = chunk.extract_constant(offset + 1);
	print!("{:<16} '", "OP_CONSTANT");
	print_value(constant.0);
	println!("'");
	constant.1
}

fn simple_instruction(name: &str, offset: usize) -> usize {
	println!("{}", name);
	offset + 1
}
