use super::*;

macro_tests!(
	interpreter_test,
	(
		sort,
		r#"{{ a?sort() }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": ["b", "a"]}"#).unwrap(), Ok(r#"["a", "b"]"#)),
			(Ctx::try_from(r#"{ "a": [3, 2, 1] }"#).unwrap(), Ok(r#"[1, 2, 3]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2, 3] }"#).unwrap(), Ok(r#"[1, 2, 3]"#)),
			(Ctx::try_from(r#"{ "a": [1, 3, 2] }"#).unwrap(), Ok(r#"[1, 2, 3]"#)),
			// TODO
		]
	)
);
