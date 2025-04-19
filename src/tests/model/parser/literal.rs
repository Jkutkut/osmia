use super::*;

macro_tests!(
	parser_test_fail,
	(
		invalid_number_01,
		"{{ 9223372036854775808 }}",
		&["not", "parse", "int"]
	),
	(
		invalid_number_02,
		"{{ 12321321312312312312312312 }}",
		&["not", "parse", "int"]
	)
);
