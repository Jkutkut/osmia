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
	(basic15, r#"{{"Hello, world!"}} {{42}}"#, "Hello, world! 42")
}
