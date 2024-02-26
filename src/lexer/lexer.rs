use crate::lexer::Token;
use crate::lexer::Tokenizer;
use std::collections::LinkedList;

pub struct Lexer<'a> {
	delimiter_start: &'a str,
	delimiter_end: &'a str
}

impl<'a> Lexer<'a> {
	pub fn new(delimiter_start: &'a str, delimiter_end: &'a str) -> Self {
		Lexer {
			delimiter_start: delimiter_start,
			delimiter_end: delimiter_end
		}
	}

	pub fn scan(&self, input: &'a str) -> Result<LinkedList<Token>, String> {
		let mut tokens = LinkedList::new();
		let mut i = 0;
		while i < input.len() {
			let delimiter_start_idx = input[i..].find(self.delimiter_start).unwrap_or(input.len() - i);
			if delimiter_start_idx > 0 {
				let chunk = &input[i..i + delimiter_start_idx];
				if !self.can_be_omitted(chunk) {
					tokens.push_back(Token::Raw(chunk));
				}
				i += delimiter_start_idx;
				if i >= input.len() {
					break;
				}
			}
			tokens.push_back(Token::DelimiterStart);
			i += self.delimiter_start.len();
			let delimiter_end_idx = match input[i..].find(self.delimiter_end) {
				Some(idx) => {
					let mut idx = idx;
					while i + idx + self.delimiter_end.len() < input.len() && 
						input[i + idx + 1..].starts_with(self.delimiter_end)
					{ // TODO refactor to work with multiple delimiters
						idx += 1;
					}
					idx
				},
				None => return Err("Unclosed delimiter".to_string())
			};
			for token in Tokenizer::new(&input[i..i + delimiter_end_idx]) {
				let token = token?;
				match Token::from_str(token) {
					Some(t) => tokens.push_back(t),
					None => tokens.push_back(Token::Value(token))
				}
			}
			tokens.push_back(Token::DelimiterEnd);
			i += delimiter_end_idx + self.delimiter_end.len();
		}
		tokens.push_back(Token::Eof);
		Ok(tokens)
	}

	fn can_be_omitted(&self, chunk: &str) -> bool {
		let mut omit = false;
		let mut chunk_chars = chunk.chars();
		loop {
			match chunk_chars.next() {
				Some(c) => match c {
					'\n' => {
						omit = true;
						break;
					},
					c => if !c.is_whitespace() {
						break;
					}
				},
				None => break
			};
		}
		if omit {
			for c in chunk.chars() {
				if !c.is_whitespace() && c != '\n' {
					return false;
				}
			}
		}
		omit
	}
}
