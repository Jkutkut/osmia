use crate::lexer::VariableLexer;
use crate::model::VariableKey;
use crate::macro_tests;

#[cfg(test)]
fn test_lex(
	raw: &str,
	expected: Option<Vec<VariableKey>>
) {
	let actual = match VariableLexer::lex(raw) {
		Some(result) => result,
		None => {
			println!("Failed to lex: {}", raw);
			assert_eq!(expected, None);
			return;
		}
	};
	let actual = actual.into_iter().collect::<Vec<VariableKey>>();
	assert_eq!(actual, expected.unwrap());
}

macro_tests!(
	test_lex,
	(
		test01,
		"foo",
		Some(vec![
			VariableKey::Key("foo".to_string())
		])
	),
	(
		test02,
		"foo.bar",
		Some(vec![
			VariableKey::Key("foo".to_string()),
			VariableKey::Key("bar".to_string())
		])
	),
	(
		test03,
		"foo[0]",
		Some(vec![
			VariableKey::Key("foo".to_string()),
			VariableKey::Index(0)
		])
	),
	(
		test04,
		"foo[0][1]",
		Some(vec![
			VariableKey::Key("foo".to_string()),
			VariableKey::Index(0),
			VariableKey::Index(1)
		])
	),
	(
		test06,
		"foo.bar[0]",
		Some(vec![
			VariableKey::Key("foo".to_string()),
			VariableKey::Key("bar".to_string()),
			VariableKey::Index(0)
		])
	),
	(
		test07,
		"foo[123].bar",
		Some(vec![
			VariableKey::Key("foo".to_string()),
			VariableKey::Index(123),
			VariableKey::Key("bar".to_string())
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
		Some("foo.bar.one.two.three".split('.')
			.map(|s| s.to_string())
			.map(VariableKey::Key).collect())
	),
	(
		test19,
		"snake_case",
		Some(vec![
			VariableKey::Key("snake_case".to_string())
		])
	),
	(
		test20,
		"camelCase",
		Some(vec![
			VariableKey::Key("camelCase".to_string())
		])
	),
	(
		test21,
		"combined_case",
		Some(vec![
			VariableKey::Key("combined_case".to_string())
		])
	),
	(
		test22,
		"camelCase",
		Some(vec![
			VariableKey::Key("camelCase".to_string())
		])
	),
	(
		test23,
		"v1",
		Some(vec![
			VariableKey::Key("v1".to_string())
		])
	),
	(
		test24,
		"combined_case.cammelCase[0].v1.l1_l2",
		Some(vec![
			VariableKey::Key("combined_case".to_string()),
			VariableKey::Key("cammelCase".to_string()),
			VariableKey::Index(0),
			VariableKey::Key("v1".to_string()),
			VariableKey::Key("l1_l2".to_string())
		])
	)
);
