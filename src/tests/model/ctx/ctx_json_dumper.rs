use crate::model::ctx::{
	Ctx,
	CtxValue,
	JsonTree
};
use crate::macro_tests;

#[cfg(test)]
fn format_node(t: &str, v: &str) -> String {
	format!(r#"{{"type":"{}","value":{}}}"#, t, v)
}

#[cfg(test)]
fn osmia_json_dump(json: &str) -> String {
	let ctx: JsonTree<String, CtxValue> = serde_json::from_str(json).unwrap();
	let ctx = Ctx::from(ctx);
	crate::Osmia::new(ctx).ctx_json_dump()
}

#[cfg(test)]
fn test_value(v: &str) {
	let dump = osmia_json_dump(format!(
		r#"{{"v": {}}}"#, v
	).as_str());
	println!("{}", dump);
	assert!(dump.contains(format!(
		r#""v":{{"type":"variable","value":{}}}"#,
		v
	).as_str()));
}

#[test]
fn osmia() {
	let osmia = crate::Osmia::default();
	let dump = osmia.ctx_json_dump();
	println!("{}", dump);
	assert!(dump.contains(r#""PI":{"type":"variable","value":3.141592653589793}"#));
}

#[test]
fn empty() {
	let osmia = crate::Osmia::new(Ctx::clean());
	let dump = osmia.ctx_json_dump();
	println!("{}", dump);
	assert_eq!(dump, r#"{"type":"object","value":{}}"#);
}

#[test]
fn array01() {
	let dump = osmia_json_dump(
		"{\"v\":[]}"
	);
	println!("{}", dump);
	assert_eq!(dump, r#"{"type":"object","value":{"v":{"type":"array","value":[]}}}"#);
}

#[test]
fn array02() {
	let dump = osmia_json_dump(
		"{\"v\":[[]]}"
	);
	println!("{}", dump);
	assert_eq!(dump, r#"{"type":"object","value":{"v":{"type":"array","value":[{"type":"array","value":[]}]}}}"#);
}

#[test]
fn obj01() {
	let dump = osmia_json_dump(
		"{\"v\":{}}"
	);
	println!("{}", dump);
	assert_eq!(dump, r#"{"type":"object","value":{"v":{"type":"object","value":{}}}}"#);
}

#[test]
fn obj02() {
	let dump = osmia_json_dump(
		"{\"v\":{\"a\":[]}}"
	);
	println!("{}", dump);
	assert_eq!(dump, r#"{"type":"object","value":{"v":{"type":"object","value":{"a":{"type":"array","value":[]}}}}}"#);
}

macro_tests!(
	test_value,
	(int01, "0"),
	(int02, "1"),
	(int03, "-12"),
	(float01, "0.0"),
	(float02, "1.0"),
	(float03, "-12.0"),
	(string01, r#""test""#),
	(string02, r#""""#),
	(bool01, "true"),
	(bool02, "false"),
	(null, "null"),
);

#[cfg(test)]
fn osmia_json_variable_dump(variable: &str, json: &str) -> Result<String, String> {
	let ctx: JsonTree<String, CtxValue> = serde_json::from_str(json).unwrap();
	let ctx = Ctx::from(ctx);
	crate::Osmia::new(ctx).ctx_json_dump_variable(variable)
}

#[cfg(test)]
fn test_variable_value(v: &str, ctx: &str, expected: &str) {
	let dump = osmia_json_variable_dump(v, ctx).unwrap();
	println!("dump:     {}\nexpected: {}", dump, expected);
	assert_eq!(dump, expected);
}

macro_tests!(
	test_variable_value,
	(var_int01, "f", r#"{"f": 0}"#, &format_node("variable", "0")),
	(var_int02, "f", r#"{"f": 0, "g": 1, "h": 2}"#, &format_node("variable", "0")),
	(var_int03, "g", r#"{"f": 0, "g": 1, "h": 2}"#, &format_node("variable", "1")),
	(var_obj01, "o", r#"{"o": {}}"#, &format_node("object", "{}")),
	(var_obj02, "o", r#"{"o": {"foo": "bar"}}"#, &format_node("object", r#"{"foo":{"type":"variable","value":"bar"}}"#)),
	(var_arr01, "a", r#"{"a": []}"#, &format_node("array", "[]")),
	(var_arr02, "a", r#"{"a": ["foo", "bar"]}"#, &format_node("array", r#"[{"type":"variable","value":"foo"},{"type":"variable","value":"bar"}]"#)),
);
