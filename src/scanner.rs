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

pub struct Token<'a> {

	pub r#type: TokenType,

	pub content: &'a str,

	pub line: u32,

}

pub struct Scanner<'a> {

	source: &'a [u8],

	start: usize,

	current: usize,

	line: u32,

}

impl<'a> Scanner<'a> {

	pub fn new(source: &'a str) -> Self {
		Self {
			source: source.as_bytes(),
			start: 0,
			current: 0,
			line: 1,
		}
	}

	fn make_token(&self, r#type: TokenType) -> Token<'a> {
		Token {
			r#type: r#type,
			// Safety: invalid sequences will already have been rejected before this point is reached
			content: unsafe { std::str::from_utf8_unchecked(&self.source[self.start..self.current]) },
			line: self.line,
		}
	}

	fn error_token(&self, message: &'a str) -> Token<'a> {
		Token {
			r#type: TokenType::Error,
			content: message,
			line: self.line,
		}
	}

	fn advance(&mut self) -> char {
		self.current += 1;
		self.source[self.current - 1] as char
	}

	fn consume_if(&mut self, character: char) -> bool {
		if self.is_at_end() {
			return false;
		}
		if (self.source[self.current] as char) != character {
			return false;
		}
		self.current += 1;
		true
	}

	fn is_at_end(&self) -> bool {
		self.current >= self.source.len()
	}

}

impl<'a> Iterator for Scanner<'a> {
	type Item = Token<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.current = self.start;
		if self.is_at_end() {
			return None;
		}

		Some(match self.advance() {
			'(' => self.make_token(TokenType::LeftParen),
			')' => self.make_token(TokenType::RightParen),
			'{' => self.make_token(TokenType::LeftBrace),
			'}' => self.make_token(TokenType::RightBrace),
			';' => self.make_token(TokenType::Semicolon),
			',' => self.make_token(TokenType::Comma),
			'.' => self.make_token(TokenType::Dot),
			'-' => self.make_token(TokenType::Minus),
			'/' => self.make_token(TokenType::Slash),
			'*' => self.make_token(TokenType::Star),
			'!' => match self.consume_if('=') {
				true => self.make_token(TokenType::BangEqual),
				false => self.make_token(TokenType::Bang)
			},
			'=' => match self.consume_if('=') {
				true => self.make_token(TokenType::EqualEqual),
				false => self.make_token(TokenType::Equal)
			},
			'<' => match self.consume_if('=') {
				true => self.make_token(TokenType::LessEqual),
				false => self.make_token(TokenType::Less)
			},
			'>' => match self.consume_if('=') {
				true => self.make_token(TokenType::GreaterEqual),
				false => self.make_token(TokenType::Greater)
			},
			_ => self.error_token("Unexpected character.")
		})
	}

}
