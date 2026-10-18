use crate::scanner::Token;

pub enum Expr {
	Number {
		number: f64,
		line: u32,
	},

	Unary {
		operator: Token,
		expr: Box<Expr>,
	},

	Binary {
		lhs: Box<Expr>,
		operator: Token,
		rhs: Box<Expr>,
	}
}
