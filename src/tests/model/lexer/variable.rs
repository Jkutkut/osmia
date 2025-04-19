use super::*;

macro_tests!(
	lexer_expression_test,
	(var_01, "foo", vec![Token::new_alpha("foo")]),
	(var_02, "_foo", vec![Token::new_alpha("_foo")]),
	(var_03, "foo_bar", vec![Token::new_alpha("foo_bar")]),
	(var_04, "_foo_bar_", vec![Token::new_alpha("_foo_bar_")]),
	(var_05, "foo bar baz", vec![
		Token::new_alpha("foo"), Token::Whitespace, Token::new_alpha("bar"),
		Token::Whitespace, Token::new_alpha("baz")
	]),
	(var_nbr_start, "1foo", vec![Token::new_number("1"), Token::new_alpha("foo")]),
	(var_nbr_end, "1.2foo", vec![Token::new_number("1.2"), Token::new_alpha("foo")]),
	(const_01, "FOO", vec![Token::new_alpha("FOO")]),
	(const_02, "_FOO", vec![Token::new_alpha("_FOO")]),
	(const_03, "_FOO_BAR_", vec![Token::new_alpha("_FOO_BAR_")]),
	(const_04, "_FOO_BAR123", vec![Token::new_alpha("_FOO_BAR123")]),
	(nbr_01, "foo1", vec![Token::new_alpha("foo1")]),
	(nbr_02, "FOO1", vec![Token::new_alpha("FOO1")]),
	(cammel_01, "fooBar", vec![Token::new_alpha("fooBar")]),
	(cammel_02, "fooBar1", vec![Token::new_alpha("fooBar1")]),
	(json_01, "foo.bar", vec![Token::new_alpha("foo"), Token::Dot, Token::new_alpha("bar")]),
	(json_02, "foo[1]", vec![Token::new_alpha("foo"), Token::ArrayStart, Token::new_number("1"), Token::ArrayEnd]),
	(json_03, "foo[1].bar", vec![
		Token::new_alpha("foo"), Token::ArrayStart, Token::new_number("1"),
		Token::ArrayEnd, Token::Dot, Token::new_alpha("bar")
	]),
	(json_04, "foo[bar][foo.baz]", vec![
		Token::new_alpha("foo"), Token::ArrayStart, Token::new_alpha("bar"), Token::ArrayEnd,
		Token::ArrayStart, Token::new_alpha("foo"), Token::Dot, Token::new_alpha("baz"), Token::ArrayEnd
	]),
	(json_05, "foo.bar\nbaz", vec![
		Token::new_alpha("foo"), Token::Dot, Token::new_alpha("bar"),
		Token::NewLine, Token::new_alpha("baz")
	]),
	(json_06, "foo.bar baz", vec![
		Token::new_alpha("foo"), Token::Dot, Token::new_alpha("bar"),
		Token::Whitespace, Token::new_alpha("baz")
	]),
	(inner_keyword_01, "if_", vec![Token::new_alpha("if_")]),
	(inner_keyword_02, "while_", vec![Token::new_alpha("while_")]),
	(inner_keyword_03, "for_", vec![Token::new_alpha("for_")]),
	(inner_keyword_04, "_in", vec![Token::new_alpha("_in")]),
	(inner_keyword_05, "_if", vec![Token::new_alpha("_if")]),
	(inner_keyword_06, "_while", vec![Token::new_alpha("_while")]),
	(inner_keyword_07, "_for", vec![Token::new_alpha("_for")]),
	(inner_keyword_08, "_if_", vec![Token::new_alpha("_if_")]),
	(inner_keyword_09, "_while_", vec![Token::new_alpha("_while_")]),
	(inner_keyword_10, "_for_", vec![Token::new_alpha("_for_")]),
);

macro_tests!(
	lexer_expression_test_fail,
	(invalid_tilda, "~foo", "'~'"),
	(invalid_at, "@foo", "'@'"),
	(invalid_dollar, "$foo", "'$'"),
	(invalid_dollar_02, "foo$foo", "'$'"),
);
