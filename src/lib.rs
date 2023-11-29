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
	While,
	Loop,

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

	// Assign,
}

impl Token<'_> {
	fn from_str(s: &str) -> Option<Token> {
		match s {
			"if" => Some(Token::If),
			"elif" => Some(Token::ElseIf),
			"else" => Some(Token::Else),
			"for" => Some(Token::For),
			"while" => Some(Token::While),
			"loop" => Some(Token::Loop),
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
			_ => None
		}
	}
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
		for token in &tokens {
			println!("{:?}", token);
		}
		for token in tokens {
			match token {
				Token::Raw(s) => output.push_str(s),
				Token::DelimiterStart => (),
				Token::DelimiterEnd => (),
				Token::Value(s) => output.push_str(s),
				_ => return Err("Not implemented".to_string())
			}
		}
		Ok(output)
	}
}
