use crate::Osmia;

fn render(code: &str) -> Result<String, String> {
	let parser = Osmia::default();
	parser.render(code)
}

#[test]
fn init() {
	let _parser = Osmia::new("{{", "}}");
	let _parser = Osmia::default();
}

#[test]
fn just_text() {
	match render("Hello, world!") {
		Ok(s) => assert_eq!(s, "Hello, world!"),
		Err(e) => panic!("Unexpected error: {}", e)
	}
}

#[test]
fn empty_render() {
		match render("") {
		Ok(s) => assert_eq!(s, ""),
		Err(e) => panic!("Unexpected error: {}", e)
	}
}

#[test]
fn literal_values() {
	let literals = vec![
		"true",
		"false",
		"null",
		"42",
		"3.14",
		r#""Hello, world!""#,
		r#""""#,
		r#""\n""#,
		r#""\r""#,
		r#""\t""#,
		"{{}}", // Empty value
	];
	let mut all_literals = String::new();
	for literal in &literals {
		let current = format!("{{{{{}}}}}", literal);
		all_literals.push_str(&current);
		match render(&current) {
			Ok(s) => assert_eq!(&s, literal),
			Err(e) => panic!("Unexpected error: {}", e)
		}
	}
	let expected: String = literals.join("");
	match render(&all_literals) {
		Ok(s) => assert_eq!(s, expected),
		Err(e) => panic!("Unexpected error: {}", e)
	}
}

#[test]
fn invalid_values() {
	let invalids = vec![
		r#"{{"}}"#,
		r#"{{'}}"#,
		r#"{{"""}}"#,
		// TODO
	];
	for invalid in &invalids {
		match render(invalid) {
			Ok(_) => panic!("Expected error at {}", invalid),
			Err(_) => ()
		}
	}
}

// #[test]
// fn combined_values() {
// 	let tests = vec![
// 		("2 + 1 = {{2 + 1}}", "2 + 1 = 3"),
// 		("2 - 1 = {{2 - 1}}", "2 - 1 = 1"),
// 		("2 * 3 = {{2 * 3}}", "2 * 3 = 6"),
// 		("6 / 2 = {{6 / 2}}", "6 / 2 = 3"),
// 		("6 % 4 = {{6 % 4}}", "6 % 4 = 2"),
// 		("2 + 3 * 4 = {{2 + 3 * 4}}", "2 + 3 * 4 = 14"),
// 		("(2 + 3) * 4 = {{{{2 + 3}} * 4}}", "(2 + 3) * 4 = 20"),
// 		("2 + 3 * 4 - 5 = {{2 + 3 * 4 - 5}}", "2 + 3 * 4 - 5 = 11"),
// 		("2 + 3 * (4 - 5) = {{2 + 3 * (4 - 5)}}", "2 + 3 * (4 - 5) = -1"),
// 		// Decimals
// 		("7 / 2 = {{7 / 2}}", "7 / 2 = 3.5"),
// 		("7 + 3.2 - 1.2 = {{7 + 3.2 - 1.2}}", "7 + 3.2 - 1.2 = 9"),
// 		("7.0 / 2.0 = {{7.0 / 2.0}}", "7.0 / 2.0 = 3.5"),
// 		// TODO
// 	];
// 	for (input, expected) in &tests {
// 		match render(input) {
// 			Ok(s) => assert_eq!(&s, expected),
// 			Err(e) => panic!("Unexpected error: {}", e)
// 		}
// 	}
// }
