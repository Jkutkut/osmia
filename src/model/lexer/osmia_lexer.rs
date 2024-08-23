use super::Lexer;
use crate::LexerCode;
use crate::OsmiaError;

pub struct OsmiaLexer<'a> {
	start_delimiter: &'a str,
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
	fn lex(&self, code: &str) -> Result<LexerCode, OsmiaError> {
		let mut scanner = LexerScanner::new(code, self.start_delimiter, self.end_delimiter);
		scanner.scan().map_err(|err| format!("Lexer error: {}", err))
	}
}

use crate::model::lexer::Token;

struct LexerScanner<'a> {
	start_delimiter: &'a str,
	end_delimiter: &'a str,
	code: &'a [u8],
	index: usize,
	current_line: usize,
	tokens: LexerCode,
	in_stmt: bool
}

impl<'a> LexerScanner<'a> {
	pub fn new(code: &'a str, start_delimiter: &'a str, end_delimiter: &'a str) -> Self {
		Self {
			start_delimiter,
			end_delimiter,
			code: code.as_bytes(),
			index: 0,
			current_line: 0,
			tokens: Vec::new(),
			in_stmt: false
		}
	}
}

impl<'a> LexerScanner<'a> {
	fn error(&self, msg: String) -> Result<LexerCode, String> {
		Err(format!("Line {}: {}", self.current_line, msg))
	}

	fn code_left(&self) -> bool {
		self.index < self.code.len()
	}

	fn current(&self) -> u8 {
		if self.current_index() >= self.code.len() {
			return 0;
		}
		self.code[self.index]
	}

	fn current_index(&self) -> usize {
		self.index
	}

	fn advance(&mut self) {
		self.index += 1;
	}

	fn consume(&mut self, lex: &str) -> bool {
		if self.is_match(lex) {
			self.index += lex.len();
			true
		} else {
			false
		}
	}

	fn is_match(&self, expected: &str) -> bool {
		let b_expected = expected.as_bytes();
		let mut i = 0;
		if self.current_index() + b_expected.len() > self.code.len() {
			return false;
		}
		while i < b_expected.len() {
			if self.code[self.current_index() + i] != b_expected[i] {
				return false;
			}
			i += 1;
		}
		true
	}

	fn pick_string(&self, start: usize, mut end: usize) -> Option<String> { if end > self.code.len() {
			end = self.code.len() - 1;
		}
		if start > end {
			panic!("Invalid range");
		}
		if start == end {
			return None;
		}
		Some(String::from_utf8_lossy(
			&self.code[start..end]
		).to_string())
	}
}

impl<'a> LexerScanner<'a> {
	fn consume_raw(&mut self) {
		let start = self.current_index();
		while self.code_left() && !self.is_match(self.start_delimiter) && self.current() != b'\n' {
			self.advance();
		}
		if let Some(content) = self.pick_string(start, self.current_index()) {
			self.tokens.push(Token::Raw(content));
		}
		self.consume_new_line();
	}

	fn consume_new_line(&mut self) {
		while self.code_left() && self.current() == b'\n' {
			self.advance();
			self.tokens.push(Token::NewLine);
		}
	}

	fn consume_start_delimiter(&mut self) {
		if self.consume(self.start_delimiter) {
			self.in_stmt = true;
			self.tokens.push(Token::StmtStart);
		}
	}

	fn consume_end_delimiter(&mut self) {
		if self.consume(self.end_delimiter) {
			self.in_stmt = false;
			self.tokens.push(Token::StmtEnd);
		}
	}

	fn consume_token(&mut self) -> Result<(), String> {
		// TODO implement
		Ok(())
	}
}

impl<'a> LexerScanner<'a> { // TODO move to public impl block
	pub fn scan(&mut self) -> Result<LexerCode, String> {
		while self.code_left() {
			if !self.in_stmt {
				self.consume_raw();
				self.consume_start_delimiter();
			}
			else {
				while self.code_left() && self.current().is_ascii_whitespace() {
					if self.current() == b'\n' {
						self.consume_new_line();
					}
					else {
						self.advance();
					}
				}
				self.consume_end_delimiter();
				self.consume_token()?;
			}
		}
		if self.in_stmt {
			return self.error(format!(
				"Unexpected end of statement. Expected '{}'",
				self.end_delimiter
			));
		}
		self.tokens.push(Token::Eof);
		Ok(self.tokens.clone())
	}
}
