use crate::lexer::Token;
use crate::syntax_tree::model::{
	Expression, Binary, Unary, Grouping, Literal, Variable,
	Stmt, Block, Assign, ConditionalBlock, ForEach, If
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
/// If             → "{{" "if" Conditional ( "{{" "elseif" Conditional )* ( "{{" "else" Block )? "{{" "fi" "}}" ;
/// Conditional    → expression "}}" Stmt 
/// While          → "{{" "while" Conditional "{{" "done" "}}" ;
/// ForEach        → "{{" "foreach" Variable "in" Variable "}}" Stmt "{{" "done" "}}" ;
/// Break          → "{{" "break" "}}" ;
/// Continue       → "{{" "continue" "}}" ;
///
/// expression     → equality ;
/// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
/// comparison     → bool_op ( ( ">" | ">=" | "<" | "<=" ) bool_op )* ;
/// bool_op        → term ( ( "&&" | "||" ) term )* ;
/// term           → factor ( ( "-" | "+" ) factor )* ;
/// factor         → unary ( ( "/" | "*" ) unary )* ;
/// unary          → ( "!" | "-" | "+" ) unary | primary ;
/// primary        →  Literal | Variable | grouping;
/// grouping       →  "(" expression ")" ;
/// ```
pub struct Parser<'a> {
	tokens: Vec<Token<'a>>,
	current: usize,
}

// Public methods

impl<'a> Parser<'a> {
	/// Constructor for the parser.
	///
	/// # Arguments
	/// `tokens` - List of tokens to parse.
	pub fn new(tokens: Vec<Token<'a>>) -> Self {
		Self {
			tokens,
			current: 0,
		}
	}

	/// Attempts to parse the list of tokens into a syntax tree.
	///
	/// # Returns
	/// `Result<Stmt, String>` - The syntax tree or an error message.
	pub fn parse(&mut self) -> Result<Stmt<'a>, String> {
		let code = self.code()?;
		if !self.is_at_end() {
			return Err(self.error("Expected end of program"));
		}
		Ok(code)
	}
}

// Tools

impl<'a> Parser<'a> {
	fn new_binary(
		left: Expression<'a>,
		operator: Token<'a>,
		right: Expression<'a>
	) -> Result<Expression<'a>, String> {
		Ok(Expression::Binary(Binary::new(
			left, operator, right
		)?))
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
		self.check_current(&Token::Eof)
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
		Err(self.error(message))
	}

	#[cfg(debug_assertions)]
	fn error(&self, message: &str) -> String {
		format!(
			"parser error: {} at '{}' (Token at index: {}).",
			message, self.get_current(), self.current
		)
	}

	#[cfg(not(debug_assertions))]
	fn error(&self, message: &str) -> String {
		format!("parser error: {} at '{}'.", message, self.get_current())
	}
}

// Grammar: Statements

impl<'a> Parser<'a> {
	fn code(&mut self) -> Result<Stmt<'a>, String> {
		self.block(None)
	}

	fn block(&mut self, end_tokens: Option<Vec<Token>>) -> Result<Stmt<'a>, String> {
		let mut statements: Vec<Stmt<'a>> = Vec::new();
		while !self.is_at_end() {
			match self.advance() {
				Token::Raw(r) => statements.push(Stmt::Raw(r)),
				Token::DelimiterStart => match self.block_stmt(&end_tokens) {
					Ok(Some(stmt)) => statements.push(stmt),
					Ok(None) => break,
					Err(e) => return Err(e)
				},
				_ => {
					return Err(self.error("Unexpected token"));
				}
			}
		}
		if statements.len() == 1 {
			return Ok(statements.pop().unwrap());
		}
		Ok(Stmt::Block(Block::new(statements)))
	}

	fn block_stmt(&mut self, end_tokens: &Option<Vec<Token>>) -> Result<Option<Stmt<'a>>, String> {
		let stmt = match self.get_current() {
			Token::Print => self.print()?,
			Token::Assign => self.assign()?,
			Token::If => self.if_stmt()?,
			Token::While => self.while_stmt()?,
			Token::For => self.foreach()?,
			Token::Continue => self.continue_stmt(),
			Token::Break => self.break_stmt(),
			Token::ElseIf | Token::Else | Token::Fi | Token::Done => {
				let is_close_block = self.check_current(&Token::Fi) || self.check_current(&Token::Done);
				if self.close_block(end_tokens, is_close_block)? {
					return Ok(None);
				}
				return Err(self.error("Unexpected token in block"));
			}
			_ => self.statement()?,
		};
		self.consume(
			Token::DelimiterEnd,
			format!("Expected '{}'", Token::DelimiterEnd).as_str(),
		)?;
		Ok(Some(stmt))
	}

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
				return Err(self.error("Expected variable before '=' in assign"));
			}
		};
		self.consume(
			Token::AssignEq,
			&format!("Expected '{}' after variable", Token::AssignEq),
		)?;
		let expression = self.expression()?;
		Ok(Stmt::Assign(Assign::new(variable, expression)))
	}

	fn foreach(&mut self) -> Result<Stmt<'a>, String> {
		self.advance();
		let variable = match self.get_current() {
			Token::Value(name) => {
				let variable = self.variable(name)?;
				self.advance();
				variable
			},
			_ => return Err(self.error(
				&format!("Expected variable after '{}' in {} statement", Token::For, Token::For)
			))
		};
		self.consume(
			Token::In,
			&format!("Expected '{}' after variable in {} statement", Token::In, Token::For),
		)?;
		let list = match self.get_current() {
			Token::Value(name) => {
				let variable = self.variable(name)?;
				self.advance();
				variable
			},
			_ => return Err(self.error(
				&format!("Expected variable after '{}' in {} statement", Token::In, Token::For)
			))
		};
		self.consume(
			Token::DelimiterEnd,
			&format!("Expected '{}' in {} statement", Token::DelimiterEnd, Token::For),
		)?;
		let block = self.block(Some(vec![
			Token::Done
		]))?;
		Ok(Stmt::ForEach(ForEach::new(variable, list, block)))
	}

	fn if_stmt(&mut self) -> Result<Stmt<'a>, String> {
		self.advance();
		let if_block = self.conditional(vec![Token::Else, Token::ElseIf, Token::Fi])?;
		let mut else_if_blocks = Vec::new();
		while self.check_current(&Token::ElseIf) {
			self.advance();
			else_if_blocks.push(
				self.conditional(vec![Token::Else, Token::ElseIf, Token::Fi])?
			);
		}
		let else_if_blocks = match else_if_blocks.len() {
			0 => None,
			_ => Some(else_if_blocks)
		};
		let mut else_block = None;
		if self.check_current(&Token::Else) {
			self.advance();
			self.consume(
				Token::DelimiterEnd,
				&format!("Expected '{}' after else statement", Token::DelimiterEnd)
			)?;
			else_block = Some(self.block(Some(vec![Token::Fi]))?);
		}
		Ok(Stmt::If(If::new(
			if_block, else_if_blocks, else_block
		)))
	}

	fn while_stmt(&mut self) -> Result<Stmt<'a>, String> {
		self.advance();
		Ok(Stmt::While(self.conditional(vec![Token::Done])?))
	}

	fn conditional(&mut self, end_tokens: Vec<Token>) -> Result<ConditionalBlock<'a>, String> {
		let expr = self.expression()?;
		self.consume(
			Token::DelimiterEnd,
			format!("Unclosed '{}' in conditional statement", Token::DelimiterEnd).as_str(),
		)?;
		let block = self.block(Some(end_tokens))?;
		Ok(ConditionalBlock::new(expr, block))
	}

	fn close_block(&mut self, end_tokens: &Option<Vec<Token>>, advance: bool) -> Result<bool, String> {
		let mut is_end_token = false;
		if let Some(ref end_tokens) = end_tokens {
			for end_token in end_tokens {
				if self.check_current(&end_token) {
					if advance {
						self.advance();
					}
					is_end_token = true;
					break;
				}
			}
		}
		Ok(is_end_token)
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
			expr = Self::new_binary(
				expr, operator, right
			)?;
		}
		Ok(expr)
	}

	fn comparison(&mut self) -> Result<Expression<'a>, String> {
		let mut expr = self.bool_op()?;
		while self.match_and_advance(&[
			Token::GreaterThan, Token::GreaterEqual,
			Token::LessThan, Token::LessEqual
		]) {
			let operator = self.get_previous().clone();
			let right = self.bool_op()?;
			expr = Self::new_binary(
				expr, operator, right
			)?;
		}
		Ok(expr)
	}

	fn bool_op(&mut self) -> Result<Expression<'a>, String> {
		let mut expr = self.term()?;
		while self.match_and_advance(&[Token::And, Token::Or]) {
			let operator = self.get_previous().clone();
			let right = self.term()?;
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
			expr = Self::new_binary(
				expr, operator, right
			)?;
		}
		Ok(expr)
	}

	fn unary(&mut self) -> Result<Expression<'a>, String> {
		if self.match_and_advance(&[Token::Not, Token::Minus, Token::Plus]) {
			let operator = self.get_previous().clone();
			let right = self.unary()?;
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
				Err(self.error("Expect literal or variable"))
			},
			Token::GroupingStart => Ok(self.grouping()?),
			_ => Err(self.error("Expected expression"))
		}
	}

	fn variable(&self, name: &'a str) -> Result<Variable<'a>, String> {
		if let Some(variable) = Variable::from_str(name) {
			return Ok(variable);
		}
		Err(self.error("Expected variable"))
	}

	fn literal(&self, text: &'a str) -> Result<Literal, String> {
		if let Some(literal) = Literal::from_str(text) {
			return Ok(literal);
		}
		Err(self.error("Expected literal"))
	}

	fn grouping(&mut self) -> Result<Expression<'a>, String> {
		self.consume(
			Token::GroupingStart,
			&format!("Expected '{}' before expression", Token::GroupingStart)
		)?;
		let expr = self.expression()?;
		self.consume(
			Token::GroupingEnd,
			&format!("Expected '{}' after expression", Token::GroupingEnd)
		)?;
		Ok(Expression::Grouping(Grouping::new(expr)))
	}
}
