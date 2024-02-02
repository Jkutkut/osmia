use crate::lexer::VariableLexer;
use crate::syntax_tree::model::VariableKey;
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
	)
);
