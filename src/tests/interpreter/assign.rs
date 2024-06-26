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
		assign_array_item,
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
	),
	(
		assign_array01,
		"{{assign v = [1, 2, 3] }}{{ v[0] }}{{ v[1] }}{{ v[2] }}",
		"{}",
		"123"
	),
	(
		assign_array02,
		r#"{{assign v = [{"name": "foo"}, "this", [3]]}}{{ v[0].name }} -- {{ v[1] }} -- {{ v[2][0] }}"#,
		"{}",
		"foo -- this -- 3"
	),
	(
		assign_object01,
		r#"{{assign v = {"foo": 1, "bar": 2}}}{{ v.foo }} -- {{ v.bar }}"#,
		"{}",
		"1 -- 2"
	),
	(
		assign_object02,
		r#"{{assign v = {"foo": {"bar": 1}, "bar": [2]}}}{{ v.foo.bar }} -- {{ v.bar[0] }}"#,
		"{}",
		"1 -- 2"
	),
	(
		assign_object03,
		r#"{{ assign obj = {"user": {"name": "Marvin"} } }}{{ assign obj.user.name = "R2D2" }}{{ obj.user.name }}"#,
		"{}",
		"R2D2"
	),
	(
		assign_object04,
		r#"{{ assign obj = {"user": {"name":"Marvin"}} }}{{ assign obj.user.name = "R2D2" }}{{ obj.user.name }}"#,
		"{}",
		"R2D2"
	),
	(
		assign_object05,
		r#"{{ assign obj = {"user": {"name":"Marvin"}}}}{{ assign obj.user.name = "R2D2" }}{{ obj.user.name }}"#,
		"{}",
		"R2D2"
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
