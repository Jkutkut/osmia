use crate::macro_tests;
use super::{test_interpreter/*, expect_error*/};

macro_tests!(
	test_interpreter,
	(
		while01,
		"{{while v < 3}}{{ v }}{{assign v = v + 1}}{{done}}",
		r#"{"v": 0}"#,
		"012"
	),
	(
		while02,
		"{{while v != 0}}{{v % 2}}{{assign v = v - 1}}{{done}}",
		r#"{"v": 10}"#,
		"0101010101"
	),
	(
		whilefalse,
		"{{while false}}123{{done}}",
		r#"{}"#,
		""
	),
	(
		while03,
		"{{while v != 10000}}{{assign v = v + 1}}{{done}}{{v}}",
		r#"{"v": 0}"#,
		"10000"
	)
);

// TODO
