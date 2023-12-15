use crate::lexer::Token;
use crate::syntax_tree::model::{Expression, Literal, Grouping, Unary, Binary};
use crate::syntax_tree::syntax_tree_printer::SyntaxTreePrinter;
use crate::syntax_tree::visitable::{Visitable};
use crate::parser::Parser;

#[test]
fn create_syntax_tree01() {
	let expected = "(5 + 3) * 2 <= !!true";
	let expression = Expression::Binary(Binary::new(
		Expression::Grouping(Grouping::new(
			Expression::Binary(Binary::new(
				Expression::Literal(Literal::Int(5)),
				Token::Plus,
				Expression::Literal(Literal::Int(3))
			).unwrap())
		)),
		Token::Multiply,
		Expression::Binary(Binary::new(
			Expression::Literal(Literal::Int(2)),
			Token::LessEqual,
			Expression::Unary(Unary::new(
				Token::Not,
				Expression::Unary(Unary::new(
					Token::Not,
					Expression::Literal(Literal::Bool(true))
				).unwrap())
			).unwrap())
		).unwrap())
	).unwrap());
	let printer = SyntaxTreePrinter;
	let result = expression.accept(&printer);
	assert_eq!(result, expected);
}

#[test]
fn create_syntax_tree02() {
	let expecteds = vec![
		"Hello, world!",
		"42",
		"42.5",
		"null",
		"true",
		"false",
	];
	let expressions = vec![
		Expression::Literal(Literal::Str(String::from("Hello, world!"))),
		Expression::Literal(Literal::Int(42)),
		Expression::Literal(Literal::Float(42.5)),
		Expression::Literal(Literal::Null),
		Expression::Literal(Literal::Bool(true)),
		Expression::Literal(Literal::Bool(false)),
	];
	let printer = SyntaxTreePrinter;
	for (i, expression) in expressions.iter().enumerate() {
		let result = expression.accept(&printer);
		assert_eq!(result, expecteds[i]);
	}
}

fn test_parser(
	tokens: Vec<Token>,
	expected: Expression
) {
	let parsed_result = match Parser::new(&tokens).parse() {
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
	test_parser(tokens, expected);
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
	test_parser(tokens, expected);
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
	test_parser(tokens, expected);
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
	test_parser(tokens, expected);
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
	test_parser(tokens, expected);
}

// TODO tests for errors
fn should_fail(
	code: Vec<Token>,
) {
	let parsed_result = Parser::new(&code).parse();
	let printer = SyntaxTreePrinter;
	if let Ok(ref parsed_result) = parsed_result {
		println!("Parsed: {}", parsed_result.accept(&printer));
	}
	assert!(parsed_result.is_err());
}

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
	should_fail(tokens);
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

// TODO tests for json_values
