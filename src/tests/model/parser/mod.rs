mod literal;
mod grouping;
mod lambda;
mod number;
mod white_box_tests;

use crate::{
	Osmia, CodeInterpreter,
	macro_tests,
	model::lexer::Token,
};

#[cfg(test)]
fn parser_test_fail_tokens(
	code: Vec<Token>,
	error_pieces: &[&str],
) {
	match Osmia::parse(code) {
		Ok(expr) => panic!("The code should not be parsed: {:?}", expr),
		Err(err) => {
			println!("Error: {}", err);
			let err = err.to_lowercase();
			for error_piece in error_pieces {
				println!("Looking for: {}", error_piece);
				assert!(err.contains(error_piece));
			}
		}
	};
}

#[cfg(test)]
fn parser_test_fail(
	code: &str,
	error_pieces: &[&str],
) {
	let lexed = Osmia::lex(code).unwrap_or_else(|err| {
		panic!("The code can not be lexed: {}", err)
	});
	parser_test_fail_tokens(lexed, error_pieces);
}

