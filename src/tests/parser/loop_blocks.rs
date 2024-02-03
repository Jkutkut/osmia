use crate::Token;
use crate::syntax_tree::model::{
	Stmt
};
use super::{test_parser, should_fail};
use crate::macro_tests;

macro_tests!(
	test_parser,
	(
		basic_blocks01,
		vec![
			Token::DelimiterStart,
			Token::Continue,
			Token::DelimiterEnd
		],
		Stmt::Continue
	),
	(
		basic_blocks02,
		vec![
			Token::DelimiterStart,
			Token::Break,
			Token::DelimiterEnd
		],
		Stmt::Break
	)
);

macro_tests!(
	should_fail,
	(
		should_fail01,
		vec![
			Token::DelimiterStart,
			Token::Continue,
			Token::Continue,
			Token::DelimiterEnd
		]
	),
	(
		should_fail02,
		vec![
			Token::DelimiterStart,
			Token::Break,
			Token::Break,
			Token::DelimiterEnd
		]
	),
	(
		should_fail03,
		vec![
			Token::DelimiterStart,
			Token::Continue,
			Token::Break,
			Token::DelimiterEnd
		]
	),
	(
		should_fail04,
		vec![
			Token::DelimiterStart,
			Token::Break,
			Token::Continue,
			Token::DelimiterEnd
		]
	)
);
