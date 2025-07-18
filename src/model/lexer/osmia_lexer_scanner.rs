use crate::model::lexer::Token;
use crate::utils::code_trace;
use crate::LexerCode;
use crate::constants::{
	START_DELIMITER,
	END_DELIMITER
};

pub struct OsmiaLexerScanner<'a> {
	code: &'a [u8],
	index: usize,
	current_line: usize,
	tokens: LexerCode,
	in_stmt: bool,
	obj_depth: usize
}

impl<'a> OsmiaLexerScanner<'a> {
	pub fn new(code: &'a str) -> Self {
		Self {
			code: code.as_bytes(),
			index: 0,
			current_line: 0,
			tokens: Vec::new(),
			in_stmt: false,
			obj_depth: 0
		}
	}

	pub fn scan(mut self) -> Result<LexerCode, String> {
		while self.code_left() {
			while !self.is_match(START_DELIMITER) && self.code_left() {
				self.consume_raw();
			}
			if self.is_match(START_DELIMITER) {
				self.consume_start_delimiter();
				if self.consume("#") {
					self.consume_comment();
					self.consume_end_delimiter();
				}
				if self.in_stmt {
					self.consume_stmt_body()?;
				}
			}
		}
		if self.in_stmt {
			return Err(self.error(format!(
				"Unexpected end of statement. Expected '{}'",
				END_DELIMITER
			)));
		}
		self.tokens.push(Token::Eof);
		Ok(self.tokens)
	}
	
	pub fn scan_stmt(mut self) -> Result<LexerCode, String> {
		self.in_stmt = true;
		self.consume_stmt_body()?;
		Ok(self.tokens)
	}
}

impl<'a> OsmiaLexerScanner<'a> {
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

	fn pick_string(&self, start: usize, mut end: usize) -> Option<String> {
		if end > self.code.len() {
			end = self.code.len() - 1;
		}
		if start > end {
			panic!("Invalid range");
		}
		Some(String::from_utf8_lossy(
			&self.code[start..end]
		).to_string())
	}

	fn pick_non_empty_string(&self, start: usize, end: usize) -> Option<String> {
		match self.pick_string(start, end)? {
			s if s.is_empty() => None,
			s => Some(s)
		}
	}
}

impl<'a> OsmiaLexerScanner<'a> {
	fn consume_raw(&mut self) {
		let start = self.current_index();
		while self.code_left() && !self.is_match(START_DELIMITER) && self.current() != b'\n' {
			self.advance();
		}
		if let Some(content) = self.pick_non_empty_string(start, self.current_index()) {
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

	fn consume_whitespace(&mut self) {
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
	}

	fn consume_start_delimiter(&mut self) {
		if self.consume(START_DELIMITER) {
			self.in_stmt = true;
			self.tokens.push(Token::StmtStart);
		}
	}

	fn consume_end_delimiter(&mut self) {
		if self.obj_depth == 0 && self.consume(END_DELIMITER) {
			self.in_stmt = false;
			self.tokens.push(Token::StmtEnd);
		}
	}

	// fn consume_stmt(&mut self) -> Result<(), String> {

	// }

	fn consume_stmt_body(&mut self) -> Result<(), String> {
		loop {
			if !self.code_left() {
				break;
			}
			self.consume_whitespace();
			self.consume_end_delimiter();
			if !self.in_stmt {
				break;
			}
			self.consume_token()?;
		}
		Ok(())
	}

	fn consume_comment(&mut self) {
		fn add_piece(lexer: &mut OsmiaLexerScanner, start: usize, end: usize) {
			if let Some(content) = lexer.pick_non_empty_string(start, end) {
				lexer.tokens.push(Token::Raw(content));
			}
		}
		self.tokens.push(Token::Comment);
		let mut start = self.current_index();
		let mut depth: usize = 0;
		while self.code_left() {
			if self.consume("\n") {
				add_piece(self, start, self.current_index() - 1);
				self.tokens.push(Token::NewLine);
				start = self.current_index();
				continue;
			}
			if self.consume(START_DELIMITER) {
				depth += 1;
			}
			if self.is_match(END_DELIMITER) {
				if depth == 0 {
					break;
				}
				depth -= 1;
			}
			self.advance();
		}
		add_piece(self, start, self.current_index());
	}

	fn consume_token(&mut self) -> Result<(), String> {
		if !self.code_left() {
			return Ok(());
		}
		if self.consume("{") {
			self.tokens.push(Token::ObjectStart);
			self.obj_depth += 1;
			return Ok(());
		}
		if self.obj_depth > 0 && self.consume("}") {
			self.tokens.push(Token::ObjectEnd);
			self.obj_depth -= 1;
			return Ok(());
		}
		if self.consume_in_order(vec![
			("(", Token::ParentStart), (")", Token::ParentEnd),
			("[", Token::ArrayStart), ("]", Token::ArrayEnd),
			("+", Token::Plus), ("-", Token::Minus), ("*", Token::Mult), ("/", Token::Div), ("%", Token::Mod),
			("...", Token::Spread), (".", Token::Dot),
			("=>", Token::Arrow),
			("==", Token::Equal), ("=", Token::Assign),
			("<=", Token::LessEqual), ("<<", Token::BitShiftLeft), ("<", Token::Less),
			(">=", Token::GreaterEqual), (">>", Token::BitShiftRight), (">", Token::Greater),
			("!=", Token::NotEqual), ("!", Token::Not),
			("&&", Token::And), ("&", Token::BitAnd),
			("||", Token::Or), ("|", Token::BitOr),
			(",", Token::Comma), (":", Token::Colon),
			(";", Token::Semicolon), ("?", Token::Question),
			("^", Token::BitXor),
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
		Ok(self.pick_non_empty_string(start, self.current_index()).ok_or(self.error(
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
		let content = self.pick_string(start + 1, self.current_index() - 1);
		self.tokens.push(Token::Str(content.unwrap()));
		Ok(())
	}

	fn consume_identifier(&mut self) -> Result<(), String> {
		let start = self.current_index();
		while self.code_left() && (self.current().is_ascii_alphanumeric() || self.current() == b'_') {
			self.advance();
		}
		let content: String = self.pick_string(start, self.current_index()).ok_or(self.error(
			"Expected identifier".to_string()
		))?;
		self.tokens.push(match content.as_str() {
			"print" => Token::Print,
			"fn" => Token::Function,
			"return" => Token::Return,
			"if" => Token::If,
			"elseif" => Token::ElseIf,
			"else" => Token::Else,
			"fi" => Token::Fi,
			"while" => Token::While,
			"for" => Token::For,
			"in" => Token::In,
			"continue" => Token::Continue,
			"break" => Token::Break,
			"done" => Token::Done,
			"true" => Token::Bool(true),
			"false" => Token::Bool(false),
			"null" => Token::Null,
			_ => Token::Alpha(content)
		});
		Ok(())
	}
}
