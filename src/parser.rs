use std::collections::HashMap;
use crate::lexer::Token;
use crate::model::{
	Expression, Binary, Unary, Grouping, Literal, Variable,
	JsonExpression, ListOrVariable,
	Stmt, Block, Assign, ConditionalBlock, ForEach, If,
	Callable, Call, MethodCall,
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
/// Print          → "{{" "print" json";" "}}" ;
/// Expression     → "{{" json "}}" ;
/// Assign         → "{{" "assign" Variable "=" json "}}" ;
/// If             → "{{" "if" Conditional ( "{{" "elseif" Conditional )* ( "{{" "else" Block )? "{{" "fi" "}}" ;
/// Conditional    → expression "}}" Stmt 
/// While          → "{{" "while" Conditional "{{" "done" "}}" ;
/// ForEach        → "{{" "foreach" Variable "in" ListOrVariable "}}" Stmt "{{" "done" "}}" ;
/// Break          → "{{" "break" "}}" ;
/// Continue       → "{{" "continue" "}}" ;
///
/// json           → object | array | expression ;
/// jsonObject     → "{" ( Literal ":" json "," )* ( Literal ":" json )? "}" ;
/// jsonArray      → "[" ( json "," )* ( json )? "]" ;
/// ListOrVariable → Variable | array ;
///
/// expression     → logic_or ;
/// logic_or       → logic_and ( "||" logic_and )* ;
/// logic_and      → equality ( "&&" equality )* ;
/// equality       → bitwise ( ( "!=" | "==" ) bitwise )* ;
/// bitwise        → comparison ( ( "&" | "|" | "^" ) comparison )* ;
/// comparison     → bitshift ( ( ">" | ">=" | "<" | "<=" ) bitshift )* ;
/// bitshift       → term ( ( ">>" | "<<" ) term )* ;
/// term           → factor ( ( "-" | "+" ) factor )* ;
/// factor         → unary ( ( "/" | "*" ) unary )* ;
/// unary          → ( "!" | "-" | "+" ) unary | method ;
/// method         → ( primary | call ) ( "?" call )* ;
/// call           → primary ( "(" arguments? ")" )+ ;
/// arguments      → json ( "," json )* ;
/// primary        → Literal | Variable | grouping;
/// grouping       → "(" expression ")" ;
/// ```
pub struct Parser {
	tokens: Vec<Token>,
	current: usize,
}

// Public methods

impl Parser {
	/// Constructor for the parser.
	///
	/// # Arguments
	/// `tokens` - List of tokens to parse.
	pub fn new(tokens: Vec<Token>) -> Self {
		Self {
			tokens,
			current: 0,
		}
	}

	/// Attempts to parse the list of tokens into a syntax tree.
	///
	/// # Returns
	/// `Result<Stmt, String>` - The syntax tree or an error message.
	pub fn parse(&mut self) -> Result<Stmt, String> {
		let code = self.code()?;
		if !self.is_at_end() {
			return Err(self.error("Expected end of program"));
		}
		Ok(code)
	}
}

// Tools

impl Parser {
	fn new_binary(
		left: Expression,
		operator: Token,
		right: Expression
	) -> Result<Expression, String> {
		Ok(Expression::Binary(Binary::new(
			left, operator, right
		)?))
	}

	fn match_and_advance(&mut self, types: &[Token]) -> bool {
		for token_type in types {
			if self.check_current(token_type) {
				self.advance();
				return true
			}
		}
		false
	}

	fn check_current(&self, token2compare: &Token) -> bool {
		self.get_current() == token2compare
	}

	fn advance(&mut self) -> &Token {
		if !self.is_at_end() {
			self.current += 1;
		}
		self.get_previous()
	}

	fn is_at_end(&self) -> bool {
		self.check_current(&Token::Eof)
	}

	fn get_current(&self) -> &Token {
		&self.tokens[self.current]
	}

	fn get_previous(&self) -> &Token {
		&self.tokens[self.current - 1]
	}

	fn consume(
		&mut self,
		token: Token,
		message: &str
	) -> Result<Token, String> {
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

impl Parser {
	fn code(&mut self) -> Result<Stmt, String> {
		self.block(None)
	}

	fn block(&mut self, end_tokens: Option<Vec<Token>>) -> Result<Stmt, String> {
		let mut statements: Vec<Stmt> = Vec::new();
		while !self.is_at_end() {
			match self.advance() {
				Token::Raw(r) => statements.push(Stmt::Raw(r.to_string())),
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

	fn block_stmt(&mut self, end_tokens: &Option<Vec<Token>>) -> Result<Option<Stmt>, String> {
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

	fn print(&mut self) -> Result<Stmt, String> {
		self.advance();
		let expression = self.json_expression()?;
		Ok(Stmt::Print(expression))
	}

	fn statement(&mut self) -> Result<Stmt, String> {
		let expression = self.json_expression()?;
		Ok(Stmt::Expression(expression))
	}

	fn assign(&mut self) -> Result<Stmt, String> {
		self.advance();
		let variable = match self.get_current() {
			Token::Value(name) => self.variable(&name.to_string())?,
			_ => {
				return Err(self.error("Expected variable before '=' in assign"));
			}
		};
		self.consume(
			Token::AssignEq,
			&format!("Expected '{}' after variable", Token::AssignEq),
		)?;
		let json = self.json_expression()?;
		Ok(Stmt::Assign(Assign::new(variable, json)))
	}

	fn foreach(&mut self) -> Result<Stmt, String> {
		self.advance();
		let variable = match self.get_current() {
			Token::Value(name) => self.variable(&name.to_string())?,
			_ => return Err(self.error(
				&format!("Expected variable after '{}' in {} statement", Token::For, Token::For)
			))
		};
		self.consume(
			Token::In,
			&format!("Expected '{}' after variable in {} statement", Token::In, Token::For),
		)?;
		let list = self.list_or_variable()?;
		self.consume(
			Token::DelimiterEnd,
			&format!("Expected '{}' in {} statement", Token::DelimiterEnd, Token::For),
		)?;
		let block = self.block(Some(vec![
			Token::Done
		]))?;
		Ok(Stmt::ForEach(ForEach::new(variable, list, block)))
	}

	fn if_stmt(&mut self) -> Result<Stmt, String> {
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

	fn while_stmt(&mut self) -> Result<Stmt, String> {
		self.advance();
		Ok(Stmt::While(self.conditional(vec![Token::Done])?))
	}

	fn conditional(&mut self, end_tokens: Vec<Token>) -> Result<ConditionalBlock, String> {
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

	fn continue_stmt(&mut self) -> Stmt {
		self.advance();
		Stmt::Continue
	}

	fn break_stmt(&mut self) -> Stmt {
		self.advance();
		Stmt::Break
	}
}

// Grammar: Json
impl Parser {
	fn json_expression(&mut self) -> Result<JsonExpression, String> {
		match self.get_current() {
			Token::ObjectStart => self.json_object(),
			Token::ArrayStart => self.json_array(),
			_ => Ok(JsonExpression::Expression(
				self.expression()?
			))
		}
	}

	fn json_array(&mut self) -> Result<JsonExpression, String> {
		self.consume(
			Token::ArrayStart,
			&format!("Expected '{}' before array", Token::ArrayStart)
		)?;
		let mut elements = Vec::new();
		while !self.check_current(&Token::ArrayEnd) && !self.is_at_end() {
			elements.push(self.json_expression()?);
			if !self.check_current(&Token::ArrayEnd) && !self.is_at_end() {
				self.consume(
					Token::Comma,
					&format!("Expected '{}' after array element", Token::Comma)
				)?;
			}
		}
		self.consume(
			Token::ArrayEnd,
			&format!("Expected '{}' after array", Token::ArrayEnd)
		)?;
		Ok(JsonExpression::Array(elements))
	}

	fn json_object(&mut self) -> Result<JsonExpression, String> {
		self.consume(
			Token::ObjectStart,
			&format!("Expected '{}' before object", Token::ObjectStart)
		)?;
		let mut object = HashMap::new();
		while !self.check_current(&Token::ObjectEnd) && !self.is_at_end() {
			let key = match self.primary() {
				Ok(Expression::Literal(Literal::Str(s))) => s,
				_ => return Err(self.error("Expected string literal as key in object"))
			};
			self.consume(
				Token::Colon,
				&format!("Expected '{}' after key in object", Token::Colon)
			)?;
			object.insert(key, self.json_expression()?);
			if !self.check_current(&Token::ObjectEnd) && !self.is_at_end() {
				self.consume(
					Token::Comma,
					&format!("Expected '{}' after object element", Token::Comma)
				)?;
			}
		}
		self.consume(
			Token::ObjectEnd,
			&format!("Expected '{}' after object", Token::ObjectEnd)
		)?;
		Ok(JsonExpression::Object(object))
	}

	fn list_or_variable(&mut self) -> Result<ListOrVariable, String> {
		let result = match self.get_current() {
			Token::Value(name) => ListOrVariable::Variable(self.variable(&name.to_string())?),
			Token::ArrayStart => ListOrVariable::List(self.json_array()?),
			_ => return Err(self.error(
				&format!("Expected variable or list after '{}' in {} statement", Token::In, Token::For)
			))
		};
		Ok(result)
	}
}

// Grammar: Expression

impl Parser {
	fn expression(&mut self) -> Result<Expression, String> {
		self.logic_or()
	}

	fn logic_or(&mut self) -> Result<Expression, String> {
		let mut expr = self.logic_and()?;
		while self.match_and_advance(&[Token::Or]) {
			let operator = self.get_previous().clone();
			let right = self.logic_and()?;
			expr = Self::new_binary(
				expr, operator, right
			)?;
		}
		Ok(expr)
	}

	fn logic_and(&mut self) -> Result<Expression, String> {
		let mut expr = self.equality()?;
		while self.match_and_advance(&[Token::And]) {
			let operator = self.get_previous().clone();
			let right = self.equality()?;
			expr = Self::new_binary(
				expr, operator, right
			)?;
		}
		Ok(expr)
	}

	fn equality(&mut self) -> Result<Expression, String> {
		let mut expr = self.bitwise()?;
		while self.match_and_advance(&[Token::NotEqual, Token::Equal]) {
			let operator = self.get_previous().clone();
			let right = self.bitwise()?;
			expr = Self::new_binary(
				expr, operator, right
			)?;
		}
		Ok(expr)
	}

	fn bitwise(&mut self) -> Result<Expression, String> {
		let mut expr = self.comparison()?;
		while self.match_and_advance(&[
			Token::BitAnd, Token::BitOr, Token::BitXor
		]) {
			let operator = self.get_previous().clone();
			let right = self.comparison()?;
			expr = Self::new_binary(
				expr, operator, right
			)?;
		}
		Ok(expr)
	}

	fn comparison(&mut self) -> Result<Expression, String> {
		let mut expr = self.bitshift()?;
		while self.match_and_advance(&[
			Token::GreaterThan, Token::GreaterEqual,
			Token::LessThan, Token::LessEqual
		]) {
			let operator = self.get_previous().clone();
			let right = self.bitshift()?;
			expr = Self::new_binary(
				expr, operator, right
			)?;
		}
		Ok(expr)
	}

	fn bitshift(&mut self) -> Result<Expression, String> {
		let mut expr = self.term()?;
		while self.match_and_advance(&[
			Token::BitShiftLeft, Token::BitShiftRight
		]) {
			let operator = self.get_previous().clone();
			let right = self.term()?;
			expr = Self::new_binary(
				expr, operator, right
			)?;
		}
		Ok(expr)
	}

	fn term(&mut self) -> Result<Expression, String> {
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

	fn factor(&mut self) -> Result<Expression, String> {
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

	fn unary(&mut self) -> Result<Expression, String> {
		if self.match_and_advance(&[Token::Not, Token::Minus, Token::Plus]) {
			let operator = self.get_previous().clone();
			let right = self.unary()?;
			let unary = Unary::new(operator, right)?;
			return Ok(Expression::Unary(unary));
		}
		self.method()
	}

	fn method(&mut self) -> Result<Expression, String> {
		let mut expr = self.call()?;
		while self.match_and_advance(&[Token::Question]) {
			let call = match self.call()? {
				Expression::Callable(c) => c,
				_ => return Err(format!(
					"Expected a method call after {}",
					Token::Question
				))
			};
			expr = Expression::Callable(Callable::new_method_call(expr, call));
		}
		Ok(expr)
	}

	fn call(&mut self) -> Result<Expression, String> {
		let mut expr = self.primary()?;
		while self.match_and_advance(&[Token::ParentStart]) {
			let args = self.arguments()?;
			self.consume(
				Token::ParentEnd,
				&format!(
					"Unclosed {:?} in call (expected {:?})",
					Token::ParentStart, Token::ParentEnd
				)
			)?;
			let call = Callable::new_call(expr, args);
			expr = Expression::Callable(call);
		}
		Ok(expr)
	}

	fn arguments(&mut self) -> Result<Vec<JsonExpression>, String> {
		let mut args = Vec::new();
		if !self.check_current(&Token::ParentEnd) {
			loop {
				let json = self.json_expression()?;
				args.push(json);
				if !self.match_and_advance(&[Token::Comma]) {
					break;
				}
			}
		}
		Ok(args)
	}

	fn primary(&mut self) -> Result<Expression, String> {
		match self.get_current() {
			Token::Value(s) => {
				if let Ok(literal) = self.literal(s) {
					self.advance();
					return Ok(Expression::Literal(literal));
				}
				if let Ok(variable) = self.variable(&s.to_string()) {
					return Ok(Expression::Variable(variable));
				}
				Err(self.error("Invalid variable name"))
			},
			Token::ParentStart => Ok(self.grouping()?),
			_ => Err(self.error("Expected expression"))
		}
	}

	fn variable(&mut self, name: & str) -> Result<Variable, String> {
		if let Some(variable) = Variable::from_str(name) {
			self.advance();
			return Ok(variable);
		}
		Err(self.error("Expected variable"))
	}

	fn literal(&self, text: & str) -> Result<Literal, String> {
		if let Some(literal) = Literal::from_str(text) {
			return Ok(literal);
		}
		Err(self.error("Expected literal"))
	}

	fn grouping(&mut self) -> Result<Expression, String> {
		self.consume(
			Token::ParentStart,
			&format!("Expected '{}' before expression", Token::ParentStart)
		)?;
		let expr = self.expression()?;
		self.consume(
			Token::ParentEnd,
			&format!("Expected '{}' after expression", Token::ParentEnd)
		)?;
		Ok(Expression::Grouping(Grouping::new(expr)))
	}
}
