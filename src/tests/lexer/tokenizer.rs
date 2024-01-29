use crate::lexer::Tokenizer;

fn compare(code: &str, expected: Vec<&str>) {
	let result = Tokenizer::new(code)
		.map(|t| t.unwrap())
		.collect::<Vec<&str>>();
	assert_eq!(result, expected);
}

fn fails(code: &str) {
	let mut fails = false;
	for t in Tokenizer::new(code) {
		if let Err(_) = t {
			fails = true;
			break;
		}
	}
	println!("this code should fail:\n{}", code);
	assert!(fails);
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
		("\"spaces:\" this words has spaces", vec![r#""spaces:""#, "this", "words", "has", "spaces"]),
		("\"tabs:\" this\twords\thas\ttabs", vec![r#""tabs:""#, "this", "words", "has", "tabs"]),
		("\"newlines:\" this\nwords\nhas\nnewlines", vec![r#""newlines:""#, "this", "words", "has", "newlines"]),
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
		(
			r#"this sentences has "double quotes" at the middle"#,
			vec!["this", "sentences", "has", r#""double quotes""#, "at", "the", "middle"]
		),
		(
			r#""double quotes" at the start"#,
			vec![r#""double quotes""#, "at", "the", "start"]
		),
		(
			r#"double "quotes at the end""#,
			vec!["double", r#""quotes at the end""#]
		),
		(
			r#""multiple double" "quotes" in the "same sentence""#,
			vec![r#""multiple double""#, r#""quotes""#, "in", "the", r#""same sentence""#]
		),
	];
	for (code, expected) in &tests {
		compare(code, expected.to_vec());
	}
}

#[test]
fn multiple_quotes() {
	let tests = [
		(
			r#"sentences "with multiple" "quotes together""#,
			vec!["sentences", r#""with multiple""#, r#""quotes together""#]
		),
		(
			r#"sentences 'with multiple' 'quotes together'"#,
			vec!["sentences", r#"'with multiple'"#, r#"'quotes together'"#]
		),
		(
			r#"sentences "with multiple" 'quotes together'"#,
			vec!["sentences", r#""with multiple""#, r#"'quotes together'"#]
		),
		(
			r#"sentences 'with multiple' "quotes together""#,
			vec!["sentences", r#"'with multiple'"#, r#""quotes together""#]
		)
	];
	for (code, expected) in &tests {
		compare(code, expected.to_vec());
	}
}

#[test]
fn quotes_together() {
	let tests = [
		r#"this text contains "quotes"" without separation""#,
		r#"this text contains "quotes"' without separation'"#,
		r#"this text contains 'quotes'" without separation"#,
		r#"this text contains 'quotes'' without separation'"#,
	];
	for code in &tests {
		fails(code);
	}
}

#[test]
fn tokens_together() {
	let tests = [
		"this 'text'contains",
		"123\"text\"",
		"123'text'",
		"123\"text\"123'text'",
		"123'text'123\"text\"",
	];
	for code in &tests {
		fails(code);
	}
}

#[test]
fn shoud_fail() {
	let tests = [
		"\"",
		"\"invalid quoted text'",
		"'",
		"'invalid quoted text\"",
	];
	for code in &tests {
		fails(code);
	}
}

// Symbols
#[test]
fn symbols01() {
	let tests = [
		("1 + 1", vec!["1", "+", "1"]),
		("1+1", vec!["1", "+", "1"]),
		("1-1", vec!["1", "-", "1"]),
		("1*1", vec!["1", "*", "1"]),
		("1/1", vec!["1", "/", "1"]),
	];
	for (code, expected) in &tests {
		compare(code, expected.to_vec());
	}
}

#[test]
fn symbols02() {
	let tests = [
		("1*(2+3)", vec!["1", "*", "(", "2", "+", "3", ")"]),
		("(2-2*(3+2/3)*2", vec!["(", "2", "-", "2", "*", "(", "3", "+", "2", "/", "3", ")", "*", "2"]),
		("(  2-2 * (3 + 2/ 3) *2", vec!["(", "2", "-", "2", "*", "(", "3", "+", "2", "/", "3", ")", "*", "2"]),
	];
	for (code, expected) in &tests {
		compare(code, expected.to_vec());
	}
}

// Variables

#[test]
fn variables01() {
	let tests = [
		(
			"a + 1 == true",
			vec!["a", "+", "1", "==", "true"]
		),
		(
			"\"hola\" + b + 'hola'",
			vec![r#""hola""#, "+", "b", "+", r#"'hola'"#]
		),
		(
			"abc1+1 + hola",
			vec!["abc1", "+", "1", "+", "hola"]
		)
	];
	for (code, expected) in &tests {
		compare(code, expected.to_vec());
	}
}

#[test]
fn variables02() {
	let tests = [
		(
			"a.b == c.d",
			vec!["a.b", "==", "c.d"]
		),
		(
			"abc[1] == def[2]",
			vec!["abc[1]", "==", "def[2]"]
		),
		(
			"abc[1].c = def[2].a",
			vec!["abc[1].c", "=", "def[2].a"]
		)
	];
	for (code, expected) in &tests {
		compare(code, expected.to_vec());
	}
}

// Operators

#[test]
fn operators() {
	let tests = [
		"==", "!=",
		"<", ">",
		"<=", ">=",
		"+", "-",
		"*", "/",
		"&&", "||",
	];
	for operator in &tests {
		let test = vec!["a", operator, "b"];
		compare(&test.join(" "), test.to_vec());
	}
}
