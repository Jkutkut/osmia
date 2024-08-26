use super::*;

macro_tests!(
	lexer_expression_test,
	(string_01, r#""hello world""#, vec![Token::new_str(r#""hello world""#)]),
	(string_02, r#"'hello world'"#, vec![Token::new_str(r#"'hello world'"#)]),
	(string_03, r#"'hello' "world""#, vec![Token::new_str(r#"'hello'"#), Token::Whitespace, Token::new_str(r#""world""#)]),
	(string_04, r#"'hello'"world""#, vec![Token::new_str(r#"'hello'"#), Token::new_str(r#""world""#)]),
	(string_05, r#"''"#, vec![Token::new_str(r#"''"#)]),
	(string_06, r#""""#, vec![Token::new_str(r#""""#)]),
	(string_new_line_01, "'hello\nworld'", vec![Token::NewLine, Token::new_str("'hello\nworld'")]),
	(string_new_line_02, "'he\nllo\n\nwor\nld'", vec![
		Token::NewLine, Token::NewLine, Token::NewLine, Token::NewLine,
		Token::new_str("'he\nllo\n\nwor\nld'")
	]),
);

macro_tests!(
	lexer_expression_test_fail,
	(invalid_string_01, r#""hello world"#, "string"),
	(invalid_string_02, r#"'hello world"#, "string"),
	(invalid_string_03, r#""hello world 3 + 2 - 2 { }"#, "string"),
);
