use crate::lexer::Token;
use crate::model::{
	Expression, Binary, Unary, Grouping, Literal, Variable,
	Stmt
};
use super::{test_parser, should_fail};
use crate::macro_tests;

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

macro_tests!(
	test_parser_expression,
	(
		basic_parser, // 1 + 2 * 3 == 7
		vec![ // 1 + 2 * 3 == 7
			Token::Value("1"),
			Token::Plus,
			Token::Value("2"),
			Token::Multiply,
			Token::Value("3"),
			Token::Equal,
			Token::Value("7")
		],
		Expression::Binary(Binary::new(
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
		).unwrap())
	),
	(
		precedence, // 1 + 2 * 3 / (4 + 5) == 1 + (2 * 3) % 9 == true
		vec![
			Token::Value("1"),
			Token::Plus,
			Token::Value("2"),
			Token::Multiply,
			Token::Value("3"),
			Token::Divide,
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
			Token::Modulo,
			Token::Value("9"),
			Token::Equal,
			Token::Value("true")
		],
		Expression::Binary(Binary::new(
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
						Token::Divide,
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
						Token::Modulo,
						Expression::Literal(Literal::Int(9))
					).unwrap())
				).unwrap())
			).unwrap()),
			Token::Equal,
			Expression::Literal(Literal::Bool(true))
		).unwrap())
	),
	(
		unary_operators01, // !!true == !false == (-(-1) == 1) != false
		vec![
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
		],
		Expression::Binary(Binary::new(
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
		).unwrap())
	),
	(
		unary_operators02, // ---1 == -(-(-1)) == -1
		vec![
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
		],
		Expression::Binary(Binary::new(
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
		).unwrap())
	),
	(
		grouping, // (1 + 2) * 3 == 9
		vec![
			Token::GroupingStart,
			Token::Value("1"),
			Token::Plus,
			Token::Value("2"),
			Token::GroupingEnd,
			Token::Multiply,
			Token::Value("3"),
			Token::Equal,
			Token::Value("9")
		],
		Expression::Binary(Binary::new(
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
		).unwrap())
	),
	(
		json_value01, // user.age == 42
		vec![Token::Value("user.age"), Token::Equal, Token::Value("42")],
		Expression::Binary(Binary::new(
			Expression::Variable(Variable::from_str("user.age").unwrap()),
			Token::Equal,
			Expression::Literal(Literal::Int(42))
		).unwrap())
	)
);

// Invalid tests

macro_tests!(
	should_fail_expression,
	(
		invalid_grouping01, // (1 + 2 * 3 == 7
		vec![
			Token::GroupingStart,
			Token::Value("1"),
			Token::Plus,
			Token::Value("2"),
			Token::Multiply,
			Token::Value("3"),
			Token::Equal,
			Token::Value("7")
		]
	),
	(
		invalid_grouping02, // 1 + 2 * 3) == 7
		vec![
			Token::Value("1"),
			Token::Plus,
			Token::Value("2"),
			Token::Multiply,
			Token::Value("3"),
			Token::GroupingEnd,
			Token::Equal,
			Token::Value("7")
		]
	),
	(empty, vec![]),
	(
		invalid_operator01, // + == 1
		vec![
			Token::Plus,
			Token::Equal,
			Token::Value("1")
		]
	),
	(
		invalid_operator02, // - == 1
		vec![Token::Minus]
	),
	(
		invalid_operator03, // * == 1
		vec![Token::Multiply]
	),
	(
		invalid_operator04, // / == 1
		vec![Token::Divide]
	),
	(
		invalid_operator05, // % == 1
		vec![Token::Modulo]
	),
	(
		invalid_operator06, // * 3 == 2
		vec![
			Token::Multiply,
			Token::Value("3"),
			Token::Equal,
			Token::Value("2")
		]
	),
	(
		invalid_operator07, // 3 * == 2
		vec![
			Token::Value("3"),
			Token::Multiply,
			Token::Equal,
			Token::Value("2")
		]
	),
	(
		invalid_operator08, // 3 + == 2
		vec![
			Token::Value("3"),
			Token::Plus,
			Token::Equal,
			Token::Value("2")
		]
	),
	(
		invalid_operator09, // 3 -
		vec![Token::Value("3"), Token::Minus]
	),
	(
		invalid_operator10, // 3 *
		vec![Token::Value("3"), Token::Multiply]
	),
	(
		invalid_operator11, // 3 /
		vec![Token::Value("3"), Token::Divide]
	),
	(
		invalid_operator12, // 3 %
		vec![Token::Value("3"), Token::Modulo]
	),
	(
		invalid_operator13, // 3 == == 2
		vec![
			Token::Value("3"),
			Token::Equal,
			Token::Equal,
			Token::Value("2")
		]
	),
	(
		invalid_number01, // 1.0.0
		vec![Token::Value("1.0.0")]
	),
	(
		invalid_number02, // 1.0.0.0
		vec![Token::Value("1.0.0.0")]
	),
	(
		invalid_number03, // 1a
		vec![Token::Value("1a")]
	),
	(
		invalid_number04, // 1.0a
		vec![Token::Value("1.0a")]
	),
	(
		invalid_number05, // 1.a0
		vec![Token::Value("1.a0")]
	),
	(
		invalid_variable01, // foo.
		vec![Token::Value("foo.")]
	),
	(
		invalid_variable02, // foo.0
		vec![Token::Value("foo.0")]
	),
	(
		invalid_variable03, // foo[
		vec![Token::Value("foo[")]
	),
	(
		invalid_variable04, // foo[0
		vec![Token::Value("foo[0")]
	)
);
