mod chunk;
mod debug;
mod value;

use crate::chunk::Chunk;
use crate::chunk::Op;

fn main() {
	let mut chunk = Chunk::new();

	chunk.write_constant(3.1415, 123);
	chunk.write(Op::Return as u8, 123);
	debug::disassemble_chunk(&chunk, "test");
}
