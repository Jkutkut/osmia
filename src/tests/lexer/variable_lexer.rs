use crate::lexer::VariableLexer;
use crate::model::VariableKey;
use crate::macro_tests;

#[cfg(test)]
fn test_lex(
	raw: &str,
	expected: Option<Vec<VariableKey<'_>>>
) {
	let actual = match VariableLexer::lex(raw) {
		Some(result) => result,
		None => {
			println!("Failed to lex: {}", raw);
			assert_eq!(expected, None);
			return;
		}
	};
	let actual = actual.into_iter().collect::<Vec<VariableKey<'_>>>();
	assert_eq!(actual, expected.unwrap());
}

macro_tests!(
	test_lex,
	(
		test01,
		"foo",
		Some(vec![
			VariableKey::Key("foo")
		])
	),
	(
		test02,
		"foo.bar",
		Some(vec![
			VariableKey::Key("foo"),
			VariableKey::Key("bar")
		])
	),
	(
		test03,
		"foo[0]",
		Some(vec![
			VariableKey::Key("foo"),
			VariableKey::Index(0)
		])
	),
	(
		test04,
		"foo[0][1]",
		Some(vec![
			VariableKey::Key("foo"),
			VariableKey::Index(0),
			VariableKey::Index(1)
		])
	),
	(
		test06,
		"foo.bar[0]",
		Some(vec![
			VariableKey::Key("foo"),
			VariableKey::Key("bar"),
			VariableKey::Index(0)
		])
	),
	(
		test07,
		"foo[123].bar",
		Some(vec![
			VariableKey::Key("foo"),
			VariableKey::Index(123),
			VariableKey::Key("bar")
		])
	),
	(test05, "", None),
	(test08, "[123]", None),
	(test09, ".", None),
	(test10, "[", None),
	(test11, "]", None),
	(test12, "foo[", None),
	(test13, "foo]", None),
	(test14, "foo[bar", None),
	(test15, "foo[bar]", None),
	(test16, "foo.[bar]", None),
	(test17, "foo[]", None),
	(
		test18,
		"foo.bar.one.two.three",
		Some("foo.bar.one.two.three".split('.').map(VariableKey::Key).collect())
	),
	(
		test19,
		"snake_case",
		Some(vec![
			VariableKey::Key("snake_case")
		])
	),
	(
		test20,
		"camelCase",
		Some(vec![
			VariableKey::Key("camelCase")
		])
	),
	(
		test21,
		"combined_case",
		Some(vec![
			VariableKey::Key("combined_case")
		])
	),
	(
		test22,
		"camelCase",
		Some(vec![
			VariableKey::Key("camelCase")
		])
	),
	(
		test23,
		"v1",
		Some(vec![
			VariableKey::Key("v1")
		])
	),
	(
		test24,
		"combined_case.cammelCase[0].v1.l1_l2",
		Some(vec![
			VariableKey::Key("combined_case"),
			VariableKey::Key("cammelCase"),
			VariableKey::Index(0),
			VariableKey::Key("v1"),
			VariableKey::Key("l1_l2")
		])
	)
);
