use crate::render;

use std::collections::HashMap;

#[test]
fn identity() {
	let template = "Hello, world!";
	let cxt: HashMap<&str, String> = HashMap::new();
	let expected = "Hello, world!";
	let actual = render(template, &cxt);
	assert_eq!(expected, actual);
}

#[test]
fn basic_test() {
	let template = "Hello, ${name}!";
	let cxt = std::collections::HashMap::from([
		("name", "world")
	]);
	let expected = "Hello, world!";
	let actual = render(template, &cxt);
	assert_eq!(expected, actual);
}

#[test]
fn fail_render() {
	let template = "Hello, ${name}!";
	let cxt = std::collections::HashMap::from([
		("name", 1)
	]);
	let expected = "Hello, world!";
	let actual = render(template, &cxt);
	assert_ne!(expected, actual);
}

#[test]
fn not_found_key() {
	let template = "Hello, ${name}!";
	let cxt = std::collections::HashMap::from([
		("name2", "world")
	]);
	let expected = "Hello, world!";
	let actual = render(template, &cxt);
	assert_ne!(expected, actual);
}
