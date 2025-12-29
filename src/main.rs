mod chunk;
#[cfg(feature = "trace")]
mod debug;
mod rle;
mod op;
mod value;
mod vm;

use std::io::Write;

use sysexits::ExitCode;

use crate::vm::InterpretError;
use crate::vm::VM;

fn main() -> ExitCode {
	let args: Vec<String> = std::env::args().collect();
	if args.len() == 1 {
		repl()
	} else if args.len() == 2 {
		run_file(&args[1][..])
	} else {
		println!("Usage: lox [path]");
		ExitCode::Usage
	}
}

fn run_file(filename: &str) -> ExitCode {
	let Ok(source) = std::fs::read_to_string(filename) else {
		return ExitCode::IoErr;
	};
	let mut vm = VM::<256>::new();
	match interpret(&mut vm, &source) {
		Ok(_) => ExitCode::Ok,
		Err(interpret_error) => interpret_error.to_exit_code(),
	}
}

fn repl() -> ExitCode {
	let mut vm = VM::<256>::new();
	let mut buffer = String::new();
	loop {
		print!("> ");
		if let Err(_) = std::io::stdout().flush() {
			return ExitCode::IoErr;
		}
		let Ok(_) = std::io::stdin().read_line(&mut buffer) else {
			return ExitCode::IoErr;
		};
		if let Err(interpret_error) = interpret(&mut vm, &buffer) {
			return interpret_error.to_exit_code();
		}
		buffer.clear();
	}
	ExitCode::Ok
}

fn interpret<const N: usize>(vm: &mut VM<N>, source: &str) -> Result<(), InterpretError> {
	Ok(())
}
