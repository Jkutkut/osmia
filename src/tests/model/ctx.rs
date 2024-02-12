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

#[cfg(test)]
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

#[cfg(test)]
fn get(
	ctx: &str,
	var: &str,
	expected: Result<&str, ()>
) {
	let ctx = str2ctx(ctx);
	let var = Variable::from_str(var).unwrap();
	println!("Test get");
	println!("  - get({})", var);
	let get = ctx.get(var);
	match expected {
		Ok(expected) => {
			let expected = Literal::from_str(expected).unwrap();
			match get {
				Ok(literal) => assert_eq!(literal, expected),
				Err(e) => panic!("Error obtaining value: {}", e)
			}
		},
		Err(_) => {
			match get {
				Err(_) => assert!(true),
				Ok(r) => panic!("This code should've failed: {}", r)
			}
		}
	}
}


#[cfg(test)]
fn invalid_set(
	ctx: &str,
	set: (&str, &str)
) {
	let mut ctx = str2ctx(ctx);
	println!("Test invalid set:");
	let variable = Variable::from_str(set.0).unwrap();
	let literal = Literal::from_str(set.1).unwrap();
	println!("  - set({}, {:?})", set.0, set.1);
	let result = ctx.set(variable, literal);
	assert!(result.is_err());
}

macro_tests!(
	test_compile,
	(
		compile01,
		r#"{"key": "value"}"#
	),
	(
		compile02,
		r#"{"key": {"key": "value"}}"#
	),
	(
		compile03,
		r#"{}"#
	),
	(
		compile04,
		r#"{"key": [1, 2, 3]}"#
	),
	(
		compile05,
		r#"{"key": {"key": [1, 2, 3]}}"#
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
	),
	(
		setarr03,
		r#"{"foo": [{"bar": []}]}"#,
		vec![
			("foo[0].bar[0]", r#""baz""#),
			("foo[1]", r#""quux""#)
		],
		r#"{"foo":[{"bar":["baz"]},"quux"]}"#
	),
	(
		override_arr,
		r#"{"key": [1,2,3]}"#,
		vec![
			("key", "21")
		],
		r#"{"key":21}"#
	),
	(
		override_obj,
		r#"{"key": {"key": [1,2,3]}}"#,
		vec![
			("key", "21")
		],
		r#"{"key":21}"#
	)
);

macro_tests!(
	get,
	(
		get01,
		r#"{"foo": "bar"}"#,
		"foo",
		Ok(r#""bar""#)
	),
	(
		get03,
		r#"{"foo": {"bar": "baz"}}"#,
		"foo.bar",
		Ok(r#""baz""#)
	),
	(
		get04,
		r#"{"foo": {"bar": ["baz"]}}"#,
		"foo.bar[0]",
		Ok(r#""baz""#)
	),
	(
		get05,
		r#"{"foo": {"bar": [{"baz": "quux"}]}}"#,
		"foo.bar[0].baz",
		Ok(r#""quux""#)
	),
	(
		get_invalid_key01,
		r#"{"foo": "bar"}"#,
		"other",
		Err(())
	),
	(
		get_invalid_key02,
		r#"{"foo": {"bar": "baz"}}"#,
		"foo.other",
		Err(())
	),
	(
		get_invalid_index01,
		r#"{"foo": {"bar": [{"baz": "quux"}]}}"#,
		"foo.bar[1].baz",
		Err(())
	),
	(
		get_invalid_index02,
		r#"{"foo": {"bar": ["baz"]}}"#,
		"foo.bar[2]",
		Err(())
	),
	(
		get_invalid_idx_obj,
		r#"{"foo": {"bar": ["baz"]}}"#,
		"foo[0]",
		Err(())
	),
	(
		get_invalid_key_arr,
		r#"{"foo": {"bar": ["baz"]}}"#,
		"foo.bar.baz",
		Err(())
	)
);


macro_tests!(
	invalid_set,
	(
		unset_obj,
		r#"{}"#,
		("key.foo", r#""bar""#)
	),
	(
		unset_arr,
		r#"{}"#,
		("key[0]", r#""bar""#)
	),
	(
		key_in_arr,
		r#"{"key": []}"#,
		("key.foo", r#""bar""#)
	),
	(
		idx_in_obj,
		r#"{"key": {}}"#,
		("key[0]", r#""bar""#)
	),
	(
		invalid_idx,
		r#"{"key": [{"foo": 12}]}"#,
		("key[1].foo", r#""bar""#)
	),
	(
		idx_in_middle_obj,
		r#"{"key": {"foo": 12}}"#,
		("key[0].foo", r#""bar""#)
	),
	(
		invalid_key,
		r#"{"key": {"foo": 12}}"#,
		("key.bar.foo", r#""bar""#)
	),
	(
		invalid_key2,
		r#"{"key": {"foo": 12}}"#,
		("key[0].bar", r#""bar""#)
	)
);
