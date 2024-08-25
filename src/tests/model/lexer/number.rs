use super::*;

macro_tests!(
	lexer_expression_test,
	(number_01, "123", vec![Token::new_number("123")]),
	(number_02, "123.2", vec![Token::new_number("123.2")]),
	(number_03, "123 . 2", vec![Token::new_number("123"), Token::Dot, Token::new_number("2")]),
	(number_04, "123 321", vec![Token::new_number("123"), Token::new_number("321")]),
	(number_05, "0.0 9.9", vec![Token::new_number("0.0"), Token::new_number("9.9")]),
	(number_06, "123 .2", vec![Token::new_number("123"), Token::Dot, Token::new_number("2")]),
	(number_07, "123 .2.2", vec![Token::new_number("123"), Token::Dot, Token::new_number("2.2")]),
	(number_08, "123456789123456789", vec![Token::new_number("123456789123456789")]),
	(number_09, "123456789123456789.123456789123456789", vec![Token::new_number("123456789123456789.123456789123456789")]),
	(number_10, "123\n23\t1", vec![Token::new_number("123"), Token::NewLine, Token::new_number("23"), Token::new_number("1")]),
);

macro_tests!(
	lexer_expression_test_fail,
	(invalid_number_01, "123.", "expected num"),
	(invalid_number_02, "123..", "expected num"),
	(invalid_number_03, "123.2.", "dot"),
	(invalid_number_04, "123.2.3", "dot"),
);
