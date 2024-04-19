use crate::{
	lexer::{Token, Lexer},
	parser::Parser,
};
use crate::interpreter::{
	Interpreter,
};
use crate::model::{
	Ctx
};

mod basic;
mod operations;
mod get_ctx;

// stmt
mod print;
mod r#if;
mod r#assign;
mod r#while;
mod r#for;

#[cfg(test)]
fn run_interpreter(
	code: &str,
	ctx: Ctx
) -> Result<String, String> {
	println!("Running interpreter: {}", code);
	println!("  - Code: {}", &code);
	let lexer = Lexer::new("{{", "}}");
	let tokens = lexer.scan(code).unwrap();
	println!("  - Tokens: {:?}", tokens);
	let tokens = tokens.iter().map(|t| t.clone()).collect::<Vec<Token>>();
	let mut parser = Parser::new(tokens);
	let code = parser.parse().unwrap();
	println!("  - Code: {:?}", code);
	let mut interpreter = Interpreter::new(ctx);
	interpreter.run(&code)
}

#[cfg(test)]
fn test_interpreter(code: &str, ctx: &str, expected: &str) { 
	let ctx = Ctx::from_str(ctx).unwrap();
	let result = match run_interpreter(code, ctx) {
		Ok(r) => r,
		Err(e) => {
			println!("  - Error: {}", e);
			panic!("Unexpected error in interpreter:\n{}", e);
		}
	};
	println!("  - Result  : {}", result);
	println!("  - Expected: {}", expected);
	assert_eq!(result, expected);
}

#[cfg(test)]
fn test_interpreter_basic(code: &str, expected: &str) {
	test_interpreter(code, "{}", expected);
}

#[cfg(test)]
fn expect_error(code: &str, ctx: &str) {
	let ctx = Ctx::from_str(ctx).unwrap();
	match run_interpreter(code, ctx) {
		Ok(_) => panic!("Expected error in interpreter"),
		Err(_) => ()
	}
}
