use crate::Token;
use crate::model::{
	Expression, Variable, Literal, Binary,
	Stmt, Assign, JsonExpression
};
use super::{test_parser, should_fail};

#[test]
fn basic_test01() {
	test_parser( // foo = "bar"
		vec![
			Token::DelimiterStart,
			Token::Assign,
			Token::Value("foo"),
			Token::AssignEq,
			Token::Value("\"bar\""),
			Token::DelimiterEnd
		],
		Stmt::Assign(
			Assign::new(
				Variable::from_str("foo").unwrap(),
				JsonExpression::Expression(Expression::Literal(Literal::from_str("\"bar\"").unwrap()))
			)
		)
	);
}

#[test]
fn basic_test02() {
	test_parser( // foo = bar
		vec![
			Token::DelimiterStart,
			Token::Assign,
			Token::Value("foo"),
			Token::AssignEq,
			Token::Value("bar"),
			Token::DelimiterEnd
		],
		Stmt::Assign(
			Assign::new(
				Variable::from_str("foo").unwrap(),
				JsonExpression::Expression(Expression::Variable(Variable::from_str("bar").unwrap()))
			)
		)
	);
}

#[test]
fn basic_test03() {
	test_parser( // foo = 2 + 2
		vec![
			Token::DelimiterStart,
			Token::Assign,
			Token::Value("foo"),
			Token::AssignEq,
			Token::Value("2"),
			Token::Plus,
			Token::Value("2"),
			Token::DelimiterEnd
		],
		Stmt::Assign(
			Assign::new(
				Variable::from_str("foo").unwrap(),
				JsonExpression::Expression(Expression::Binary(Binary::new(
					Expression::Literal(Literal::from_str("2").unwrap()),
					Token::Plus,
					Expression::Literal(Literal::from_str("2").unwrap())
				).unwrap()))
			)
		)
	);
}

#[test]
fn fail01() {
	should_fail(vec![
		Token::DelimiterStart,
		Token::Assign,
		Token::Value("foo"),
		Token::AssignEq,
		Token::DelimiterEnd
	]);
}

#[test]
fn fail02() {
	should_fail(vec![
		Token::DelimiterStart,
		Token::Assign,
		Token::Value("foo"),
		Token::AssignEq
	]);
}

#[test]
fn fail03() {
	should_fail(vec![
		Token::DelimiterStart,
		Token::Assign,
		Token::Value("foo")
	]);
}

#[test]
fn fail04() {
	should_fail(vec![
		Token::DelimiterStart,
		Token::Assign
	]);
}

#[test]
fn fail05() {
	should_fail(vec![
		Token::DelimiterStart,
		Token::Assign,
		Token::Value("foo"),
		Token::AssignEq,
		Token::DelimiterEnd
	]);
}

#[test]
fn fail06() {
	should_fail(vec![
		Token::DelimiterStart,
		Token::Assign,
		Token::DelimiterEnd
	]);
}
