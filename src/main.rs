mod chunk;
mod debug;
mod value;

use crate::chunk::Chunk;
use crate::chunk::Op;

fn main() {
	let mut chunk = Chunk::new();

	let const_position = chunk.add_constant(1.2);
	chunk.write(Op::Constant as u8);
	chunk.write(const_position as u8);

	chunk.write(Op::Return as u8);
	debug::disassemble_chunk(&chunk, "test");
}
