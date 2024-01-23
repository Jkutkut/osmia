use crate::lexer::Token;
use crate::syntax_tree::model::{
	Expression, Literal, Binary,
	Grouping, Unary
};
use crate::syntax_tree::syntax_tree_printer::SyntaxTreePrinter;
use crate::syntax_tree::visitable::{Visitable};

#[test]
fn create_syntax_tree01() {
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
	for (i, expr) in expressions.iter().enumerate() {
		println!("Testing {}º...", i);
		let result = expr.accept(&printer);
		assert_eq!(result, expecteds[i]);
	}
}

#[test]
fn create_syntax_tree02() {
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
