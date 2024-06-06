use crate::lexer::Tokenizer;
use crate::macro_tests;

#[cfg(test)]
fn compare(code: &str, expected: Vec<&str>) {
	let result = Tokenizer::new(code)
		.map(|t| t.unwrap())
		.collect::<Vec<&str>>();
	assert_eq!(result, expected);
}

#[cfg(test)]
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

macro_tests!(
	compare,
	(basic_test, "", vec![]),
	(basic_test2, "this is a test", vec!["this", "is", "a", "test"]),
	(multiple_spaces_test, "this is   a    test", vec!["this", "is", "a", "test"]),
	(white_spaces01, "\"spaces:\" this words has spaces", vec![r#""spaces:""#, "this", "words", "has", "spaces"]),
	(white_spaces02, "\"tabs:\" this\twords\thas\ttabs", vec![r#""tabs:""#, "this", "words", "has", "tabs"]),
	(white_spaces03, "\"newlines:\" this\nwords\nhas\nnewlines", vec![r#""newlines:""#, "this", "words", "has", "newlines"]),
	(simple_quotes01, "this sentences has 'single quotes' at the middle", vec!["this", "sentences", "has", "'single quotes'", "at", "the", "middle"]),
	(simple_quotes02, "'single quotes' at the start", vec!["'single quotes'", "at", "the", "start"]),
	(simple_quotes03, "single 'quotes at the end'", vec!["single", "'quotes at the end'"]),
	(simple_quotes04, "'multiple single' 'quotes' in the 'same sentence'", vec!["'multiple single'", "'quotes'", "in", "the", "'same sentence'"]),
	(simple_quotes05, "this 'is' a 'test'", vec!["this", "'is'", "a", "'test'"]),
	(
		double_quotes01,
		r#"this sentences has "double quotes" at the middle"#,
		vec!["this", "sentences", "has", r#""double quotes""#, "at", "the", "middle"]
	),
	(
		double_quotes02,
		r#""double quotes" at the start"#,
		vec![r#""double quotes""#, "at", "the", "start"]
	),
	(
		double_quotes03,
		r#"double "quotes at the end""#,
		vec!["double", r#""quotes at the end""#]
	),
	(
		double_quotes04,
		r#""multiple double" "quotes" in the "same sentence""#,
		vec![r#""multiple double""#, r#""quotes""#, "in", "the", r#""same sentence""#]
	),
	(
		multiple_quotes01,
		r#"sentences "with multiple" "quotes together""#,
		vec!["sentences", r#""with multiple""#, r#""quotes together""#]
	),
	(
		multiple_quotes02,
		r#"sentences 'with multiple' 'quotes together'"#,
		vec!["sentences", r#"'with multiple'"#, r#"'quotes together'"#]
	),
	(
		multiple_quotes03,
		r#"sentences "with multiple" 'quotes together'"#,
		vec!["sentences", r#""with multiple""#, r#"'quotes together'"#]
	),
	(
		multiple_quotes04,
		r#"sentences 'with multiple' "quotes together""#,
		vec!["sentences", r#"'with multiple'"#, r#""quotes together""#]
	)
);

macro_tests!(
	fails,
	(quotes_together01,r#"this text contains "quotes"" without separation""#),
	(quotes_together02, r#"this text contains "'quotes' without separation"#),
	(quotes_together03, r#"this text contains "quotes"' without separation'"#),
	(quotes_together04, r#"this text contains 'quotes'" without separation"#),
	(quotes_together05, "this 'text'contains"),
	(quotes_together06, "123\"text\""),
	(quotes_together07, "123'text'"),
	(quotes_together08, "123\"text\"123'text'"),
	(quotes_together09, "123'text'123\"text\""),
	(shoud_fail01, "\""),
	(shoud_fail02, "\"invalid quoted text'"),
	(shoud_fail03, "'"),
	(shoud_fail04, "'invalid quoted text\"")
);

macro_tests!(
	compare,
	(symbols01, "1 + 1", vec!["1", "+", "1"]),
	(symbols02, "1+1", vec!["1", "+", "1"]),
	(symbols03, "1-1", vec!["1", "-", "1"]),
	(symbols04, "1*1", vec!["1", "*", "1"]),
	(symbols05, "1/1", vec!["1", "/", "1"]),
	(symbols06, "1%1", vec!["1", "%", "1"]),
	(symbols07, "1*(2+3)", vec!["1", "*", "(", "2", "+", "3", ")"]),
	(symbols08, "(2-2*(3+2/3)*2", vec!["(", "2", "-", "2", "*", "(", "3", "+", "2", "/", "3", ")", "*", "2"]),
	(symbols09, "(  2-2 * (3 + 2/ 3) *2", vec!["(", "2", "-", "2", "*", "(", "3", "+", "2", "/", "3", ")", "*", "2"]),
	(variables01, "a + 1 == true", vec!["a", "+", "1", "==", "true"]),
	(variables02, "abc1+1 + hola", vec!["abc1", "+", "1", "+", "hola"]),
	(variables03, "a.b == c.d", vec!["a.b", "==", "c.d"]),
	(variables04, "abc[1] == def[2]", vec!["abc[1]", "==", "def[2]"]),
	(variables05, "a.b[1] == c.d[2]", vec!["a.b[1]", "==", "c.d[2]"]),
	(variables06, "a.b[1] == c.d[2] == e.f[3]", vec!["a.b[1]", "==", "c.d[2]", "==", "e.f[3]"]),
	(variable07, "a + 1 == true", vec!["a", "+", "1", "==", "true"]),
	(variable08, "\"hola\" + b + 'hola'", vec![r#""hola""#, "+", "b", "+", r#"'hola'"#]),
	(variable09, "abc1+1 + hola", vec!["abc1", "+", "1", "+", "hola"]),
	(variable10, "a.b == c.d", vec!["a.b", "==", "c.d"]),
	(variable11, "abc[1] == def[2]", vec!["abc[1]", "==", "def[2]"]),
	(variable12, "a.b[1].c == def[2].ghi", vec!["a.b[1].c", "==", "def[2].ghi"]),
	(operator01, "a == b", vec!["a", "==", "b"]),
	(operator02, "a != b", vec!["a", "!=", "b"]),
	(operator03, "a < b", vec!["a", "<", "b"]),
	(operator04, "a > b", vec!["a", ">", "b"]),
	(operator05, "a <= b", vec!["a", "<=", "b"]),
	(operator06, "a >= b", vec!["a", ">=", "b"]),
	(operator07, "a + b", vec!["a", "+", "b"]),
	(operator08, "a - b", vec!["a", "-", "b"]),
	(operator09, "a * b", vec!["a", "*", "b"]),
	(operator10, "a / b", vec!["a", "/", "b"]),
	(operator11, "a % b", vec!["a", "%", "b"]),
	(operator12, "a && b", vec!["a", "&&", "b"]),
	(operator13, "a || b", vec!["a", "||", "b"]),
	(operator14, "a & b", vec!["a", "&", "b"]),
	(operator15, "a | b", vec!["a", "|", "b"]),
	(operator16, "a ^ b", vec!["a", "^", "b"]),
	(operator17, "a << b", vec!["a", "<<", "b"]),
	(operator18, "a >> b", vec!["a", ">>", "b"])
);
