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
	),
	(
		capitalize,
		r#"{{ value?capitalize() }}"#,
		vec![
			(Ctx::try_from(r#"{ "value": "" }"#).unwrap(), Ok("")),
			(Ctx::try_from(r#"{ "value": "hello" }"#).unwrap(), Ok("Hello")),
			(Ctx::try_from(r#"{ "value": "Hello" }"#).unwrap(), Ok("Hello")),
			(Ctx::try_from(r#"{ "value": "heLLo" }"#).unwrap(), Ok("HeLLo")),
			(Ctx::try_from(r#"{ "value": "hElLo world! this is a test" }"#).unwrap(), Ok("HElLo World! This Is A Test")),
			(Ctx::try_from(r#"{ "value": "FJASLKFJLSA" }"#).unwrap(), Ok("FJASLKFJLSA")),
			(Ctx::try_from(r#"{ "value": "a b c d E F G H I" }"#).unwrap(), Ok("A B C D E F G H I")),
			(Ctx::try_from(r#"{ "value": "!A!a.e.I.O.U" }"#).unwrap(), Ok("!A!A.E.I.O.U")),
			(Ctx::try_from(r#"{ "value": null }"#).unwrap(), Err(vec!["not", "capitalize"])),
			(Ctx::try_from(r#"{ "value": true }"#).unwrap(), Err(vec!["not", "capitalize"])),
			(Ctx::try_from(r#"{ "value": false }"#).unwrap(), Err(vec!["not", "capitalize"])),
			(Ctx::try_from(r#"{ "value": 0 }"#).unwrap(), Err(vec!["not", "capitalize"])),
			(Ctx::try_from(r#"{ "value": 1.1 }"#).unwrap(), Err(vec!["not", "capitalize"])),
		]
	),
	(
		starts_with,
		r#"{{ s?starts_with(t) }}"#,
		vec![
			(Ctx::try_from(r#"{ "s": "hello", "t": "h" }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "s": "hello", "t": "H" }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "s": "hey", "t": "hey" }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "s": "hello world", "t": "hello" }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "s": "hello", "t": "Hello" }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "s": "hello", "t": "hellO" }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "s": "hello", "t": "ell" }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "s": "hello", "t": "lo" }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "s": "hello", "t": "" }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "s": "h", "t": "" }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "s": "", "t": "" }"#).unwrap(), Ok("true")),
		]
	),
	(
		ends_with,
		r#"{{ s?ends_with(t) }}"#,
		vec![
			(Ctx::try_from(r#"{ "s": "hello", "t": "O" }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "s": "hello", "t": "o" }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "s": "hey", "t": "hey" }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "s": "hello world", "t": "hello" }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "s": "hello", "t": "Hello" }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "s": "hello", "t": "hellO" }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "s": "hello", "t": "ell" }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "s": "hello", "t": "lo" }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "s": "hello", "t": "" }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "s": "h", "t": "" }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "s": "", "t": "" }"#).unwrap(), Ok("true")),
		]
	),
	(
		ensure_starts_with,
		r#"{{ s?ensure_starts_with(t) }}"#,
		vec![
			(Ctx::try_from(r#"{ "s": "ello", "t": "H" }"#).unwrap(), Ok("Hello")),
			(Ctx::try_from(r#"{ "s": "Hello", "t": "H" }"#).unwrap(), Ok("Hello")),
			(Ctx::try_from(r#"{ "s": "https://example.com", "t": "https://" }"#).unwrap(), Ok("https://example.com")),
			(Ctx::try_from(r#"{ "s": "example.com", "t": "https://" }"#).unwrap(), Ok("https://example.com")),
			(Ctx::try_from(r#"{ "s": "Hel", "t": "Hello" }"#).unwrap(), Ok("HelloHel")),
			(Ctx::try_from(r#"{ "s": "", "t": "" }"#).unwrap(), Ok("")),
			(Ctx::try_from(r#"{ "s": ".", "t": "" }"#).unwrap(), Ok(".")),
			(Ctx::try_from(r#"{ "s": "", "t": "foo" }"#).unwrap(), Ok("foo")),
		]
	),
	(
		ensure_ends_with,
		r#"{{ s?ensure_ends_with(t) }}"#,
		vec![
			(Ctx::try_from(r#"{ "s": "Hello", "t": "o" }"#).unwrap(), Ok("Hello")),
			(Ctx::try_from(r#"{ "s": "Hell", "t": "o" }"#).unwrap(), Ok("Hello")),
			(Ctx::try_from(r#"{ "s": "Hellooo", "t": "o" }"#).unwrap(), Ok("Hellooo")),
			(Ctx::try_from(r#"{ "s": "robots.txt", "t": ".txt" }"#).unwrap(), Ok("robots.txt")),
			(Ctx::try_from(r#"{ "s": "robots", "t": ".txt" }"#).unwrap(), Ok("robots.txt")),
			(Ctx::try_from(r#"{ "s": "", "t": "" }"#).unwrap(), Ok("")),
			(Ctx::try_from(r#"{ "s": ".", "t": "" }"#).unwrap(), Ok(".")),
			(Ctx::try_from(r#"{ "s": "", "t": "foo" }"#).unwrap(), Ok("foo")),
		]
	),
	(
		index_of,
		r#"{{ s?index_of(t) }}"#,
		vec![
			(Ctx::try_from(r#"{ "s": "hello", "t": "h" }"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{ "s": "hello 2", "t": "hello" }"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{ "s": "hello 3", "t": "l" }"#).unwrap(), Ok("2")),
			(Ctx::try_from(r#"{ "s": "hello 4", "t": "x" }"#).unwrap(), Ok("-1")),
			(Ctx::try_from(r#"{ "s": "hello 5", "t": "" }"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{ "s": "", "t": "" }"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{ "s": "hello hello", "t": "hello" }"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{ "s": "ello hell hello hello", "t": "hello" }"#).unwrap(), Ok("10")),
		]
	),
	(
		last_index_of,
		r#"{{ s?last_index_of(t) }}"#,
		vec![
			(Ctx::try_from(r#"{ "s": "hello", "t": "h" }"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{ "s": "hello 2", "t": "hello" }"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{ "s": "hello 3", "t": "l" }"#).unwrap(), Ok("3")),
			(Ctx::try_from(r#"{ "s": "hello 4", "t": "x" }"#).unwrap(), Ok("-1")),
			(Ctx::try_from(r#"{ "s": "hello 5", "t": "" }"#).unwrap(), Ok("7")),
			(Ctx::try_from(r#"{ "s": "", "t": "" }"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{ "s": "hello hello", "t": "hello" }"#).unwrap(), Ok("6")),
			(Ctx::try_from(r#"{ "s": "ello hell hello hello", "t": "hello" }"#).unwrap(), Ok("16")),
		]
	)
);
