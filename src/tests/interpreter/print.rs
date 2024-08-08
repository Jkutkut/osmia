use crate::macro_tests;
use super::{test_interpreter};

macro_tests!(
	test_interpreter,
	(
		print01,
		"print: {{print 1 }}",
		r#"{}"#,
		"print: "
	),
	(
		print02,
		"print: {{print 1 + 1.2 }}",
		"{}",
		"print: "
	),
	(
		print03,
		"{{print [1, 2, 3] }}",
		"{}",
		""
	),
	(
		print04,
		r#"{{print {"a": 1, "b": 2} }}"#,
		"{}",
		""
	)
);
