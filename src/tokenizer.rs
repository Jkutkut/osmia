pub struct Tokenizer<'a> {
	text: &'a str,
	start: usize,
	current: usize,
	in_quotes: Option<char>,
}

impl<'a> Tokenizer<'a> {
	pub fn new(text: &'a str) -> Self {
		Self {
			text,
			start: 0,
			current: 0,
			in_quotes: None
		}
	}
}

impl<'a> std::iter::Iterator for Tokenizer<'a> {
	type Item = Result<&'a str, String>;

	fn next(&mut self) -> Option<Self::Item> {
		while self.current < self.text.len() {
			let c = self.text.chars().nth(self.current).unwrap();
			if c == '"' || c == '\'' {
				match self.in_quotes {
					None => self.in_quotes = Some(c),
					Some(q) => {
						if q == c {
							self.in_quotes = None;
						}
					}
				}
			}
			if c.is_whitespace() && self.in_quotes.is_none() {
				let token = &self.text[self.start..self.current].trim();
				self.start = self.current;
				if token.len() > 0 {
					return Some(Ok(token));
				}
			}
			self.current += 1;
		}
		if self.current >= self.text.len() && self.current != self.start {
			if self.in_quotes.is_some() {
				return Some(Err("Unclosed quotes!".to_string()));
			}
			let token = &self.text[self.start..self.current].trim();
			self.start = self.current;
			if token.len() > 0 {
				return Some(Ok(token));
			}
		}
		None
	}
}
