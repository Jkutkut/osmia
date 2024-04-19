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
	)
);
