use std::collections::VecDeque;

use crate::lexer::Token;
use crate::lexer::Tokenizer;

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

	pub fn new_osmia() -> Self {
		Lexer::new("{{", "}}")
	}

	pub fn scan(&self, input: &'a str) -> Result<Vec<Token>, String> {
		let mut tokens: Vec<Token> = Vec::new();
		let input_arr: &[u8] = input.as_bytes();
		let mut i = 0;
		let mut start = 0;
		while i < input.len() {
			let current_char = input_arr[i] as char;
			if current_char == '\n' {
				Self::add_raw_token(&mut tokens, Self::cut_input(&input, start, i));
				Self::add_raw_token(&mut tokens, Some("\n".to_string()));
				start = i + 1;
				while i < input.len() {
					let c = input_arr[i] as char;
					if !c.is_whitespace() {
						break;
					}
					i += 1;
				}
				Self::add_raw_token(&mut tokens, Self::cut_input(&input, start, i));
				start = i;
				continue;
			}
			else if
				self.delimiter_start.starts_with(current_char) &&
				input[i..].starts_with(self.delimiter_start)
			{
				Self::add_raw_token(&mut tokens, Self::cut_input(&input, start, i));
				Self::add_token(&mut tokens, Some(Token::DelimiterStart));
				i += self.delimiter_start.len();
				start = i;
				let mut stack: VecDeque<char> = VecDeque::new();
				while i < input.len() - self.delimiter_end.len() {
					let next_char = input_arr[i] as char;
					if
						self.delimiter_end.starts_with(next_char) &&
						input[i..].starts_with(self.delimiter_end) &&
						stack.is_empty()
					{
						break;
					}
					let current_char = input_arr[i] as char;
					for (s, e) in [('{', '}'), ('[', ']'), ('(', ')')] {
						if current_char == s {
							stack.push_back(e);
							break;
						}
						if current_char == e {
							if let Some(stack_element) = stack.pop_back() {
								if stack_element != e {
									return Err(format!("Invalid close delimiter: {}", current_char));
								}
							}
							else {
								return Err(format!("Invalid close delimiter: {}", current_char));
							}
							break;
						}
					}
					i += 1;
				}
				if !stack.is_empty() {
					return Err(format!("{} was not closed", stack.back().unwrap()));
				}
				if i >= input.len() || !input[i..].starts_with(self.delimiter_end) {
					return Err(format!("'{}' was not closed with '{}'", self.delimiter_start, self.delimiter_end));
				}
				for token in Tokenizer::new(&input[start..i]) {
					let token = token?;
					match Token::from_str(token) {
						Some(t) => tokens.push(t),
						None => tokens.push(Token::Value(token.to_string()))
					}
				}
				tokens.push(Token::DelimiterEnd);
				start = i + self.delimiter_end.len();
				i = start;
				continue;
			}
			i += 1;
		}
		if start < input.len() {
			Self::add_raw_token(&mut tokens, Self::cut_input(&input, start, input.len()));
		}
		Self::add_token(&mut tokens, Some(Token::Eof));
		Self::clean_tokens(&mut tokens);
		Ok(tokens)
	}

	fn cut_input(input: &str, from: usize, to: usize) -> Option<String> {
		if from >= to {
			return None;
		}
		Some(input[from..to].to_string())
	}

	fn add_token(tokens: &mut Vec<Token>, token: Option<Token>) {
		if let Some(token) = token {
			tokens.push(token);
		}
	}

	fn add_raw_token(tokens: &mut Vec<Token>, token: Option<String>) {
		if let Some(token) = token {
			tokens.push(Token::Raw(token));
		}
	}

	fn clean_tokens(tokens: &mut Vec<Token>) {
		let mut i: usize = tokens.len() - 1;
		let mut end: usize;
		while i > 0 {
			end = i;
			while i > 0 {
				i -= 1;
				if Self::is_new_line_token(&tokens[i]) {
					break;
				}
			}
			if Self::is_new_line_token(&tokens[i]) {
				i += 1;
			}
			let line = &tokens[i..end + 1];
			let mut row_has_printable_token = false;
			let mut k = 0;
			while k < line.len() - 1 && !row_has_printable_token {
				match &line[k] {
					Token::Raw(_) => row_has_printable_token = !Self::is_whitespace_token(&line[k]),
					Token::DelimiterStart => {
						match &line[k + 1] {
							Token::Value(_) => row_has_printable_token = true,
							_ => ()
						}
					},
					_ => ()
				}
				k += 1;
			}
			if !row_has_printable_token {
				let mut ts: Vec<Token> = Vec::new();
				let mut line = line.to_vec();
				if Self::is_new_line_token(&tokens[end]) {
					ts.push(tokens.remove(end));
					line.remove(line.len() - 1);
				}
				if Self::is_whitespace_token(&tokens[i]) {
					ts.push(tokens.remove(i));
					line.remove(0);
				}
			}
			if i > 0 {
				i -= 1;
			}
		}
	}

	fn is_new_line_token(token: &Token) -> bool {
		match token {
			Token::Raw(chunk) => chunk == "\n",
			_ => false
		}
	}

	fn is_whitespace_token(token: &Token) -> bool {
		match token {
			Token::Raw(chunk) => chunk.chars().all(|c| c.is_whitespace()),
			_ => false
		}
	}
}
