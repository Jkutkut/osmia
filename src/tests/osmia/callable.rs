use super::*;

macro_tests!(
	test,
	(
		function_call01,
		Some("{{ foo() }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		function_call02,
		Some("{{ foo(bar) }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::new_alpha("bar"),
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		function_call03,
		Some("{{ foo(12, 34) }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::new_number("12"),
			Token::Comma,
			Token::Whitespace,
			Token::new_number("34"),
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		function_call_multiple01,
		Some("{{ foo()() }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::ParentEnd,
			Token::ParentStart,
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		function_call_multiple02,
		Some("{{ foo(1)(2) }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::new_number("1"),
			Token::ParentEnd,
			Token::ParentStart,
			Token::new_number("2"),
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		function_call_multiple03,
		Some(r#"{{ foo(1 + bar(v), "hello") }}"#),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::new_number("1"),
			Token::Whitespace,
			Token::Plus,
			Token::Whitespace,
			Token::new_alpha("bar"),
			Token::ParentStart,
			Token::new_alpha("v"),
			Token::ParentEnd,
			Token::Comma,
			Token::Whitespace,
			Token::new_str("hello"),
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		function_call_multiple04,
		Some(r#"{{ foo(bar(foo(bar()))) }}"#),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::new_alpha("bar"),
			Token::ParentStart,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::new_alpha("bar"),
			Token::ParentStart,
			Token::ParentEnd,
			Token::ParentEnd,
			Token::ParentEnd,
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		function_call_multiple05,
		Some("{{ foo((1 + 1)) }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::ParentStart,
			Token::new_number("1"),
			Token::Whitespace,
			Token::Plus,
			Token::Whitespace,
			Token::new_number("1"),
			Token::ParentEnd,
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		function_call_multiple06,
		Some("{{ foo((1)) }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::ParentStart,
			Token::new_number("1"),
			Token::ParentEnd,
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		method_call_01,
		Some("{{ var?foo() }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("var"),
			Token::Question,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		method_call_02,
		Some("{{ var[2]?foo() }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("var"),
			Token::ArrayStart,
			Token::new_number("2"),
			Token::ArrayEnd,
			Token::Question,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		method_call_03,
		Some("{{ usr.name?foo() }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("usr"),
			Token::Dot,
			Token::new_alpha("name"),
			Token::Question,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		method_call_04,
		Some("{{ 1?foo() }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_number("1"),
			Token::Question,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		method_call_05,
		Some(r#"{{ "str"?foo() }}"#),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_str("str"),
			Token::Question,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		method_call_06,
		Some("{{ null?foo() }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::Null,
			Token::Question,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		method_call_07,
		Some("{{ true?foo() }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::Bool(true),
			Token::Question,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		method_call_08,
		Some("{{ false?foo() }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::Bool(false),
			Token::Question,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		method_call_09,
		Some("{{ (1)?foo() }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::ParentStart,
			Token::new_number("1"),
			Token::ParentEnd,
			Token::Question,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		method_call_10,
		Some("{{ 1 + 1?foo() }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_number("1"),
			Token::Whitespace,
			Token::Plus,
			Token::Whitespace,
			Token::new_number("1"),
			Token::Question,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		method_call_11,
		Some("{{ true && 1?foo() }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::Bool(true),
			Token::Whitespace,
			Token::And,
			Token::Whitespace,
			Token::new_number("1"),
			Token::Question,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		method_call_12,
		Some("{{ foo()?bar()?baz() }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::ParentEnd,
			Token::Question,
			Token::new_alpha("bar"),
			Token::ParentStart,
			Token::ParentEnd,
			Token::Question,
			Token::new_alpha("baz"),
			Token::ParentStart,
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
	(
		method_call_13,
		Some("{{ foo(1, 2, 3)?add(add(1, 2), 3) }}"),
		Some(vec![
			Token::StmtStart,
			Token::Whitespace,
			Token::new_alpha("foo"),
			Token::ParentStart,
			Token::new_number("1"),
			Token::Comma,
			Token::Whitespace,
			Token::new_number("2"),
			Token::Comma,
			Token::Whitespace,
			Token::new_number("3"),
			Token::ParentEnd,
			Token::Question,
			Token::new_alpha("add"),
			Token::ParentStart,
			Token::new_alpha("add"),
			Token::ParentStart,
			Token::new_number("1"),
			Token::Comma,
			Token::Whitespace,
			Token::new_number("2"),
			Token::ParentEnd,
			Token::Comma,
			Token::Whitespace,
			Token::new_number("3"),
			Token::ParentEnd,
			Token::Whitespace,
			Token::StmtEnd,
			Token::Eof
		]),
		None,
		None
	),
);
