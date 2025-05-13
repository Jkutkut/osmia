use crate::model::ctx::{
	Ctx,
	CtxValue,
	JsonTree
};
use crate::macro_tests;

fn osmia_json_dump(json: &str) -> String {
	let ctx: JsonTree<String, CtxValue> = serde_json::from_str(json).unwrap();
	let ctx = Ctx::from(ctx);
	crate::Osmia::new(ctx).ctx_json_dump()
}

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
