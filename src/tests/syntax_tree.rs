use crate::token::Token;
use crate::syntax_tree::model::{Expression, Literal, Grouping, Unary, Binary};
use crate::syntax_tree::syntax_tree_printer::SyntaxTreePrinter;
use crate::syntax_tree::visitable::{Visitable};
use crate::parser::Parser;


#[test]
fn test() {
	assert_eq!(1, 1);
}

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
	let parsed_result = match Parser::new(tokens).parse() {
		Ok(expr) => expr,
		Err(err) => panic!("Parser threw an error:\n{}", err),
	};
	let printer = SyntaxTreePrinter;
	println!("\nOriginal:\n{}", expected.accept(&printer));
	println!("Parsed:\n{}", parsed_result.accept(&printer));
	assert_eq!(parsed_result, expected);
}

#[test]
fn basic_parser01() {
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

// TODO tests for precedence
// TODO tests for associativity
// TODO tests for unary operators
// TODO tests for grouping
// TODO tests for literals
// TODO tests for errors
