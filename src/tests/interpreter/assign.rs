use crate::macro_tests;
use super::{test_interpreter, expect_error};

macro_tests!(
	test_interpreter,
	(
		assign01,
		"{{assign v = 1 }}{{ v }}",
		r#"{}"#,
		"1"
	),
	(
		assign02,
		"{{assign v = 1 }}{{ v }}",
		r#"{"v": 2}"#,
		"1"
	),
	(
		assign_string,
		"{{assign v = \"foo\" }}{{ v }}",
		r#"{}"#,
		"foo"
	),
	(
		assign_int,
		"{{assign v = 1 }}{{ v }}",
		r#"{}"#,
		"1"
	),
	(
		assign_float,
		"{{assign v = 1.1 }}{{ v }}",
		r#"{}"#,
		"1.1"
	),
	(
		assign_bool,
		"{{assign v = true }}{{ v }} -- {{assign v = false }}{{ v }}",
		r#"{}"#,
		"true -- false"
	),
	(
		assign_null,
		"{{assign v = null }}{{ v }}",
		r#"{}"#,
		"null"
	),
	(
		assign_array,
		"{{assign v[2] = 2 }}{{ v[0] }}{{ v[1] }}{{ v[2] }}",
		r#"{"v": [1, 2, 3]}"#,
		"122"
	),
	(
		assign_override01,
		"{{assign v = 1 }}{{v}}",
		r#"{"v": {}}"#,
		"1"
	),
	(
		assign_override02,
		"{{assign v = 1 }}{{v}}",
		r#"{"v": [123, 2, 3]}"#,
		"1"
	),
	(
		assign_variable01,
		"{{assign foo = bar }}{{foo}}",
		r#"{"bar": 2}"#,
		"2"
	),
	(
		assign_variable02,
		"{{assign foo = bar * foo }}{{foo}}",
		r#"{"bar": 2, "foo": 2}"#,
		"4"
	)
);

macro_tests!(
	expect_error,
	(
		invalid_assign01,
		"{{assign v.foo = 1 }}",
		r#"{}"#
	),
	(
		invalid_assign02,
		"{{assign v[1] = 1 }}",
		r#"{}"#
	),
	(
		invalid_assign03,
		"{{assign v[1] = 1 }}",
		r#"{"v": {"h": 2}}"#
	),
	(
		invalid_assign04,
		"{{assign v.foo = 1 }}",
		r#"{"v": [123, 2, 3]}"#
	)
);
