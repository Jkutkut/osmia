use super::*;

macro_tests!(
	test,
	(
		print01,
		Some("print: {{print 1 }}"),
		Some(vec![
			Token::new_raw("print: "),
			Token::StmtStart,
			Token::Print,
			Token::Whitespace,
			Token::new_number("1"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
		// r#"{}"#,
		// "print: "
	),
	(
		print02,
		Some("print: {{print 1 + 1.2 }}"),
		Some(vec![
			Token::new_raw("print: "),
			Token::StmtStart,
			Token::Print,
			Token::Whitespace,
			Token::new_number("1"),
			Token::Whitespace,
			Token::Plus,
			Token::Whitespace,
			Token::new_number("1.2"),
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
		// "{}",
		// "print: "
	),
	(
		print03,
		Some("{{print [1, 2, 3] }}"),
		Some(vec![
			Token::StmtStart,
			Token::Print,
			Token::Whitespace,
			Token::ArrayStart,
			Token::new_number("1"),
			Token::Comma,
			Token::Whitespace,
			Token::new_number("2"),
			Token::Comma,
			Token::Whitespace,
			Token::new_number("3"),
			Token::ArrayEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
		// "{}",
		// ""
	),
	(
		print04,
		Some(r#"{{print {"a": 1, "b": 2} }}"#),
		Some(vec![
			Token::StmtStart,
			Token::Print,
			Token::Whitespace,
			Token::ObjectStart,
			Token::new_str("a"),
			Token::Colon,
			Token::Whitespace,
			Token::new_number("1"),
			Token::Comma,
			Token::Whitespace,
			Token::new_str("b"),
			Token::Colon,
			Token::Whitespace,
			Token::new_number("2"),
			Token::ObjectEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
		// "{}",
		// ""
	)
);
