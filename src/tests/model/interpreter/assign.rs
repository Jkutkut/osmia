use super::*;

macro_tests!(
	interpreter_test,
	(
		assign01,
		"{{ v = 1 }}{{ v }}",
		vec![
			(Ctx::try_from(r#"{}"#).unwrap(), Ok("1")),
			(Ctx::try_from(r#"{"v": 2}"#).unwrap(), Ok("1"))
		]
	),
	(
		assign_string,
		"{{ v = \"foo\" }}{{ v }}",
		vec![
			(Ctx::try_from(r#"{}"#).unwrap(), Ok("foo"))
		]
	),
	(
		assign_int,
		"{{ v = 1 }}{{ v }}",
		vec![
			(Ctx::try_from(r#"{}"#).unwrap(), Ok("1"))
		]
	),
	(
		assign_float,
		"{{ v = 1.1 }}{{ v }}",
		vec![
			(Ctx::try_from(r#"{}"#).unwrap(), Ok("1.1"))
		]
	),
	(
		assign_bool,
		"{{ v = true }}{{ v }} -- {{ v = false }}{{ v }}",
		vec![
			(Ctx::try_from(r#"{}"#).unwrap(), Ok("true -- false"))
		]
	),
	(
		assign_null,
		"{{ v = null }}{{ v }}",
		vec![
			(Ctx::try_from(r#"{}"#).unwrap(), Ok("null"))
		]
	),
	(
		assign_array_item,
		"{{ v[2] = 2 }}{{ v[0] }}{{ v[1] }}{{ v[2] }}",
		vec![
			(Ctx::try_from(r#"{"v": [1, 2, 3]}"#).unwrap(), Ok("122"))
		]
	),
	(
		assign_object_item,
		"{{ v.foo = 2 }}{{ v.foo }}",
		vec![
			(Ctx::try_from(r#"{"v": {"foo": 1}}"#).unwrap(), Ok("2")),
			(Ctx::try_from(r#"{"v": {}}"#).unwrap(), Ok("2")),
			(Ctx::try_from(r#"{}"#).unwrap(), Err(vec!["not", "found"]))
		]
	),
	(
		assign_override01,
		"{{ v = 1 }}{{v}}",
		vec![
			(Ctx::try_from(r#"{"v": {}}"#).unwrap(), Ok("1"))
		]
	),
	(
		assign_override02,
		"{{ v = 1 }}{{v}}",
		vec![
			(Ctx::try_from(r#"{"v": [123, 2, 3]}"#).unwrap(), Ok("1"))
		]
	),
	(
		assign_variable01,
		"{{ foo = bar }}{{foo}}",
		vec![
			(Ctx::try_from(r#"{"bar": 2}"#).unwrap(), Ok("2"))
		]
	),
	(
		assign_variable02,
		"{{ foo = bar * foo }}{{foo}}",
		vec![
			(Ctx::try_from(r#"{"bar": 2, "foo": 2}"#).unwrap(), Ok("4"))
		]
	),
	(
		assign_array01,
		"{{ v = [1, 2, 3] }}{{ v[0] }}{{ v[1] }}{{ v[2] }}",
		vec![
			(Ctx::try_from(r#"{}"#).unwrap(), Ok("123"))
		]
	),
	(
		assign_array02,
		r#"{{ v = [{"name": "foo"}, "this", [3]]}}{{ v[0].name }} -- {{ v[1] }} -- {{ v[2][0] }}"#,
		vec![
			(Ctx::try_from("{}").unwrap(), Ok("foo -- this -- 3"))
		]
	),
	(
		assign_array03,
		r#"{{ v = [] }}{{ v[10] = 1 }}{{ v }}"#,
		vec![
			(Ctx::try_from("{}").unwrap(), Ok("[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]"))
		]
	),
	(
		assign_object01,
		r#"{{ v = {"foo": 1, "bar": 2}}}{{ v.foo }} -- {{ v.bar }}"#,
		vec![
			(Ctx::try_from("{}").unwrap(), Ok("1 -- 2"))
		]
	),
	(
		assign_object02,
		r#"{{ v = {"foo": {"bar": 1}, "bar": [2]}}}{{ v.foo.bar }} -- {{ v.bar[0] }}"#,
		vec![
			(Ctx::try_from("{}").unwrap(), Ok("1 -- 2"))
		]
	),
	(
		assign_object03,
		r#"{{  obj = {"user": {"name": "Marvin"} } }}{{  obj.user.name = "R2D2" }}{{ obj.user.name }}"#,
		vec![
			(Ctx::try_from("{}").unwrap(), Ok("R2D2"))
		]
	),
	(
		assign_object04,
		r#"{{  obj = {"user": {"name":"Marvin"}} }}{{  obj.user.name = "R2D2" }}{{ obj.user.name }}"#,
		vec![
			(Ctx::try_from("{}").unwrap(), Ok("R2D2"))
		]
	),
	(
		assign_object05,
		r#"{{  obj = {"user": {"name":"Marvin"}}}}{{  obj.user.name = "R2D2" }}{{ obj.user.name }}"#,
		vec![
			(Ctx::try_from("{}").unwrap(), Ok("R2D2"))
		]
	),

	// Errors
	(
		invalid_assign01,
		"{{ v.foo = 1 }}",
		vec![
			(Ctx::try_from(r#"{}"#).unwrap(), Err(vec!["not", "found"])),
			(Ctx::try_from(r#"{"v": [123, 2, 3]}"#).unwrap(), Err(vec!["key", "array"]))
		]
	),
	(
		invalid_assign02,
		"{{ v[1] = 1 }}{{ v }}",
		vec![
			(Ctx::try_from(r#"{}"#).unwrap(), Err(vec!["not", "found"])),
			(Ctx::try_from(r#"{"v": {"h": 2}}"#).unwrap(), Err(vec!["index", "object"])),
		]
	),
	(
		invalid_assign03,
		"{{ v[1].foo = 1 }}{{ v }}",
		vec![
			(Ctx::try_from(r#"{"v": []}"#).unwrap(), Err(vec!["index", "bounds"])),
		]
	),
	(
		multiple_assingn01,
		"{{ v = 1 }}{{ v = 2 }}{{ v }}",
		vec![
			(Ctx::new(), Ok("2")),
		]
	),
	(
		assign_not_printable,
		"{{ v = 0 }}\n\t{{ v = v + 1 }}\n\t\t{{ v = v + 1 }}\n\t\t\t{{ v = v + 1 }}\n\t\n\t{{ v = v + 1 }}\n{{ v }}",
		vec![
			(Ctx::new(), Ok("4")),
		]
	)
);
