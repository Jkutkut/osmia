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
}

impl OsmiaParserImpl {
	fn consume_new_lines(&mut self) {
		while !self.done() && self.check_current(&Token::NewLine) {
			self.advance();
			self.line += 1;
		}
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
			_ => self.expr()?.into(),
		};
		self.consume(
			Token::StmtEnd,
			|parser| parser.error(&format!("Expected '{:?}'", Token::StmtEnd))
		)?;
		Ok(Some(stmt))
	}

	fn expr(&mut self) -> Result<Expr, OsmiaError> {
		self.consume_new_lines();
		self.primary() // TODO
	}

	fn primary(&mut self) -> Result<Expr, OsmiaError> {
		self.literal()
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
}
