use crate::Token;
use crate::syntax_tree::model::{
	Variable, Expression,
	Stmt, ForEach
};
use super::{test_parser, should_fail};

#[test]
fn basic_test() {
	test_parser( // {{for a in lst}}This line is constant{{end}}
		vec![
			Token::DelimiterStart,
			Token::For,
			Token::Value("a"),
			Token::In,
			Token::Value("lst"),
			Token::DelimiterEnd,
			Token::Raw("This line is constant"),
			Token::DelimiterStart,
			Token::Done,
			Token::DelimiterEnd
		],
		Stmt::ForEach(ForEach::new(
			Variable::from_str("a").unwrap(),
			Variable::from_str("lst").unwrap(),
			Stmt::Raw("This line is constant")
		))
	);
}

#[test]
fn nested() {
	test_parser( // {{for arr in matrix}}{{for cell in arr}}{{print cell}}{{end}}{{end}}
		vec![
			Token::DelimiterStart,
			Token::For,
			Token::Value("arr"),
			Token::In,
			Token::Value("matrix"),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::For,
			Token::Value("cell"),
			Token::In,
			Token::Value("arr"),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Print,
			Token::Value("cell"),
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Done,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Done,
			Token::DelimiterEnd
		],
		Stmt::ForEach(ForEach::new(
			Variable::from_str("arr").unwrap(),
			Variable::from_str("matrix").unwrap(),
			Stmt::ForEach(ForEach::new(
				Variable::from_str("cell").unwrap(),
				Variable::from_str("arr").unwrap(),
				Stmt::Print(
					Expression::Variable(
						Variable::from_str("cell").unwrap()
					)
				)
			))
		))
	);
}

// Fail tests

#[test]
fn fail01() {
	should_fail(vec![
		Token::DelimiterStart,
		Token::For,
		Token::Value("a"),
		Token::In,
		Token::Value("lst"),
		Token::DelimiterEnd
	]);
}

#[test]
fn fail02() {
	should_fail(vec![
		Token::DelimiterStart,
		Token::For,
		Token::Value("a"),
		Token::In
	]);
}

#[test]
fn fail03() {
	should_fail(vec![
		Token::DelimiterStart,
		Token::For,
		Token::Value("a")
	]);
}

#[test]
fn fail04() {
	should_fail(vec![
		Token::DelimiterStart,
		Token::For
	]);
}
