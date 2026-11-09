use std::rc::Rc;

use crate::ast::BinaryOp;
use crate::ast::Expr;
use crate::ast::UnaryOp;
use crate::chunk::Chunk;
use crate::op::*;
use crate::parser::ParseError;
use crate::parser::StmtIterator;
use crate::value::Value;

pub fn compile(source: &str) -> Result<Chunk, ParseError> {
	let mut chunk = Chunk::new();
	let Some(mut parser) = StmtIterator::new(source) else {
		return Ok(chunk);
	};

	compile_expr(&mut chunk, &parser.expression()?);
	chunk.write(OP_RETURN, 9999);
	parser.expect_end()?;

	Ok(chunk)
}

fn compile_expr(chunk: &mut Chunk, stmt: &Expr) {
	match stmt {
		Expr::String(string) => chunk.write_constant(Value::String(Rc::from(string.clone())), chunk.last_line()),
		Expr::Number { number, line } => chunk.write_constant(Value::Number(*number), *line),
		Expr::Nil => chunk.write(OP_NIL, chunk.last_line()),
		Expr::True => chunk.write(OP_TRUE, chunk.last_line()),
		Expr::False => chunk.write(OP_FALSE, chunk.last_line()),
		Expr::Unary { op, expr, line } => match op {
			UnaryOp::Negate => {
				compile_expr(chunk, &*expr);
				chunk.write(OP_NEGATE, *line);
			},
			UnaryOp::Not => {
				compile_expr(chunk, &*expr);
				chunk.write(OP_NOT, *line);
			}
		},
		Expr::Binary { lhs, op, rhs, line } => {
			compile_expr(chunk, &*lhs);
			compile_expr(chunk, &*rhs);
			match op {
				BinaryOp::Add => chunk.write(OP_ADD, *line),
				BinaryOp::Subtract => chunk.write(OP_SUBTRACT, *line),
				BinaryOp::Multiply => chunk.write(OP_MULTIPLY, *line),
				BinaryOp::Divide => chunk.write(OP_DIVIDE, *line),
				BinaryOp::Equal => chunk.write(OP_EQUAL, *line),
				BinaryOp::NotEqual => chunk.write_twice(OP_EQUAL, OP_NOT, *line),
				BinaryOp::Greater => chunk.write(OP_GREATER, *line),
				BinaryOp::GreaterEqual => chunk.write_twice(OP_LESS, OP_NOT, *line),
				BinaryOp::Less => chunk.write(OP_LESS, *line),
				BinaryOp::LessEqual => chunk.write_twice(OP_GREATER, OP_NOT, *line)
			};
		},
	}
}
