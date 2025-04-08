use super::*;

macro_tests!(
	interpreter_test,
	(
		keys,
		r#"{{ o?keys()?sort() }}"#,
		vec![
			(Ctx::try_from(r#"{ "o": {}}"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "o": {"foo": 1}}"#).unwrap(), Ok(r#"["foo"]"#)),
			(Ctx::try_from(r#"{ "o": {"foo": 1, "bar": 2}}"#).unwrap(), Ok(r#"["bar", "foo"]"#)),
			(Ctx::try_from(r#"{ "o": {"foo": 1, "bar": 2, "baz": 3}}"#).unwrap(), Ok(r#"["bar", "baz", "foo"]"#)),
		]
	),
	(
		values,
		r#"{{ o?values()?sort() }}"#,
		vec![
			(Ctx::try_from(r#"{ "o": {}}"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "o": {"foo": 1}}"#).unwrap(), Ok(r#"[1]"#)),
			(Ctx::try_from(r#"{ "o": {"foo": 1, "bar": 2}}"#).unwrap(), Ok(r#"[1, 2]"#)),
			(Ctx::try_from(r#"{ "o": {"foo": 1, "bar": 2, "baz": 3}}"#).unwrap(), Ok(r#"[1, 2, 3]"#)),
		]
	),
	(
		entries,
		r#"{{ o?entries()?sort() }}"#,
		vec![
			(Ctx::try_from(r#"{ "o": {}}"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "o": {"foo": 1}}"#).unwrap(), Ok(r#"[{"key": "foo", "value": 1}]"#)),
			(Ctx::try_from(r#"{ "o": {"foo": 1, "bar": 2}}"#).unwrap(), Ok(r#"[{"key": "bar", "value": 2}, {"key": "foo", "value": 1}]"#)),
		]
	),
	(
		get,
		r#"{{ o?get(k, "?") }}"#,
		vec![
			(Ctx::try_from(r#"{ "o": {}, "k": "foo"}"#).unwrap(), Ok(r#"?"#)),
			(Ctx::try_from(r#"{ "o": {"foo": 1}, "k": "foo"}"#).unwrap(), Ok(r#"1"#)),
			(Ctx::try_from(r#"{ "o": {"foo": 1, "bar": 2}, "k": "foo"}"#).unwrap(), Ok(r#"1"#)),
			(Ctx::try_from(r#"{ "o": {"foo": 1, "bar": 2}, "k": "baz"}"#).unwrap(), Ok(r#"?"#)),
			(Ctx::try_from(r#"{ "o": {"foo": 1, "bar": 2}, "k": 1}"#).unwrap(), Err(vec!["Invalid", "key"])),
			(Ctx::try_from(r#"{ "o": {"foo": 1, "bar": 2}, "k": null}"#).unwrap(), Err(vec!["Invalid", "key"])),
		]
	)
);
