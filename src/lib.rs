mod macros;
mod model;
mod utils;
mod types;

#[cfg(test)]
mod tests;

use types::{
	LexerCode,
	ParserCode,
	OsmiaOutput,

	OsmiaError,
};
use model::ctx;
use model::lexer::{
	Lexer, OsmiaLexer,
};
use model::parser::{
	Parser, OsmiaParser,
};
use model::interpreter::{
	Interpreter, OsmiaInterpreter,
};

pub trait CodeInterpreter: for<'a> From<&'a str> {
	type Output;
	type Error;

	type LexerCode;
	type ParserCode;
	type Ctx;

	const VERSION: &'static str = env!("CARGO_PKG_VERSION");

	fn new_lexer() -> impl Lexer<Self::LexerCode, Self::Error>;
	fn new_parser() -> impl Parser<Self::LexerCode, Self::ParserCode, Self::Error>;
	fn new_interpreter(ctx: &mut Self::Ctx) -> impl Interpreter<Self::ParserCode, Self::Output, Self::Error>;

	fn lex(code: &str) -> Result<Self::LexerCode, Self::Error> {
		Self::new_lexer().lex(code)
	}

	fn parse(code: Self::LexerCode) -> Result<Self::ParserCode, Self::Error> {
		Self::new_parser().parse(code)
	}

	fn interpret(ctx: &mut Self::Ctx, code: Self::ParserCode) -> Result<Self::Output, Self::Error> {
		Self::new_interpreter(ctx).interpret(code)
	}

	fn run(&mut self, code: &str) -> Result<Self::Output, Self::Error>;
}

/// Default osmia template engine API.
pub struct Osmia {
	ctx: types::Ctx,
}

impl Osmia {
	fn new(ctx: types::Ctx) -> Self {
		Self {
			ctx,
		}
	}

	pub fn run_code(&mut self, code: &str) -> Result<OsmiaOutput, OsmiaError> {
		self.run(code)
	}
}

impl Default for Osmia {
	fn default() -> Self {
		Self::new(types::Ctx::new())
	}
}

impl CodeInterpreter for Osmia {
	type Output = OsmiaOutput;
	type Error = String;

	type LexerCode = LexerCode;
	type ParserCode = ParserCode;
	type Ctx = types::Ctx;

	fn new_lexer() -> impl Lexer<Self::LexerCode, Self::Error> {
		OsmiaLexer::osmia()
	}

	fn new_parser() -> impl Parser<Self::LexerCode, Self::ParserCode, Self::Error> {
		OsmiaParser::new()
	}

	fn new_interpreter(ctx: &mut Self::Ctx) -> impl Interpreter<Self::ParserCode, Self::Output, Self::Error> {
		OsmiaInterpreter::new(ctx)
	}

	fn run(&mut self, code: &str) -> Result<Self::Output, Self::Error> {
		let lexed = Self::lex(code)?;
		let parsed = Self::parse(lexed)?;
		Self::interpret(&mut self.ctx, parsed)
	}
}

impl From<&str> for Osmia {
	fn from(_: &str) -> Self {
		todo!() // TODO
		// Parse json str as ctx
	}
}
