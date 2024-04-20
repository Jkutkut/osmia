mod macros;
mod model;
mod parser;
mod lexer;
mod syntax_tree;
mod tree_walker;
mod interpreter;

#[cfg(test)]
mod tests;

use interpreter::Interpreter;
use model::Ctx;
use parser::Parser;
use lexer::Lexer;
use lexer::Token;
use model::Stmt;

pub struct Osmia {
}


impl Osmia {
	pub fn new() -> Interpreter {
		Self::from_json("{}").unwrap()
	}

	pub fn from_json(json: &str) -> Result<Interpreter, String> {
		let ctx = Self::new_ctx(json)?;
		let interpreter = Interpreter::new(ctx);
		Ok(interpreter)
	}

	fn new_ctx(ctx: &str) -> Result<Ctx, String> {
		Ctx::from_str(ctx)
	}

	pub fn code(code: &str) -> Result<Stmt, String> {
		Self::custom_code(code, "{{", "}}")
	}

	pub fn custom_code(code: &str, start_delimiter: &str, end_delimiter: &str) -> Result<Stmt, String> {
		let lexer = Lexer::new(start_delimiter, end_delimiter);
		let tokens = lexer.scan(code)?;
		let tokens = tokens.iter().map(|t| t.clone()).collect::<Vec<Token>>();
		Parser::new(tokens).parse()
	}
}
