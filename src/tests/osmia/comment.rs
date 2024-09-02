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
	)
);
