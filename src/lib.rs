mod macros;
mod model;
mod utils;

#[cfg(test)]
mod tests;

pub trait CodeInterpreter: for<'a> From<&'a str> {
	type Output;
	// TODO error

	const VERSION: &'static str = env!("CARGO_PKG_VERSION");

	fn new_lexer() -> impl Lexer;
	fn new_parser() -> impl Parser;
	fn new_interpreter() -> impl Interpreter<Self::Output>;

	fn run(&self, code: &str) -> Self::Output {
		let lexer = Self::new_lexer();
		let parser = Self::new_parser();
		let interpreter = Self::new_interpreter();
		let lexed = lexer.lex(code);
		let parsed = parser.parse(lexed);
		interpreter.interpret(parsed)
	}
}

/// Default osmia template engine API.
pub struct Osmia;

impl Osmia {
	pub fn new() -> Self {
		Self
	}

	pub fn run_code(&self, code: &str) -> String {
		self.run(code)
	}
}

impl CodeInterpreter for Osmia {
	type Output = OsmiaOutput;

	fn new_lexer() -> impl Lexer {
		OsmiaLexer::osmia()
	}

	fn new_parser() -> impl Parser {
		OsmiaParser::new()
	}

	fn new_interpreter() -> impl Interpreter<Self::Output> {
		OsmiaInterpreter::new()
	}
}

impl From<&str> for Osmia {
	fn from(_: &str) -> Self {
		todo!() // TODO
		// Parse json str as ctx
	}
}

// Lexer
type LexerCode = Vec<String>;

pub trait Lexer {
	fn lex(&self, code: &str) -> LexerCode;
}

struct OsmiaLexer<'a> {
	#[allow(dead_code)]
	start_delimiter: &'a str,
	#[allow(dead_code)]
	end_delimiter: &'a str
}

impl<'a> OsmiaLexer<'a> {
	pub fn new(start_delimiter: &'a str, end_delimiter: &'a str) -> Self {
		Self { start_delimiter, end_delimiter }
	}

	pub fn osmia() -> Self {
		Self::new("{{", "}}")
	}
}

impl Lexer for OsmiaLexer<'_> {
	#[allow(unused_variables)]
	fn lex(&self, code: &str) -> LexerCode {
		todo!() // TODO
		// Lex code
	}
}

// Parser
type ParserCode = String;

pub trait Parser {
	fn parse(&self, code: LexerCode) -> ParserCode;
}

pub struct OsmiaParser;

impl OsmiaParser {
	pub fn new() -> Self {
		Self
	}
}

impl Parser for OsmiaParser {
	#[allow(unused_variables)]
	fn parse(&self, code: LexerCode) -> ParserCode {
		todo!() // TODO
		// Parse code
	}
}

// Interpreter
type OsmiaOutput = String;

pub trait Interpreter<T> {
	fn interpret(&self, code: ParserCode) -> T;
}

struct OsmiaInterpreter;

impl OsmiaInterpreter {
	pub fn new() -> Self {
		Self
	}
}

impl Interpreter<OsmiaOutput> for OsmiaInterpreter {
	#[allow(unused_variables)]
	fn interpret(&self, code: ParserCode) -> OsmiaOutput {
		todo!() // TODO
		// Interpret code
	}
}
