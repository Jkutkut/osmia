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
		None
		// None,
		// "Hello, world!"
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
		None
		// Some(r#"{ "name": "world" }"#),
		// "Hello, world!"
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
		None
		// Some(r#"{ "name": "world" }"#),
		// "Hello, world! The syntax is \"Hello, {{name}}!\"."
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
		Some(vec![
			Token::new_raw("Hello, "),
			Token::StmtStart,
			Token::new_alpha("name"),
			Token::StmtEnd,
			Token::new_raw("!"),
			Token::NewLine,
			Token::new_raw("\t\t"),
			Token::StmtStart,
			Token::For,
			Token::Whitespace,
			Token::new_alpha("item"),
			Token::Whitespace,
			Token::In,
			Token::Whitespace,
			Token::new_alpha("items"),
			Token::StmtEnd,
			Token::NewLine,
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
			Token::new_raw("\t\t"),
			Token::StmtStart,
			Token::Done,
			Token::StmtEnd,
			Token::Eof
		]),
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
		Some(vec![
			Token::new_raw("Hello, "),
			Token::StmtStart,
			Token::new_alpha("name"),
			Token::StmtEnd,
			Token::new_raw("!"),
			Token::NewLine,
			Token::new_raw("\t\t"),
			Token::StmtStart,
			Token::For,
			Token::Whitespace,
			Token::new_alpha("item"),
			Token::Whitespace,
			Token::In,
			Token::Whitespace,
			Token::new_alpha("items"),
			Token::StmtEnd,
			Token::NewLine,
			Token::new_raw("\t\t<li>Element "),
			Token::StmtStart,
			Token::new_alpha("item"),
			Token::StmtEnd,
			Token::new_raw("</li>"),
			Token::NewLine,
			Token::new_raw("\t\t"),
			Token::StmtStart,
			Token::Done,
			Token::StmtEnd,
			Token::Eof
		]),
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
		Some(vec![
			Token::new_raw("Hey!"),
			Token::NewLine,
			Token::new_raw("\t\t"),
			Token::StmtStart,
			Token::If,
			Token::Whitespace,
			Token::Bool(true),
			Token::StmtEnd,
			Token::NewLine,
			Token::new_raw("\t\t\tTrue!"),
			Token::NewLine,
			Token::new_raw("\t\t"),
			Token::StmtStart,
			Token::Fi,
			Token::StmtEnd,
			Token::NewLine,
			Token::new_raw("\t\t"),
			Token::StmtStart,
			Token::If,
			Token::Whitespace,
			Token::Bool(false),
			Token::StmtEnd,
			Token::NewLine,
			Token::new_raw("\t\t\tFalse!"),
			Token::NewLine,
			Token::new_raw("\t\t"),
			Token::StmtStart,
			Token::Fi,
			Token::StmtEnd,
			Token::NewLine,
			Token::new_raw("\t\t"),
			Token::StmtStart,
			Token::If,
			Token::Whitespace,
			Token::Bool(true),
			Token::StmtEnd,
			Token::NewLine,
			Token::new_raw("\t\t\tvar: "),
			Token::StmtStart,
			Token::new_alpha("var"),
			Token::StmtEnd,
			Token::NewLine,
			Token::new_raw("\t\t"),
			Token::StmtStart,
			Token::Fi,
			Token::StmtEnd,
			Token::Eof
		]),
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
		{{if age >= 18}}You are an adult.{{fi}}."),
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
			Token::new_raw("."),
			Token::Eof
		]),
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
