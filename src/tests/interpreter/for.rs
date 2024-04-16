use crate::macro_tests;
use super::{test_interpreter/*, expect_error*/};

macro_tests!(
	test_interpreter,
	(
		foreach01,
		"{{for v in [1, 2, 3]}}{{ v }}{{done}}",
		r#"{}"#,
		"123"
	),
	(
		foreach02,
		"{{for v in [1, 2, 3]}}{{ v }}{{done}}",
		r#"{"v": 3}"#,
		"123"
	),
	(
		foreach03,
		r#"{{for v in [true, false, null, "hello world"]}}{{ v }}{{done}}"#,
		r#"{"v": 3}"#,
		"truefalsenullhello world"
	),
	(
		foreach04,
		"{{for v in []}}{{ v }}{{done}}",
		r#"{}"#,
		""
	),
	(
		foreach05,
		"{{for v in [1 + v, 2, 3]}}{{ v }}{{done}}",
		r#"{"v": 3}"#,
		"423"
	),
	(
		foreachvariable01,
		"{{for v in arr}}{{ v }}{{done}}",
		r#"{"arr": [1, 2, 3]}"#,
		"123"
	),
	(
		foreachvariable02,
		"{{for v in arr}} {{ v }}{{done}}",
		r#"{"arr": [true, "2", null]}"#,
		" true 2 null"
	),
	(
		foreachvariable03,
		"{{for v in arr}}{{ v }}{{done}}",
		r#"{"arr": []}"#,
		""
	),
	(
		foreachvariable04,
		"{{for v in arr}}{{ v }}{{done}}",
		r#"{"arr": [12.3]}"#,
		"12.3"
	),
	(
		foreach06,
		"{{for v in [[1, 2], [3, 4]]}}{{ v[0] }} -- {{ v[1] }},{{done}}",
		"{}",
		"1 -- 2,3 -- 4,"
	),
	(
		foreach07,
		r#"{{for v in [{"name": "foo"}, {"name": "bar" + extra}]}}{{ v.name }},{{done}}"#,
		r#"{"extra": 12}"#,
		"foo,bar12,"
	)
);

// TODO
