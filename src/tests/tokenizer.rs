use crate::tokenizer::Tokenizer;

fn compare(code: &str, expected: Vec<&str>) {
	let result = Tokenizer::new(code)
		.map(|t| t.unwrap())
		.collect::<Vec<&str>>();
	assert_eq!(result, expected);
}

#[test]
fn basic_test() {
	compare(
		"",
		vec![]
	);
}

#[test]
fn basic_test2() {
	compare(
		"this is a test",
		vec!["this", "is", "a", "test"]
	);
}

#[test]
fn multiple_spaces_test() {
	let tests = [
		("this is   a    test", vec!["this", "is", "a", "test"]),
		("this is a test   ", vec!["this", "is", "a", "test"]),
		("   this is a test", vec!["this", "is", "a", "test"]),
		("   this is   a test   ", vec!["this", "is", "a", "test"]),
	];
	for (code, expected) in &tests {
		compare(code, expected.to_vec());
	}
}

#[test]
fn white_spaces() {
	let tests = [
		("spaces: this words has spaces", vec!["spaces:", "this", "words", "has", "spaces"]),
		("tabs: this\twords\thas\ttabs", vec!["tabs:", "this", "words", "has", "tabs"]),
		("newlines: this\nwords\nhas\nnewlines", vec!["newlines:", "this", "words", "has", "newlines"])
	];
	for (code, expected) in &tests {
		compare(code, expected.to_vec());
	}
}

#[test]
fn simple_quotes() {
	let tests = [
		("this sentences has 'single quotes' at the middle", vec!["this", "sentences", "has", "'single quotes'", "at", "the", "middle"]),
		("'single quotes' at the start", vec!["'single quotes'", "at", "the", "start"]),
		("single 'quotes at the end'", vec!["single", "'quotes at the end'"]),
		("'multiple single' 'quotes' in the 'same sentence'", vec!["'multiple single'", "'quotes'", "in", "the", "'same sentence'"]),
	];
	for (code, expected) in &tests {
		compare(code, expected.to_vec());
	}
}

#[test]
fn double_quotes() {
	let tests = [
		("this sentences has \"double quotes\" at the middle", vec!["this", "sentences", "has", "\"double quotes\"", "at", "the", "middle"]),
		("\"double quotes\" at the start", vec!["\"double quotes\"", "at", "the", "start"]),
		("double \"quotes at the end\"", vec!["double", "\"quotes at the end\""]),
		("\"multiple double\" \"quotes\" in the \"same sentence\"", vec!["\"multiple double\"", "\"quotes\"", "in", "the", "\"same sentence\""]),
	];
	for (code, expected) in &tests {
		compare(code, expected.to_vec());
	}
}
