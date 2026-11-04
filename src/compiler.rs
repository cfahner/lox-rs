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
		Expr::Number { number, line } => chunk.write_constant(Value::Number(*number), *line),
		Expr::Unary { op, expr, line } => match op {
			UnaryOp::Negate => {
				compile_expr(chunk, &*expr);
				chunk.write(OP_NEGATE, *line);
			},
		},
		Expr::Binary { lhs, op, rhs, line } => {
			compile_expr(chunk, &*lhs);
			compile_expr(chunk, &*rhs);
			chunk.write(match op {
				BinaryOp::Add => OP_ADD,
				BinaryOp::Subtract => OP_SUBTRACT,
				BinaryOp::Multiply => OP_MULTIPLY,
				BinaryOp::Divide => OP_DIVIDE,
			}, *line);
		},
	}
}
