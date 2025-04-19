use crate::macro_tests;
use crate::utils::code_trace;

#[cfg(test)]
const CODE: &'static str = r#"
fn main() {
	println!("Hello, world!");
}
"#;

#[cfg(test)]
fn test_str(
	idx: usize,
	expected: &str
) {
	assert_eq!(code_trace(CODE, idx, ""), expected);
}

macro_tests! {
	test_str,
	(code_trace_01, 0, ""),
	(code_trace_02, 1, "fn main() {\n^"),
	(code_trace_03, 2, "fn main() {\n ^"),
	(code_trace_04, 3, "fn main() {\n  ^"),
	(code_trace_05, CODE.find('{').unwrap(), "fn main() {\n          ^"),
}
