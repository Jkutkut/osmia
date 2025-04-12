use super::*;

macro_tests!(
	test_code,
	(
		identity,
		"Hello, world!",
		None,
		None,
		vec![
			(Ctx::new(), Ok("Hello, world!")),
		],
	),
	(
		basic_test01,
		"Hello, {{name}}!",
		None,
		None,
		vec![
			(Ctx::try_from(r#"{ "name": "world" }"#).unwrap(), Ok("Hello, world!")),
		]
	),
	(
		basic_test02,
		"Hello, {{name}}! A number {{number}}, a boolean {{boolean}} an a null {{null}}.",
		None,
		None,
		vec![
			(Ctx::try_from(r#"
				{
					"name": "world",
					"number": 42,
					"boolean": true,
					"null": null
				}
			"#).unwrap(), Ok("Hello, world! A number 42, a boolean true an a null null.")),
		]
	),
	(
		basic_test03,
		"Hello, {{name}}! Single value from lst {{lst[1]}} and an object element {{object.obj.key}}.",
		None,
		None,
		vec![
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
		]
	),
	(
		similar_syntax,
		r#"Hello, {{name}}! The syntax is "Hello, {{"{{name}}!"}}"."#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"{ "name": "world" }"#).unwrap(), Ok(r#"Hello, world! The syntax is "Hello, {{name}}!"."#)),
		]
	),
	// (
	// 	comment,
	// 	r#"Hello!{{print "This is a comment"}}"#,
	// 	None,
	// 	None,
	// 	vec![
	// 		(Ctx::new(), Ok("Hello!")),
	// 	]
	// ),
	(
		complex_value01,
		"Hello, {{name.first}} {{name.last}}!",
		None,
		None,
		vec![
			(Ctx::try_from(r#"
				{
					"name": {
						"first": "John",
						"last": "Doe"
					}
				}
			"#).unwrap(), Ok("Hello, John Doe!")),
		]
	),
	(
		complex_value02,
		"This is a complex value: {{obj.obj[0].obj}}",
		None,
		None,
		vec![
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
		]
	),
	(
		foreach01,
		r#"Hello, {{name}}!
		{{for item in items}}
		<a href="{{item.url}}">{{item.name}}</a>
		{{done}}"#,
		None,
		None,
		vec![(
			Ctx::try_from(r#"
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
			Ok("Hello, world!\n\t\t\n\t\t<a href=\"https://example01.com\">Example01</a>\n\t\t\n\t\t<a href=\"https://example02.org\">Example02</a>\n\t\t")
		)]
	),
	(
		foreach02,
		r#"Hello, {{name}}!
		{{for item in items}}
		<li>Element {{item}}</li>
		{{done}}"#,
		None,
		None,
		vec![
			(Ctx::try_from(r#"
				{
					"name": "world",
					"items": [1, 2, 3]
				}
			"#).unwrap(),
			Ok("Hello, world!\n\t\t\n\t\t<li>Element 1</li>\n\t\t\n\t\t<li>Element 2</li>\n\t\t\n\t\t<li>Element 3</li>\n\t\t"))
		]
	),
	(
		foreach03,
		r#"{{for item in items}}<ul>{{for i in item.arr}}<li>{{i}}</li>{{done}}</ul>{{done}}"#,
		None,
		None,
		vec![
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
		]
	),
	(
		conditional01,
		"{{user.name}} is {{if user.age == 18}}an adult{{fi}}.",
		None,
		None,
		vec![
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
		]
	),
	(
		conditional02,
		"{{for user in users}}{{if user.age >= 18}}{{user.name}} is an adult.{{fi}}{{done}}",
		None,
		None,
		vec![
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
		]
	),
	(
		conditional03,
		r#"
Hey!
{{if true}}
	True!
{{fi}}
{{if false}}
	False!
{{fi}}
{{if true}}
	var: {{var}}
{{fi}}"#.trim(),
		None,
		None,
		vec![
			(Ctx::try_from(r#"
				{
					"var": "value"
				}
				"#).unwrap(),
				Ok("Hey!\n\n\tTrue!\n\n\n\n\tvar: value\n")
			),
		],
	),
	(
		spacing01,
		"
Hello, {{name}}!
{{if age >= 18}}You are an adult{{fi}}.".trim(),
		None,
		None,
		vec![
			(Ctx::try_from(r#"
				{
					"name": "John",
					"age": 18
				}
			"#).unwrap(),
			Ok("Hello, John!\nYou are an adult.")),
		]
	),
	(
		not_found_key01,
		"Hello, {{usr.name}}!",
		None,
		None,
		vec![
			(Ctx::try_from(r#"{ "user": { "name": "John" } }"#).unwrap(), Err(vec!["usr", "not", "found"])),
			(Ctx::try_from(r#"{ "usr": { "nombre": "John" } }"#).unwrap(), Err(vec!["name", "not", "found"])),
		]
	),
	(
		not_found_key02,
		"Hello, {{lst[i]}}!",
		None,
		None,
		vec![
			(Ctx::try_from(r#"{ "i": 4, "lst": [1, 2, 3] }"#).unwrap(), Err(vec!["index", "out", "bounds"])),
			(Ctx::try_from(r#"{ "i": 4, "lst": { "a": 1, "b": 2 } }"#).unwrap(), Err(vec!["object", "index"])),
			(Ctx::try_from(r#"{ "i": 0, "lst": { } }"#).unwrap(), Err(vec!["object", "index"])),
		]
	),
	(
		foreach_fail01,
		"Hello, {{name}}!
		{{for item in items}}
			<a href=\"{{item.url}}\">{{item.name}}</a>
		{{done}}",
		None,
		None,
		vec![
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
		]
	)
);
