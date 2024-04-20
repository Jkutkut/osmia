use crate::lexer::Token;
use crate::model::{
	Expression, Literal,
	Stmt, ConditionalBlock, Block
};
use super::{test_parser, should_fail};

#[test]
fn basic_test() {
	test_parser( // {{ while true }} {{print "hello"}} {{done}}
		vec![
			Token::DelimiterStart,
			Token::While,
			Token::Value("true".to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value("\"hello\"".to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Done,
			Token::DelimiterEnd
		],
		Stmt::While(ConditionalBlock::new(
			Expression::Literal(Literal::Bool(true)),
			Stmt::Print(Expression::Literal(
				Literal::from_str("\"hello\"").unwrap()
			))
		))
	);
}

#[test]
fn basic_test02() {
	test_parser( // {{ while true }} {{123}} {{print 456}} {{done}}
		vec![
			Token::DelimiterStart,
			Token::While,
			Token::Value("true".to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Value("123".to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value("456".to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Done,
			Token::DelimiterEnd
		],
		Stmt::While(ConditionalBlock::new(
			Expression::Literal(Literal::Bool(true)),
			Stmt::Block(Block::new(
				vec![
					Stmt::Expression(Expression::Literal(
						Literal::from_str("123").unwrap()
					)),
					Stmt::Print(Expression::Literal(
						Literal::from_str("456").unwrap()
					))
				]
			))
		))
	);
}

#[test]
fn nested_01() {
	test_parser( // {{while true}} {{print "loop"}} {{while true}} {{print "nested"}} {{done}} {{done}}
		vec![
			Token::DelimiterStart,
			Token::While,
			Token::Value("true".to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value("\"loop\"".to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::While,
			Token::Value("true".to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value("\"nested\"".to_string()),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Done,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Done,
			Token::DelimiterEnd
		],
		Stmt::While(ConditionalBlock::new(
			Expression::Literal(Literal::Bool(true)),
			Stmt::Block(Block::new(
				vec![
					Stmt::Print(Expression::Literal(
						Literal::from_str("\"loop\"").unwrap()
					)),
					Stmt::While(ConditionalBlock::new(
						Expression::Literal(Literal::Bool(true)),
						Stmt::Print(Expression::Literal(
							Literal::from_str("\"nested\"").unwrap()
						))
					))
				]
			))
		))
	);
}

#[test]
fn should_fail01() {
	// unclosed
	should_fail(vec![
		Token::DelimiterStart,
		Token::While,
		Token::Value("true".to_string()),
		Token::DelimiterEnd,
		Token::DelimiterStart,
		Token::Print,
		Token::Value("\"hello\"".to_string()),
		Token::DelimiterEnd
	]);
}

#[test]
fn should_fail02() {
	// unclosed
	should_fail(vec![
		Token::DelimiterStart,
		Token::While,
		Token::Value("true".to_string()),
		Token::DelimiterEnd,
		Token::DelimiterStart,
		Token::Print,
		Token::Value("\"hello\"".to_string()),
		Token::DelimiterEnd,
		Token::DelimiterStart,
		Token::Done
	]);
}
