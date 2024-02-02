use crate::Token;
use crate::syntax_tree::model::{
	Stmt
};
use super::{test_parser, should_fail};

#[test]
fn basic_blocks() {
	test_parser(
		vec![
			Token::DelimiterStart,
			Token::Continue,
			Token::DelimiterEnd
		],
		Stmt::Continue
	);
	test_parser(
		vec![
			Token::DelimiterStart,
			Token::Break,
			Token::DelimiterEnd
		],
		Stmt::Break
	);
}

#[test]
fn should_fail01() {
	should_fail(vec![
		Token::DelimiterStart,
		Token::Continue,
		Token::Continue,
		Token::DelimiterEnd
	]);
	should_fail(vec![
		Token::DelimiterStart,
		Token::Break,
		Token::Break,
		Token::DelimiterEnd
	]);
	should_fail(vec![
		Token::DelimiterStart,
		Token::Continue,
		Token::Break,
		Token::DelimiterEnd
	]);
	should_fail(vec![
		Token::DelimiterStart,
		Token::Break,
		Token::Continue,
		Token::DelimiterEnd
	]);
}
