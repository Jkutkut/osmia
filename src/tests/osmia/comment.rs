use super::*;

macro_tests!(
	test,
	(
		comment_00,
		Some("{{#}} {{# }}"),
		Some(vec![
			Token::StmtStart,
			Token::Comment,
			Token::StmtEnd,
			Token::new_raw(" "),
			Token::StmtStart,
			Token::Comment,
			Token::new_raw(" "),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::Block(vec![
			Stmt::new_comment(""),
			Stmt::new_raw(" "),
			Stmt::new_comment(" "),
		].into())),
		Some(vec![(Ctx::new(), Ok(" "))])
	),
	(
		comment_01,
		Some("{{# Hey there! }}"),
		Some(vec![
			Token::StmtStart,
			Token::Comment,
			Token::new_raw(" Hey there! "),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::new_comment(" Hey there! ")),
		Some(vec![(Ctx::new(), Ok(""))])
	),
	(
		comment_02,
		Some("{{#Hey there!}}"),
		Some(vec![
			Token::StmtStart,
			Token::Comment,
			Token::new_raw("Hey there!"),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::new_comment("Hey there!")),
		Some(vec![(Ctx::new(), Ok(""))])
	),
	(
		comment_03,
		Some("{{# \n\n\n\n Hey\nthere!\n\n }}"),
		Some(vec![
			Token::StmtStart,
			Token::Comment,
			Token::new_raw(" "),
			Token::NewLineNonPrintable,
			Token::NewLine,
			Token::NewLine,
			Token::NewLine,
			Token::new_raw(" Hey"),
			Token::NewLine,
			Token::new_raw("there!"),
			Token::NewLine,
			Token::NewLine,
			Token::new_raw(" "),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::new_comment(" \n\n\n\n Hey\nthere!\n\n ")),
		Some(vec![(Ctx::new(), Ok(""))])
	),
	(
		comment_04,
		Some("{{# {{ this should not be parsed }} this should not be parsed }}"),
		Some(vec![
			Token::StmtStart,
			Token::Comment,
			Token::new_raw(" {{ this should not be parsed }} this should not be parsed "),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::new_comment(" {{ this should not be parsed }} this should not be parsed ")),
		Some(vec![(Ctx::new(), Ok(""))])
	),
	(
		comment_05,
		Some("{{# this {{ is a comment {{ }} }} }}"),
		Some(vec![
			Token::StmtStart,
			Token::Comment,
			Token::new_raw(" this {{ is a comment {{ }} }} "),
			Token::StmtEnd,
			Token::Eof
		]),
		Some(Stmt::new_comment(" this {{ is a comment {{ }} }} ")),
		Some(vec![(Ctx::new(), Ok(""))])
	),
	(
		comment_06,
		Some("{{#
	This is a multi line comment.
	Code can be added here and it will be ignored as long as there is no closing delimiter.
	{{ 1 + 1 }}
	}}"),
		None,
		None,
		Some(vec![
			(Ctx::new(), Ok("")),
		])
	)
);
