use serde_json::{
	json,
	Value
};
use crate::{
	Token,
	Parser,
	Lexer
};
use crate::interpreter::{
	ExitStatus,
	Interpreter,
	InterpreterValue,
};
// use crate::syntax_tree::model::{
// 	Stmt
// };
use crate::macro_tests;

#[cfg(test)]
fn test_interpreter(code: &str, mut ctx: Value, expected: &str) {
	println!("Testing interpreter:");
	println!("  - Code: {}", &code);
	let lexer = Lexer::new("{{", "}}");
	let tokens = lexer.scan(code).unwrap();
	let tokens = tokens.iter().map(|t| t.clone()).collect::<Vec<Token>>();
	println!("  - Tokens: {:?}", tokens);
	let mut parser = Parser::new(tokens);
	let code = parser.parse().unwrap();
	println!("  - Code: {:?}", code);
	let ctx = match ctx.as_object_mut() {
		Some(c) => c,
		None => panic!("ctx must be a JSON object")
	};
	let mut interpreter = Interpreter::new(code, ctx);
	let (status, result) = match interpreter.run() {
		Err(e) => {
			println!("  - Error: {}", e);
			panic!("Unexpected error in interpreter");
		},
		Ok(r) => r
	};
	println!("  - Status: {:?}", status);
	println!("  - Result: {:?}", result);
	if status != ExitStatus::Okay {
		panic!("Unexpected exit status: {:?}", status);
	}
	match result {
		InterpreterValue::String(s) => assert_eq!(s, expected),
		_ => panic!("Unexpected result: {:?}", result)
	}
}

macro_tests!(
	test_interpreter,
	(
		static_template,
		"hello world!",
		json!({}),
		"hello world!"
	),
	(
		int01,
		"{{ 1 + 2 }} {{ 1 - 2 }} {{ 3 * 5 }} {{ 3 / 5 }} {{ 3 % 5 }} {{ 7 && 5 }} {{ 7 || 5 }}",
		json!({}),
		"3 -1 15 0 3 5 7"
	),
	(
		float01,
		"{{ 1.2 + 2.0 }} {{ 1.2 - 2.0 }} {{ 3.2 * 5.2 }} {{ 3.2 / 4.2 }} {{ 3.2 % 5.0 }} {{ 7.2 && 5.0 }} {{ 7.0 || 5.2 }}",
		json!({}),
		"3.2 -0.8 16.64 0.7619047619047619 3.2 5 7"
	),
	(
		int02,
		"{{ 3 > 5 }} {{ 3 >= 5 }} {{ 3 < 5 }} {{ 3 <= 5 }} {{ 3 == 5 }} {{ 3 != 5 }}",
		json!({}),
		"false false true true false true"
	),
	(
		float02,
		"{{ 3.0 > 5.0 }} {{ 3.0 >= 5.0 }} {{ 3.0 < 5.0 }} {{ 3.0 <= 5.0 }} {{ 3.0 == 5.0 }} {{ 3.0 != 5.0 }}",
		json!({}),
		"false false true true false true"
	),
	(
		int_float01,
		"{{ 1 + 2.0 }} {{ 1.0 + 2 }} {{ 1 - 2.0 }} {{ 1.0 - 2 }} {{ 1 * 2.0 }} {{ 1.0 * 2 }} {{ 1 / 2.0 }} {{ 1.0 / 2 }}",
		json!({}),
		"3 3 -1 -1 2 2 0.5 0.5"
	),
	(
		division,
		"{{ 1 / 2 }} {{ 1.0 / 2 }} {{ 1 / 2.0 }} {{ 1.0 / 2.0 }}",
		json!({}),
		"0 0.5 0.5 0.5"
	),
	(
		bool01,
		"{{ true && true }} {{ true && false }} {{ false && true }} {{ false && false }}",
		json!({}),
		"true false false false"
	),
	(
		bool02,
		"{{ true || true }} {{ true || false }} {{ false || true }} {{ false || false }}",
		json!({}),
		"true true true false"
	),
	(
		str01,
		r#"{{ "hello" + " world" }},{{"null: " + null}},{{"true: " + true}},{{"false: " + false}},{{"int: " + 1}},{{"float: " + 1.1}}"#,
		json!({}),
		"hello world,null: null,true: true,false: false,int: 1,float: 1.1"
	),
	(
		str02,
		r#"{{null + "null"}},{{true + "true"}},{{false + "false"}},{{1 + "int"}},{{1.6 + "float"}}"#,
		json!({}),
		"nullnull,truetrue,falsefalse,1int,1.6float"
	),
	(
		unary01,
		"{{ !true }} {{ !false }} {{ !!true }} {{ !!false }}",
		json!({}),
		"false true true false"
	),
	(
		unary02,
		"{{ -+-3 }} {{-2}}",
		json!({}),
		"3 -2"
	),
	(
		bool03,
		"{{ 1 + true }} {{ 1 + false }} {{ 0 + true }} {{ 0 + false }} {{ 3 + true }} {{ 3 + false }}",
		json!({}),
		"true true true false true true"
	),
	(
		bool04,
		"{{ 1 - true }} {{ 1 - false }} {{ 0 - true }} {{ 0 - false }} {{ 3 - true }} {{ 3 - false }}",
		json!({}),
		"true false false false true false"
	),
	(
		unary03,
		"{{ !!3 }} {{ !!-3 }} {{ !!0 }}",
		json!({}),
		"true true false"
	)
);
