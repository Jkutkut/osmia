mod macros;
mod model;
mod utils;
mod types;

#[cfg(test)]
mod tests;

use types::{
	LexerCode,
};
use model::ctx;
use model::lexer::{
	Lexer, OsmiaLexer,
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

type OsmiaError = String;

/// Default osmia template engine API.
pub struct Osmia {
	ctx: ctx::Ctx,
}

impl Osmia {
	fn new(ctx: ctx::Ctx) -> Self {
		Self {
			ctx,
		}
	}
}

impl Default for Osmia {
	fn default() -> Self {
		Self::new(ctx::Ctx::Object(std::collections::HashMap::new())) // TODO
	}
}

impl CodeInterpreter for Osmia {
	type Output = OsmiaOutput;
	type Error = String;

	type LexerCode = LexerCode;
	type ParserCode = ParserCode;
	type Ctx = ctx::Ctx;

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
/// if             → "{{" "if" conditional ( elseif )* ( else )? "{{" "fi" "}}"
/// elseif         → "{{" "elseif" conditional
/// else           → "{{" "else" block
/// conditional    → expression "}}" stmt
/// while          → "{{" "while" conditional "done" "}}"
/// for            → "{{" "for" identifier "in" iterable "}}" stmt "{{" "done" "}}"
/// iterable       → expression
/// break          → "{{" "break" "}}"
/// continue       → "{{" "continue" "}}"
/// return         → "{{" "return" expression? "}}"
/// function       → "{{" "fn" identifier ( ";" parameters )? "}}" block "{{" "done" "}}"
/// parameters     → parameter ( "," parameter )* ( "," "..." identifier)?
/// parameter      → identifier ( "=" expression )?
///
/// expression     → lambda | logic_or
/// lambda         → "fn" "(" parameters? ")" "=>" expression
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
/// primary        → literal | call | array | object | grouping
/// literal        → float | int | string | boolean | null
/// call           → variable ( "(" arguments? ")" )*
/// arguments      → expression ( "," expression )*
/// variable       → obj
/// obj            → array ( "." identifier )*
/// arr            → identifier ( "[" expression "]" )*
/// array          → "[" ( expression? ( "," expression )* )? "]"
/// object         → "{" ( object_entry ( "," object_entry )* )? "}"
/// object_entry   → expression ":" expression
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
use std::cell::RefCell;

type OsmiaOutput = String;

pub trait Interpreter<I, T, E> {
	fn interpret(&self, code: I) -> Result<T, E>;
}

struct OsmiaInterpreter<'ctx> {
	#[allow(dead_code)]
	ctx: RefCell<&'ctx mut ctx::Ctx>,
}


impl<'ctx> OsmiaInterpreter<'ctx> {
	pub fn new(ctx: &'ctx mut ctx::Ctx) -> Self {
		Self {
			ctx: std::cell::RefCell::new(ctx),
		}
	}
}

impl Interpreter<ParserCode, OsmiaOutput, OsmiaError> for OsmiaInterpreter<'_> {
	#[allow(unused_variables)]
	fn interpret(&self, code: ParserCode) -> Result<OsmiaOutput, OsmiaError> {
		todo!() // TODO
		// Interpret code
	}
}
