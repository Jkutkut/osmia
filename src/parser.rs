use crate::lexer::Token;
use crate::syntax_tree::model::{
	Expression, Binary, Unary, Grouping, Literal, Variable,
	Stmt, Block, Assign
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
/// Assign         → "{{" "assign" Variable "=" expression "}}" ;
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
/// primary        →  Literal | Variable | grouping;
/// grouping       →  "(" expression ")" ;
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
		let result = self.block()?;
		Ok(result)
	}

	// TODO move logic
	fn block(&mut self) -> Result<Stmt<'a>, String> {
		let mut statements: Vec<Stmt<'a>> = Vec::new();
		while !self.is_at_end() {
			match self.advance() {
				Token::Raw(r) => {
					statements.push(Stmt::Raw(r));
				},
				Token::DelimiterStart => {
					let stmt = match self.get_current() {
						Token::Print => self.print()?,
						Token::Assign => self.assign()?,
						// TODO if
						// TODO while
						// TODO for
						Token::Continue => self.continue_stmt(),
						Token::Break => self.break_stmt(),
						_ => self.statement()?,
					};
					self.consume(
						Token::DelimiterEnd,
						format!("Expected '{}'", Token::DelimiterEnd).as_str(),
					)?;
					statements.push(stmt);
				},
				_ => {
					return Err(self.error(self.get_current(), "Unexpected token"));
				}
			}
		}
		if statements.len() == 1 {
			return Ok(statements.pop().unwrap());
		}
		Ok(Stmt::Block(Block::new(statements)))
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
		self.get_current() == token2compare
	}

	fn advance(&mut self) -> &Token<'a> {
		if !self.is_at_end() {
			self.current += 1;
		}
		self.get_previous()
	}

	fn is_at_end(&self) -> bool {
		self.current >= self.tokens.len()
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
		Err(self.error(self.get_current(), message))
	}

	fn error(&self, token: &Token<'a>, message: &str) -> String {
		format!("{} at '{}'.", message, token)
	}
}

// Grammar: Statements

impl<'a> Parser<'a> {
	fn print(&mut self) -> Result<Stmt<'a>, String> {
		self.advance();
		let expression = self.expression()?;
		Ok(Stmt::Print(expression))
	}

	fn statement(&mut self) -> Result<Stmt<'a>, String> {
		let expression = self.expression()?;
		Ok(Stmt::Expression(expression))
	}

	fn assign(&mut self) -> Result<Stmt<'a>, String> {
		self.advance();
		let variable = match self.get_current() {
			Token::Value(name) => {
				let variable = self.variable(name)?;
				self.advance();
				variable
			},
			_ => {
				return Err(self.error(
					self.get_current(),
					"Expected variable before '=' in assign."
				));
			}
		};
		self.consume(
			Token::Equal,
			"Expected '=' after variable",
		)?;
		let expression = self.expression()?;
		Ok(Stmt::Assign(Assign::new(variable, expression)))
	}

	fn continue_stmt(&mut self) -> Stmt<'a> {
		self.advance();
		Stmt::Continue
	}

	fn break_stmt(&mut self) -> Stmt<'a> {
		self.advance();
		Stmt::Break
	}
}

// Grammar: Expression

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
				if let Ok(literal) = self.literal(s) {
					self.advance();
					return Ok(Expression::Literal(literal));
				}
				if let Ok(variable) = self.variable(s) {
					self.advance();
					return Ok(Expression::Variable(variable));
				}
				Err(self.error(self.get_current(), "Expect literal or variable."))
			},
			Token::GroupingStart => Ok(self.grouping()?),
			_ => Err(self.error(self.get_current(), "Expected expression"))
		}
	}

	fn variable(&self, name: &'a str) -> Result<Variable<'a>, String> {
		if let Some(variable) = Variable::from_str(name) {
			#[cfg(debug_assertions)]
			{
				println!("variable: {:?}", &variable);
			}
			return Ok(variable);
		}
		Err(self.error(self.get_current(), "Expected variable."))
	}

	fn literal(&self, text: &'a str) -> Result<Literal, String> {
		if let Some(literal) = Literal::from_str(text) {
			#[cfg(debug_assertions)]
			{
				println!("literal: {:?}", &literal);
			}
			return Ok(literal);
		}
		Err(self.error(self.get_current(), "Expected literal."))
	}

	fn grouping(&mut self) -> Result<Expression<'a>, String> {
		self.consume(
			Token::GroupingStart,
			&format!("Expected '{}' before expression.", Token::GroupingStart)
		)?;
		let expr = self.expression()?;
		self.consume(
			Token::GroupingEnd,
			&format!("Expected '{}' after expression.", Token::GroupingEnd)
		)?;
		Ok(Expression::Grouping(Grouping::new(expr)))
	}
}
