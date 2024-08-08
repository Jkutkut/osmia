use crate::lexer::Token;
use crate::model::{
	Expression, JsonExpression,
	Binary, Unary, Grouping, Literal, Variable,
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
	let expected = Stmt::Expression(JsonExpression::Expression(expected));
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
			Token::Value("1".to_string()),
			Token::Plus,
			Token::Value("2".to_string()),
			Token::Multiply,
			Token::Value("3".to_string()),
			Token::Equal,
			Token::Value("7".to_string())
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
			Token::Value("1".to_string()),
			Token::Plus,
			Token::Value("2".to_string()),
			Token::Multiply,
			Token::Value("3".to_string()),
			Token::Divide,
			Token::ParentStart,
			Token::Value("4".to_string()),
			Token::Plus,
			Token::Value("5".to_string()),
			Token::ParentEnd,
			Token::Equal,
			Token::Value("1".to_string()),
			Token::Plus,
			Token::ParentStart,
			Token::Value("2".to_string()),
			Token::Multiply,
			Token::Value("3".to_string()),
			Token::ParentEnd,
			Token::Modulo,
			Token::Value("9".to_string()),
			Token::Equal,
			Token::Value("true".to_string())
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
			Token::Value("true".to_string()),
			Token::Equal,
			Token::Not,
			Token::Value("false".to_string()),
			Token::Equal,
			Token::ParentStart,
			Token::Minus,
			Token::ParentStart,
			Token::Minus,
			Token::Value("1".to_string()),
			Token::ParentEnd,
			Token::Equal,
			Token::Value("1".to_string()),
			Token::ParentEnd,
			Token::NotEqual,
			Token::Value("false".to_string())
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
			Token::Value("1".to_string()),
			Token::Equal,
			Token::Minus,
			Token::ParentStart,
			Token::Minus,
			Token::ParentStart,
			Token::Minus,
			Token::ParentStart,
			Token::Value("1".to_string()),
			Token::ParentEnd,
			Token::ParentEnd,
			Token::ParentEnd,
			Token::Equal,
			Token::Minus,
			Token::Value("1".to_string())
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
			Token::ParentStart,
			Token::Value("1".to_string()),
			Token::Plus,
			Token::Value("2".to_string()),
			Token::ParentEnd,
			Token::Multiply,
			Token::Value("3".to_string()),
			Token::Equal,
			Token::Value("9".to_string())
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
		vec![Token::Value("user.age".to_string()), Token::Equal, Token::Value("42".to_string())],
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
			Token::ParentStart,
			Token::Value("1".to_string()),
			Token::Plus,
			Token::Value("2".to_string()),
			Token::Multiply,
			Token::Value("3".to_string()),
			Token::Equal,
			Token::Value("7".to_string())
		]
	),
	(
		invalid_grouping02, // 1 + 2 * 3) == 7
		vec![
			Token::Value("1".to_string()),
			Token::Plus,
			Token::Value("2".to_string()),
			Token::Multiply,
			Token::Value("3".to_string()),
			Token::ParentEnd,
			Token::Equal,
			Token::Value("7".to_string())
		]
	),
	(empty, vec![]),
	(
		invalid_operator01, // + == 1
		vec![
			Token::Plus,
			Token::Equal,
			Token::Value("1".to_string())
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
			Token::Value("3".to_string()),
			Token::Equal,
			Token::Value("2".to_string())
		]
	),
	(
		invalid_operator07, // 3 * == 2
		vec![
			Token::Value("3".to_string()),
			Token::Multiply,
			Token::Equal,
			Token::Value("2".to_string())
		]
	),
	(
		invalid_operator08, // 3 + == 2
		vec![
			Token::Value("3".to_string()),
			Token::Plus,
			Token::Equal,
			Token::Value("2".to_string())
		]
	),
	(
		invalid_operator09, // 3 -
		vec![Token::Value("3".to_string()), Token::Minus]
	),
	(
		invalid_operator10, // 3 *
		vec![Token::Value("3".to_string()), Token::Multiply]
	),
	(
		invalid_operator11, // 3 /
		vec![Token::Value("3".to_string()), Token::Divide]
	),
	(
		invalid_operator12, // 3 %
		vec![Token::Value("3".to_string()), Token::Modulo]
	),
	(
		invalid_operator13, // 3 == == 2
		vec![
			Token::Value("3".to_string()),
			Token::Equal,
			Token::Equal,
			Token::Value("2".to_string())
		]
	),
	(
		invalid_number01, // 1.0.0
		vec![Token::Value("1.0.0".to_string())]
	),
	(
		invalid_number02, // 1.0.0.0
		vec![Token::Value("1.0.0.0".to_string())]
	),
	(
		invalid_number03, // 1a
		vec![Token::Value("1a".to_string())]
	),
	(
		invalid_number04, // 1.0a
		vec![Token::Value("1.0a".to_string())]
	),
	(
		invalid_number05, // 1.a0
		vec![Token::Value("1.a0".to_string())]
	),
	(
		invalid_variable01, // foo.
		vec![Token::Value("foo.".to_string())]
	),
	(
		invalid_variable02, // foo.0
		vec![Token::Value("foo.0".to_string())]
	),
	(
		invalid_variable03, // foo[
		vec![Token::Value("foo[".to_string())]
	),
	(
		invalid_variable04, // foo[0
		vec![Token::Value("foo[0".to_string())]
	)
);
