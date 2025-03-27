use super::*;

macro_tests!(
	interpreter_test,
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
