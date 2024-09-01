mod literal;
mod grouping;

use crate::{
	Osmia, CodeInterpreter,
	macro_tests,
};

#[cfg(test)]
fn parser_test_fail(
	code: &str,
	error_pieces: &[&str],
) {
	let lexed = Osmia::lex(code).unwrap_or_else(|err| {
		panic!("The code can not be lexed: {}", err)
	});
	match Osmia::parse(lexed) {
		Ok(expr) => panic!("The code should not be parsed: {:?}", expr),
		Err(err) => {
			println!("Error: {}", err);
			for error_piece in error_pieces {
				println!("Looking for: {}", error_piece);
				assert!(err.contains(error_piece));
			}
		}
	};
}
