use crate::lexer::Token;
use crate::syntax_tree::model::{
	Expression, Binary, Unary, Grouping, Literal, Variable,
	Stmt
};

/// Parses a list of tokens into a syntax tree.
///
/// ## Structure:
/// ```text
/// program        → Stmt
/// Stmt           → Block | Raw | Print | Expression | Assign | If | While | ForEach | Break | Continue
///
/// Block          → Stmt[] ;
/// Raw            → "..." ;
/// Print          → "{{" "print" expression ";" "}}" ;
/// Expression     → "{{" expression "}}" ;
/// Assign         → "{{" "assign" Variable "=" expression ";" "}}" ;
/// If             → "{{" "if" expression "}}" Stmt ( "{{" "elseif" expression "}}" Stmt )* ( "{{" "else" "}}" Stmt )? "{{" "fi" "}}" ;
/// While          → "{{" "while" expression "}}" Stmt "{{" "done" "}}" ;
/// ForEach        → "{{" "foreach" Variable "in" Variable "}}" Stmt "{{" "done" "}}" ;
/// Break          → "{{" "break" "}}" ;
/// Continue       → "{{" "continue" "}}" ;
///
/// expression     → equality ;
/// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
/// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
/// term           → factor ( ( "-" | "+" ) factor )* ;
/// factor         → unary ( ( "/" | "*" ) unary )* ;
/// unary          → ( "!" | "-" ) unary | primary ;
/// primary        →  Literal | Variable | "(" expression ")" ;
/// ```
pub struct Parser<'a> {
	tokens: &'a [Token<'a>],
	current: usize,
}

// Public methods

impl<'a> Parser<'a> {
	pub fn new(tokens: &'a [Token<'a>]) -> Self {
		Self {
			tokens,
			current: 0,
		}
	}

	pub fn parse(&mut self) -> Result<Stmt<'a>, String> {
		#[cfg(debug_assertions)]
		{
			println!("parse: {:?}", self.tokens);
		}
		self.advance(); // TODO remove
		self.advance(); // TODO remove
		let result = Stmt::Expression(self.expression()?);
		self.advance(); // TODO remove
		if !self.is_at_end() {
			return Err(self.error(self.get_current(), "Unexpected token."));
		}
		Ok(result)
	}

}

// Tools

impl<'a> Parser<'a> {
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

	fn match_and_advance(&mut self, types: &[Token<'a>]) -> bool {
		for token_type in types {
			if self.check_current(token_type) {
				self.advance();
				return true
			}
		}
		false
	}

	fn check_current(&self, token2compare: &Token<'a>) -> bool {
		if self.is_at_end() {
			return false;
		}
		self.get_current() == token2compare
	}

	fn advance(&mut self) -> &Token<'a> {
		if !self.is_at_end() {
			self.current += 1;
		}
		self.get_previous()
	}

	fn is_at_end(&self) -> bool {
		self.current >= self.tokens.len() - 1
	}

	fn get_current(&self) -> &Token<'a> {
		&self.tokens[self.current]
	}

	fn get_previous(&self) -> &Token<'a> {
		&self.tokens[self.current - 1]
	}

	fn consume(
		&mut self,
		token: Token<'a>,
		message: &str
	) -> Result<Token<'a>, String> {
		if self.check_current(&token) {
			return Ok(self.advance().clone());
		}
		Err(format!("{}", message))
	}

	fn error(&self, token: &Token<'a>, message: &str) -> String {
		format!("{} at '{}'.", message, token)
	}
}

// Grammar

impl<'a> Parser<'a> {
	fn expression(&mut self) -> Result<Expression<'a>, String> {
		self.equality()
	}

	fn equality(&mut self) -> Result<Expression<'a>, String> {
		let mut expr = self.comparison()?;
		while self.match_and_advance(&[Token::NotEqual, Token::Equal]) {
			let operator = self.get_previous().clone();
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

	fn comparison(&mut self) -> Result<Expression<'a>, String> {
		let mut expr = self.term()?;
		while self.match_and_advance(&[
			Token::GreaterThan, Token::GreaterEqual,
			Token::LessThan, Token::LessEqual
		]) {
			let operator = self.get_previous().clone();
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
		while self.match_and_advance(&[Token::Minus, Token::Plus]) {
			let operator = self.get_previous().clone();
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
		while self.match_and_advance(&[
			Token::Multiply, Token::Divide, Token::Modulo
		]) {
			let operator = self.get_previous().clone();
			let right = self.unary()?;
			#[cfg(debug_assertions)]
			{
				println!("factor:\n  {:?}\n  {}\n  {:?}", &expr, operator, &right);
			}
			expr = Self::new_binary(
				expr, operator, right
			)?;
		}
		Ok(expr)
	}

	fn unary(&mut self) -> Result<Expression<'a>, String> {
		if self.match_and_advance(&[Token::Not, Token::Minus]) {
			let operator = self.get_previous().clone();
			let right = self.unary()?;
			#[cfg(debug_assertions)]
			{
				println!("unary: {} {:?}", operator, right);
			}
			let unary = Unary::new(operator, right)?;
			return Ok(Expression::Unary(unary));
		}
		self.primary()
	}

	fn primary(&mut self) -> Result<Expression<'a>, String> {
		match self.get_current() {
			Token::Value(s) => {
				if let Some(literal) = Literal::from_str(s) {
					self.advance();
					#[cfg(debug_assertions)]
					{
						println!("literal: {:?}", &literal);
					}
					return Ok(Expression::Literal(literal));
				}
				if let Some(variable) = Variable::from_str(s) {
					self.advance();
					#[cfg(debug_assertions)]
					{
						println!("variable: {:?}", &variable);
					}
					return Ok(Expression::Variable(variable));
				}
				Err(self.error(self.get_current(), "Expect literal or variable."))
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
			_ => Err(self.error(self.get_current(), "Expected expression"))
		}
	}
}
