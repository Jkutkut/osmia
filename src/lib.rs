mod macros;
mod model;
mod utils;

#[cfg(test)]
mod tests;

use model::ctx::Ctx;

pub trait CodeInterpreter: for<'a> From<&'a str> {
	type Output;
	type Error;

	type LexerCode;
	type ParserCode;

	const VERSION: &'static str = env!("CARGO_PKG_VERSION");

	fn new_lexer() -> impl Lexer<Self::LexerCode, Self::Error>;
	fn new_parser() -> impl Parser<Self::LexerCode, Self::ParserCode, Self::Error>;
	fn new_interpreter() -> impl Interpreter<Self::ParserCode, Self::Output, Self::Error>;

	fn run(&self, code: &str) -> Result<Self::Output, Self::Error> {
		let lexer = Self::new_lexer();
		let lexed = lexer.lex(code)?;
		let parser = Self::new_parser();
		let parsed = parser.parse(lexed)?;
		let interpreter = Self::new_interpreter();
		interpreter.interpret(parsed)
	}
}

type OsmiaError = String;

/// Default osmia template engine API.
pub struct Osmia<'ctx> {
	#[allow(dead_code)]
	ctx: std::cell::RefCell<&'ctx mut Ctx>,
}

impl<'ctx> Osmia<'ctx> {
	pub fn new(ctx: &'ctx mut Ctx) -> Self {
		Self {
			ctx: std::cell::RefCell::new(ctx),
		}
	}
}

impl CodeInterpreter for Osmia<'_> {
	type Output = OsmiaOutput;
	type Error = String;

	type LexerCode = LexerCode;
	type ParserCode = ParserCode;

	fn new_lexer() -> impl Lexer<Self::LexerCode, Self::Error> {
		OsmiaLexer::osmia()
	}

	fn new_parser() -> impl Parser<Self::LexerCode, Self::ParserCode, Self::Error> {
		OsmiaParser::new()
	}

	fn new_interpreter() -> impl Interpreter<Self::ParserCode, Self::Output, Self::Error> {
		OsmiaInterpreter::new()
	}
}

impl From<&str> for Osmia<'_> {
	fn from(_: &str) -> Self {
		todo!() // TODO
		// Parse json str as ctx
	}
}

// Lexer
type LexerCode = Vec<String>;

pub trait Lexer<T, E> {
	fn lex(&self, code: &str) -> Result<T, E>;
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

impl Lexer<LexerCode, OsmiaError> for OsmiaLexer<'_> {
	#[allow(unused_variables)]
	fn lex(&self, code: &str) -> Result<LexerCode, OsmiaError> {
		todo!() // TODO
		// Lex code
	}
}

// Parser

/// Parsing documentation: // TODO
///
/// ## Structure:
/// ```text
/// program        → stmt
/// stmt           → block | raw | evaluation | print | comment | assign |
///                  if | while | foreach |
///                  break | continue | return |
///                  function
///
/// block          → ( stmt )*
/// raw            → "..."
/// evaluation     → "{{" expression "}}"
/// print          → "{{" "print" expression "}}"
/// comment        → "{{" "#" expression "}}"
/// assign         → "{{" identifier "=" expression "}}"
/// if             → "{{" "if" conditional ( "{{" "elseif" conditional )* ( "{{" "else" block )? "{{" "fi" "}}"
/// conditional    → expression "}}" stmt
/// while          → "{{" "while" conditional "done" "}}"
/// for            → "{{" "for" identifier "in" iterable "}}" stmt "{{" "done" "}}"
//  iterable       → expression
/// break          → "{{" "break" "}}"
/// continue       → "{{" "continue" "}}"
/// return         → "{{" "return" expression? "}}"
/// function       → "{{" "fn" identifier ( ";" parameters )? "}}" block "{{" "done" "}}"
/// parameters     → parameter ( "," parameter )* ( "," "..." identifier)?
/// parameter      → identifier ( "=" expression )?
///
///
//  lambda         → "(" parameters? ")" "=> {" block "}"
///
///
//  expression     → logic_or
/// logic_or       → logic_and ( "||" logic_and )*
/// logic_and      → equality ( "&&" equality )*
/// equality       → bitwise ( ( "!=" | "==" ) bitwise )*
/// bitwise        → comparison ( ( "&" | "|" | "^" ) comparison )*
/// comparison     → bitshift ( ( ">" | ">=" | "<" | "<=" ) bitshift )*
/// bitshift       → term ( ( ">>" | "<<" ) term )*
/// term           → factor ( ( "-" | "+" ) factor )*
/// factor         → unary ( ( "/" | "*" ) unary )*
/// unary          → ( "!" | "-" | "+" )* method_call
/// method_call    → primary ( "?" call )*
//  primary        → literal | variable | call | grouping | array | object
/// call           → variable ( "(" arguments? ")" )*
/// arguments      → expression ( "," expression )*
//  variable       → obj
/// obj            → array ( "." identifier )*
/// arr            → identifier ( "[" expression "]" )*
//  identifier     → ?
/// literal        → float | int | string | boolean | null
/// array          → "[" ( expression? ( "," expression )* )? "]"
/// object         → "{" ( expression ":" expression ( "," expression ":" expression )* )? "}"
/// grouping       → "(" expression ")"
/// ```
type ParserCode = String;

pub trait Parser<I, T, E> {
	fn parse(&self, code: I) -> Result<T, E>;
}

pub struct OsmiaParser;

impl OsmiaParser {
	pub fn new() -> Self {
		Self
	}
}

impl Parser<LexerCode, ParserCode, OsmiaError> for OsmiaParser {
	#[allow(unused_variables)]
	fn parse(&self, code: LexerCode) -> Result<ParserCode, OsmiaError> {
		todo!() // TODO
		// Parse code
	}
}

// Interpreter
type OsmiaOutput = String;

pub trait Interpreter<I, T, E> {
	fn interpret(&self, code: I) -> Result<T, E>;
}

struct OsmiaInterpreter;

impl OsmiaInterpreter {
	pub fn new() -> Self {
		Self
	}
}

impl Interpreter<ParserCode, OsmiaOutput, OsmiaError> for OsmiaInterpreter {
	#[allow(unused_variables)]
	fn interpret(&self, code: ParserCode) -> Result<OsmiaOutput, OsmiaError> {
		todo!() // TODO
		// Interpret code
	}
}
