mod lexer;
mod syntax_tree;
pub mod parser;

use lexer::{Lexer, Token};

#[cfg(test)]
mod tests;

// type Json = serde_json::Value;
// type JsonRef<'a> = &'a Json;

pub struct Osmia<'a> {
	lexer: Lexer<'a>
}

impl<'a> Osmia<'a> {
	pub fn new(delimiter_start: &'a str, delimiter_end: &'a str) -> Self {
		Self {
			lexer: Lexer::new(delimiter_start, delimiter_end)
		}
	}

	pub fn default() -> Self {
		Self::new("{{", "}}")
	}

	pub fn render(&self, code: &str/*, _ctx: JsonRef*/) -> Result<String, String> {
		// TODO handle ctx
		let scan_result = self.lexer.scan(code)?;
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
