use crate::scanner::Token;

pub enum Expr {
	Number(f64),

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
