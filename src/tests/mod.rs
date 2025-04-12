mod model;
mod osmia;
mod utils;
mod gh_issues;

use crate::macro_tests;
use crate::Osmia;
use crate::CodeInterpreter;
use crate::types::{
	Ctx,
};

#[allow(dead_code)]
#[cfg(test)]
fn test_code(
	code: &str,
	tokens: Option<<Osmia as CodeInterpreter>::LexerCode>,
	parsed: Option<<Osmia as CodeInterpreter>::ParserCode>,
	execution: Vec<(Ctx, Result<&str, Vec<&str>>)>
) {
	test(Some(code), tokens, parsed, Some(execution));
}

#[allow(dead_code)]
#[cfg(test)]
fn test(
	code: Option<&str>,
	tokens: Option<<Osmia as CodeInterpreter>::LexerCode>,
	parsed: Option<<Osmia as CodeInterpreter>::ParserCode>,
	execution: Option<Vec<(Ctx, Result<&str, Vec<&str>>)>>
) {
	// Lexing
	let mut lexed_code = None;
	if let Some(code) = code {
		println!("- Lexing code...");
		println!("code: {:?}", code);
		lexed_code = match Osmia::lex(code) {
			Ok(lex) => Some(lex),
			Err(err) => panic!("The code can not be lexed: {}", err)
		};
	}
	match (&lexed_code, tokens) {
		(Some(lexed_code), Some(tokens)) => {
			println!("- Comparing lexed code...");
			println!("real    : {:?}", lexed_code);
			println!("expected: {:?}", tokens);
			for (i, (real, expected)) in lexed_code.iter().zip(tokens.iter()).enumerate() {
				if real != expected {
					println!("{:?} vs {:?}: They are not the same. (idx: {})", real, expected, i);
				}
			}
			assert_eq!(lexed_code, &tokens);
		},
		(None, Some(tokens)) => lexed_code = Some(tokens.to_vec()),
		_ => ()
	};
	// Parsing
	if let (None, None) = (&parsed, &execution) {
		return;
	}
	let mut parsed_code = None;
	if let Some(tokens) = lexed_code {
		println!("- Parsing lexed code...");
		parsed_code = match Osmia::parse(tokens) {
			Ok(expr) => Some(expr),
			Err(err) => panic!("The code can not be parsed: {}", err)
		};
	}
	match (&parsed_code, parsed) {
		(Some(parsed_code), Some(parsed)) => {
			println!("- Comparing parsed code...");
			println!("Parsed:\n{:#?}", parsed_code);
			println!("Original - Expected:\n{:#?}\n", parsed);
			assert_eq!(parsed_code, &parsed);
		},
		(None, Some(parsed)) => parsed_code = Some(parsed),
		_ => ()
	};
	// Executing
	match (parsed_code, execution) {
		(Some(parsed_code), Some(execution_tests)) => {
			println!("- Executing code...");
			for (mut ctx, output) in execution_tests {
				let result = Osmia::interpret(&mut ctx, parsed_code.clone());
				match output {
					Ok(expected_output) => match result {
						Ok(output) => assert_eq!(output, expected_output),
						Err(err) => panic!("The code can not be executed:\n{}", err)
					},
					Err(error_pieces) => match result {
						Err(err) => {
							println!("The code can not be executed:\n{}", err);
							let err = err.to_lowercase();
							for piece in error_pieces {
								println!("  - {}", piece);
								assert!(err.contains(piece.to_lowercase().as_str()));
							}
						},
						Ok(r) => panic!("The code can not be executed but the output is:\n{}", r)
					}
				}
			}
		},
		(None, Some(_)) => panic!("There is no code to execute!"),
		_ => ()
	};
}
