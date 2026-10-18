pub enum UnaryOp {
	Negate,
}

pub enum BinaryOp {
	Add,
	Subtract,
	Multiply,
	Divide,
}

pub enum Expr {
	Number {
		number: f64,
		line: u32,
	},

	Unary {
		op: UnaryOp,
		expr: Box<Expr>,
		line: u32,
	},

	Binary {
		lhs: Box<Expr>,
		op: BinaryOp,
		rhs: Box<Expr>,
		line: u32,
	},
}
