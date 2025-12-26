mod chunk;
mod rle;
mod value;
mod vm;

use chunk::Chunk;
use vm::VM;

fn main() {
	let mut vm = VM::new();
	let mut chunk = Chunk::new();

	chunk.write_constant(3.1415, 123);
	chunk.write(chunk::OP_RETURN, 123);

	let _ = vm.interpret(&chunk);
}
