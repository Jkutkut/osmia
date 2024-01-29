use crate::Token;
use crate::syntax_tree::model::{
	Stmt
};
use super::{test_parser};

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
