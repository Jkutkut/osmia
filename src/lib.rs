mod macros;
mod constants;
mod model;
mod utils;
mod types;
mod stdlib;

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

pub trait CodeInterpreter: for<'a> TryFrom<&'a str> {
	type Output;
	type InterpreterError;

	type LexerCode;
	type ParserCode;
	type Ctx;

	const VERSION: &'static str = env!("CARGO_PKG_VERSION");

	fn new_lexer() -> impl Lexer<Self::LexerCode, Self::InterpreterError>;
	fn new_parser() -> impl Parser<Self::LexerCode, Self::ParserCode, Self::InterpreterError>;
	fn new_interpreter(ctx: &mut Self::Ctx) -> impl Interpreter<Self::ParserCode, Self::Output, Self::InterpreterError>;

	fn lex(code: &str) -> Result<Self::LexerCode, Self::InterpreterError> {
		Self::new_lexer().lex(code)
	}

	fn parse(code: Self::LexerCode) -> Result<Self::ParserCode, Self::InterpreterError> {
		Self::new_parser().parse(code)
	}

	fn interpret(ctx: &mut Self::Ctx, code: Self::ParserCode) -> Result<Self::Output, Self::InterpreterError> {
		Self::new_interpreter(ctx).interpret(code)
	}

	fn run(&mut self, code: &str) -> Result<Self::Output, Self::InterpreterError>;
}

/// Default osmia template engine API.
///
/// # Examples
///	```rust
///	use osmia::Osmia;
///
///	let mut osmia = Osmia::default();
///	let output = osmia.run_code("1 + 1 = {{ 1 + 1 }}").unwrap();
///	assert_eq!(output, "1 + 1 = 2".to_string());
///	```
///
///	## Json context:
///	```rust
///	use osmia::Osmia;
///
///	let mut osmia = Osmia::try_from(r#"{ "name": "Marvin" }"#).unwrap();
///	let output = osmia.run_code("Hello {{ name }}!").unwrap();
///	assert_eq!(output, "Hello Marvin!".to_string());
///	```
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
	type InterpreterError = OsmiaError;

	type LexerCode = LexerCode;
	type ParserCode = ParserCode;
	type Ctx = types::Ctx;

	fn new_lexer() -> impl Lexer<Self::LexerCode, Self::InterpreterError> {
		OsmiaLexer::new()
	}

	fn new_parser() -> impl Parser<Self::LexerCode, Self::ParserCode, Self::InterpreterError> {
		OsmiaParser::new()
	}

	fn new_interpreter(ctx: &mut Self::Ctx) -> impl Interpreter<Self::ParserCode, Self::Output, Self::InterpreterError> {
		OsmiaInterpreter::new(ctx)
	}

	fn run(&mut self, code: &str) -> Result<Self::Output, Self::InterpreterError> {
		let lexed = Self::lex(code)?;
		let parsed = Self::parse(lexed)?;
		Self::interpret(&mut self.ctx, parsed)
	}
}

impl TryFrom<&str> for Osmia {
	type Error = OsmiaError;

	fn try_from(ctx: &str) -> Result<Self, Self::Error> {
		Ok(Self::new(types::Ctx::try_from(ctx)?))
	}
}
