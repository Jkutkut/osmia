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

use crate::utils::code_trace;
impl<'a> LexerScanner<'a> {
	fn error(&self, msg: String) -> String {
		let code_str: String = self.code.iter().map(|b| *b as char).collect();
		if cfg!(debug_assertions) {
			code_trace(
				&code_str, self.current_index(),
				&format!(
					"Line {}: {}\nTokens: {:?}",
					self.current_line, msg,
					self.tokens
				)
			)
		}
		else {
			code_trace(
				&code_str, self.current_index(),
				&format!("Line {}: {}", self.current_line, msg)
			)
		}
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
		if !self.code_left() {
			return Ok(());
		}
		// TODO implement
		{
			let special_tokens = ["..."];
			for special_token in special_tokens {
				if self.consume(special_token) {
					self.tokens.push(Token::try_from(special_token).unwrap());
					return Ok(());
				}
			}
			let mut pieces: Vec<String> = Vec::new();
			if self.current_index() + 1 < self.code.len() {
				pieces.push(self.pick_string(
					self.current_index(), self.current_index() + 2
				).unwrap());
			}
			pieces.push((self.current() as char).to_string());
			for piece in pieces {
				match Token::try_from(piece.as_str()) {
					Err(_) => (),
					Ok(token) => {
						self.consume(piece.as_str());
						self.tokens.push(token);
						return Ok(());
					}
				}
			}
		}
		if self.current().is_ascii_digit() {
			let mut nbr = self.consume_int()?;
			if self.code_left() && self.current() == b'.' {
				self.advance();
				let frac = self.consume_int()?;
				println!("Frac: {}", frac);
				if self.code_left() && self.current() == b'.' {
					return Err(self.error("Unexpected dot in float number".to_string()));
				}
				nbr = format!("{}.{}", nbr, frac);
			}
			self.tokens.push(Token::Number(nbr));
		}
		else {
			return Err(self.error(format!(
				"Unexpected token at {:?}",
				self.current() as char
			)));
		}
		Ok(())
	}

	fn consume_int(&mut self) -> Result<String, String> {
		let start = self.current_index();
		while self.code_left() && self.current().is_ascii_digit() {
			self.advance();
		}
		Ok(self.pick_string(start, self.current_index()).ok_or(self.error(
			"Expected numeric digits".to_string()
		))?)
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
					if self.current() != b'\n' {
						self.advance();
					}
					self.consume_new_line();
				}
				self.consume_end_delimiter();
				if self.in_stmt {
					self.consume_token()?;
				}
			}
		}
		if self.in_stmt {
			return Err(self.error(format!(
				"Unexpected end of statement. Expected '{}'",
				self.end_delimiter
			)));
		}
		self.tokens.push(Token::Eof);
		Ok(self.tokens.clone())
	}
}
