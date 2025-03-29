use super::*;

macro_tests!(
	interpreter_test,
	(
		not,
		r#"{{ b?not() }}"#,
		vec![
			(Ctx::try_from(r#"{ "b": true }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "b": false }"#).unwrap(), Ok("true")),
		]
	),
	(
		and,
		r#"{{ a?and(b) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": true, "b": true }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "a": true, "b": false }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "a": false, "b": true }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "a": false, "b": false }"#).unwrap(), Ok("false")),
		]
	),
	(
		or,
		r#"{{ a?or(b) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": true, "b": true }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "a": true, "b": false }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "a": false, "b": true }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "a": false, "b": false }"#).unwrap(), Ok("false")),
		]
	),
	(
		nand,
		r#"{{ a?nand(b) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": true, "b": true }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "a": true, "b": false }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "a": false, "b": true }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "a": false, "b": false }"#).unwrap(), Ok("true")),
		]
	),
	(
		nor,
		r#"{{ a?nor(b) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": true, "b": true }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "a": true, "b": false }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "a": false, "b": true }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "a": false, "b": false }"#).unwrap(), Ok("true")),
		]
	),
	(
		xor,
		r#"{{ a?xor(b) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": true, "b": true }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "a": true, "b": false }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "a": false, "b": true }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "a": false, "b": false }"#).unwrap(), Ok("false")),
		]
	),
	(
		xnor,
		r#"{{ a?xnor(b) }}"#,
		vec![
			(Ctx::try_from(r#"{ "a": true, "b": true }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "a": true, "b": false }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "a": false, "b": true }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "a": false, "b": false }"#).unwrap(), Ok("true")),
		]
	)
);
