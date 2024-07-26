mod macros;
mod model;
mod parser;
mod lexer;
mod syntax_tree;
mod tree_walker;
mod interpreter;
mod utils;

#[cfg(test)]
mod tests;

use interpreter::Interpreter;
use model::Ctx;
use parser::Parser;
use lexer::Lexer;
use model::Stmt;

/// This simple API is designed to use the osmia template engine.
///
/// # Syntax
/// ## Text:
/// By default, all text outside the delimiters will be treated as a plain text.
///
/// ```text
/// Hey there! This is a plain text
/// ```
/// ```text
/// Hey there! This is a plain text
/// ```
///
/// ## Expression:
/// Evaluates and prints the given expression.
/// ```text
/// 1 + 2 = {{ 1 + 2 }}
/// ```
/// ```text
/// 1 + 2 = 3
/// ```
///
/// ## Assign:
/// Assigns the given expression to the given variable in the context.
/// ```text
/// {{ assign foo = 1 + 2 }}
/// The result is {{foo}}
/// ```
/// ```text
/// The result is 3
/// ```
///
/// ## Print:
/// Prints the given expression in the stdin. It can be useful when debugging.
/// ```text
/// {{ print "Hello world!" }}
/// ```
/// ```text
/// ```
///
/// ## If:
/// ### Single if:
/// ```text
/// {{ if 1 == 1 }}
/// Math is not broken!
/// {{ fi }}
/// ```
/// ```text
/// Math is not broken!
/// ```
///
/// ### If, else:
/// ```text
/// {{ if 1 == 2 }}
/// Math is not broken!
/// {{ else }}
/// Nope, math is right >D
/// {{ fi }}
/// ```
/// ```text
/// Nope, math is right >D
/// ```
///
/// ### If, else if:
/// ```text
/// {{assign value = 2}}
/// {{ if value == 1 }}
/// Nope
/// {{ elseif value == 2 }}
/// The value is 2!
/// {{ else }}
/// Nope
/// {{ fi }}
/// ```
/// ```text
/// The value is 2!
/// ```
///
/// ## While:
/// ```text
/// {{ assign i = 0 }}
/// {{ while i < 5 }}
/// {{ assign i = i + 1 }}
/// i = {{ i }}
/// {{done}}
/// ```
/// ```text
/// i = 1
/// i = 2
/// i = 3
/// i = 4
/// i = 5
/// ```
///
/// ## For:
/// ```text
/// {{ for i in [1, 2, 3] }}
///   i = {{ i }}
/// {{ done }}
/// {{assign arr = [
///   {"name": "Marvin1"},
///   {"name": "Marvin2"},
///   {"name": "Marvin3"}
/// ]}}
/// {{for user in arr}}
/// User: {{ user.name }}
/// {{done}}
/// ```
/// ```text
/// i = 1
/// i = 2
/// i = 3
/// User: Marvin1
/// User: Marvin2
/// User: Marvin3
/// ```
///
/// ## Continue, break:
/// ```text
/// {{continue}}
/// {{break}}
/// ```
///
/// ## Advanced control:
/// All variable in the context can be used and edited similarly to
/// variables in JavaScript.
/// ```text
/// {{ assign obj = {"user": {"name": "Marvin"} } }}
/// Old name: {{ obj.user.name }}
/// {{ assign obj.user.name = "R2D2" }}
/// New name: {{ obj.user.name }}
/// ```
/// ```text
/// Old name: Marvin
/// New name: R2D2
/// ```
///
/// # Examples
///
/// ## Basic execution
/// ```rust
/// use osmia::Osmia;
///
/// let mut interpreter = Osmia::new();
/// let code = Osmia::code("Hello world! {{ 1 + 2 }}").unwrap();
/// let result = interpreter.run(&code).unwrap();
/// assert_eq!(result, "Hello world! 3");
/// ```
///
/// ## Context
/// The context can be passed as a JSON object in the form of a string.
/// ```rust
/// use osmia::Osmia;
///
/// let context_json = r#"{"foo": "bar"}"#;
/// let code = "The value of foo is {{ foo }}";
/// let mut interpreter = Osmia::from_json(context_json).unwrap();
/// let code = Osmia::code(code).unwrap();
/// let result = interpreter.run(&code).unwrap();
/// assert_eq!(result, "The value of foo is bar");
/// ```
///
/// ## Experimental features: Custom delimiters
/// You can change the delimiters used by the code lexer.
/// ```rust
/// use osmia::Osmia;
///
/// let mut interpreter = Osmia::from_json(r#"{"user": {"name": "Marvin"}}"#).unwrap();
/// let code = Osmia::custom_code("User: ${ user.name }", "${", "}").unwrap();
/// let result = interpreter.run(&code).unwrap();
/// assert_eq!(result, "User: Marvin");
/// ```
pub struct Osmia;


impl Osmia {
	pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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
		Parser::new(tokens).parse()
	}
}
