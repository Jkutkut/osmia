use std::collections::LinkedList;

mod tokenizer;

use tokenizer::Tokenizer;

#[cfg(test)]
mod tests;

// type Json = serde_json::Value;
// type JsonRef<'a> = &'a Json;

#[derive(Debug)]
enum Token<'a> {
	DelimiterStart,
	DelimiterEnd,

	Raw(&'a str),
	Value(&'a str),

	If,
	ElseIf,
	Else,

	For,

	Equal,
	NotEqual,
	LessThan,
	LessEqual,
	GreaterThan,
	GreaterEqual,

	Plus,
	Minus,
	Multiply,
	Divide,
	Modulo,

	And,
	Or,
	Not,

	GroupingStart,
	GroupingEnd,

	// Assign,
}

impl Token<'_> {
	fn from_str(s: &str) -> Option<Token> {
		match s {
			"if" => Some(Token::If),
			"elif" => Some(Token::ElseIf),
			"else" => Some(Token::Else),
			"for" => Some(Token::For),
			"==" => Some(Token::Equal),
			"!=" => Some(Token::NotEqual),
			"<" => Some(Token::LessThan),
			"<=" => Some(Token::LessEqual),
			">" => Some(Token::GreaterThan),
			">=" => Some(Token::GreaterEqual),
			"+" => Some(Token::Plus),
			"-" => Some(Token::Minus),
			"*" => Some(Token::Multiply),
			"/" => Some(Token::Divide),
			"%" => Some(Token::Modulo),
			"&&" => Some(Token::And),
			"||" => Some(Token::Or),
			"!" => Some(Token::Not),
			"(" => Some(Token::GroupingStart),
			")" => Some(Token::GroupingEnd),
			_ => None
		}
	}

// 	fn is_conditional(&self) -> bool {
// 		match self {
// 			Token::If => true,
// 			Token::ElseIf => true,
// 			Token::Else => true,
// 			Token::For => true,
// 			_ => false
// 		}
// 	}
}

struct Scanner<'a> {
	delimiter_start: &'a str,
	delimiter_end: &'a str
}

impl<'a> Scanner<'a> {
	fn new(delimiter_start: &'a str, delimiter_end: &'a str) -> Self {
		Scanner {
			delimiter_start: delimiter_start,
			delimiter_end: delimiter_end
		}
	}

	fn scan(&self, input: &'a str) -> Result<LinkedList<Token>, String> {
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

pub struct Lexer<'a> {
	scanner: Scanner<'a>
}

impl<'a> Lexer<'a> {
	pub fn new(delimiter_start: &'a str, delimiter_end: &'a str) -> Self {
		Self {
			scanner: Scanner::new(delimiter_start, delimiter_end)
		}
	}

	pub fn default() -> Self {
		Self::new("{{", "}}")
	}

	pub fn render(&self, code: &str/*, _ctx: JsonRef*/) -> Result<String, String> {
		// TODO handle ctx
		let scan_result = self.scanner.scan(code)?;
		let tokens = scan_result.iter().collect::<Vec<&Token>>();

		let mut output = String::new();
		#[cfg(test)]
		{
			println!("** Rendering **");
			for token in &tokens {
				println!("{:?}", token);
			}
			println!("** **\n");
		}
		let mut i = 0;
		while i < tokens.len() {
			match tokens[i] {
				Token::Raw(s) => output.push_str(s),
				Token::DelimiterStart => {
					match self.render_block(&tokens[i..]) {
						Err(e) => return Err(e),
						Ok((rendered_block, tokens_consumed)) => {
							output.push_str(&rendered_block);
							i += tokens_consumed; // TODO -1?
						}
					}
				},
				_ => return Err(format!("Unexpected token {:?}", tokens[i]))
			}
			i += 1;
		}
		Ok(output)
	}


	fn find_end_of_block(&self, tokens: &[&Token]) -> Result<usize, String> {
		let mut i = 1;
		while i < tokens.len() {
			match tokens[i] {
				Token::DelimiterEnd => {
					if i == 1 {
						return Err("Empty block".to_string());
					}
					return Ok(i);
				},
				_ => i += 1
			}
		}
		return Err("Unclosed block".to_string());
	}

	fn bound_block(&self, tokens: &[&Token]) -> Result<usize, String> {
		let end_idx = match self.find_end_of_block(tokens) {
			Err(e) => return Err(e),
			Ok(idx) => idx
		};
		if end_idx == 1 {
			return Err("Empty block".to_string());
		}
		Ok(end_idx)
	}

	fn render_block(&self, tokens: &[&Token]) -> Result<(String, usize), String> {
		let end_idx = self.bound_block(tokens)?;
		let size: usize = end_idx - 1;
		#[cfg(test)]
		{
			println!("** Rendering block **");
			println!("Size: {}, end_idx: {}\n", size, end_idx);
			for (i, token) in tokens.iter().enumerate() {
				println!("{}: {:?}", i, token);
			}
			println!("** **\n");
		}
		// Conditional block can only be the first token
		// The rest must be non-conditional tokens
		// let i = if tokens[0].is_conditional() {1} else {0};
		// TODO
		if size == 1 {
			match tokens[1] {
				Token::Value(s) => return Ok((s.to_string(), end_idx)),
				t => return Err(format!("Unexpected token {:?}", t))
			}
		}
		Err("Not implemented".to_string())
	}
}
