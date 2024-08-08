use crate::lexer::Token;
use crate::model::{
	Expression, Binary, Literal, JsonExpression,
	Stmt
};
use super::{test_parser, should_fail};

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
fn test_parser_print(tokens: Vec<Token>, expected: JsonExpression) {
	let tokens = token_expression_to_token_stmt(tokens);
	let expected = Stmt::Print(expected);
	test_parser(tokens, expected);
}

#[test]
fn basic_parser() {
	let tokens = vec![ // 1 + 2 * 3 == 7
		Token::Value("1".to_string()),
		Token::Plus,
		Token::Value("2".to_string()),
		Token::Multiply,
		Token::Value("3".to_string()),
		Token::Equal,
		Token::Value("7".to_string())
	];
	let expected = JsonExpression::Expression(Expression::Binary(Binary::new(
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
	).unwrap()));
	test_parser_print(tokens, expected);
}

#[test]
fn should_fail01() {
	should_fail(vec![
		Token::DelimiterStart,
		Token::Print,
		Token::Value("1".to_string()),
	]);
}

#[test]
fn should_fail02() {
	should_fail(vec![
		Token::DelimiterStart,
		Token::Print,
		Token::DelimiterEnd
	]);
}

#[test]
fn should_fail03() {
	should_fail(vec![
		Token::Print,
	]);
}
