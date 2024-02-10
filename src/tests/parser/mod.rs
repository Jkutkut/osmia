mod expression;
mod print;
mod raw;
mod assign;
mod loop_blocks;
mod r#if;
mod r#while;
mod foreach;

use crate::lexer::Token;
use crate::model::Stmt;
use crate::parser::Parser;
use crate::tree_walker::SyntaxTreePrinter;
use crate::syntax_tree::Visitable;

#[cfg(test)]
fn add_eof(
	tokens: Vec<Token>
) -> Vec<Token> {
	let mut new_tokens = Vec::new();
	new_tokens.extend(tokens);
	new_tokens.push(Token::Eof);
	new_tokens
}

#[cfg(test)]
fn test_parser(
	tokens: Vec<Token>,
	expected: Stmt
) {
	let tokens = add_eof(tokens);
	let parsed_result = match Parser::new(tokens).parse() {
		Ok(expr) => expr,
		Err(err) => panic!("Parser threw an error:\n{}", err),
	};
	let printer = SyntaxTreePrinter;
	let parsed_result_str = parsed_result.accept(&printer);
	let expected_str = expected.accept(&printer);
	println!("\nParsed:              {}", parsed_result_str);
	println!("Original - Expected: {}\n", expected_str);
	assert_eq!(parsed_result_str, expected_str);
	println!("Parsed:              {:#?}", parsed_result);
	println!("Original - Expected: {:#?}\n", expected);
	assert_eq!(parsed_result, expected);
}

#[cfg(test)]
fn should_fail(
	code: Vec<Token>,
) {
	let code = add_eof(code);
	let parsed_result = Parser::new(code).parse();
	let printer = SyntaxTreePrinter;
	if let Ok(ref parsed_result) = parsed_result {
		println!("Parsed: {}", parsed_result.accept(&printer));
	}
	assert!(parsed_result.is_err());
}
