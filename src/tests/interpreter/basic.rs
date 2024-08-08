use crate::macro_tests;
use super::{test_interpreter_basic};

macro_tests! {
	test_interpreter_basic,
	(empty, "", ""),
	(just_text, "Hello, world!", "Hello, world!"),
	(basic01, "{{true}}", "true"),
	(basic02, "{{false}}", "false"),
	(basic03, "{{null}}", "null"),
	(basic04, "{{42}}", "42"),
	(basic05, "{{3.14}}", "3.14"),
	(basic06, r#"{{"Hello, world!"}}"#, "Hello, world!"),
	(basic07, r#"{{""}}"#, ""),
	(basic08, r#"{{"\n"}}"#, "\\n"),
	(basic09, r#"{{"\r"}}"#, "\\r"),
	(basic10, r#"{{"\t"}}"#, "\\t"),
	(basic11, "{{true}} {{false}}", "true false"),
	(basic12, "{{true}} {{false}} {{null}}", "true false null"),
	(basic13, "{{true}} {{false}} {{42}}", "true false 42"),
	(basic14, "{{true}} {{false}} {{3.14}}", "true false 3.14"),
	(basic15, r#"{{"Hello, world!"}} {{42}}"#, "Hello, world! 42"),
	(json01, "{{ [1, 2, 3] }}", "[1, 2, 3]"),
	(json02, r#"{{ {"a": 1, "b": 2, "c": 3} }}"#, "{a: 1, b: 2, c: 3}"),
	(
		json03,
		r#"{{ [ 1, 2, {"foo": [3, 4]} ] }}"#,
		"[1, 2, {foo: [3, 4]}]"
	),
	(
		json04,
		r#"{{ {"bar": [4, 5, 6], "foo": [1, 2, 3]} }}"#,
		"{bar: [4, 5, 6], foo: [1, 2, 3]}"
	),
	(
		json05,
		r#"{{ {} }} {{ [] }}"#,
		"{} []"
	),
	(
		json06,
		r#"{{ { } }} {{ [ ] }}"#,
		"{} []"
	)
}
