use super::*;

macro_tests!(
	interpreter_test,
	(
		sort,
		r#"{{ a?sort() }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b"] }"#).unwrap(), Ok(r#"["a", "b"]"#)),
			(Ctx::try_from(r#"{ "a": ["b", "a"]}"#).unwrap(), Ok(r#"["a", "b"]"#)),
			(Ctx::try_from(r#"{ "a": [3, 2, 1] }"#).unwrap(), Ok(r#"[1, 2, 3]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2, 3] }"#).unwrap(), Ok(r#"[1, 2, 3]"#)),
			(Ctx::try_from(r#"{ "a": [1, 3, 2] }"#).unwrap(), Ok(r#"[1, 2, 3]"#)),
			(Ctx::try_from(r#"{ "a": [true, false, null, 2, 2.3, "str"] }"#).unwrap(), Ok(r#"[false, true, null, 2, 2.3, "str"]"#)),
			(Ctx::try_from(r#"{ "a": ["a", true, false, null, 2, 2.3, "str"] }"#).unwrap(), Ok(r#"[false, "a", true, null, 2, 2.3, "str"]"#)),
		]
	),
	(
		sort_by,
		r#"{{ a?sort_by(fn (a, b) => a - b) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": [2, 1] }"#).unwrap(), Ok(r#"[1, 2]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2] }"#).unwrap(), Ok(r#"[1, 2]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2, 4, 3] }"#).unwrap(), Ok(r#"[1, 2, 3, 4]"#)),
		]
	),
	(
		sort_by02,
		r#"{{ a?sort_by(fn (a, b) => b - a) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": [2, 1] }"#).unwrap(), Ok(r#"[2, 1]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2] }"#).unwrap(), Ok(r#"[2, 1]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2, 4, 3] }"#).unwrap(), Ok(r#"[4, 3, 2, 1]"#)),
			(Ctx::try_from(r#"{ "a": [null, 2] }"#).unwrap(), Ok(r#"[null, 2]"#)),
		]
	),
	(
		// Sort alphabetically except if the first character is 'c'
		sort_by03,
		r#"{{ a?sort_by(fn (a, b) => (a?starts_with("c"))?then(
			(b?starts_with("c"))?then((a < b)?then(1, -1), 1),
			(b?starts_with("c"))?then(-1, (b < a)?then(1, -1))
		)) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b"] }"#).unwrap(), Ok(r#"["a", "b"]"#)),
			(Ctx::try_from(r#"{ "a": ["b", "a"]}"#).unwrap(), Ok(r#"["a", "b"]"#)),
			(Ctx::try_from(r#"{ "a": ["c", "a", "b"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "c", "b"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["c", "b", "a"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["b", "c", "a"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["hey", "call", "foo"] }"#).unwrap(), Ok(r#"["foo", "hey", "call"]"#)),
		]
	),
	(
		sort_by04,
		r#"{{ a?sort_by(fn (a, b) => a < b) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": [2, 1] }"#).unwrap(), Ok(r#"[1, 2]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2] }"#).unwrap(), Ok(r#"[1, 2]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2, 4, 3] }"#).unwrap(), Ok(r#"[1, 2, 3, 4]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b"] }"#).unwrap(), Ok(r#"["a", "b"]"#)),
			(Ctx::try_from(r#"{ "a": ["b", "a"]}"#).unwrap(), Ok(r#"["a", "b"]"#)),
			(Ctx::try_from(r#"{ "a": ["c", "a", "b"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "c", "b"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["c", "b", "a"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["b", "c", "a"] }"#).unwrap(), Ok(r#"["a", "b", "c"]"#)),
			(Ctx::try_from(r#"{ "a": ["hey", "call", "foo"] }"#).unwrap(), Ok(r#"["call", "foo", "hey"]"#)),
		]
	),
	(
		sort_by05,
		r#"{{ a?sort_by(fn (a, b) => a > b) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": [] }"#).unwrap(), Ok(r#"[]"#)),
			(Ctx::try_from(r#"{ "a": [2, 1] }"#).unwrap(), Ok(r#"[2, 1]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2] }"#).unwrap(), Ok(r#"[2, 1]"#)),
			(Ctx::try_from(r#"{ "a": [1, 2, 4, 3] }"#).unwrap(), Ok(r#"[4, 3, 2, 1]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b"] }"#).unwrap(), Ok(r#"["b", "a"]"#)),
			(Ctx::try_from(r#"{ "a": ["b", "a"]}"#).unwrap(), Ok(r#"["b", "a"]"#)),
			(Ctx::try_from(r#"{ "a": ["c", "a", "b"] }"#).unwrap(), Ok(r#"["c", "b", "a"]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "c", "b"] }"#).unwrap(), Ok(r#"["c", "b", "a"]"#)),
			(Ctx::try_from(r#"{ "a": ["a", "b", "c"] }"#).unwrap(), Ok(r#"["c", "b", "a"]"#)),
			(Ctx::try_from(r#"{ "a": ["c", "b", "a"] }"#).unwrap(), Ok(r#"["c", "b", "a"]"#)),
			(Ctx::try_from(r#"{ "a": ["b", "c", "a"] }"#).unwrap(), Ok(r#"["c", "b", "a"]"#)),
			(Ctx::try_from(r#"{ "a": ["hey", "call", "foo"] }"#).unwrap(), Ok(r#"["hey", "foo", "call"]"#)),
		]
	),
);
