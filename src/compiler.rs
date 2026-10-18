use crate::ast::Expr;
use crate::chunk::Chunk;
use crate::op::*;
use crate::parser::ParseError;
use crate::parser::StmtIterator;
use crate::scanner::TokenKind;
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
		Expr::Number { number, line } => chunk.write_constant(Value::new(*number), *line),
		Expr::Unary { operator, expr } => match operator.kind {
			TokenKind::Minus => {
				compile_expr(chunk, &*expr);
				chunk.write(OP_NEGATE, operator.line);
			},
			_ => unreachable!()
		},
		Expr::Binary { lhs, operator, rhs } => {
			compile_expr(chunk, &*lhs);
			compile_expr(chunk, &*rhs);
			chunk.write(match operator.kind {
				TokenKind::Plus => OP_ADD,
				TokenKind::Minus => OP_SUBTRACT,
				TokenKind::Star => OP_MULTIPLY,
				TokenKind::Slash => OP_DIVIDE,
				_ => unreachable!()
			}, operator.line);
		}
	}
}
