pub enum TokenType {

	/* single character tokens */

	LeftParen,

	RightParen,

	LeftBrace,

	RightBrace,

	Comma,

	Dot,

	Minus,

	Plus,

	Semicolon,

	Slash,

	Star,

	/* one or two character tokens */

	Bang,

	BangEqual,

	Equal,

	EqualEqual,

	Greater,

	GreaterEqual,

	Less,

	LessEqual,

	/* literals */

	Identifier,

	String,

	Number,

	/* keywords */

	And,

	Class,

	Else,

	False,

	For,

	Fun,

	If,

	Nil,

	Or,

	Print,

	Return,

	Super,

	This,

	True,

	Var,

	While,

	Error,

	Eof,

}

pub struct Token {

	pub r#type: TokenType,

	pub lexeme: String,

	pub line: u32,

}

pub struct Scanner { }

impl Scanner {

	pub fn new(source: &str) -> Self {
		Self { }
	}

}

impl Iterator for Scanner {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		None
	}

}
