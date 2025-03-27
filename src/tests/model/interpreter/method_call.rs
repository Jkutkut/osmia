use super::*;

macro_tests!(
	interpreter_test,
	(
		len,
		r#"{{ value?len() }}"#,
		vec![
			(Ctx::try_from(r#"{ "value": "Hello" }"#).unwrap(), Ok("5")),
			(Ctx::try_from(r#"{ "value": [] }"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{ "value": [1, 2] }"#).unwrap(), Ok("2")),
			// TODO more cases
		]
	),
	(
		upper,
		r#"{{ value?upper() }}"#,
		vec![
			(Ctx::try_from(r#"{ "value": "hello" }"#).unwrap(), Ok("HELLO")),
			(Ctx::try_from(r#"{ "value": "Hello" }"#).unwrap(), Ok("HELLO")),
			(Ctx::try_from(r#"{ "value": "heLLo" }"#).unwrap(), Ok("HELLO")),
		]
	),
	(
		lower,
		r#"{{ ("HE" + llo)?lower() }}"#,
		vec![
			(Ctx::try_from(r#"{ "llo": "llo" }"#).unwrap(), Ok("hello")),
			(Ctx::try_from(r#"{ "llo": "lLo" }"#).unwrap(), Ok("hello")),
			(Ctx::try_from(r#"{ "llo": "LLO" }"#).unwrap(), Ok("hello")),
		]
	),
	(
		then,
		r#"{{ value?then("yeah", "nah") }}"#,
		vec![
			(Ctx::try_from(r#"{ "value": true }"#).unwrap(), Ok("yeah")),
			(Ctx::try_from(r#"{ "value": false }"#).unwrap(), Ok("nah")),
		]
	),
	(
		has_content,
		r#"{{ value?has_content() }}"#,
		vec![
			(Ctx::try_from(r#"{ "value": "Hello" }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "value": true }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "value": false }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "value": 0 }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "value": 1 }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "value": -1 }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "value": 0.0 }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "value": 1.0 }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "value": -1.0 }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "value": "" }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "value": null }"#).unwrap(), Ok("false")),
		]
	),
	(
		has_content_lambda,
		r#"{{ (fn() => false)?has_content() }}"#,
		vec![
			(Ctx::new(), Ok("true")),
		]
	),
	(
		has_content_callable,
		r#"{{ math.floor?has_content() }}"#,
		vec![
			(Ctx::new(), Ok("true")),
		]
	),
	(
		trim,
		r#"{{ value?trim() }}"#,
		vec![
			(Ctx::try_from(r#"{ "value": "  Hello  " }"#).unwrap(), Ok("Hello")),
			(Ctx::try_from(r#"{ "value": "\n\nHello\n" }"#).unwrap(), Ok("Hello")),
			(Ctx::try_from(r#"{ "value": "Hello" }"#).unwrap(), Ok("Hello")),
			(Ctx::try_from(r#"{ "value": "  " }"#).unwrap(), Ok("")),
			(Ctx::try_from(r#"{ "value": "  \n\n  " }"#).unwrap(), Ok("")),
			(Ctx::try_from(r#"{ "value": "-\n\n-" }"#).unwrap(), Ok("-\n\n-")),
		]
	)
);
