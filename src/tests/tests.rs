use crate::macro_tests;
use crate::Osmia;

#[cfg(test)]
fn test_io(
	input: &str,
	ctx: Option<&str>,
	expected: &str
) {
	let mut interpreter = match ctx {
		None => Osmia::new(),
		Some(ctx) => Osmia::from_json(ctx).unwrap()
	};
	let code = Osmia::code(input).unwrap();
	let result = interpreter.run(&code).unwrap();
	assert_eq!(result, expected);
}

#[cfg(test)]
fn test_execution_error(
	input: &str,
	ctx: Option<&str>
) {
	let mut interpreter = match ctx {
		None => Osmia::new(),
		Some(ctx) => Osmia::from_json(ctx).unwrap()
	};
	let code = Osmia::code(input).unwrap();
	let result = interpreter.run(&code);
	assert!(result.is_err());
}

macro_tests!(
	test_io,
	(
		identity,
		"Hello, world!",
		None,
		"Hello, world!"
	),
	(
		basic_test01,
		"Hello, {{name}}!",
		Some(r#"{ "name": "world" }"#),
		"Hello, world!"
	),
	(
		basic_test02,
		"Hello, {{name}}! A number {{number}}, a boolean {{boolean}} an a null {{null}}.",
		Some(r#"
			{
				"name": "world",
				"number": 42,
				"boolean": true,
				"null": null
			}
		"#),
		"Hello, world! A number 42, a boolean true an a null null."
	),
	(
		basic_test03,
		"Hello, {{name}}! Single value from lst {{lst[1]}} and an object element {{object.obj.key}}.",
		Some(r#"
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
		"#),
		"Hello, world! Single value from lst 22 and an object element value2."
	),
	(
		similar_syntax,
		r#"Hello, {{name}}! The syntax is "Hello, {{"{{name}}!"}}"."#,
		Some(r#"{ "name": "world" }"#),
		"Hello, world! The syntax is \"Hello, {{name}}!\"."
	),
	(
		comment,
		r#"Hello!{{print "This is a comment"}}"#,
		None,
		"Hello!"
	),
	(
		complex_value01,
		"Hello, {{name.first}} {{name.last}}!",
		Some(r#"
			{
				"name": {
					"first": "John",
					"last": "Doe"
				}
			}
		"#),
		"Hello, John Doe!"
	),
	(
		complex_value02,
		"This is a complex value: {{obj.obj[0].obj}}",
		Some(r#"
			{
				"obj": {
					"obj": [
						{
							"obj": "value"
						}
					]
				}
			}
		"#),
		"This is a complex value: value"
	),
	// (
	// 	foreach01,
	// 	r#"Hello, {{name}}!
	// 	{{for item in items}}
	// 	<a href="{{item.url}}">{{item.name}}</a>
	// 	{{done}}"#,
	// 	Some(r#"
	// 		{
	// 			"name": "world",
	// 			"items": [
	// 				{
	// 					"url": "https://example01.com",
	// 					"name": "Example01"
	// 				},
	// 				{
	// 					"url": "https://example02.org",
	// 					"name": "Example02"
	// 				}
	// 			]
	// 		}
	// 	"#),
	// 	"Hello, world!
	// 	<a href=\"https://example01.com\">Example01</a>
	// 	<a href=\"https://example02.org\">Example02</a>"
	// ),
	// (
	// 	foreach02,
	// 	r#"Hello, {{name}}!
	// 	{{for item in items}}
	// 	<li>Element {{item}}</li>
	// 	{{done}}"#,
	// 	Some(r#"
	// 		{
	// 			"name": "world",
	// 			"items": [1, 2, 3]
	// 		}
	// 	"#),
	// 	"Hello, world!
	// 	<li>Element 1</li>
	// 	<li>Element 2</li>
	// 	<li>Element 3</li>"
	// ),
	(
		foreach03,
		r#"{{for item in items}}<ul>{{for i in item.arr}}<li>{{i}}</li>{{done}}</ul>{{done}}"#,
		Some(r#"
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
		"#),
		"<ul><li>1</li><li>2</li><li>3</li></ul><ul><li>4</li><li>5</li><li>6</li></ul>"
	),
	(
		conditional01,
		"{{user.name}} is {{if user.age == 18}}an adult{{fi}}.",
		Some(r#"
			{
				"user": {
					"name": "John",
					"age": 18
				}
			}
		"#),
		"John is an adult."
	),
	(
		conditional02,
		"{{for user in users}}{{if user.age >= 18}}{{user.name}} is an adult.{{fi}}{{done}}",
		Some(r#"
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
		"#),
		"John is an adult.Jack is an adult."
	)
);

macro_tests!(
	test_execution_error,
	(
		not_found_key01,
		"Hello, {{name}}!",
		Some(r#"{ "name2": "world" }"#)
	),
	(
		not_found_key02,
		"Hello, {{lst[4]}}!",
		Some(r#"{ "lst": [1, 2, 3] }"#)
	),
	(
		foreach_fail01,
		"Hello, {{name}}!
		{{for item in items}}
			<a href=\"{{item.url}}\">{{item.name}}</a>
		{{done}}",
		Some(r#"
			{
				"name": "world",
				"items": [
					{
						"url": "https://example01.com",
						"name": "Example01"
					},
					{
						"url": "https://example02.org"
					}
				]
			}
		"#)
	)
);
