use super::*;

macro_tests!(
	parser_test_fail,
	(
		lambda_07,
		"{{ fn (foo, op1 = 42, bar) => foo }}",
		&["mandatory", "after", "optional"]
	)
);
