use crate::macro_tests;
use super::{test_interpreter, expect_error};

macro_tests!(
	test_interpreter,
	(
		variable01,
		"{{ foo }}",
		r#"{"foo": "bar"}"#,
		"bar"
	),
	(
		variable02,
		"{{ foo.bar }}",
		r#"{"foo": {"bar": "baz"}}"#,
		"baz"
	),
	(
		variable03,
		"{{ foo.bar.baz }}",
		r#"{"foo": {"bar": {"baz": "qux"}}}"#,
		"qux"
	),
	(
		variable04,
		"{{ arr[0] }} {{ arr[1] }}",
		r#"{"arr": ["foo", "bar"]}"#,
		"foo bar"
	),
	(
		variable05,
		"{{ arr[0].name }} {{ arr[1].surname }}",
		r#"{"arr": [{"name": "foo"}, {"name": "bar", "surname": "baz"}]}"#,
		"foo baz"
	)
);

macro_tests!(
	expect_error,
	(
		invalid_variable01,
		"{{ foo }}",
		r#"{}"#
	),
	(
		invalid_variable02,
		"{{ foo.bar }}",
		r#"{"foo": []}"#
	),
	(
		invalid_variable03,
		"{{ foo.bar }}",
		r#"{"foo": {"other": 42}}"#
	),
	(
		invalid_variable_index01,
		"{{ arr[0] }}",
		r#"{"arr": []}"#
	),
	(
		invalid_variable_index02,
		"{{ arr[12] }}",
		r#"{"arr": [1, 2, 3]}"#
	)
);

// TODO: Allow this advanced tests
// macro_tests!(
// 	test_interpreter,
// 	(
// 		advanced_json_control01,
// 		r#"{{ foo[v] }}"#,
// 		r#"{"foo": [1, 2, 3], "v": 1}"#,
// 		"2"
// 	),
// 	(
// 		advanced_json_control02,
// 		r#"{{ foo["bar"] }}"#,
// 		r#"{"foo": {"bar": "baz"}, "v": 1}"#,
// 		"baz"
// 	),
// 	(
// 		advanced_json_control03,
// 		r#"{{ foo['bar'] }}"#,
// 		r#"{"foo": {"bar": "baz"}, "v": 1}"#,
// 		"baz"
// 	),
// 	(
// 		advanced_json_control04,
// 		r#"{{ foo[v] }}"#,
// 		r#"{"foo": {"bar": "baz"}, "v": "bar"}"#,
// 		"baz"
// 	)
// );
