use super::Parser;
use crate::types::{
	LexerCode,
	ParserCode,
	OsmiaError,
};
use crate::model::code::*;

pub struct OsmiaParser;

impl OsmiaParser {
	pub fn new() -> Self {
		Self
	}
}

impl Parser<LexerCode, ParserCode, OsmiaError> for OsmiaParser {
	fn parse(&self, code: LexerCode) -> Result<ParserCode, OsmiaError> {
		Ok(OsmiaParserImpl::new(code).parse()?)
	}
}

use crate::model::lexer::Token;

struct OsmiaParserImpl {
	line: usize,
	code: LexerCode,
	current: usize,
}

impl OsmiaParserImpl {
	pub fn new(code: LexerCode) -> Self {
		Self {
			line: 1, code,
			current: 0
		}
	}

	pub fn parse(&mut self) -> Result<ParserCode, OsmiaError> {
		let code = self.code()?;
		if !self.done() {
			return Err(self.error(&format!(
				"Unexpected token {:?}",
				self.get_current()
			)));
		}
		Ok(code)
	}

	fn code(&mut self) -> Result<ParserCode, OsmiaError> {
		self.block()
	}
}

impl OsmiaParserImpl {
	fn error(&self, msg: &str) -> String {
		format!(
			"Parser error: Line {}: {}",
			self.line, msg
		)
	}

	fn done(&self) -> bool {
		self.check_current(&Token::Eof)
	}

	fn check_current(&self, token: &Token) -> bool {
		self.get_current() == token
	}

	fn get_current(&self) -> &Token {
		&self.code[self.current]
	}

	fn get_previous(&self) -> &Token {
		&self.code[self.current - 1]
	}

	fn advance(&mut self) -> &Token {
		if !self.done() {
			self.current += 1;
		}
		self.get_previous()
	}

	fn consume(&mut self, token: Token, error: fn(parser: &Self) -> OsmiaError) -> Result<(), OsmiaError> {
		match self.check_current(&token) {
			true => {
				self.advance();
				Ok(())
			}
			false => Err(error(self))
		}
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

	fn consume_while_match(&mut self, types: &[Token]) {
		while self.match_and_advance(types) {}
	}
}

impl OsmiaParserImpl {
	fn consume_new_lines(&mut self) { // TODO refactor
		self.consume_while_match(&[Token::NewLine]);
	}

	fn block(&mut self) -> Result<Stmt, OsmiaError> {
		self.breakable_block(&None)
	}

	fn breakable_block(&mut self, break_with: &Option<Vec<Token>>) -> Result<Stmt, OsmiaError> {
		let mut statements: Block = Block::new();
		loop {
			self.consume_new_lines();
			if self.done() {
				break;
			}
			match self.advance() {
				Token::Raw(r) => statements.push(Stmt::Raw(r.to_string())),
				Token::StmtStart => match self.stmt(break_with)? {
					None => break,
					Some(stmt) => statements.push(stmt),
				},
				_ => return Err(self.error(&format!(
					"Unexpected token {:?}",
					self.get_current()
				)))
			}
		}
		if statements.len() == 1 {
			let mut arr: Vec<Stmt> = statements.into();
			return Ok(arr.pop().unwrap().into());
		}
		Ok(statements.into())
	}

	fn stmt(&mut self, return_none_with: &Option<Vec<Token>>) -> Result<Option<Stmt>, OsmiaError> {
		let stmt: Stmt = match self.get_current() {
			_ => self.expr_stmt()?,
		};
		self.consume(
			Token::StmtEnd,
			|parser| parser.error(&format!("Expected '{:?}'", Token::StmtEnd))
		)?;
		Ok(Some(stmt))
	}

	fn expr_stmt(&mut self) -> Result<Stmt, OsmiaError> {
		self.consume_while_match(&[Token::NewLine, Token::Whitespace]); // TODO newlines not counted
		let stmt = self.expr()?.into();
		self.consume_while_match(&[Token::NewLine, Token::Whitespace]); // TODO newlines not counted
		Ok(stmt)
	}

	fn parameters(&mut self) -> Result<(), OsmiaError> { // TODO
		todo!() // TODO
		// self.parameter()
		// ,
		// self.parameter()
		// *
		// ...
		// self.identifier()
		// ?
	}

	fn parameter(&mut self) -> Result<(), OsmiaError> { // TODO
		todo!() // TODO
		// self.identifier()
		// :
		// self.expr()
	}

	fn expr(&mut self) -> Result<Expr, OsmiaError> {
		match self.get_current() {
			Token::Function => self.lambda(),
			_ => self.logic_or()
		}
	}

	fn lambda(&mut self) -> Result<Expr, OsmiaError> {
		todo!() // TODO
	}

	fn logic_or(&mut self) -> Result<Expr, OsmiaError> {
		self.logic_and() // TODO
	}

	fn logic_and(&mut self) -> Result<Expr, OsmiaError> {
		self.equality() // TODO
	}

	fn equality(&mut self) -> Result<Expr, OsmiaError> {
		self.bitwise() // TODO
	}

	fn bitwise(&mut self) -> Result<Expr, OsmiaError> {
		self.comparison() // TODO
	}

	fn comparison(&mut self) -> Result<Expr, OsmiaError> {
		self.bitshift() // TODO
	}

	fn bitshift(&mut self) -> Result<Expr, OsmiaError> {
		self.term() // TODO
	}

	fn term(&mut self) -> Result<Expr, OsmiaError> {
		self.factor() // TODO
	}

	fn factor(&mut self) -> Result<Expr, OsmiaError> {
		self.unary() // TODO
	}

	fn unary(&mut self) -> Result<Expr, OsmiaError> {
		self.method_call() // TODO
	}

	fn method_call(&mut self) -> Result<Expr, OsmiaError> {
		self.primary() // TODO
		// if ?
		// self.call() // TODO
	}

	fn primary(&mut self) -> Result<Expr, OsmiaError> {
		match self.get_current() {
			Token::ArrayStart => self.array(),
			Token::ObjectStart => self.object(),
			Token::ParentStart => self.grouping(),
			Token::Str(_) | Token::Number(_) | Token::Bool(_) | Token::Null => self.literal(),
			_ => self.call(),
		}
	}

	fn literal(&mut self) -> Result<Expr, OsmiaError> {
		let expr = match self.get_current() {
			Token::Null => Expr::Null,
			Token::Bool(b) => Expr::Bool(*b),
			Token::Str(s) => Expr::Str(s.to_string()),
			Token::Number(n) => {
				if let Ok(i) = n.parse::<i64>() {
					Expr::Int(i)
				}
				else if let Ok(f) = n.parse::<f64>() {
					Expr::Float(f)
				}
				else {
					return Err(self.error(&format!(
						"Could not parse number: {}",
						n
					)));
				}
			},
			_ => return Err(self.error(&format!(
				"Unexpected literal {:?}",
				self.get_current()
			)))
		};
		self.advance();
		Ok(expr)
	}

	fn call(&mut self) -> Result<Expr, OsmiaError> {
		self.variable() // TODO
		// if (
		// self.arguments() // TODO
		// )
	}

	fn arguments(&mut self) -> Result<Expr, OsmiaError> {
		// self.expr()
		// ,
		// self.expr()
		// *
		todo!() // TODO
	}

	fn variable(&mut self) -> Result<Expr, OsmiaError> {
		Ok(self.obj()?.into()) // TODO
	}

	fn obj(&mut self) -> Result<Variable, OsmiaError> {
		self.arr() // TODO
		// .
		// self.identifier() // TODO
		// *
	}

	fn arr(&mut self) -> Result<Variable, OsmiaError> {
		self.identifier() // TODO
		// [
		// self.expr() // TODO
		// ]*
	}

	fn array(&mut self) -> Result<Expr, OsmiaError> {
		// [ // TODO
		// self.expr() // TODO
		// *
		// ]
		todo!() // TODO
	}

	fn object(&mut self) -> Result<Expr, OsmiaError> {
		// { // TODO
		// self.object_entry() // TODO
		// *
		// }
		todo!() // TODO
	}

	fn object_entry(&mut self) -> Result<Expr, OsmiaError> {
		self.expr() // TODO
		// :
		// self.expr() // TODO
	}

	fn grouping(&mut self) -> Result<Expr, OsmiaError> {
		// (
		self.expr() // TODO
		// )
	}

	fn identifier(&mut self) -> Result<Variable, OsmiaError> {
		let key = match self.advance() {
			Token::Alpha(s) => s.as_str().into(),
			_ => return Err(self.error(&format!(
				"Invalid identifier: {:?}",
				self.get_previous()
			)))
		};
		Ok(Variable::from_vec(vec![key]))
	}
}
