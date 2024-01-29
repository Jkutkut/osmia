use crate::Token;
use crate::syntax_tree::model::{
	Expression, Variable, Literal, Binary,
	Stmt, Assign
};
use super::{test_parser};

#[test]
fn basic_test01() {
	test_parser(
		vec![
			Token::DelimiterStart,
			Token::Assign,
			Token::Value("foo"),
			Token::Equal,
			Token::Value("\"bar\""),
			Token::DelimiterEnd
		],
		Stmt::Assign(
			Assign::new(
				Variable::from_str("foo").unwrap(),
				Expression::Literal(Literal::from_str("\"bar\"").unwrap())
			)
		)
	);
}

#[test]
fn basic_test02() {
	test_parser(
		vec![
			Token::DelimiterStart,
			Token::Assign,
			Token::Value("foo"),
			Token::Equal,
			Token::Value("bar"),
			Token::DelimiterEnd
		],
		Stmt::Assign(
			Assign::new(
				Variable::from_str("foo").unwrap(),
				Expression::Variable(Variable::from_str("bar").unwrap())
			)
		)
	);
}

#[test]
fn basic_test03() {
	test_parser(
		vec![
			Token::DelimiterStart,
			Token::Assign,
			Token::Value("foo"),
			Token::Equal,
			Token::Value("2"),
			Token::Plus,
			Token::Value("2"),
			Token::DelimiterEnd
		],
		Stmt::Assign(
			Assign::new(
				Variable::from_str("foo").unwrap(),
				Expression::Binary(Binary::new(
					Expression::Literal(Literal::from_str("2").unwrap()),
					Token::Plus,
					Expression::Literal(Literal::from_str("2").unwrap())
				).unwrap())
			)
		)
	);
}
