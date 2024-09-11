use super::*;

macro_tests!(
	parser_test_fail_tokens,
	(
		invalid_token_in_block,
		vec![Token::If, Token::Eof],
		&["unexpected", "block"]
	),
	(
		invalid_token_in_comment,
		vec![
			Token::StmtStart,
			Token::Comment,
			Token::If,
			Token::StmtEnd,
			Token::Eof
		],
		&["parse", "comment"]
	),
	(
		invalid_token_in_comment_02,
		vec![
			Token::StmtStart,
			Token::Comment,
			Token::new_str("Some invalid token"),
			Token::StmtEnd,
			Token::Eof
		],
		&["parse", "comment"]
	),
	(
		invalid_method_call_01,
		vec![
			Token::StmtStart,
			Token::new_alpha("a"),
			Token::Question,
			Token::new_alpha("b"),
			Token::StmtEnd,
			Token::Eof
		],
		&["method", "call"]
	),
	(
		invalid_method_call_02,
		vec![
			Token::StmtStart,
			Token::new_alpha("a"),
			Token::Dot,
			Token::new_alpha("c"),
			Token::Question,
			Token::new_alpha("b"),
			Token::StmtEnd,
			Token::Eof
		],
		&["method", "call"]
	),
	(
		invalid_identifier,
		vec![
			Token::StmtStart,
			Token::new_alpha("a"),
			Token::Dot,
			Token::new_number("3"),
			Token::StmtEnd,
			Token::Eof
		],
		&["identifier", "invalid"]
	)
);
