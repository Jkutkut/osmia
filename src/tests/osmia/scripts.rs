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
		Some(Stmt::Raw("Hello, world!".to_string())),
		None
		// None,
		// "Hello, world!"
	),
	(
		basic_test01,
		Some("Hello, {{name}}!"),
		None,
		None,
		None
		// Some(r#"{ "name": "world" }"#),
		// "Hello, world!"
	),
	(
		basic_test02,
		Some("Hello, {{name}}! A number {{number}}, a boolean {{boolean}} an a null {{null}}."),
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"name": "world",
		// 		"number": 42,
		// 		"boolean": true,
		// 		"null": null
		// 	}
		// "#),
		// "Hello, world! A number 42, a boolean true an a null null."
	),
	(
		basic_test03,
		Some("Hello, {{name}}! Single value from lst {{lst[1]}} and an object element {{object.obj.key}}."),
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"name": "world",
		// 		"lst": [11, 22, 33],
		// 		"object": {
		// 			"key": "value",
		// 			"obj": {
		// 				"key": "value2"
		// 			}
		// 		}
		// 	}
		// "#),
		// "Hello, world! Single value from lst 22 and an object element value2."
	),
	(
		similar_syntax,
		Some(r#"Hello, {{name}}! The syntax is "Hello, {{"{{name}}!"}}"."#),
		None,
		None,
		None
		// Some(r#"{ "name": "world" }"#),
		// "Hello, world! The syntax is \"Hello, {{name}}!\"."
	),
	(
		comment,
		Some(r#"Hello!{{print "This is a comment"}}"#),
		None,
		None,
		None
		// None,
		// "Hello!"
	),
	(
		complex_value01,
		Some("Hello, {{name.first}} {{name.last}}!"),
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"name": {
		// 			"first": "John",
		// 			"last": "Doe"
		// 		}
		// 	}
		// "#),
		// "Hello, John Doe!"
	),
	(
		complex_value02,
		Some("This is a complex value: {{obj.obj[0].obj}}"),
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"obj": {
		// 			"obj": [
		// 				{
		// 					"obj": "value"
		// 				}
		// 			]
		// 		}
		// 	}
		// "#),
		// "This is a complex value: value"
	),
	(
		foreach01,
		Some(r#"Hello, {{name}}!
		{{for item in items}}
		<a href="{{item.url}}">{{item.name}}</a>
		{{done}}"#),
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"name": "world",
		// 		"items": [
		// 			{
		// 				"url": "https://example01.com",
		// 				"name": "Example01"
		// 			},
		// 			{
		// 				"url": "https://example02.org",
		// 				"name": "Example02"
		// 			}
		// 		]
		// 	}
		// "#),
		// "Hello, world!
		// <a href=\"https://example01.com\">Example01</a>
		// <a href=\"https://example02.org\">Example02</a>\n"
	),
	(
		foreach02,
		Some(r#"Hello, {{name}}!
		{{for item in items}}
		<li>Element {{item}}</li>
		{{done}}"#),
		// Some(r#"
		None,
		None,
		None
		// 	{
		// 		"name": "world",
		// 		"items": [1, 2, 3]
		// 	}
		// "#),
		// "Hello, world!
		// <li>Element 1</li>
		// <li>Element 2</li>
		// <li>Element 3</li>\n"
	),
	(
		foreach03,
		Some(r#"{{for item in items}}<ul>{{for i in item.arr}}<li>{{i}}</li>{{done}}</ul>{{done}}"#),
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"items": [
		// 			{
		// 				"arr": [1, 2, 3]
		// 			},
		// 			{
		// 				"arr": [4, 5, 6]
		// 			}
		// 		]
		// 	}
		// "#),
		// "<ul><li>1</li><li>2</li><li>3</li></ul><ul><li>4</li><li>5</li><li>6</li></ul>"
	),
	(
		conditional01,
		Some("{{user.name}} is {{if user.age == 18}}an adult{{fi}}."),
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"user": {
		// 			"name": "John",
		// 			"age": 18
		// 		}
		// 	}
		// "#),
		// "John is an adult."
	),
	(
		conditional02,
		Some("{{for user in users}}{{if user.age >= 18}}{{user.name}} is an adult.{{fi}}{{done}}"),
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"users": [
		// 			{
		// 				"name": "John",
		// 				"age": 18
		// 			},
		// 			{
		// 				"name": "Jane",
		// 				"age": 17
		// 			},
		// 			{
		// 				"name": "Jack",
		// 				"age": 19
		// 			}
		// 		]
		// 	}
		// "#),
		// "John is an adult.Jack is an adult."
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
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"var": "value"
		// 	}
		// "#),
		// "Hey!
		// 	True!
		// 	var: value\n"
	),
	(
		spacing01,
		Some("Hello, {{name}}!
		{{if age >= 18}}You are an adult{{fi}}."),
		None,
		None,
		None
		// Some(r#"
		// 	{
		// 		"name": "John",
		// 		"age": 18
		// 	}
		// "#),
		// "Hello, John!
		// You are an adult."
	)
);
