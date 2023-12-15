use crate::lexer::Token;
use crate::syntax_tree::model::{Expression, Binary, Unary, Grouping, Literal};

/// Parses a list of tokens into a syntax tree.
///
/// ## Structure:
/// ```text
/// expression     → equality ;
/// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
/// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
/// term           → factor ( ( "-" | "+" ) factor )* ;
/// factor         → unary ( ( "/" | "*" ) unary )* ;
/// unary          → ( "!" | "-" ) unary | primary ;
/// primary        →  Literal | "(" expression ")" ;
/// ```
pub struct Parser<'a> {
	tokens: Vec<Token<'a>>,
	current: usize,
}

impl<'a> Parser<'a> {
	pub fn new(tokens: Vec<Token<'a>>) -> Self {
		Self {
			tokens,
			current: 0,
		}
	}

	pub fn parse(&mut self) -> Result<Expression<'a>, String> {
		#[cfg(debug_assertions)]
		{
			println!("parse: {:?}", self.tokens);
		}
		let result = self.expression()?;
		if !self.is_at_end() {
			return self.error(self.get_current(), "Unexpected token.");
		}
		Ok(result)
	}

	fn expression(&mut self) -> Result<Expression<'a>, String> {
		self.equality()
	}

	fn new_binary(
		left: Expression<'a>,
		operator: Token<'a>,
		right: Expression<'a>
	) -> Result<Expression<'a>, String> {
		let binary = Binary::new(
			left, operator, right
		)?;
		Ok(Expression::Binary(binary))
	}

	fn equality(&mut self) -> Result<Expression<'a>, String> {
		let mut expr = self.comparison()?;
		#[cfg(debug_assertions)]
		{
			println!("equality: {:?} -> next: {:?}", &expr, &self.get_current());
		}
		while self.match_and_advance(vec![Token::NotEqual, Token::Equal]) {
			let operator = self.get_previous();
			let right = self.comparison()?;
			#[cfg(debug_assertions)]
			{
				println!("equality: {:?} {} {:?}", &expr, &operator, &right);
			}
			expr = Self::new_binary(
				expr, operator, right
			)?;
		}
		Ok(expr)
	}

	fn match_and_advance(&mut self, types: Vec<Token<'a>>) -> bool {
		for token_type in types {
			if self.check_current(token_type) {
				self.advance();
				return true
			}
		}
		false
	}

	fn check_current(&self, token2compare: Token<'a>) -> bool {
		if self.is_at_end() {
			return false;
		}
		self.get_current() == token2compare
	}

	fn advance(&mut self) -> Token<'a> {
		if !self.is_at_end() {
			self.current += 1;
		}
		self.get_previous()
	}

	fn is_at_end(&self) -> bool {
		self.current >= self.tokens.len() - 1
	}

	fn get_current(&self) -> Token<'a> { // TODO use references
		self.tokens[self.current].clone()
	}

	fn get_previous(&self) -> Token<'a> { // TODO use references
		self.tokens[self.current - 1].clone()
	}

	fn comparison(&mut self) -> Result<Expression<'a>, String> {
		let mut expr = self.term()?;
		while self.match_and_advance(vec![
			Token::GreaterThan, Token::GreaterEqual,
			Token::LessThan, Token::LessEqual
		]) {
			let operator = self.get_previous();
			let right = self.term()?;
			#[cfg(debug_assertions)]
			{
				println!("comparison: {:?} {} {:?}", &expr, &operator, &right);
			}
			expr = Self::new_binary(
				expr, operator, right
			)?;
		}
		Ok(expr)
	}

	fn term(&mut self) -> Result<Expression<'a>, String> {
		let mut expr = self.factor()?;
		while self.match_and_advance(vec![Token::Minus, Token::Plus]) {
			let operator = self.get_previous();
			let right = self.factor()?;
			#[cfg(debug_assertions)]
			{
				println!("term: {:?} {} {:?}", &expr, &operator, &right);
			}
			expr = Self::new_binary(
				expr, operator, right
			)?;
		}
		Ok(expr)
	}

	fn factor(&mut self) -> Result<Expression<'a>, String> {
		let mut expr = self.unary()?;
		while self.match_and_advance(vec![
			Token::Multiply, Token::Divide, Token::Modulo
		]) {
			let operator = self.get_previous();
			let right = self.unary()?;
			#[cfg(debug_assertions)]
			{
				println!("factor:\n  {:?}\n  {}\n  {:?}", &expr, &operator, &right);
			}
			expr = Self::new_binary(
				expr, operator, right
			)?;
		}
		Ok(expr)
	}

	fn unary(&mut self) -> Result<Expression<'a>, String> {
		if self.match_and_advance(vec![Token::Not, Token::Minus]) {
			let operator = self.get_previous();
			let right = self.unary()?;
			#[cfg(debug_assertions)]
			{
				println!("unary: {} {:?}", &operator, &right);
			}
			let unary = Unary::new(operator, right)?;
			return Ok(Expression::Unary(unary));
		}
		self.primary()
	}

	fn primary(&mut self) -> Result<Expression<'a>, String> {
		match self.get_current() {
			Token::Value(s) => {
				self.advance();
				match Literal::from_str(s) {
					Some(literal) => {
						#[cfg(debug_assertions)]
						{
							println!("literal: {:?}", &literal);
						}
						Ok(Expression::Literal(literal))
					},
					None => self.error(self.get_current(), "Expect literal.")
				}
			},
			Token::GroupingStart => {
				self.advance();
				let expr = self.expression()?;
				self.consume(
					Token::GroupingEnd,
					&format!("Expected '{}' after expression.", Token::GroupingEnd)
				)?;
				#[cfg(debug_assertions)]
				{
					println!("grouping: ({:?})", &expr);
				}
				Ok(Expression::Grouping(Grouping::new(expr)))
			},
			_ => {
				self.error(self.get_current(), "Expected expression")
			}
		}
	}

	fn consume(
		&mut self,
		token: Token<'a>,
		message: &str
	) -> Result<Token<'a>, String> {
		if self.check_current(token) {
			return Ok(self.advance());
		}
		Err(format!("{}", message))
	}

	fn error(&self, token: Token<'a>, message: &str) -> Result<Expression<'a>, String> {
		Err(format!("{} at '{}'.", message, token))
	}

}
