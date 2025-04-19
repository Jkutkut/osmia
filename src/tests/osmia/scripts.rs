use super::*;

macro_tests!(
	test,
	(
		identity,
		Some("Hello, world!"),
		Some(vec![
			Token::new_raw("Hello, world!"),
			Token::Eof
		]),
		Some(Stmt::new_raw("Hello, world!")),
		Some(vec![
			(Ctx::new(), Ok("Hello, world!")),
		])
	),
	(
		basic_test01,
		Some("Hello, {{name}}!"),
		Some(vec![
			Token::new_raw("Hello, "),
			Token::StmtStart,
			Token::new_alpha("name"),
			Token::StmtEnd,
			Token::new_raw("!"),
			Token::Eof
		]),
		None,
		Some(vec![
			(Ctx::try_from(r#"{ "name": "world" }"#).unwrap(), Ok("Hello, world!")),
		])
	),
	(
		basic_test02,
		Some("Hello, {{name}}! A number {{number}}, a boolean {{boolean}} an a null {{null}}."),
		Some(vec![
			Token::new_raw("Hello, "),
			Token::StmtStart,
			Token::new_alpha("name"),
			Token::StmtEnd,
			Token::new_raw("! A number "),
			Token::StmtStart,
			Token::new_alpha("number"),
			Token::StmtEnd,
			Token::new_raw(", a boolean "),
			Token::StmtStart,
			Token::new_alpha("boolean"),
			Token::StmtEnd,
			Token::new_raw(" an a null "),
			Token::StmtStart,
			Token::Null,
			Token::StmtEnd,
			Token::new_raw("."),
			Token::Eof
		]),
		None,
		Some(vec![
			(Ctx::try_from(r#"
				{
					"name": "world",
					"number": 42,
					"boolean": true,
					"null": null
				}
			"#).unwrap(), Ok("Hello, world! A number 42, a boolean true an a null null.")),
		])
	),
	(
		basic_test03,
		Some("Hello, {{name}}! Single value from lst {{lst[1]}} and an object element {{object.obj.key}}."),
		Some(vec![
			Token::new_raw("Hello, "),
			Token::StmtStart,
			Token::new_alpha("name"),
			Token::StmtEnd,
			Token::new_raw("! Single value from lst "),
			Token::StmtStart,
			Token::new_alpha("lst"),
			Token::ArrayStart,
			Token::new_number("1"),
			Token::ArrayEnd,
			Token::StmtEnd,
			Token::new_raw(" and an object element "),
			Token::StmtStart,
			Token::new_alpha("object"),
			Token::Dot,
			Token::new_alpha("obj"),
			Token::Dot,
			Token::new_alpha("key"),
			Token::StmtEnd,
			Token::new_raw("."),
			Token::Eof
		]),
		None,
		Some(vec![
			(Ctx::try_from(r#"
				{
					"name": "world",
					"lst": [11, 22, 33],
					"object": {
						"key": "value",
						"obj": {
							"key": "value2"
						}
					}
				}
			"#).unwrap(), Ok("Hello, world! Single value from lst 22 and an object element value2.")),
		])
	),
	(
		similar_syntax,
		Some(r#"Hello, {{name}}! The syntax is "Hello, {{"{{name}}!"}}"."#),
		Some(vec![
			Token::new_raw("Hello, "),
			Token::StmtStart,
			Token::new_alpha("name"),
			Token::StmtEnd,
			Token::new_raw("! The syntax is \"Hello, "),
			Token::StmtStart,
			Token::new_str("{{name}}!"),
			Token::StmtEnd,
			Token::new_raw("\"."),
			Token::Eof
		]),
		None,
		Some(vec![
			(Ctx::try_from(r#"{ "name": "world" }"#).unwrap(), Ok(r#"Hello, world! The syntax is "Hello, {{name}}!"."#)),
		])
	),
	(
		comment,
		Some(r#"Hello!{{print "This is a comment"}}"#),
		Some(vec![
			Token::new_raw("Hello!"),
			Token::StmtStart,
			Token::Print,
			Token::Whitespace,
			Token::new_str("This is a comment"),
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		Some(vec![
			(Ctx::new(), Ok("Hello!")),
		])
	),
	(
		complex_value01,
		Some("Hello, {{name.first}} {{name.last}}!"),
		Some(vec![
			Token::new_raw("Hello, "),
			Token::StmtStart,
			Token::new_alpha("name"),
			Token::Dot,
			Token::new_alpha("first"),
			Token::StmtEnd,
			Token::new_raw(" "),
			Token::StmtStart,
			Token::new_alpha("name"),
			Token::Dot,
			Token::new_alpha("last"),
			Token::StmtEnd,
			Token::new_raw("!"),
			Token::Eof
		]),
		None,
		Some(vec![
			(Ctx::try_from(r#"
				{
					"name": {
						"first": "John",
						"last": "Doe"
					}
				}
			"#).unwrap(), Ok("Hello, John Doe!")),
		])
	),
	(
		complex_value02,
		Some("This is a complex value: {{obj.obj[0].obj}}"),
		Some(vec![
			Token::new_raw("This is a complex value: "),
			Token::StmtStart,
			Token::new_alpha("obj"),
			Token::Dot,
			Token::new_alpha("obj"),
			Token::ArrayStart,
			Token::new_number("0"),
			Token::ArrayEnd,
			Token::Dot,
			Token::new_alpha("obj"),
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		Some(vec![
			(Ctx::try_from(r#"
				{
					"obj": {
						"obj": [
							{
								"obj": "value"
							}
						]
					}
				}
			"#).unwrap(), Ok("This is a complex value: value")),
		])
	),
	(
		foreach01,
		Some(r#"Hello, {{name}}!
		{{for item in items}}
		<a href="{{item.url}}">{{item.name}}</a>
		{{done}}"#),
		Some(vec![
			Token::new_raw("Hello, "),
			Token::StmtStart,
			Token::new_alpha("name"),
			Token::StmtEnd,
			Token::new_raw("!"),
			Token::NewLine,
			Token::new_non_printable("\t\t"),
			Token::StmtStart,
			Token::For,
			Token::Whitespace,
			Token::new_alpha("item"),
			Token::Whitespace,
			Token::In,
			Token::Whitespace,
			Token::new_alpha("items"),
			Token::StmtEnd,
			Token::NewLineNonPrintable,
			Token::new_raw("\t\t<a href=\""),
			Token::StmtStart,
			Token::new_alpha("item"),
			Token::Dot,
			Token::new_alpha("url"),
			Token::StmtEnd,
			Token::new_raw("\">"),
			Token::StmtStart,
			Token::new_alpha("item"),
			Token::Dot,
			Token::new_alpha("name"),
			Token::StmtEnd,
			Token::new_raw("</a>"),
			Token::NewLine,
			Token::new_non_printable("\t\t"),
			Token::StmtStart,
			Token::Done,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		Some(vec![
			(Ctx::try_from(r#"
				{
					"name": "world",
					"items": [
						{
							"url": "https://example01.com",
							"name": "Example01"
						},
						{
							"url": "https://example02.org",
							"name": "Example02"
						}
					]
				}
			"#).unwrap(),
			Ok("Hello, world!\n\t\t<a href=\"https://example01.com\">Example01</a>\n\t\t<a href=\"https://example02.org\">Example02</a>\n"))
		])
	),
	(
		foreach02,
		Some(r#"Hello, {{name}}!
		{{for item in items}}
		<li>Element {{item}}</li>
		{{done}}"#),
		Some(vec![
			Token::new_raw("Hello, "),
			Token::StmtStart,
			Token::new_alpha("name"),
			Token::StmtEnd,
			Token::new_raw("!"),
			Token::NewLine,
			Token::new_non_printable("\t\t"),
			Token::StmtStart,
			Token::For,
			Token::Whitespace,
			Token::new_alpha("item"),
			Token::Whitespace,
			Token::In,
			Token::Whitespace,
			Token::new_alpha("items"),
			Token::StmtEnd,
			Token::NewLineNonPrintable,
			Token::new_raw("\t\t<li>Element "),
			Token::StmtStart,
			Token::new_alpha("item"),
			Token::StmtEnd,
			Token::new_raw("</li>"),
			Token::NewLine,
			Token::new_non_printable("\t\t"),
			Token::StmtStart,
			Token::Done,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		Some(vec![
			(Ctx::try_from(r#"
				{
					"name": "world",
					"items": [1, 2, 3]
				}
			"#).unwrap(),
			Ok("Hello, world!\n\t\t<li>Element 1</li>\n\t\t<li>Element 2</li>\n\t\t<li>Element 3</li>\n"))
		])
	),
	(
		foreach03,
		Some(r#"{{for item in items}}<ul>{{for i in item.arr}}<li>{{i}}</li>{{done}}</ul>{{done}}"#),
		Some(vec![
			Token::StmtStart,
			Token::For,
			Token::Whitespace,
			Token::new_alpha("item"),
			Token::Whitespace,
			Token::In,
			Token::Whitespace,
			Token::new_alpha("items"),
			Token::StmtEnd,
			Token::new_raw("<ul>"),
			Token::StmtStart,
			Token::For,
			Token::Whitespace,
			Token::new_alpha("i"),
			Token::Whitespace,
			Token::In,
			Token::Whitespace,
			Token::new_alpha("item"),
			Token::Dot,
			Token::new_alpha("arr"),
			Token::StmtEnd,
			Token::new_raw("<li>"),
			Token::StmtStart,
			Token::new_alpha("i"),
			Token::StmtEnd,
			Token::new_raw("</li>"),
			Token::StmtStart,
			Token::Done,
			Token::StmtEnd,
			Token::new_raw("</ul>"),
			Token::StmtStart,
			Token::Done,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		Some(vec![
			(Ctx::try_from(r#"
				{
					"items": [
						{
							"arr": [1, 2, 3]
						},
						{
							"arr": [4, 5, 6]
						}
					]
				}
			"#).unwrap(),
			Ok(r#"<ul><li>1</li><li>2</li><li>3</li></ul><ul><li>4</li><li>5</li><li>6</li></ul>"#))
		])
	),
	(
		conditional01,
		Some("{{user.name}} is {{if user.age == 18}}an adult{{fi}}."),
		Some(vec![
			Token::StmtStart,
			Token::new_alpha("user"),
			Token::Dot,
			Token::new_alpha("name"),
			Token::StmtEnd,
			Token::new_raw(" is "),
			Token::StmtStart,
			Token::If,
			Token::Whitespace,
			Token::new_alpha("user"),
			Token::Dot,
			Token::new_alpha("age"),
			Token::Whitespace,
			Token::Equal,
			Token::Whitespace,
			Token::new_number("18"),
			Token::StmtEnd,
			Token::new_raw("an adult"),
			Token::StmtStart,
			Token::Fi,
			Token::StmtEnd,
			Token::new_raw("."),
			Token::Eof
		]),
		None,
		Some(vec![
			(Ctx::try_from(r#"
				{
					"user": {
						"name": "John",
						"age": 18
					}
				}
			"#).unwrap(),
			Ok("John is an adult.")
			),
		])
	),
	(
		conditional02,
		Some("{{for user in users}}{{if user.age >= 18}}{{user.name}} is an adult.{{fi}}{{done}}"),
		Some(vec![
			Token::StmtStart,
			Token::For,
			Token::Whitespace,
			Token::new_alpha("user"),
			Token::Whitespace,
			Token::In,
			Token::Whitespace,
			Token::new_alpha("users"),
			Token::StmtEnd,
			Token::StmtStart,
			Token::If,
			Token::Whitespace,
			Token::new_alpha("user"),
			Token::Dot,
			Token::new_alpha("age"),
			Token::Whitespace,
			Token::GreaterEqual,
			Token::Whitespace,
			Token::new_number("18"),
			Token::StmtEnd,
			Token::StmtStart,
			Token::new_alpha("user"),
			Token::Dot,
			Token::new_alpha("name"),
			Token::StmtEnd,
			Token::new_raw(" is an adult."),
			Token::StmtStart,
			Token::Fi,
			Token::StmtEnd,
			Token::StmtStart,
			Token::Done,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		Some(vec![
			(Ctx::try_from(r#"
				{
					"users": [
						{
							"name": "John",
							"age": 18
						},
						{
							"name": "Jane",
							"age": 17
						},
						{
							"name": "Jack",
							"age": 19
						}
					]
				}
			"#).unwrap(),
			Ok("John is an adult.Jack is an adult.")
			),
		])
	),
	(
		conditional03,
		Some(r#"Hey!
		{{if true}}
			True!
		{{fi}}
		{{if false}}
			False!
		{{fi}}
		{{if true}}
			var: {{var}}
		{{fi}}"#),
		Some(vec![
			Token::new_raw("Hey!"),
			Token::NewLine,
			Token::new_non_printable("\t\t"),
			Token::StmtStart,
			Token::If,
			Token::Whitespace,
			Token::Bool(true),
			Token::StmtEnd,
			Token::NewLineNonPrintable,
			Token::new_raw("\t\t\tTrue!"),
			Token::NewLine,
			Token::new_non_printable("\t\t"),
			Token::StmtStart,
			Token::Fi,
			Token::StmtEnd,
			Token::NewLineNonPrintable,
			Token::new_non_printable("\t\t"),
			Token::StmtStart,
			Token::If,
			Token::Whitespace,
			Token::Bool(false),
			Token::StmtEnd,
			Token::NewLineNonPrintable,
			Token::new_raw("\t\t\tFalse!"),
			Token::NewLine,
			Token::new_non_printable("\t\t"),
			Token::StmtStart,
			Token::Fi,
			Token::StmtEnd,
			Token::NewLineNonPrintable,
			Token::new_non_printable("\t\t"),
			Token::StmtStart,
			Token::If,
			Token::Whitespace,
			Token::Bool(true),
			Token::StmtEnd,
			Token::NewLineNonPrintable,
			Token::new_raw("\t\t\tvar: "),
			Token::StmtStart,
			Token::new_alpha("var"),
			Token::StmtEnd,
			Token::NewLine,
			Token::new_non_printable("\t\t"),
			Token::StmtStart,
			Token::Fi,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		Some(vec![
			(Ctx::try_from(r#"
				{
					"var": "value"
				}
				"#).unwrap(),
				Ok("Hey!\n\t\t\tTrue!\n\t\t\tvar: value\n")
			),
		])
	),
	(
		spacing01,
		Some("Hello, {{name}}!
		{{if age >= 18}}You are an adult.{{fi}}"),
		Some(vec![
			Token::new_raw("Hello, "),
			Token::StmtStart,
			Token::new_alpha("name"),
			Token::StmtEnd,
			Token::new_raw("!"),
			Token::NewLine,
			Token::new_raw("\t\t"),
			Token::StmtStart,
			Token::If,
			Token::Whitespace,
			Token::new_alpha("age"),
			Token::Whitespace,
			Token::GreaterEqual,
			Token::Whitespace,
			Token::new_number("18"),
			Token::StmtEnd,
			Token::new_raw("You are an adult."),
			Token::StmtStart,
			Token::Fi,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		Some(vec![
			(Ctx::try_from(r#"
				{
					"name": "John",
					"age": 18
				}
			"#).unwrap(),
			Ok("Hello, John!\n\t\tYou are an adult.")),
		])
	),
	(
		not_found_key01,
		Some("Hello, {{usr.name}}!"),
		None,
		None,
		Some(vec![
			(Ctx::try_from(r#"{ "user": { "name": "John" } }"#).unwrap(), Err(vec!["usr", "not", "found"])),
			(Ctx::try_from(r#"{ "usr": { "nombre": "John" } }"#).unwrap(), Err(vec!["name", "not", "found"])),
		])
	),
	(
		not_found_key02,
		Some("Hello, {{lst[i]}}!"),
		None,
		None,
		Some(vec![
			(Ctx::try_from(r#"{ "i": 4, "lst": [1, 2, 3] }"#).unwrap(), Err(vec!["index", "out", "bounds"])),
			(Ctx::try_from(r#"{ "i": 4, "lst": { "a": 1, "b": 2 } }"#).unwrap(), Err(vec!["object", "index"])),
			(Ctx::try_from(r#"{ "i": 0, "lst": { } }"#).unwrap(), Err(vec!["object", "index"])),
		])
	),
	(
		foreach_fail01,
		Some("Hello, {{name}}!
		{{for item in items}}
			<a href=\"{{item.url}}\">{{item.name}}</a>
		{{done}}"),
		None,
		None,
		Some(vec![
			(Ctx::try_from(r#"
				{
					"name": "world",
					"items": [
						{
							"url": "https://example01.com",
							"name": "Example01"
						},
						123
					]
				}"#
			).unwrap(),Err(vec!["access", "value", "url"])),
		])
	)
);
