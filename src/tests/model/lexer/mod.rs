mod token;
mod raw;

use crate::macro_tests;
use crate::model::lexer::{
	Lexer, OsmiaLexer,
	Token
};
use crate::tests::test;

#[cfg(test)]
fn lexer_test(
	code: &str,
	tokens: Vec<Token>
) {
	let mut new_tokens = Vec::new();
	new_tokens.extend(tokens);
	new_tokens.push(Token::Eof);
	test(Some(code), Some(new_tokens), None, None);
}

#[cfg(test)]
fn lexer_test_fail(
	code: &str,
	piece_error: &str
) {
	let lexer = OsmiaLexer::osmia();
	match lexer.lex(code) {
		Ok(t) => panic!("Should fail but gave: {:?}", t),
		Err(err) => {
			println!("Error: {}", err);
			println!("Expected piece: {}", piece_error);
			assert!(err.to_lowercase().contains(piece_error));
		}
	}
}
