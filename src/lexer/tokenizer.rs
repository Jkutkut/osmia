/// A simple word tokenizer.
///
/// Takes a str and returns an iterator with the words.
///
/// Words are a sequence of non-whitespace characters and
/// all the content between quotes.
pub struct Tokenizer<'a> {
	text: &'a str,
	current: usize,
	in_quotes: Option<char>,
}

impl<'a> Tokenizer<'a> {
	pub fn new(text: &'a str) -> Self {
		Self {
			text,
			current: 0,
			in_quotes: None
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

	fn get_static_token(&mut self) -> Option<&'a str> {
		let c = self.text.chars().nth(self.current)?;
		let single_char_tokens = "+-*/=()";
		if single_char_tokens.contains(c) {
			let token = &self.text[self.current..self.current + 1];
			self.current += 1;
			return Some(token);
		}
		None
	}

	fn get_dynamic_token(&mut self) -> Option<&'a str> {
		let valid_chars = "_.[]";
		let mut chars = self.text.chars();
		let mut some_c = chars.nth(self.current);
		let mut i = 0;
		while let Some(c) = some_c {
			if self.is_token_value(c) || c.is_alphanumeric() {
				i += 1;
				some_c = chars.next()
			}
			else {
				break;
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
				#[cfg(debug_assertions)]
				{
					println!("Text: {}", self.text);
					println!("Current: {}", self.current);
					println!("Current text: {}", self.text[self.current..].chars().collect::<String>());
				}
				return Some(Err(format!(r#"Unexpected character: {}"#, self.text.chars().nth(self.current).unwrap())));
			}
		}
		None
	}
}
