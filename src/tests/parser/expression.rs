use crate::Token;
use crate::syntax_tree::model::{
	Expression, Binary, Unary, Grouping, Literal, Variable,
	Stmt
};
use super::{test_parser, should_fail};

#[cfg(test)]
fn token_expression_to_token_stmt(tokens: Vec<Token>) -> Vec<Token> {
	let mut new_tokens = vec![
		Token::DelimiterStart,
	];
	new_tokens.extend(tokens);
	new_tokens.push(Token::DelimiterEnd);
	new_tokens
}

#[cfg(test)]
fn test_parser_expression(tokens: Vec<Token>, expected: Expression) {
	let tokens = token_expression_to_token_stmt(tokens);
	let expected = Stmt::Expression(expected);
	test_parser(tokens, expected);
}

#[cfg(test)]
fn should_fail_expression(tokens: Vec<Token>) {
	let tokens = token_expression_to_token_stmt(tokens);
	should_fail(tokens);
}

// Valid tests

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
	test_parser_expression(tokens, expected);
}

#[test]
fn precedence() {
	// 1 + 2 * 3 * (4 + 5) == 1 + (2 * 3) * 9 == true
	let tokens = vec![
		Token::Value("1"),
		Token::Plus,
		Token::Value("2"),
		Token::Multiply,
		Token::Value("3"),
		Token::Multiply,
		Token::GroupingStart,
		Token::Value("4"),
		Token::Plus,
		Token::Value("5"),
		Token::GroupingEnd,
		Token::Equal,
		Token::Value("1"),
		Token::Plus,
		Token::GroupingStart,
		Token::Value("2"),
		Token::Multiply,
		Token::Value("3"),
		Token::GroupingEnd,
		Token::Multiply,
		Token::Value("9"),
		Token::Equal,
		Token::Value("true")
	];
	let expected = Expression::Binary(Binary::new(
		Expression::Binary(Binary::new(
			Expression::Binary(Binary::new(
				Expression::Literal(Literal::Int(1)),
				Token::Plus,
				Expression::Binary(Binary::new(
					Expression::Binary(Binary::new(
						Expression::Literal(Literal::Int(2)),
						Token::Multiply,
						Expression::Literal(Literal::Int(3))
					).unwrap()),
					Token::Multiply,
					Expression::Grouping(Grouping::new(
						Expression::Binary(Binary::new(
							Expression::Literal(Literal::Int(4)),
							Token::Plus,
							Expression::Literal(Literal::Int(5))
						).unwrap())
					))
				).unwrap())
			).unwrap()),
			Token::Equal,
			Expression::Binary(Binary::new(
				Expression::Literal(Literal::Int(1)),
				Token::Plus,
				Expression::Binary(Binary::new(
					Expression::Grouping(Grouping::new(
						Expression::Binary(Binary::new(
							Expression::Literal(Literal::Int(2)),
							Token::Multiply,
							Expression::Literal(Literal::Int(3))
						).unwrap())
					)),
					Token::Multiply,
					Expression::Literal(Literal::Int(9))
				).unwrap())
			).unwrap())
		).unwrap()),
		Token::Equal,
		Expression::Literal(Literal::Bool(true))
	).unwrap());
	test_parser_expression(tokens, expected);
}

#[test]
fn unary_operators01() {
	// !!true == !false == (-(-1) == 1) != false
	let tokens = vec![
		Token::Not,
		Token::Not,
		Token::Value("true"),
		Token::Equal,
		Token::Not,
		Token::Value("false"),
		Token::Equal,
		Token::GroupingStart,
		Token::Minus,
		Token::GroupingStart,
		Token::Minus,
		Token::Value("1"),
		Token::GroupingEnd,
		Token::Equal,
		Token::Value("1"),
		Token::GroupingEnd,
		Token::NotEqual,
		Token::Value("false")
	];
	let expected = Expression::Binary(Binary::new(
		Expression::Binary(Binary::new(
			Expression::Binary(Binary::new(
				Expression::Unary(Unary::new(
					Token::Not,
					Expression::Unary(Unary::new(
						Token::Not,
						Expression::Literal(Literal::Bool(true))
					).unwrap())
				).unwrap()),
				Token::Equal,
				Expression::Unary(Unary::new(
					Token::Not,
					Expression::Literal(Literal::Bool(false))
				).unwrap())
			).unwrap()),
			Token::Equal,
			Expression::Grouping(Grouping::new(
				Expression::Binary(Binary::new(
					Expression::Unary(Unary::new(
						Token::Minus,
						Expression::Grouping(Grouping::new(
							Expression::Unary(Unary::new(
								Token::Minus,
								Expression::Literal(Literal::Int(1))
							).unwrap())
						))
					).unwrap()),
					Token::Equal,
					Expression::Literal(Literal::Int(1))
				).unwrap())
			))
		).unwrap()),
		Token::NotEqual,
		Expression::Literal(Literal::Bool(false))
	).unwrap());
	test_parser_expression(tokens, expected);
}

#[test]
fn unary_operators02() {
	// ---1 == -(-(-1)) == -1
	let tokens = vec![
		Token::Minus,
		Token::Minus,
		Token::Minus,
		Token::Value("1"),
		Token::Equal,
		Token::Minus,
		Token::GroupingStart,
		Token::Minus,
		Token::GroupingStart,
		Token::Minus,
		Token::GroupingStart,
		Token::Value("1"),
		Token::GroupingEnd,
		Token::GroupingEnd,
		Token::GroupingEnd,
		Token::Equal,
		Token::Minus,
		Token::Value("1")
	];
	let expected = Expression::Binary(Binary::new(
		Expression::Binary(Binary::new(
			Expression::Unary(Unary::new(
				Token::Minus,
				Expression::Unary(Unary::new(
					Token::Minus,
					Expression::Unary(Unary::new(
						Token::Minus,
						Expression::Literal(Literal::Int(1))
					).unwrap())
				).unwrap())
			).unwrap()),
			Token::Equal,
			Expression::Unary(Unary::new(
				Token::Minus,
				Expression::Grouping(Grouping::new(
					Expression::Unary(Unary::new(
						Token::Minus,
						Expression::Grouping(Grouping::new(
							Expression::Unary(Unary::new(
								Token::Minus,
								Expression::Grouping(Grouping::new(
									Expression::Literal(Literal::Int(1))
								))
							).unwrap())
						))
					).unwrap())
				))
			).unwrap())
		).unwrap()),
		Token::Equal,
		Expression::Unary(Unary::new(
			Token::Minus,
			Expression::Literal(Literal::Int(1))
		).unwrap())
	).unwrap());
	test_parser_expression(tokens, expected);
}

#[test]
fn grouping() {
	// (1 + 2) * 3 == 9
	let tokens = vec![
		Token::GroupingStart,
		Token::Value("1"),
		Token::Plus,
		Token::Value("2"),
		Token::GroupingEnd,
		Token::Multiply,
		Token::Value("3"),
		Token::Equal,
		Token::Value("9")
	];
	let expected = Expression::Binary(Binary::new(
		Expression::Binary(Binary::new(
			Expression::Grouping(Grouping::new(
				Expression::Binary(Binary::new(
					Expression::Literal(Literal::Int(1)),
					Token::Plus,
					Expression::Literal(Literal::Int(2))
				).unwrap())
			)),
			Token::Multiply,
			Expression::Literal(Literal::Int(3))
		).unwrap()),
		Token::Equal,
		Expression::Literal(Literal::Int(9))
	).unwrap());
	test_parser_expression(tokens, expected);
}

#[test]
fn json_value01() {
	// user.age == 42
	let tokens = vec![Token::Value("user.age"), Token::Equal, Token::Value("42")];
	let expected = Expression::Binary(Binary::new(
		Expression::Variable(Variable::from_str("user.age").unwrap()),
		Token::Equal,
		Expression::Literal(Literal::Int(42))
	).unwrap());
	test_parser_expression(tokens, expected);
}

// Invalid tests

#[test]
fn invalid_grouping01() {
	// (1 + 2 * 3 == 7
	let tokens = vec![
		Token::GroupingStart,
		Token::Value("1"),
		Token::Plus,
		Token::Value("2"),
		Token::Multiply,
		Token::Value("3"),
		Token::Equal,
		Token::Value("7")
	];
	should_fail_expression(tokens);
}

#[test]
fn invalid_grouping02() {
	// 1 + 2 * 3) == 7
	let tokens = vec![
		Token::Value("1"),
		Token::Plus,
		Token::Value("2"),
		Token::Multiply,
		Token::Value("3"),
		Token::GroupingEnd,
		Token::Equal,
		Token::Value("7")
	];
	should_fail(tokens);
}
