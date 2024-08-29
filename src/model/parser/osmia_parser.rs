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
}

impl OsmiaParserImpl {

	fn consume_new_lines(&mut self) {
		while !self.done() && self.check_current(&Token::NewLine) {
			self.advance();
			self.line += 1;
		}
	}

	fn block(&mut self) -> Result<ParserCode, OsmiaError> {
		let mut statements: Block = Block::new();
		while !self.done() {
			self.consume_new_lines();
			match self.advance() {
				Token::Raw(r) => statements.push(Stmt::Raw(r.to_string())),
				_ => {
					return Err(self.error("Unexpected token"));
				}
			}
		}
		if statements.len() == 1 {
			let mut arr: Vec<Stmt> = statements.into();
			return Ok(arr.pop().unwrap().into());
		}
		Ok(statements.into())
	}
}
