use std::fmt;

use crate::ast::Expr;
use crate::scanner::Scanner;
use crate::scanner::Token;
use crate::scanner::TokenKind;

const ERRMSG_SYNTAX: &str = "Syntax error";

#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
	None,
	Assignment,
	Or,
	And,
	Equality,
	Comparison,
	Term,
	Factor,
	Unary,
	Call,
	Primary
}

impl Precedence {

	pub fn next(&self) -> Self {
		use Precedence::*;
		match *self {
			None => Assignment,
			Assignment => Or,
			Or => And,
			And => Equality,
			Equality => Comparison,
			Comparison => Term,
			Term => Factor,
			Factor => Unary,
			Unary => Call,
			Call => Primary,
			Primary => Primary
		}
	}

	pub fn from_infix_token_kind(token_kind: &TokenKind) -> Self {
		use TokenKind::*;
		match token_kind {
			Plus | Minus => Self::Term,
			Star | Slash => Self::Factor,
			_ => Self::None
		}
	}

}

#[derive(Debug)]
pub struct ParseError {
	line: u32,
	context: String,
	message: String,
}

impl ParseError {

	pub fn new_at_token(token: &Token, message: &str) -> Self {
		Self::new(token.line, &format!("at `{}`", &token.content), message)
	}

	pub fn new_at_end(line: u32, message: &str) -> Self {
		Self::new(line, "at end", message)
	}

	fn new(line: u32, context: &str, message: &str) -> Self {
		Self { line, context: context.to_owned(), message: message.to_owned() }
	}

}

impl fmt::Display for ParseError {

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[line {}] Error {}: {}", self.line, self.context, self.message)
	}

}

pub struct StmtIterator<'src> {
	scanner: Scanner<'src>,
	current: Token,
	end_reached: bool,
}

impl<'src> StmtIterator<'src> {

	pub fn new(source: &'src str) -> Option<Self> {
		let mut scanner = Scanner::new(source);
		if let Some(token) = scanner.next() {
			return Some(Self {
				scanner,
				current: token,
				end_reached: false,
			});
		}

		None
	}

	fn parse_infix(&mut self, left: Expr, operator: Token) -> Result<Expr, ParseError> {
		let precedence = Precedence::from_infix_token_kind(&operator.kind);

		let right = self.parse_precedence(precedence.next())?;

		Ok(Expr::Binary {
			lhs: Box::new(left),
			operator: operator,
			rhs: Box::new(right),
		})
	}

	fn parse_prefix(&mut self, token: Token) -> Result<Expr, ParseError> {
		match token.kind {
			TokenKind::Number => match token.content.parse::<f64>() {
				Ok(number) => Ok(Expr::Number { number, line: token.line }),
				_ => Err(ParseError::new_at_token(&token, "Expected valid number."))
			},
			TokenKind::Minus => Ok(Expr::Unary {
				operator: token,
				expr: Box::new(self.parse_precedence(Precedence::Unary)?)
			}),
			TokenKind::LeftParen => {
				let expr = self.parse_precedence(Precedence::Assignment)?;
				self.consume(&TokenKind::RightParen, "Expected ')' character.")?;
				Ok(expr)
			},
			_ => Err(ParseError::new_at_token(&token, "Expected expression."))
		}
	}

	fn parse_precedence(&mut self, prec: Precedence) -> Result<Expr, ParseError> {
		let token = self.advance()?;
		let mut left = self.parse_prefix(token)?;

		while prec <= Precedence::from_infix_token_kind(&self.current.kind) {
			let operator = self.advance()?;
			left = self.parse_infix(left, operator)?;
		}

		Ok(left)
	}

	// Temporarily public while statement parsing is not implemented
	pub fn expression(&mut self) -> Result<Expr, ParseError> {
		self.parse_precedence(Precedence::Assignment)
	}

	fn advance(&mut self) -> Result<Token, ParseError> {
		let mut error: Option<ParseError> = None;

		while let Some(token) = self.scanner.next() {
			match token.kind {
				TokenKind::Error => error = error.or(Some(ParseError::new_at_token(&token, ERRMSG_SYNTAX))),
				_ => return Ok(std::mem::replace(&mut self.current, token))
			};
		};

		if self.end_reached {
			return Err(error.unwrap_or_else(|| ParseError::new_at_end(self.scanner.line, ERRMSG_SYNTAX)));
		}

		self.end_reached = true;
		Ok(self.current.clone())
	}

	fn consume(&mut self, kind: &TokenKind, message_on_error: &str) -> Result<Token, ParseError> {
		if std::mem::discriminant(&self.current.kind) == std::mem::discriminant(kind) {
			return self.advance();
		}

		Err(ParseError::new_at_token(&self.current, message_on_error))
	}

	pub fn expect_end(&mut self) -> Result<(), ParseError> {
		match self.scanner.next() {
			Some(_) => Err(ParseError::new_at_end(self.scanner.line, "Expected end of expression.")),
			None => Ok(())
		}
	}

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_should_return_some_when_source_contains_valid_tokens() {
		let source = "3 + 4;";

		let sut = StmtIterator::new(source);

		assert!(sut.is_some());
	}

	#[test]
	fn new_should_return_none_when_source_is_empty() {
		let source = "";

		let sut = StmtIterator::new(source);

		assert!(sut.is_none());
	}

	#[test]
	fn new_should_return_none_when_source_is_whitespace() {
		let source = " ";

		let sut = StmtIterator::new(source);

		assert!(sut.is_none());
	}

	#[test]
	fn expression_with_factor_should_take_precedence_over_term() {
		let mut order1 = StmtIterator::new("1 + 2 * 5").unwrap();
		let mut order2 = StmtIterator::new("2 * 5 + 1").unwrap();

		let result1 = order1.expression().unwrap();
		let result2 = order2.expression().unwrap();

		let Expr::Binary { operator: operator1, .. } = &result1 else { unreachable!(); };
		let Expr::Binary { operator: operator2, .. } = &result2 else { unreachable!(); };
		assert!(std::mem::discriminant(&operator1.kind) == std::mem::discriminant(&operator2.kind));
	}

	#[test]
	fn expression_with_group_should_change_precedence() {
		let mut expr1 = StmtIterator::new("1 + 2 * 5").unwrap();
		let mut expr2 = StmtIterator::new("(1 + 2) * 5").unwrap();

		let result1 = expr1.expression().unwrap();
		let result2 = expr2.expression().unwrap();

		let Expr::Binary { operator: operator1, .. } = &result1 else { unreachable!(); };
		assert!(matches!(operator1.kind, TokenKind::Plus));
		let Expr::Binary { operator: operator2, .. } = &result2 else { unreachable!(); };
		assert!(matches!(operator2.kind, TokenKind::Star));
	}

	#[test]
	fn expression_should_recognize_unary_negation() {
		let mut sut = StmtIterator::new("1 - -1").unwrap();

		let result = sut.expression().unwrap();

		let Expr::Binary { rhs, .. } = &result else { unreachable!(); };
		assert!(matches!(**rhs, Expr::Unary { .. }));
	}

	#[test]
	fn expression_should_error_if_incomplete() {
		let mut sut = StmtIterator::new("1 +").unwrap();

		let result = sut.expression();

		assert!(matches!(result, Err(error) if error.context == "at end"));
	}

}
