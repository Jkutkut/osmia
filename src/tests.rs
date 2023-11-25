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
fn basic_test() {
	let template = "Hello, {{name}}!";
	let cxt = serde_json::from_str::<Json>(r#"{ "name": "world" }"#).unwrap();
	let expected = "Hello, world!";
	match osmia_syntax().render(template, &cxt) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}

#[test]
fn not_found_key() {
	let template = "Hello, {{name}}!";
	let ctx = serde_json::from_str::<Json>(r#"{ "name2": "world" }"#).unwrap();
	match osmia_syntax().render(template, &ctx) {
		Ok(actual) => panic!("Expected error, got: {}", actual),
		// contains "key not found" and "name"
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

// #[test]
// fn for_each() {
// 	let template = "Hello, {{name}}!
// {{for item in items}}
// 	<a href=\"{{item.url}}\">{{item.name}}</a>
// {{end}}";
// 	let ctx: Json = serde_json::from_str(r#"
// 		{
// 			"name": "world",
// 			"items": [
// 				{
// 					"url": "https://example.com",
// 					"name": "Example"
// 				},
// 				{
// 					"url": "https://example.org",
// 					"name": "Example"
// 				}
// 			]
// 		}
// 	"#).unwrap();
// 	let expected = "Hello, world!\n\t<a href=\"https://example.com\">Example</a>\n\t<a href=\"https://example.org\">Example</a>";
// 	match osmia_syntax().render(template, &ctx) {
// 		Ok(actual) => assert_eq!(expected, actual),
// 		Err(err) => panic!("{}", err)
// 	}
// }
