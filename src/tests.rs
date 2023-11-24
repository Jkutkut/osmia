use crate::render;

use std::collections::HashMap;

#[test]
fn identity() {
	let template = "Hello, world!";
	let cxt: HashMap<&str, String> = HashMap::new();
	let expected = "Hello, world!";
	match render(template, &cxt) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}

#[test]
fn basic_test() {
	let template = "Hello, {{name}}!";
	let cxt = std::collections::HashMap::from([
		("name", "world")
	]);
	let expected = "Hello, world!";
	match render(template, &cxt) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}

#[test]
fn fail_render() {
	let template = "Hello, {{name}}!";
	let cxt = std::collections::HashMap::from([
		("name", 1)
	]);
	let expected = "Hello, world!";
	match render(template, &cxt) {
		Ok(actual) => assert_ne!(expected, actual),
		Err(err) => assert_eq!(err, "Key not found: name")
	}
}

#[test]
fn not_found_key() {
	let template = "Hello, {{name}}!";
	let cxt = std::collections::HashMap::from([
		("name2", "world")
	]);
	let expected = "Hello, world!";
	match render(template, &cxt) {
		Ok(actual) => assert_ne!(expected, actual),
		Err(err) => assert_eq!(err, "Key not found: name")
	}
}

#[test]
fn similar_syntax() {
	let template = "Hello, {{name}}! The syntax to us a variable is {{{name}}}.";
	let cxt = std::collections::HashMap::from([
		("name", "world")
	]);
	let expected = "Hello, world! The syntax to us a variable is {{name}}.";
	match render(template, &cxt) {
		Ok(actual) => assert_eq!(expected, actual),
		Err(err) => panic!("{}", err)
	}
}
