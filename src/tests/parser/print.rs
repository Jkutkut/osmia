use crate::Token;
use crate::syntax_tree::model::{
	Expression, Binary, Literal,
	Stmt
};
use super::{test_parser};

#[cfg(test)]
fn token_expression_to_token_stmt(tokens: Vec<Token>) -> Vec<Token> {
	let mut new_tokens = vec![
		Token::DelimiterStart,
		Token::Print
	];
	new_tokens.extend(tokens);
	new_tokens.push(Token::DelimiterEnd);
	new_tokens
}

#[cfg(test)]
fn test_parser_print(tokens: Vec<Token>, expected: Expression) {
	let tokens = token_expression_to_token_stmt(tokens);
	let expected = Stmt::Print(expected);
	test_parser(tokens, expected);
}

#[test]
fn basic_parser() {
	let tokens = vec![ // 1 + 2 * 3 == 7
		Token::Value("1"),
		Token::Plus,
		Token::Value("2"),
		Token::Multiply,
		Token::Value("3"),
		Token::Equal,
		Token::Value("7")
	];
	let expected = Expression::Binary(Binary::new(
		Expression::Binary(Binary::new(
			Expression::Literal(Literal::Int(1)),
			Token::Plus,
			Expression::Binary(Binary::new(
				Expression::Literal(Literal::Int(2)),
				Token::Multiply,
				Expression::Literal(Literal::Int(3))
			).unwrap())
		).unwrap()),
		Token::Equal,
		Expression::Literal(Literal::Int(7))
	).unwrap());
	test_parser_print(tokens, expected);
}
