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
use crate::utils::code_trace;

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

	pub fn scan(&mut self) -> Result<LexerCode, String> {
		while self.code_left() {
			if !self.in_stmt {
				self.consume_raw();
				self.consume_start_delimiter();
			}
			else {
				let mut white_space = false;
				while self.code_left() && self.current().is_ascii_whitespace() {
					if self.current() != b'\n' {
						self.advance();
						if !white_space {
							self.tokens.push(Token::Whitespace);
							white_space = true;
						}
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

	fn consume_in_order(&mut self, options: Vec<(&str, Token)>) -> bool {
		for (lex, token) in options {
			if self.consume(lex) {
				self.tokens.push(token);
				return true;
			}
		}
		false
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
		if self.consume_in_order(vec![
			("(", Token::ParentStart), (")", Token::ParentEnd), ("{", Token::ObjectStart),
			("}", Token::ObjectEnd), ("[", Token::ArrayStart), ("]", Token::ArrayEnd),
			("+", Token::Plus), ("-", Token::Minus), ("*", Token::Mult), ("/", Token::Div), ("%", Token::Mod),
			("...", Token::Spread), (".", Token::Dot),
			("==", Token::Equal), ("=", Token::AssignEq),
			("<=", Token::LessEqual), ("<<", Token::BitShiftLeft), ("<", Token::Less),
			(">=", Token::GreaterEqual), (">>", Token::BitShiftRight), (">", Token::Greater),
			("!=", Token::NotEqual), ("!", Token::Not),
			("&&", Token::And), ("&", Token::BitAnd),
			("||", Token::Or), ("|", Token::BitOr),
			("#", Token::Comment), (",", Token::Comma), (":", Token::Colon),
			(";", Token::SemiColon), ("?", Token::Question),
			("^", Token::BitXor),
			("print", Token::Print), ("assign", Token::Assign),
			("fn", Token::Function), ("return", Token::Return),
			("if", Token::If), ("elseif", Token::ElseIf), ("else", Token::Else), ("fi", Token::Fi),
			("while", Token::While), ("for", Token::For), ("in", Token::In),
			("continue", Token::Continue), ("break", Token::Break), ("done", Token::Done),
			("true", Token::Bool(true)), ("false", Token::Bool(false)), ("null", Token::Null),
		]) {
			return Ok(());
		}
		match self.current() as char {
			'0'..='9' => self.consume_number()?,
			'"' | '\'' => self.consume_string()?,
			'_' | 'a'..='z' | 'A'..='Z' => self.consume_identifier()?,
			_ => return Err(self.error(format!(
				"Unexpected token at {:?}",
				self.current() as char
			)))
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

	fn consume_number(&mut self) -> Result<(), String> {
		let mut nbr = self.consume_int()?;
		if self.code_left() && self.current() == b'.' {
			self.advance();
			let frac = self.consume_int()?;
			if self.code_left() && self.current() == b'.' {
				return Err(self.error("Unexpected dot in float number".to_string()));
			}
			nbr = format!("{}.{}", nbr, frac);
		}
		self.tokens.push(Token::Number(nbr));
		Ok(())
	}

	fn consume_string(&mut self) -> Result<(), String> {
		let start = self.current_index();
		let delim = self.current();
		loop {
			self.advance();
			self.consume_new_line();
			if !self.code_left() || self.current() == delim {
				break;
			}
		}
		if self.current() != delim {
			return Err(self.error(format!(
				"Unexpected end of string. Expected {:?}",
				delim
			)));
		}
		self.advance();
		let content = self.pick_string(start, self.current_index());
		self.tokens.push(Token::Str(content.unwrap()));
		Ok(())
	}

	fn consume_identifier(&mut self) -> Result<(), String> {
		let start = self.current_index();
		while self.code_left() && (self.current().is_ascii_alphanumeric() || self.current() == b'_') {
			self.advance();
		}
		let content = self.pick_string(start, self.current_index());
		self.tokens.push(Token::Alpha(content.ok_or(self.error(
			"Expected identifier".to_string()
		))?));
		Ok(())
	}
}
