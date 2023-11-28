use crate::Lexer;

fn render(code: &str) -> Result<String, String> {
	let parser = Lexer::default();
	parser.render(code)
}

#[test]
fn init() {
	let _parser = Lexer::new("{{", "}}");
	let _parser = Lexer::default();
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
		"\"Hello, world!\"",
		"\"\"",
		"\"\\\"\"",
		"\"\\n\"",
		"\"\\r\"",
		"\"\\t\""
	];
	for literal in literals {
		match render(&format!("{{{{{}}}}}", literal)) {
			Ok(s) => assert_eq!(s, literal),
			Err(e) => panic!("Unexpected error: {}", e)
		}
	}
}
