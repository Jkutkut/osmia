use super::*;

macro_tests!(
	parser_test_fail,
	(
		invalid_grouping_01,
		"{{ ( ) }}",
		&["identifier"]
	),
	(
		invalid_grouping_02,
		"{{ ( 12 }}",
		&["end", "grouping"]
	),
	(
		invalid_grouping_03,
		"{{ 12 ) }}",
		&["')'"]
	)
);
