use std::collections::LinkedList;

#[cfg(test)]
mod tests;

// type Json = serde_json::Value;
// type JsonRef<'a> = &'a Json;

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

	fn scan(&self, input: &'a str) -> LinkedList<Token> {
		let mut tokens = LinkedList::new();
		// TODO
		tokens.push_back(Token::Raw(input));
		tokens
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
		let scan_result = self.scanner.scan(code);
		let tokens = scan_result.iter().collect::<Vec<&Token>>();

		let mut output = String::new();
		for token in tokens {
			match token {
				Token::Raw(s) => output.push_str(s),
				_ => return Err("Not implemented".to_string())
			}
		}
		Ok(output)
	}
}
