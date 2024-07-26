use crate::utils::code_trace;

/// A simple word tokenizer.
///
/// Takes a str and returns an iterator with the words.
///
/// Words are a sequence of non-whitespace characters and
/// all the content between quotes.
pub struct Tokenizer<'a> {
	text: &'a str,
	current: usize,
}

impl<'a> Tokenizer<'a> {
	pub fn new(text: &'a str) -> Self {
		Self {
			text,
			current: 0,
		}
	}

	fn is_token_value(&self, c: char) -> bool {
		let valid_chars = "_.[]";
		c.is_alphanumeric() || valid_chars.contains(c)
	}

	fn is_quote(&self, c: char) -> bool {
		c == '"' || c == '\''
	}

	fn is_whitespace(&self) -> bool {
		let c = self.text.chars().nth(self.current).unwrap();
		c.is_whitespace()
	}

	/// Attempts to get the next quoted token.
	///
	/// Returns None if no token was found.
	/// Returns Err if an unclosed quote was found.
	/// Returns Ok(token) if a quote was found.
	fn quotes(&mut self) -> Option<Result<&'a str, String>> {
		let mut chars = self.text.chars();
		let quote = chars.nth(self.current)?;
		if !self.is_quote(quote) {
			return None;
		}
		let mut i = 1;
		while let Some(c) = chars.next() {
			if c == quote {
				let token = &self.text[self.current..self.current + i + 1];
				self.current += i + 1;
				return Some(Ok(token));
			}
			i += 1;
		}
		None
	}

	/// Attempts to get the next static token.
	///
	/// Returns None if no token was found.
	/// Returns Some(token) if a token was found.
	///
	/// Note: a static token is a reserved word or an operator.
	fn get_static_token(&mut self) -> Option<&'a str> {
		let mut chars = self.text.chars();
		let c = chars.nth(self.current)?;
		if let Some(_) = chars.next() {
			match &self.text[self.current..self.current + 2] {
				"==" | "!=" | "<=" | ">=" | "&&" | "||" | "<<" | ">>" => {
					self.current += 2;
					return Some(&self.text[self.current - 2..self.current]);
				}
				_ => ()
			}
		}
		let single_char_tokens = "+-*/=()<>!%{}[],:&|^";
		if single_char_tokens.contains(c) {
			let token = &self.text[self.current..self.current + 1];
			self.current += 1;
			return Some(token);
		}
		None
	}

	/// Attempts to get the next dynamic token.
	///
	/// Returns None if no token was found.
	/// Returns Some(token) if a token was found.
	///
	/// Note: a dynamic token is a variable.
	/// Note: the validity of the token is not checked and must be handled by the executor.
	/// TODO Use stack to allow dynamic tokens to be nested
	fn get_dynamic_token(&mut self) -> Option<&'a str> {
		let mut chars = self.text.chars();
		let mut some_c = chars.nth(self.current);
		let mut in_array: bool = false;
		let mut i = 0;
		while let Some(c) = some_c {
			if !self.is_token_value(c) && !c.is_alphanumeric() {
				break;
			}
			i += 1;
			some_c = chars.next();
			if c == '[' {
				if in_array {
					i -= 1;
					break;
				}
				in_array = true;
			}
			else if c == ']' {
				if !in_array {
					i -= 1;
					break;
				}
				in_array = false;
			}
		}
		if i > 0 {
			let token = &self.text[self.current..self.current + i];
			self.current += i;
			return Some(token);
		}
		None
	}
}

/// Iterator trait implementation.
///
/// Returns the next word in the text or an error as a String.
impl<'a> std::iter::Iterator for Tokenizer<'a> {
	type Item = Result<&'a str, String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current > 0 && self.current < self.text.len() {
			let mut chars = self.text.chars();
			let previous_char = chars.nth(self.current - 1)?;
			let current = chars.next()?;
			if !(current.is_whitespace() || previous_char.is_whitespace()) &&
				("\"'".contains(previous_char) || "\"'".contains(current)) &&
				(!":,[{".contains(previous_char) && !":,]})".contains(current))
			{
				return Some(Err(code_trace(
					&self.text, self.current,
					&format!(
						"Missing whitespace between {:?} and {:?}:",
						previous_char, current
					)
				)));
			}
		}
		while self.current < self.text.len() {
			if let Some(quotes) = self.quotes() {
				match quotes {
					Err(_) => return Some(Err("Unclosed quotes!".to_string())),
					q => return Some(q),
				}
			}
			else if let Some(token) = self.get_static_token() {
				return Some(Ok(token));
			}
			else if let Some(token) = self.get_dynamic_token() {
				return Some(Ok(token));
			}
			else if self.is_whitespace() {
				self.current += 1;
			}
			else {
				let c = self.text.chars().nth(self.current).unwrap();
				return Some(Err(format!(
					r#"Unexpected character: '{}', ascii: {:#x}"#,
					c, c as u32
				)));
			}
		}
		None
	}
}
