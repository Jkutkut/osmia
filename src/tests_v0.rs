use crate::{osmia_syntax, Json};

// use std::collections::HashMap;

#[test]
fn identity() {
	let template = "Hello, world!";
	let cxt = Json::Object(serde_json::Map::new());
	let expected = "Hello, world!";
	match osmia_syntax().render(template, &cxt) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}

#[test]
fn basic_test01() {
	let template = "Hello, {{name}}!";
	let cxt = serde_json::from_str::<Json>(r#"{ "name": "world" }"#).unwrap();
	let expected = "Hello, world!";
	match osmia_syntax().render(template, &cxt) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}

#[test]
fn basic_test02() {
	let template = "Hello, {{name}}! A number {{number}}, a boolean {{boolean}} an a null {{null}}.";
	let cxt = serde_json::from_str::<Json>(r#"
		{
			"name": "world",
			"number": 42,
			"boolean": true,
			"null": null
		}
	"#).unwrap();
	let expected = "Hello, world! A number 42, a boolean true an a null null.";
	match osmia_syntax().render(template, &cxt) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}

#[test]
fn basic_test03() {
	let template = "Hello, {{name}}! an array {{lst}}, single value from lst {{lst[1]}} and an object {{object}}.";
	let cxt = serde_json::from_str::<Json>(r#"
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
	"#).unwrap();
	let expected = "Hello, world! an array [11,22,33], single value from lst 22 and an object {\"key\":\"value\",\"obj\":{\"key\":\"value2\"}}.";
	match osmia_syntax().render(template, &cxt) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}

#[test]
fn not_found_key01() {
	let template = "Hello, {{name}}!";
	let ctx = serde_json::from_str::<Json>(r#"{ "name2": "world" }"#).unwrap();
	match osmia_syntax().render(template, &ctx) {
		Ok(actual) => panic!("Expected error, got: {}", actual),
		Err(err) => {
			let err = err.to_string();
			assert!(
				err.to_lowercase().contains("key not found") &&
				err.contains("name")
			)
		}
	}
}


#[test]
fn not_found_key02() {
	let template = "Hello, {{lst[-1]}}!";
	let ctx = serde_json::from_str::<Json>(r#"{ "lst": [1, 2, 3] }"#).unwrap();
	match osmia_syntax().render(template, &ctx) {
		Ok(actual) => panic!("Expected error, got: {}", actual),
		Err(err) => {
			let err = err.to_string();
			assert!(err.to_lowercase().contains("invalid index"))
		}
	}
}

#[test]
fn not_found_key03() {
	let template = "Hello, {{lst[3]}}!";
	let ctx = serde_json::from_str::<Json>(r#"{ "lst": [1, 2, 3] }"#).unwrap();
	match osmia_syntax().render(template, &ctx) {
		Ok(actual) => panic!("Expected error, got: {}", actual),
		Err(err) => {
			let err = err.to_string();
			assert!(err.to_lowercase().contains("index out of bounds"))
		}
	}
}

#[test]
fn invalid_ctx() {
	let template = "Hello, {{name}}!";
	let ctx = serde_json::from_str::<Json>(r#"[]"#).unwrap();
	match osmia_syntax().render(template, &ctx) {
		Ok(actual) => panic!("Expected error, got: {}", actual),
		Err(err) => {
			let err = err.to_string();
			assert!(err.contains("should be"));
			assert!(err.contains("object"));
		}
	}
	let ctx = serde_json::from_str::<Json>(r#"null"#).unwrap();
	match osmia_syntax().render(template, &ctx) {
		Ok(actual) => panic!("Expected error, got: {}", actual),
		Err(err) => {
			let err = err.to_string();
			assert!(err.contains("should be"));
			assert!(err.contains("object"));
		}
	}
	let ctx = serde_json::from_str::<Json>(r#"true"#).unwrap();
	match osmia_syntax().render(template, &ctx) {
		Ok(actual) => panic!("Expected error, got: {}", actual),
		Err(err) => {
			let err = err.to_string();
			assert!(err.contains("should be"));
			assert!(err.contains("object"));
		}
	}
	let ctx = serde_json::from_str::<Json>(r#"1"#).unwrap();
	match osmia_syntax().render(template, &ctx) {
		Ok(actual) => panic!("Expected error, got: {}", actual),
		Err(err) => {
			let err = err.to_string();
			assert!(err.contains("should be"));
			assert!(err.contains("object"));
		}
	}
}

#[test]
fn similar_syntax() {
	let template = "Hello, {{name}}! The syntax is \"Hello, {{{name}}}!\".";
	let cxt = serde_json::from_str::<Json>(r#"{ "name": "world" }"#).unwrap();
	let expected = "Hello, world! The syntax is \"Hello, {{name}}!\".";
	match osmia_syntax().render(template, &cxt) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}

#[test]
fn comment() {
	let template = "Hello!{* This is a comment *}";
	let cxt = serde_json::from_str::<Json>(r#"{ "name": "world" }"#).unwrap();
	let expected = "Hello!";
	match osmia_syntax().render(template, &cxt) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}

#[test]
fn comment_with_brackets() {
	let template = "Hello!{* This is a comment with brackets: { { } } *}";
	let cxt = serde_json::from_str::<Json>(r#"{ "name": "world" }"#).unwrap();
	let expected = "Hello!";
	match osmia_syntax().render(template, &cxt) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}

#[test]
fn comment_with_brackets_and_text() {
	let template = "Hello, {{name}}!{* This is a comment with brackets: {{ {{{ }}} }} *}";
	let cxt = serde_json::from_str::<Json>(r#"{ "name": "world" }"#).unwrap();
	let expected = "Hello, world!";
	match osmia_syntax().render(template, &cxt) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}

#[test]
fn complex_value01() {
	let template = "Hello, {{name.first}} {{name.last}}!";
	let cxt = serde_json::from_str::<Json>(r#"
		{
			"name": {
				"first": "John",
				"last": "Doe"
			}
		}
	"#).unwrap();
	let expected = "Hello, John Doe!";
	match osmia_syntax().render(template, &cxt) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}

#[test]
fn complex_value02() {
	let template = "This is a complex value: {{obj.obj[0].obj}}";
	let cxt = serde_json::from_str::<Json>(r#"
		{
			"obj": {
				"obj": [
					{
						"obj": "value"
					}
				]
			}
		}
	"#).unwrap();
	let expected = "This is a complex value: value";
	match osmia_syntax().render(template, &cxt) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}

#[test]
fn for_each01() {
	let template = r#"Hello, {{name}}!
{{for item in items}}
	<a href="{{item.url}}">{{item.name}}</a>
{{end}}"#;
	let ctx: Json = serde_json::from_str(r#"
		{
			"name": "world",
			"items": [
				{
					"url": "https://example.com",
					"name": "Example"
				},
				{
					"url": "https://example.org",
					"name": "Example"
				}
			]
		}
	"#).unwrap();
	let expected = "Hello, world!\n\n\t<a href=\"https://example.com\">Example</a>\n\n\t<a href=\"https://example.org\">Example</a>\n";
	match osmia_syntax().render(template, &ctx) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}

#[test]
fn for_each02() {
	let template = r#"Hello, {{name}}!
{{for item in items}}
	<li>Element {{item}}</li>
{{end}}"#;
	let ctx: Json = serde_json::from_str(r#"
		{
			"name": "world",
			"items": [1, 2, 3, 4, 5]
		}
	"#).unwrap();
	let expected = "Hello, world!\n\n\t<li>Element 1</li>\n\n\t<li>Element 2</li>\n\n\t<li>Element 3</li>\n\n\t<li>Element 4</li>\n\n\t<li>Element 5</li>\n";
	match osmia_syntax().render(template, &ctx) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}

#[test]
fn for_each03() {
	let template = r#"{{for item in items}}<li>{{item.arr[1]}}</li>{{end}}"#;
	let ctx: Json = serde_json::from_str(r#"
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
	"#).unwrap();
	let expected = "<li>2</li><li>5</li>";
	match osmia_syntax().render(template, &ctx) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}

#[test]
fn for_each_fail01() {
	let template = r#"Hello, {{name}}!
{{for item in items}}
	<a href="{{item.url}}">{{item.name}}</a>
{{end}}"#;
	let ctx: Json = serde_json::from_str(r#"
		{
			"name": "world",
			"items": [
				{
					"url": "https://example.com",
					"name": "Example"
				},
				{
					"url": "https://example.org",
					"a": "Example"
				}
			]
		}
	"#).unwrap();
	match osmia_syntax().render(template, &ctx) {
		Ok(actual) => panic!("Expected error, got: {}", actual),
		Err(err) => {
			let err = err.to_string();
			assert!(
				err.to_lowercase().contains("key not found") &&
				err.contains("name")
			)
		}
	}
}

#[test]
fn conditional01() {
	let template = r#"
		{{user.name}} is {{if user.age == 18}}an adult{{if}}.
	"#;
	let ctx: Json = serde_json::from_str(r#"
		{
			"user": {
				"name": "John",
				"age": 18
			}
		}
	"#).unwrap();
	let expected = "\n\t\tJohn is an adult.\n\t";
	match osmia_syntax().render(template, &ctx) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}

#[test]
fn conditional02() {
	let template = r#"
		{{for user in users}}{{if user.age >= 18}}{{user.name}} is an adult.{{if}}{{end}}
	"#.trim();
	let ctx: Json = serde_json::from_str(r#"
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
	"#).unwrap();
	let expected = "John is an adult.Jack is an adult.";
	match osmia_syntax().render(template, &ctx) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}
#[test]
fn conditional03() {
	let template = r#"
		{{for user in users}}
			{{user.name}} is {{if user.age >= 18}}an adult{{else}}a child{{if}}.
		{{end}}
	"#;
	let ctx: Json = serde_json::from_str(r#"
		{
			"users": [
				{
					"name": "John",
					"age": 18
				},
				{
					"name": "Jane",
					"age": 17
				}
			]
		}
	"#).unwrap();
	let expected = "\n\t\tJohn is an adult.\n\t\n\t\tJane is a child.\n\t\n\t";
	match osmia_syntax().render(template, &ctx) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}
