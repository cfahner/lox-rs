mod chunk;
mod debug;

use crate::chunk::Chunk;
use crate::chunk::Op;

fn main() {
	let mut chunk = Chunk::new();
	chunk.write(Op::Return as u8);
	debug::disassemble_chunk(&chunk, "test");
}
