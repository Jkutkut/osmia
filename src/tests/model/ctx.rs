use serde_json::json;

use crate::macro_tests;
use crate::model::{
	Ctx, Variable, Literal
};

#[cfg(test)]
fn str2ctx(
	json: &str
) -> Ctx {
	println!("  - json: {}", json);
	Ctx::from_str(json).unwrap()
}

#[cfg(test)]
fn test_compile(
	json: &str
) {
	println!("Test ctx:");
	let ctx = str2ctx(json);
	println!("  - ctx: {:?}", ctx);
}

fn set(
	ctx: &str,
	sets: Vec<(&str, &str)>,
	expected: &str
) {
	let mut ctx = str2ctx(ctx);
	println!("Test set:");
	for (key, value) in sets {
		let variable = Variable::from_str(key).unwrap();
		let literal = Literal::from_str(value).unwrap();
		println!("  - set({}, {:?})", key, value);
		let result = ctx.set(variable, literal);
		if let Err(e) = result {
			panic!("{}", e);
		}
	}
	println!("  - ctx:  {}", ctx);
	let expected_ctx = str2ctx(expected);
	println!("  - ectx: {}", expected_ctx);
	assert_eq!(ctx, expected_ctx);
}

macro_tests!(
	test_compile,
	(
		compile01,
		r#"{"key": "value"}"#
	)
);

macro_tests!(
	set,
	(
		set01,
		"{}",
		vec![
			("key", r#""value""#),
		],
		"{\"key\":\"value\"}"
	),
	(
		set02,
		"{}",
		vec![
			("keynull", "null"),
			("keytrue", "true"),
			("keyfalse", "false"),
		],
		"{\"keynull\":null,\"keytrue\":true,\"keyfalse\":false}"
	),
	(
		set03,
		"{}",
		vec![
			("key0", "0"),
			("key1", "1"),
			("keyneg1", "-1"),
		],
		"{\"key0\":0,\"key1\":1,\"keyneg1\":-1}"
	),
	(
		set04,
		"{}",
		vec![
			("keyfloat", "0.1"),
			("keyfloat2", "-0.1"),
		],
		"{\"keyfloat\":0.1,\"keyfloat2\":-0.1}"
	)
);
