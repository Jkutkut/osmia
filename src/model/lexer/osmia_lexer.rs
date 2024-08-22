use super::Lexer;
use crate::LexerCode;
use crate::OsmiaError;

pub struct OsmiaLexer<'a> {
	#[allow(dead_code)]
	start_delimiter: &'a str,
	#[allow(dead_code)]
	end_delimiter: &'a str
}

impl<'a> OsmiaLexer<'a> {
	pub fn new(start_delimiter: &'a str, end_delimiter: &'a str) -> Self {
		Self { start_delimiter, end_delimiter }
	}

	pub fn osmia() -> Self {
		Self::new("{{", "}}")
	}
}

impl Lexer<LexerCode, OsmiaError> for OsmiaLexer<'_> {
	#[allow(unused_variables)]
	fn lex(&self, code: &str) -> Result<LexerCode, OsmiaError> {
		todo!() // TODO
		// Lex code
	}
}
