mod token;
mod raw;
mod number;
mod simple_token;

use crate::macro_tests;
use crate::model::lexer::{
	Lexer, OsmiaLexer,
	Token
};
use crate::tests::test;

#[cfg(test)]
fn expr_wrap(
	code: &mut String,
	tokens: &mut Vec<Token>
) {
	code.insert_str(0, "{{");
	code.push_str("}}");
	tokens.insert(0, Token::StmtStart);
	tokens.push(Token::StmtEnd);
}

#[cfg(test)]
fn lexer_test(
	code: &str,
	mut tokens: Vec<Token>
) {
	tokens.push(Token::Eof);
	test(Some(code), Some(tokens), None, None);
}

#[cfg(test)]
fn lexer_expression_test(
	code: &str,
	mut tokens: Vec<Token>
) {
	let mut code = code.to_string();
	expr_wrap(&mut code, &mut tokens);
	lexer_test(&code, tokens);
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

#[cfg(test)]
fn lexer_expression_test_fail(
	code: &str,
	piece_error: &str
) {
	let mut code = code.to_string();
	expr_wrap(&mut code, &mut vec![]);
	lexer_test_fail(&code, piece_error);
}
