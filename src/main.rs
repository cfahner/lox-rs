mod chunk;
mod debug;
mod rle;
mod value;
mod vm;

use chunk::Chunk;
use chunk::Op;
use vm::VM;

fn main() {
	let mut vm = VM::new();
	let mut chunk = Chunk::new();

	chunk.write_constant(3.1415, 123);
	chunk.write(Op::Return as u8, 123);

	let _ = vm.interpret(&chunk);
}
