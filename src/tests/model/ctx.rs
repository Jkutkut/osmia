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
	match Ctx::from_str(json) {
		Ok(ctx) => ctx,
		Err(e) => panic!("Unable to parse json: {}", e)
	}
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
	let expected_ctx = str2ctx(expected);
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
	),
	(
		setobj01,
		r#"{"key": {}}"#,
		vec![
			("key.foo", r#""bar""#),
		],
		r#"{"key":{"foo":"bar"}}"#
	),
	(
		setobj02,
		r#"{"key": {},"keytwo": {"foo": {}}}"#,
		vec![
			("key.foo", r#""bar""#),
			("keytwo.foo.bar", r#""baz""#),
		],
		r#"{"key":{"foo":"bar"},"keytwo":{"foo":{"bar":"baz"}}}"#
	),
	(
		setobj03,
		r#"{"objone": {"objtwo": {}}}"#,
		vec![
			("valcero", "0"),
			("objone.valone", "null"),
			("objone.objtwo.valtwo", "-3.3"),
		],
		r#"{"valcero":0,"objone":{"valone":null,"objtwo":{"valtwo":-3.3}}}"#
	),
	(
		setarr01,
		r#"{"key": []}"#,
		vec![
			("key[0]", r#""foo""#),
		],
		r#"{"key":["foo"]}"#
	),
	(
		setarr02,
		r#"{"key": []}"#,
		vec![
			("key[0]", r#""foo""#),
			("key[1]", r#""bar""#),
			("key[4]", r#""baz""#)
		],
		r#"{"key":["foo","bar",null,null,"baz"]}"#
	)
);

// TODO create obj if not exists
// TODO create arr if not exists
// TODO valid tests
//  - get
//  - set with index + key
//  - override arr with value
//  - override obj with value
//  - override obj with arr
//  - override arr with obj
// TODO invalid
//   - find exceptions
//   - invalid index
//   - invalid key
