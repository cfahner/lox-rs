mod chunk;
#[cfg(feature = "trace")]
mod debug;
mod rle;
mod op;
mod value;
mod vm;

use chunk::Chunk;
use op::*;
use vm::VM;
use value::Value;

fn main() {
	let mut vm = VM::<256>::new();
	let mut chunk = Chunk::new();

	chunk.write_constant(Value::new(3.1415), 1);
	chunk.write(OP_NEGATE, 1);
	chunk.write(OP_RETURN, 1);

	let _ = vm.interpret(&chunk);
}
