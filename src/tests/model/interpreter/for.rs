use super::*;

macro_tests!(
	interpreter_test,
	(
		foreach01,
		"{{for v in [1, 2, 3]}}{{ v }}{{done}}",
		vec![
			(Ctx::try_from(r#"{}"#).unwrap(), Ok("123"))
		]
	),
	(
		foreach02,
		"{{for v in [1, 2, 3]}}{{ v }}{{done}}",
		vec![
			(Ctx::try_from(r#"{"v": 3}"#).unwrap(), Ok("123"))
		]
	),
	(
		foreach03,
		r#"{{for v in [true, false, null, "hello world"]}}{{ v }}{{done}}"#,
		vec![
			(Ctx::try_from(r#"{"v": 3}"#).unwrap(), Ok("truefalsenullhello world"))
		]
	),
	(
		foreach04,
		"{{for v in []}}{{ v }}{{done}}",
		vec![
			(Ctx::try_from(r#"{}"#).unwrap(), Ok(""))
		]
	),
	(
		foreach05,
		"{{for v in [1 + v, 2, 3]}}{{ v }}{{done}}",
		vec![
			(Ctx::try_from(r#"{"v": 3}"#).unwrap(), Ok("423"))
		]
	),
	(
		foreachvariable01,
		"{{for v in arr}}{{ v }}{{done}}",
		vec![
			(Ctx::try_from(r#"{"arr": [1, 2, 3]}"#).unwrap(), Ok("123"))
		]
	),
	(
		foreachvariable02,
		"{{for v in arr}} {{ v }}{{done}}",
		vec![
			(Ctx::try_from(r#"{"arr": [true, "2", null]}"#).unwrap(), Ok(" true 2 null"))
		]
	),
	(
		foreachvariable03,
		"{{for v in arr}}{{ v }}{{done}}",
		vec![
			(Ctx::try_from(r#"{"arr": []}"#).unwrap(), Ok(""))
		]
	),
	(
		foreachvariable04,
		"{{for v in arr}}{{ v }}{{done}}",
		vec![
			(Ctx::try_from(r#"{"arr": [12.3]}"#).unwrap(), Ok("12.3"))
		]
	),
	(
		foreach06,
		"{{for v in [[1, 2], [3, 4]]}}{{ v[0] }} -- {{ v[1] }},{{done}}",
		vec![
			(Ctx::try_from("{}").unwrap(), Ok("1 -- 2,3 -- 4,"))
		]
	),
	(
		foreach07,
		r#"{{for v in [{"name": "foo"}, {"name": "bar" + extra}]}}{{ v.name }},{{done}}"#,
		vec![
			(Ctx::try_from(r#"{"extra": 12}"#).unwrap(), Ok("foo,bar12,"))
		]
	)
);

macro_tests!(
	interpreter_test,
	(
		foreach_dict01,
		r#"{{for v in {"name": "foo", "extra": 12}}}{{ v.key }}:{{ v.value }},{{done}}"#,
		vec![
			(Ctx::try_from(r#"{}"#).unwrap(), Ok("extra:12,name:foo,")),
			(Ctx::try_from(r#"{"v": {"key": "value"}}"#).unwrap(), Ok("extra:12,name:foo,")),
		]
	),
	(
		foreach_dict02,
		r#"{{for v in {}}}{{ v.key }}:{{ v.value }},{{done}}"#,
		vec![
			(Ctx::try_from(r#"{}"#).unwrap(), Ok("")),
			(Ctx::try_from(r#"{"v": {"key": "value"}}"#).unwrap(), Ok("")),
		]
	)
);
