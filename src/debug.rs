use crate::chunk::Chunk;
use crate::chunk::Op;

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

	match Op::try_from(byte) {
		Ok(op) => match op {
			Op::Return => simple_instruction("OP_RETURN", offset),
		},
		_ => {
			println!("Unknown opcode {}", byte);
			offset + 1
		}
	}
}

fn simple_instruction(name: &str, offset: usize) -> usize {
	println!("{}", name);
	offset + 1
}
