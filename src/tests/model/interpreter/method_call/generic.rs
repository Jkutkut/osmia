use super::*;

macro_tests!(
	interpreter_test,
	(
		len,
		r#"{{ value?len() }}"#,
		vec![
			(Ctx::try_from(r#"{ "value": "Hello" }"#).unwrap(), Ok("5")),
			(Ctx::try_from(r#"{ "value": "" }"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{ "value": [] }"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{ "value": [1, 2] }"#).unwrap(), Ok("2")),
			(Ctx::try_from(r#"{ "value": true }"#).unwrap(), Err(vec!["len"])),
			(Ctx::try_from(r#"{ "value": null }"#).unwrap(), Err(vec!["len"])),
			(Ctx::try_from(r#"{ "value": 0 }"#).unwrap(), Err(vec!["len"])),
			(Ctx::try_from(r#"{ "value": 1.1 }"#).unwrap(), Err(vec!["len"])),
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
		to_bool,
		r#"{{ value?to_bool() }}"#,
		vec![
			(Ctx::try_from(r#"{ "value": "Hello" }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "value": true }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "value": false }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "value": 0 }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "value": 1 }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "value": -1 }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "value": 0.0 }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "value": 1.0 }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "value": -1.0 }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "value": "" }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "value": null }"#).unwrap(), Ok("false")),
		]
	),
	(
		to_float,
		r#"{{ value?to_float() }}"#,
		vec![
			(Ctx::try_from(r#"{ "value": 0 }"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{ "value": 1 }"#).unwrap(), Ok("1")),
			(Ctx::try_from(r#"{ "value": 1.0 }"#).unwrap(), Ok("1")),
			(Ctx::try_from(r#"{ "value": -1.0 }"#).unwrap(), Ok("-1")),
			(Ctx::try_from(r#"{ "value": "0.00" }"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{ "value": "1.00" }"#).unwrap(), Ok("1")),
			(Ctx::try_from(r#"{ "value": "-1.00" }"#).unwrap(), Ok("-1")),
			(Ctx::try_from(r#"{ "value": "-1.01" }"#).unwrap(), Ok("-1.01")),
			(Ctx::try_from(r#"{ "value": "1.01" }"#).unwrap(), Ok("1.01")),
			(Ctx::try_from(r#"{ "value": null }"#).unwrap(), Err(vec!["null", "float"])),
			(Ctx::try_from(r#"{ "value": "" }"#).unwrap(), Err(vec!["\"\"", "float"])),
			(Ctx::try_from(r#"{ "value": "Hello" }"#).unwrap(), Err(vec!["Hello", "float"])),
			(Ctx::try_from(r#"{ "value": true }"#).unwrap(), Err(vec!["true", "float"])),
			(Ctx::try_from(r#"{ "value": false }"#).unwrap(), Err(vec!["false", "float"])),
		]
	),
	(
		to_int,
		r#"{{ value?to_int() }}"#,
		vec![
			(Ctx::try_from(r#"{ "value": 0 }"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{ "value": 1 }"#).unwrap(), Ok("1")),
			(Ctx::try_from(r#"{ "value": 1.0 }"#).unwrap(), Ok("1")),
			(Ctx::try_from(r#"{ "value": -1.0 }"#).unwrap(), Ok("-1")),
			(Ctx::try_from(r#"{ "value": "0" }"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{ "value": "1" }"#).unwrap(), Ok("1")),
			(Ctx::try_from(r#"{ "value": "-1" }"#).unwrap(), Ok("-1")),
			(Ctx::try_from(r#"{ "value": "-1.01" }"#).unwrap(), Err(vec!["-1.01", "int"])),
			(Ctx::try_from(r#"{ "value": "1.01" }"#).unwrap(), Err(vec!["1.01", "int"])),
			(Ctx::try_from(r#"{ "value": null }"#).unwrap(), Err(vec!["null", "int"])),
			(Ctx::try_from(r#"{ "value": "" }"#).unwrap(), Err(vec!["\"\"", "int"])),
			(Ctx::try_from(r#"{ "value": "Hello" }"#).unwrap(), Err(vec!["Hello", "int"])),
			(Ctx::try_from(r#"{ "value": true }"#).unwrap(), Err(vec!["true", "int"])),
			(Ctx::try_from(r#"{ "value": false }"#).unwrap(), Err(vec!["false", "int"])),
		]
	),
	(
		to_string,
		r#"{{ value?to_string() }}"#,
		vec![
			(Ctx::try_from(r#"{ "value": 0 }"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{ "value": 1 }"#).unwrap(), Ok("1")),
			(Ctx::try_from(r#"{ "value": 1.0 }"#).unwrap(), Ok("1")),
			(Ctx::try_from(r#"{ "value": -1.0 }"#).unwrap(), Ok("-1")),
			(Ctx::try_from(r#"{ "value": "0" }"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{ "value": "1" }"#).unwrap(), Ok("1")),
			(Ctx::try_from(r#"{ "value": "-1" }"#).unwrap(), Ok("-1")),
			(Ctx::try_from(r#"{ "value": "-1.01" }"#).unwrap(), Ok("-1.01")),
			(Ctx::try_from(r#"{ "value": "1.01" }"#).unwrap(), Ok("1.01")),
			(Ctx::try_from(r#"{ "value": null }"#).unwrap(), Ok("null")),
			(Ctx::try_from(r#"{ "value": "" }"#).unwrap(), Ok("")),
			(Ctx::try_from(r#"{ "value": "Hello" }"#).unwrap(), Ok("Hello")),
			(Ctx::try_from(r#"{ "value": true }"#).unwrap(), Ok("true")),
			(Ctx::try_from(r#"{ "value": false }"#).unwrap(), Ok("false")),
			(Ctx::try_from(r#"{ "value": [] }"#).unwrap(), Ok("[]")),
			(Ctx::try_from(r#"{ "value": [0] }"#).unwrap(), Ok("[0]")),
			(Ctx::try_from(r#"{ "value": [0, 1] }"#).unwrap(), Ok("[0, 1]")),
			(Ctx::try_from(r#"{ "value": {} }"#).unwrap(), Ok("{}")),
			(Ctx::try_from(r#"{ "value": { "a": 0 } }"#).unwrap(), Ok("{\"a\": 0}")),
			(Ctx::try_from(r#"{ "value": { "a": 0, "b": 1 } }"#).unwrap(), Ok("{\"a\": 0, \"b\": 1}")),
		]
	),
	(
		r#type,
		r#"{{ value?type() }}"#,
		vec![
			(Ctx::try_from(r#"{ "value": 0 }"#).unwrap(), Ok("int")),
			(Ctx::try_from(r#"{ "value": 1 }"#).unwrap(), Ok("int")),
			(Ctx::try_from(r#"{ "value": 1.0 }"#).unwrap(), Ok("float")),
			(Ctx::try_from(r#"{ "value": -1.0 }"#).unwrap(), Ok("float")),
			(Ctx::try_from(r#"{ "value": "0" }"#).unwrap(), Ok("string")),
			(Ctx::try_from(r#"{ "value": "1" }"#).unwrap(), Ok("string")),
			(Ctx::try_from(r#"{ "value": "-1" }"#).unwrap(), Ok("string")),
			(Ctx::try_from(r#"{ "value": "-1.01" }"#).unwrap(), Ok("string")),
			(Ctx::try_from(r#"{ "value": "1.01" }"#).unwrap(), Ok("string")),
			(Ctx::try_from(r#"{ "value": null }"#).unwrap(), Ok("null")),
			(Ctx::try_from(r#"{ "value": "" }"#).unwrap(), Ok("string")),
			(Ctx::try_from(r#"{ "value": "Hello" }"#).unwrap(), Ok("string")),
			(Ctx::try_from(r#"{ "value": true }"#).unwrap(), Ok("bool")),
			(Ctx::try_from(r#"{ "value": false }"#).unwrap(), Ok("bool")),
		]
	),
	(
		type_lambda,
		r#"{{ (fn () => true)?type() }}"#,
		vec![
			(Ctx::new(), Ok("lambda")),
		]
	),
	(
		type_ft,
		r#"{{ math.abs?type() }}"#,
		vec![
			(Ctx::new(), Ok("function")),
		]
	),
	(
		switch,
		r#"{{ value?switch(true, "foo") }}"#,
		vec![
			(Ctx::try_from(r#"{ "value": true }"#).unwrap(), Ok("foo")),
			(Ctx::try_from(r#"{ "value": false }"#).unwrap(), Ok("null")),
		]
	),
	(
		switch_none,
		r#"{{ value?switch() }}"#,
		vec![
			(Ctx::try_from(r#"{ "value": true }"#).unwrap(), Ok("null")),
			(Ctx::try_from(r#"{ "value": false }"#).unwrap(), Ok("null")),
		]
	),
	(
		switch_default,
		r#"{{ value?switch(true, "foo", 2) }}"#,
		vec![
			(Ctx::try_from(r#"{ "value": true }"#).unwrap(), Ok("foo")),
			(Ctx::try_from(r#"{ "value": false }"#).unwrap(), Ok("2")),
		]
	),
	(
		switch_multiple,
		r#"{{ value?switch(1, "foo", 1.2, "bar") }}"#,
		vec![
			(Ctx::try_from(r#"{ "value": 1 }"#).unwrap(), Ok("foo")),
			(Ctx::try_from(r#"{ "value": 1.2 }"#).unwrap(), Ok("bar")),
			(Ctx::try_from(r#"{ "value": 2 }"#).unwrap(), Ok("null")),
		]
	),
	(
		switch_multiple_default,
		r#"{{ value?switch(1, "foo", 1.2, "bar", 2) }}"#,
		vec![
			(Ctx::try_from(r#"{ "value": 1 }"#).unwrap(), Ok("foo")),
			(Ctx::try_from(r#"{ "value": 1.2 }"#).unwrap(), Ok("bar")),
			(Ctx::try_from(r#"{ "value": 2 }"#).unwrap(), Ok("2")),
		]
	),
	(
		switch_type,
		r#"{{ value?type()?switch(
			"bool", "boolean!",
			"int", "int!",
			"float", "float!",
			"string", "string!",
			"null", "null!",
			"array", "array!",
			"object", "object!"
		) }}"#,
		vec![
			(Ctx::try_from(r#"{ "value": true }"#).unwrap(), Ok("boolean!")),
			(Ctx::try_from(r#"{ "value": false }"#).unwrap(), Ok("boolean!")),
			(Ctx::try_from(r#"{ "value": 0 }"#).unwrap(), Ok("int!")),
			(Ctx::try_from(r#"{ "value": 0.0 }"#).unwrap(), Ok("float!")),
			(Ctx::try_from(r#"{ "value": "foo" }"#).unwrap(), Ok("string!")),
			(Ctx::try_from(r#"{ "value": null }"#).unwrap(), Ok("null!")),
			(Ctx::try_from(r#"{ "value": [] }"#).unwrap(), Ok("array!")),
			(Ctx::try_from(r#"{ "value": {} }"#).unwrap(), Ok("object!")),
		]
	)
);
