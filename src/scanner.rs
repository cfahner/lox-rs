pub enum TokenKind {

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

	pub kind: TokenKind,

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

	fn make_token(&self, kind: TokenKind) -> Token<'a> {
		Token {
			kind: kind,
			// Safety: invalid sequences will already have been rejected before this point is reached
			content: unsafe { std::str::from_utf8_unchecked(&self.source[self.start..self.current]) },
			line: self.line,
		}
	}

	fn error_token(&self, message: &'a str) -> Token<'a> {
		Token {
			kind: TokenKind::Error,
			content: message,
			line: self.line,
		}
	}

	fn consume_identifier(&mut self) -> Token<'a> {
		while is_alpha(self.peek()) || is_digit(self.peek()) {
			self.discard();
		}
		// Safety: each byte has already been checked and is either alpha or digit
		let identifier = unsafe { std::str::from_utf8_unchecked(&self.source[self.start..self.current]) };
		self.make_token(match identifier {
			"and" => TokenKind::And,
			"class" => TokenKind::Class,
			"else" => TokenKind::Else,
			"false" => TokenKind::False,
			"for" => TokenKind::For,
			"fun" => TokenKind::Fun,
			"if" => TokenKind::If,
			"nil" => TokenKind::Nil,
			"or" => TokenKind::Or,
			"print" => TokenKind::Print,
			"return" => TokenKind::Return,
			"super" => TokenKind::Super,
			"this" => TokenKind::This,
			"true" => TokenKind::True,
			"var" => TokenKind::Var,
			"while" => TokenKind::While,
			_ => TokenKind::Identifier
		})
	}

	fn consume_string(&mut self) -> Token<'a> {
		while self.peek() != '"' && !self.is_at_end() {
			if self.peek() == '\n' {
				self.line += 1;
			}
			self.discard();
		}
		if self.is_at_end() {
			return self.error_token("Unterminated string.");
		}
		self.discard(); // advance beyond the closing quote
		self.make_token(TokenKind::String)
	}

	fn consume_number(&mut self) -> Token<'a> {
		while is_digit(self.peek()) {
			self.discard();
		}
		// fraction
		if self.peek() == '.' && is_digit_in_option(self.peek_next()) {
			self.discard();
			while is_digit(self.peek()) {
				self.discard();
			}
		}
		self.make_token(TokenKind::Number)
	}

	fn skip_whitespace(&mut self) {
		loop {
			let c = self.peek();
			match c {
				'\r' | '\t' | ' ' => self.discard(),
				'\n' => {
					self.line += 1;
					self.discard();
				},
				'/' => {
					if self.peek_next() == Some('/') {
						while self.peek() != '\n' && !self.is_at_end() {
							self.discard();
						}
					} else {
						return;
					}
				},
				_ => break
			};
		}
	}

	fn consume(&mut self) -> char {
		self.current += 1;
		self.source[self.current - 1] as char
	}

	fn peek(&self) -> char {
		self.source[self.current] as char
	}

	fn peek_next(&self) -> Option<char> {
		match self.source.get(self.current + 1) {
			Some(byte) => Some(*byte as char),
			None => None
		}
	}

	fn discard(&mut self) {
		self.current += 1;
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

		self.skip_whitespace();

		Some(match self.consume() {
			'(' => self.make_token(TokenKind::LeftParen),
			')' => self.make_token(TokenKind::RightParen),
			'{' => self.make_token(TokenKind::LeftBrace),
			'}' => self.make_token(TokenKind::RightBrace),
			';' => self.make_token(TokenKind::Semicolon),
			',' => self.make_token(TokenKind::Comma),
			'.' => self.make_token(TokenKind::Dot),
			'-' => self.make_token(TokenKind::Minus),
			'/' => self.make_token(TokenKind::Slash),
			'*' => self.make_token(TokenKind::Star),
			'!' => match self.consume_if('=') {
				true => self.make_token(TokenKind::BangEqual),
				false => self.make_token(TokenKind::Bang)
			},
			'=' => match self.consume_if('=') {
				true => self.make_token(TokenKind::EqualEqual),
				false => self.make_token(TokenKind::Equal)
			},
			'<' => match self.consume_if('=') {
				true => self.make_token(TokenKind::LessEqual),
				false => self.make_token(TokenKind::Less)
			},
			'>' => match self.consume_if('=') {
				true => self.make_token(TokenKind::GreaterEqual),
				false => self.make_token(TokenKind::Greater)
			},
			'"' => self.consume_string(),
			'0'..='9' => self.consume_number(),
			_ => self.error_token("Unexpected character.")
		})
	}

}

fn is_digit_in_option(char_option: Option<char>) -> bool {
	match char_option {
		Some(c) => is_digit(c),
		None => false
	}
}

fn is_digit(character: char) -> bool {
	match (character) {
		'0'..='9' => true,
		_ => false
	}
}

fn is_alpha(character: char) -> bool {
	match character {
		'a'..='z' | 'A'..='Z' | '_' => true,
		_ => false
	}
}
