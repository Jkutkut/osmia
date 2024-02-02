use crate::lexer::Token;
use crate::lexer::Lexer;

use std::collections::LinkedList;

#[cfg(test)]
fn lexer() -> Lexer<'static> {
	Lexer::new("{{", "}}")
}

#[cfg(test)]
fn check_result(
	real: Result<LinkedList<Token<'_>>, String>,
	expected: Vec<Token<'_>>
) {
	if let Err(err) = real {
		panic!("Error: {}", err);
	}
	let real = real.unwrap();
	let real = real.into_iter().collect::<Vec<Token<'_>>>();
	println!("real    : {:?}", real);
	println!("expected: {:?}", expected);
	assert_eq!(real, expected);
}

#[cfg(test)]
fn lexer_test(
	input: &str,
	expected: Vec<Token<'_>>
) {
	let lexer = lexer();
	let result = lexer.scan(input);
	check_result(result, expected);
}

#[test]
fn basics_lexer01() {
	let tests: Vec<(&str, Vec<Token<'_>>)> = vec![
		(
			"", vec![Token::Eof]
		),
		(
			"{{1}}",
			vec![
				Token::DelimiterStart,
				Token::Value("1"),
				Token::DelimiterEnd,
				Token::Eof
			]
		),
		(
			"{{1.0}}",
			vec![
				Token::DelimiterStart,
				Token::Value("1.0"),
				Token::DelimiterEnd,
				Token::Eof
			]
		),
		(
			"{{true}}",
			vec![
				Token::DelimiterStart,
				Token::Value("true"),
				Token::DelimiterEnd,
				Token::Eof
			]
		),
		(
			"{{false}}",
			vec![
				Token::DelimiterStart,
				Token::Value("false"),
				Token::DelimiterEnd,
				Token::Eof
			]
		),
		(
			"{{null}}",
			vec![
				Token::DelimiterStart,
				Token::Value("null"),
				Token::DelimiterEnd,
				Token::Eof
			]
		),
		(
			"{{\"hello world\"}}",
			vec![
				Token::DelimiterStart,
				Token::Value(r#""hello world""#),
				Token::DelimiterEnd,
				Token::Eof
			]
		)
	];
	for (test, expected) in tests {
		lexer_test(test, expected);
	}
}

#[test]
fn basics_lexer02() {
	let tests: Vec<(&str, Vec<Token<'_>>)> = vec![
		(
			"{{1}} test",
			vec![
				Token::DelimiterStart,
				Token::Value("1"),
				Token::DelimiterEnd,
				Token::Raw(" test"),
				Token::Eof
			]
		),
		(
			"test {{123}}",
			vec![
				Token::Raw("test "),
				Token::DelimiterStart,
				Token::Value("123"),
				Token::DelimiterEnd,
				Token::Eof
			]
		),
		(
			"test {{true}} test",
			vec![
				Token::Raw("test "),
				Token::DelimiterStart,
				Token::Value("true"),
				Token::DelimiterEnd,
				Token::Raw(" test"),
				Token::Eof
			]
		),
		(
			"{{123}} test {{456}}",
			vec![
				Token::DelimiterStart,
				Token::Value("123"),
				Token::DelimiterEnd,
				Token::Raw(" test "),
				Token::DelimiterStart,
				Token::Value("456"),
				Token::DelimiterEnd,
				Token::Eof
			]
		),
		(
			"test {{1}}tcc{{true}}vvvvv{{false}}aaaa",
			vec![
				Token::Raw("test "),
				Token::DelimiterStart,
				Token::Value("1"),
				Token::DelimiterEnd,
				Token::Raw("tcc"),
				Token::DelimiterStart,
				Token::Value("true"),
				Token::DelimiterEnd,
				Token::Raw("vvvvv"),
				Token::DelimiterStart,
				Token::Value("false"),
				Token::DelimiterEnd,
				Token::Raw("aaaa"),
				Token::Eof
			]
		)
	];
	for (test, expected) in tests {
		lexer_test(test, expected);
	}
}
			

#[test]
fn invalid_tests() {
	let tests: Vec<&str> = vec![
		"{{",
		"this is invalid {{",
		"{{ this is invalid",
	];
	let lexer = lexer();
	for test in tests {
		let result = lexer.scan(test);
		if let Ok(_) = result {
			panic!("Expected error, but got success.");
		}
	}
}

#[test]
fn advance_tests01() {
	let tests: Vec<(&str, Vec<Token<'_>>)> = vec![
		(
			"{{ 1 + 1 }}",
			vec![
				Token::DelimiterStart,
				Token::Value("1"),
				Token::Plus,
				Token::Value("1"),
				Token::DelimiterEnd,
				Token::Eof
			]
		),
		(
			"{{ 1 + 1 - 1 * 1 / 1 }}",
			vec![
				Token::DelimiterStart,
				Token::Value("1"),
				Token::Plus,
				Token::Value("1"),
				Token::Minus,
				Token::Value("1"),
				Token::Multiply,
				Token::Value("1"),
				Token::Divide,
				Token::Value("1"),
				Token::DelimiterEnd,
				Token::Eof
			]
		),
	];
	for (test, expected) in tests {
		lexer_test(test, expected);
	}
}

// TODO test multiple expressions
// TODO test whitespaces
// TODO test all operators
// TODO test all types
// TODO test all keywords
// TODO test all delimiters
// TODO test all separators
// TODO test multiple expressions with same output
