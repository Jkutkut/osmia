mod parser;
mod interpreter;
mod model;
mod lexer;
mod syntax_tree_printer;
mod tests;

use crate::{
	lexer::{Token, Lexer},
	model::{
		Ctx,
		Stmt
	},
	Parser,
	interpreter::Interpreter,
	tree_walker::SyntaxTreePrinter,
};
use crate::syntax_tree::{
	Visitable,
};

#[allow(dead_code)]
#[cfg(test)]
fn test(
	code: Option<&str>,
	tokens: Option<Vec<Token>>,
	parsed: Option<Stmt>,
	execution: Option<(Ctx, &str)>
) {
	let mut lexed_code: Option<Vec<Token>> = None;
	let mut parsed_code: Option<Stmt> = None;
	// Lexing
	if let Some(code) = code {
		println!("- Lexing code...");
		lexed_code = match Lexer::new_osmia().scan(code) {
			Ok(lex) => Some(lex),
			Err(err) => panic!("The code can not be lexed: {}", err)
		};
	}
	match (&lexed_code, tokens) {
		(Some(lexed_code), Some(tokens)) => {
			println!("- Comparing lexed code...");
			println!("real    : {:?}", lexed_code);
			println!("expected: {:?}", tokens);
			for (real, expected) in lexed_code.iter().zip(tokens.iter()) {
				if real != expected {
					println!("{:?} vs {:?}: They are not the same.", real, expected);
				}
			}
			assert_eq!(lexed_code, &tokens);
		},
		(None, Some(tokens)) => lexed_code = Some(tokens.to_vec()),
		_ => ()
	};
	// Parsing
	if let Some(tokens) = lexed_code {
		println!("- Parsing lexed code...");
		parsed_code = match Parser::new(tokens).parse() {
			Ok(expr) => Some(expr),
			Err(err) => panic!("The code can not be parsed: {}", err)
		};
	}
	match (&parsed_code, parsed) {
		(Some(parsed_code), Some(parsed)) => {
			println!("- Comparing parsed code...");
			let printer = SyntaxTreePrinter;
			let parsed_code_str = parsed_code.accept(&printer);
			let expected_str = parsed.accept(&printer);
			println!("real    : {}", parsed_code_str);
			println!("expected: {}", expected_str);
			assert_eq!(parsed_code_str, expected_str);
			println!("Parsed:\n{:#?}", parsed_code);
			println!("Original - Expected:\n{:#?}\n", parsed);
			assert_eq!(parsed_code, &parsed);
		},
		(None, Some(parsed)) => parsed_code = Some(parsed),
		_ => ()
	};
	// Executing
	match (parsed_code, execution) {
		(Some(parsed_code), Some((ctx, expected_output))) => {
			println!("- Executing code...");
			let mut interpreter = Interpreter::new(ctx);
			match interpreter.run(&parsed_code) {
				Ok(output) => assert_eq!(output, expected_output),
				Err(err) => panic!("The code can not be executed:\n{}", err)
			}
		},
		(None, Some(_)) => panic!("There is no code to execute!"),
		_ => ()
	};
}
