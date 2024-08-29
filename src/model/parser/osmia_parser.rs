use super::Parser;
use crate::types::{
	LexerCode,
	ParserCode,
	OsmiaError,
};

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
	fn new(code: LexerCode) -> Self {
		Self {
			line: 1, code,
			current: 0
		}
	}

	fn parse(&mut self) -> Result<ParserCode, OsmiaError> {
		let code = self.code()?;
		if !self.done() {
			return Err(self.error(format!(
				"Unexpected token {:?}",
				self.get_current()
			)));
		}
		Ok(code)
	}

	fn code(&mut self) -> Result<ParserCode, OsmiaError> {
		todo!();
	}
}

impl OsmiaParserImpl {
	fn error(&self, msg: String) -> String {
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
}
