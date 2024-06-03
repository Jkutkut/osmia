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

	pub fn scan(&self, input: &'a str) -> Result<Vec<Token>, String> {
		let mut tokens: Vec<Token> = Vec::new();
		let mut i = 0;
		while i < input.len() {
			let delimiter_start_idx = input[i..].find(self.delimiter_start).unwrap_or(input.len() - i);
			if delimiter_start_idx > 0 {
				let chunk = &input[i..i + delimiter_start_idx];
				i += delimiter_start_idx;
				if !self.can_be_omitted(chunk) {
					let (chunk, trimmed) = self.trim_last_empty_line(chunk);
					tokens.push(Token::Raw(chunk));
					if let Some(trimmed) = trimmed {
						tokens.push(Token::Raw(trimmed));
					}
				}
				if i >= input.len() {
					break;
				}
			}
			tokens.push(Token::DelimiterStart);
			i += self.delimiter_start.len();
			let mut stack = std::collections::VecDeque::new();
			let mut delimiter_end_idx = 0;
			while i + delimiter_end_idx < input.len() - self.delimiter_end.len() {
				if input[i + delimiter_end_idx..].starts_with(self.delimiter_end) &&
					stack.is_empty() {
					break;
				}
				let current_char = input.chars().nth(i + delimiter_end_idx).unwrap();
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
				delimiter_end_idx += 1;
			}
			if !stack.is_empty() {
				return Err(format!("{} was not closed", stack.back().unwrap()));
			}
			if i + delimiter_end_idx >= input.len() ||
				!input[i + delimiter_end_idx..].starts_with(self.delimiter_end) {
				return Err(format!("Osmia delimiter was not closed"));
			}
			for token in Tokenizer::new(&input[i..i + delimiter_end_idx]) {
				let token = token?;
				match Token::from_str(token) {
					Some(t) => tokens.push(t),
					None => tokens.push(Token::Value(token.to_string()))
				}
			}
			tokens.push(Token::DelimiterEnd);
			i += delimiter_end_idx + self.delimiter_end.len();
		}
		tokens.push(Token::Eof);
		self.clean_tokens(&mut tokens);
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

	fn trim_last_empty_line(&self, chunk: &str) -> (String, Option<String>) {
		let last_new_line = match chunk.rfind('\n') {
			None => return (chunk.to_string(), None),
			Some(idx) => idx
		};
		let trimmed_chunk = chunk[0..last_new_line].to_string();
		let trimmed_piece = chunk[last_new_line..].to_string();
		for c in trimmed_piece.chars() {
			if !c.is_whitespace() && c != '\n' {
				return (chunk.to_string(), None);
			}
		}
		(trimmed_chunk, Some(trimmed_piece))
	}

	fn clean_tokens(&self, tokens: &mut Vec<Token>) {
		let mut i = 0;
		println!("clean_tokens: {:?}", tokens);
		while i < tokens.len() - 1 {
			let token = &tokens[i];
			i += 1;
			if let Token::Raw(chunk) = token {
				if !chunk.starts_with('\n') || !self.is_all_whitespace(&chunk[1..]) {
					continue;
				}
				let mut shoud_be_omitted = true;
				let mut j = 0;
				while i + j < tokens.len() {
					let next_token = &tokens[i + j];
					j += 1;
					match next_token {
						Token::Raw(chunk) => {
							shoud_be_omitted = chunk.starts_with('\n');
							break;
						},
						Token::DelimiterStart => {
							if let Token::Value(_) = &tokens[i] {
								shoud_be_omitted = false;
								break;
							}
						},
						_ => ()
					}
				}
				if shoud_be_omitted {
					if i + j == tokens.len() {
						tokens[i - 1] = Token::Raw(chunk.trim_end().to_string());
					}
					else {
						tokens.remove(i - 1);
						i -= 1;
					}
				}
			}
		}
	}

	fn is_all_whitespace(&self, chunk: &str) -> bool {
		chunk.len() > 0 && chunk.chars().all(|c| c.is_whitespace())
	}
}
