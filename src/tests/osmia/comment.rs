use super::*;

macro_tests!(
	test,
	(
		comment_01,
		Some("{{# Hey there! }}"),
		Some(vec![
			Token::StmtStart,
			Token::new_comment("Hey there!"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::new_comment("Hey there!")),
		None
	),
	(
		comment_02,
		Some("{{#Hey there!}}"),
		Some(vec![
			Token::StmtStart,
			Token::new_comment("Hey there!"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::new_comment("Hey there!")),
		None
	),
	(
		comment_03,
		Some("{{# \n\n Hey\nthere!\n\n }}"),
		Some(vec![
			Token::StmtStart,
			Token::NewLine,
			Token::NewLine,
			Token::NewLine,
			Token::NewLine,
			Token::NewLine,
			Token::new_comment("Hey\nthere!"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::new_comment("Hey\nthere!")),
		None
	),
	(
		comment_04,
		Some("{{# {{ this should not be parsed }} this should not be parsed }}"),
		Some(vec![
			Token::StmtStart,
			Token::new_comment("{{ this should not be parsed }} this should not be parsed"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::new_comment("{{ this should not be parsed }} this should not be parsed")),
		None
	),
	(
		comment_05,
		Some("{{# this {{ is a comment {{ }} }} }}"),
		Some(vec![
			Token::StmtStart,
			Token::new_comment("this {{ is a comment {{ }} }}"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::new_comment("this {{ is a comment {{ }} }}")),
		None
	)
);
