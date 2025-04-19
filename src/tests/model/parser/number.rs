use super::*;

macro_tests!(
	parser_test_fail_tokens,
	(
		invalid_float_01,
		vec![
			Token::StmtStart,
			Token::new_number("asdf12.123"),
			Token::StmtEnd,
			Token::Eof
		],
		&["parse", "float"]
	),
	(
		invalid_float_02,
		vec![
			Token::StmtStart,
			Token::new_number("12f.123"),
			Token::StmtEnd,
			Token::Eof
		],
		&["parse", "float"]
	),
	(
		invalid_float_03,
		vec![
			Token::StmtStart,
			Token::new_number("12.f123"),
			Token::StmtEnd,
			Token::Eof
		],
		&["parse", "float"]
	),
	(
		invalid_float_04,
		vec![
			Token::StmtStart,
			Token::new_number("12.123s"),
			Token::StmtEnd,
			Token::Eof
		],
		&["parse", "float"]
	),
	(
		invalid_float_05,
		vec![
			Token::StmtStart,
			Token::new_number("12..123"),
			Token::StmtEnd,
			Token::Eof
		],
		&["parse", "float"]
	),
	(
		invalid_int_01,
		vec![
			Token::StmtStart,
			Token::new_number("1a"),
			Token::StmtEnd,
			Token::Eof
		],
		&["parse", "int"]
	),
	(
		invalid_int_02,
		vec![
			Token::StmtStart,
			Token::new_number("a1"),
			Token::StmtEnd,
			Token::Eof
		],
		&["parse", "int"]
	),
);

macro_tests!(
	parser_test_fail,
	(
		invalid_int_03,
		"{{ 12321312312312312312312312312 }}",
		&["parse", "int"]
	),
	(
		invalid_int_04,
		"{{ 9223372036854775808 }}",
		&["parse", "int"]
	),
	(
		invalid_int_05,
		"{{ -9223372036854775809 }}",
		&["parse", "int"]
	),
);
