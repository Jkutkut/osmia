use crate::token::Token;
use crate::tokenizer::Tokenizer;
use std::collections::LinkedList;

pub struct Scanner<'a> {
	delimiter_start: &'a str,
	delimiter_end: &'a str
}

impl<'a> Scanner<'a> {
	pub fn new(delimiter_start: &'a str, delimiter_end: &'a str) -> Self {
		Scanner {
			delimiter_start: delimiter_start,
			delimiter_end: delimiter_end
		}
	}

	pub fn scan(&self, input: &'a str) -> Result<LinkedList<Token>, String> {
		#[cfg(test)]
		{
			println!("** Scanning input **\n{}\n", input);
		}
		let mut tokens = LinkedList::new();
		let mut last = 0;
		let mut i = 0;
		while i < input.len() {
			let delimiter_start_idx = input[i..].find(self.delimiter_start).unwrap_or(input.len() - i);
			if delimiter_start_idx > 0 {
				tokens.push_back(Token::Raw(&input[last..i + delimiter_start_idx]));
				i += delimiter_start_idx;
				if i >= input.len() {
					break;
				}
			}
			tokens.push_back(Token::DelimiterStart);
			i += self.delimiter_start.len();
			let delimiter_end_idx = match input[i..].find(self.delimiter_end) {
				Some(idx) => idx,
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
			last = i;
		}
		#[cfg(test)]
		{
			for token in &tokens {
				println!("{:?}", token);
			}
			println!("** Scanning done **\n");
		}
		Ok(tokens)
	}
}
