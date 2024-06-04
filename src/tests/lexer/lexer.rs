use crate::lexer::Token;
use crate::lexer::Lexer;

use crate::macro_tests;

#[cfg(test)]
fn lexer() -> Lexer<'static> {
	Lexer::new("{{", "}}")
}

#[cfg(test)]
fn check_result(
	real: Result<Vec<Token>, String>,
	expected: Vec<Token>
) {
	if let Err(err) = real {
		panic!("Error: {}", err);
	}
	let real = real.unwrap();
	println!("real    : {:?}", real);
	println!("expected: {:?}", expected);
	for (real, expected) in real.iter().zip(expected.iter()) {
		if real != expected {
			println!("real vs expected: {:?} vs {:?}", real, expected);
		}
	}
	assert_eq!(real, expected);
}

#[cfg(test)]
fn lexer_test(
	input: &str,
	expected: Vec<Token>
) {
	let lexer = lexer();
	let result = lexer.scan(input);
	check_result(result, expected);
}

#[cfg(test)]
fn lexer_fail_test(
	input: &str
) {
	let lexer = lexer();
	let result = lexer.scan(input);
	if let Ok(_) = result {
		panic!("Should fail");
	}
	assert!(result.is_err());
}

macro_tests!(
	lexer_test,
	(
		basic01,
		"",
		vec![Token::Eof]
	),
	(
		basic02,
		"{{1}}",
		vec![
			Token::DelimiterStart,
			Token::Value("1".to_string()),
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		basic03,
		"{{1.0}}",
		vec![
			Token::DelimiterStart,
			Token::Value("1.0".to_string()),
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		basic04,
		"{{true}}",
		vec![
			Token::DelimiterStart,
			Token::Value("true".to_string()),
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		basic05,
		"{{false}}",
		vec![
			Token::DelimiterStart,
			Token::Value("false".to_string()),
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		basic06,
		"{{null}}",
		vec![
			Token::DelimiterStart,
			Token::Value("null".to_string()),
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		basic07,
		"{{\"hello world\"}}",
		vec![
			Token::DelimiterStart,
			Token::Value(r#""hello world""#.to_string()),
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		expr_str,
		"{{1}} test",
		vec![
			Token::DelimiterStart,
			Token::Value("1".to_string()),
			Token::DelimiterEnd,
			Token::Raw(" test".to_string()),
			Token::Eof
		]
	),
	(
		str_expr,
		"test {{123}}",
		vec![
			Token::Raw("test ".to_string()),
			Token::DelimiterStart,
			Token::Value("123".to_string()),
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		str_expr_str,
		"test {{true}} test",
		vec![
			Token::Raw("test ".to_string()),
			Token::DelimiterStart,
			Token::Value("true".to_string()),
			Token::DelimiterEnd,
			Token::Raw(" test".to_string()),
			Token::Eof
		]
	),
	(
		expr_str_expr,
		"{{123}} test {{456}}",
		vec![
			Token::DelimiterStart,
			Token::Value("123".to_string()),
			Token::DelimiterEnd,
			Token::Raw(" test ".to_string()),
			Token::DelimiterStart,
			Token::Value("456".to_string()),
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		multiple_expr,
		"test {{1}}tcc{{true}}vvvvv{{false}}aaaa",
		vec![
			Token::Raw("test ".to_string()),
			Token::DelimiterStart,
			Token::Value("1".to_string()),
			Token::DelimiterEnd,
			Token::Raw("tcc".to_string()),
			Token::DelimiterStart,
			Token::Value("true".to_string()),
			Token::DelimiterEnd,
			Token::Raw("vvvvv".to_string()),
			Token::DelimiterStart,
			Token::Value("false".to_string()),
			Token::DelimiterEnd,
			Token::Raw("aaaa".to_string()),
			Token::Eof
		]
	),
	(
		whitespaces,
		"{{\n\r\t 1\n+\n  1\t-1}}",
		vec![
			Token::DelimiterStart,
			Token::Value("1".to_string()),
			Token::Plus,
			Token::Value("1".to_string()),
			Token::Minus,
			Token::Value("1".to_string()),
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		operators,
		"{{+-*/%! && ||}}",
		vec![
			Token::DelimiterStart,
			Token::Plus,
			Token::Minus,
			Token::Multiply,
			Token::Divide,
			Token::Modulo,
			Token::Not,
			Token::And,
			Token::Or,
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		operators2,
		"{{()[]{}}}",
		vec![
			Token::DelimiterStart,
			Token::GroupingStart,
			Token::GroupingEnd,
			Token::ArrayStart,
			Token::ArrayEnd,
			Token::ObjectStart,
			Token::ObjectEnd,
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		operators3,
		"{{== != <= >= < >}}",
		vec![
			Token::DelimiterStart,
			Token::Equal,
			Token::NotEqual,
			Token::LessEqual,
			Token::GreaterEqual,
			Token::LessThan,
			Token::GreaterThan,
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		operators4,
		"{{= : ,}}",
		vec![
			Token::DelimiterStart,
			Token::AssignEq,
			Token::Colon,
			Token::Comma,
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		keywords,
		r#"
		{{print}}{{assign =}}
		{{if}}{{elseif}}{{else}}{{fi}}
		{{while}}{{for in}}{{done}}
		{{continue}}{{break}}
		"#,
		vec![
			Token::DelimiterStart,
			Token::Print,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Assign,
			Token::AssignEq,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::If,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::ElseIf,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Else,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Fi,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::While,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::For,
			Token::In,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Done,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Continue,
			Token::DelimiterEnd,
			Token::DelimiterStart,
			Token::Break,
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		json01,
		"{{[1, 2, 3]}}",
		vec![
			Token::DelimiterStart,
			Token::ArrayStart,
			Token::Value("1".to_string()),
			Token::Comma,
			Token::Value("2".to_string()),
			Token::Comma,
			Token::Value("3".to_string()),
			Token::ArrayEnd,
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		json02,
		r#"{{{"a": 1}}}"#,
		vec![
			Token::DelimiterStart,
			Token::ObjectStart,
			Token::Value(r#""a""#.to_string()),
			Token::Colon,
			Token::Value("1".to_string()),
			Token::ObjectEnd,
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		json03,
		r#"{{{"a": 1, "b":{"b": 2}}}}"#,
		vec![
			Token::DelimiterStart,
			Token::ObjectStart,
			Token::Value(r#""a""#.to_string()),
			Token::Colon,
			Token::Value("1".to_string()),
			Token::Comma,
			Token::Value(r#""b""#.to_string()),
			Token::Colon,
			Token::ObjectStart,
			Token::Value(r#""b""#.to_string()),
			Token::Colon,
			Token::Value("2".to_string()),
			Token::ObjectEnd,
			Token::ObjectEnd,
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		json04,
		r#"{{{"a": [], "b":{"c":{"d": 2}}}}}"#,
		vec![
			Token::DelimiterStart,
			Token::ObjectStart,
			Token::Value(r#""a""#.to_string()),
			Token::Colon,
			Token::ArrayStart,
			Token::ArrayEnd,
			Token::Comma,
			Token::Value(r#""b""#.to_string()),
			Token::Colon,
			Token::ObjectStart,
			Token::Value(r#""c""#.to_string()),
			Token::Colon,
			Token::ObjectStart,
			Token::Value(r#""d""#.to_string()),
			Token::Colon,
			Token::Value("2".to_string()),
			Token::ObjectEnd,
			Token::ObjectEnd,
			Token::ObjectEnd,
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		json05,
		r#"{{[{"a": 1}, [{"b": "2"}]]}}"#,
		vec![
			Token::DelimiterStart,
			Token::ArrayStart,
			Token::ObjectStart,
			Token::Value(r#""a""#.to_string()),
			Token::Colon,
			Token::Value("1".to_string()),
			Token::ObjectEnd,
			Token::Comma,
			Token::ArrayStart,
			Token::ObjectStart,
			Token::Value(r#""b""#.to_string()),
			Token::Colon,
			Token::Value(r#""2""#.to_string()),
			Token::ObjectEnd,
			Token::ArrayEnd,
			Token::ArrayEnd,
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		json06,
		r#"{{[null]}}"#,
		vec![
			Token::DelimiterStart,
			Token::ArrayStart,
			Token::Value("null".to_string()),
			Token::ArrayEnd,
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		json07,
		r#"{{[[true],[false]]}}"#,
		vec![
			Token::DelimiterStart,
			Token::ArrayStart,
			Token::ArrayStart,
			Token::Value("true".to_string()),
			Token::ArrayEnd,
			Token::Comma,
			Token::ArrayStart,
			Token::Value("false".to_string()),
			Token::ArrayEnd,
			Token::ArrayEnd,
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		json08,
		r#"{{{"a": true}}}"#,
		vec![
			Token::DelimiterStart,
			Token::ObjectStart,
			Token::Value(r#""a""#.to_string()),
			Token::Colon,
			Token::Value("true".to_string()),
			Token::ObjectEnd,
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		expression01,
		"{{ (1 + 2) }}",
		vec![
			Token::DelimiterStart,
			Token::GroupingStart,
			Token::Value("1".to_string()),
			Token::Plus,
			Token::Value("2".to_string()),
			Token::GroupingEnd,
			Token::DelimiterEnd,
			Token::Eof
		]
	),
	(
		same_expression01,
		"{{ (1 + 2) }} {{ ( 1+2 ) }}",
		vec![
			Token::DelimiterStart,
			Token::GroupingStart,
			Token::Value("1".to_string()),
			Token::Plus,
			Token::Value("2".to_string()),
			Token::GroupingEnd,
			Token::DelimiterEnd,
			Token::Raw(" ".to_string()),
			Token::DelimiterStart,
			Token::GroupingStart,
			Token::Value("1".to_string()),
			Token::Plus,
			Token::Value("2".to_string()),
			Token::GroupingEnd,
			Token::DelimiterEnd,
			Token::Eof
		]
	)
);

macro_tests!(
	lexer_fail_test,
	(invalid01, "{{"),
	(invalid02, "this is invalid {{"),
	(invalid03, "{{ this is invalid"),
	(invalid04, "{{ this is invalid$ }}"),
	(invalid05, "{{ #print }}")
);
