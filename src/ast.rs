pub enum UnaryOp {
	Negate,
	Not,
}

pub enum BinaryOp {
	Add,
	Subtract,
	Multiply,
	Divide,
	Equal,
	NotEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual
}

pub enum Expr {
	String(String),
	Number {
		number: f64,
		line: u32,
	},
	Nil,
	True,
	False,
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
