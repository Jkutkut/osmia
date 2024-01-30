use crate::Token;
use crate::syntax_tree::model::{
	Expression, Literal,
	Stmt, ConditionalBlock, Block
};
use super::{test_parser};

#[test]
fn basic_test() {
	test_parser( // {{ while true }} {{print "hello"}} {{done}}
		vec![
			Token::DelimiterStart,
			Token::While,
			Token::Value("true"),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value("\"hello\""),
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
			Token::Value("true"),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Value("123"),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value("456"),
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
			Token::Value("true"),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value("\"loop\""),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::While,
			Token::Value("true"),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value("\"nested\""),
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
