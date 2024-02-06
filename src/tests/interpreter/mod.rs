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
	Interpreter,
};
// use crate::syntax_tree::model::{
// 	Stmt
// };
use crate::macro_tests;

#[cfg(test)]
fn run_interpreter(
	code: &str,
	mut ctx: Value
) -> Result<String, String> {
	println!("Running interpreter: {}", code);
	println!("  - Code: {}", &code);
	let lexer = Lexer::new("{{", "}}");
	let tokens = lexer.scan(code).unwrap();
	println!("  - Tokens: {:?}", tokens);
	let tokens = tokens.iter().map(|t| t.clone()).collect::<Vec<Token>>();
	let mut parser = Parser::new(tokens);
	let code = parser.parse().unwrap();
	println!("  - Code: {:?}", code);
	let ctx = match ctx.as_object_mut() {
		Some(c) => c,
		None => panic!("ctx must be a JSON object")
	};
	let mut interpreter = Interpreter::new(code, ctx);
	interpreter.run()
}

#[cfg(test)]
fn test_interpreter(code: &str, mut ctx: Value, expected: &str) {
	let result = match run_interpreter(code, ctx) {
		Ok(r) => r,
		Err(e) => {
			println!("  - Error: {}", e);
			panic!("Unexpected error in interpreter:\n{}", e);
		}
	};
	println!("  - Result  : {}", result);
	println!("  - Expected: {}", expected);
	assert_eq!(result, expected);
}

#[cfg(test)]
fn expect_error(code: &str, ctx: Value) {
	match run_interpreter(code, ctx) {
		Ok(_) => panic!("Expected error in interpreter"),
		Err(_) => ()
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
	),
	(
		grouping01,
		"{{ (1 + 2) * 3 }}",
		json!({}),
		"9"
	),
	(
		grouping02,
		"{{ 1 + (2 * 3) }}",
		json!({}),
		"7"
	),
	(
		grouping03,
		"{{ (1 + 2) * (3 + 4) }}",
		json!({}),
		"21"
	)
);

macro_tests!(
	expect_error,
	(
		invalid_div01,
		"{{ 1 / 0 }}",
		json!({})
	),
	(
		invalid_div02,
		"{{ 1.0 / 0 }}",
		json!({})
	),
	(
		invalid_div03,
		"{{ 1 / 0.0 }}",
		json!({})
	),
	(
		invalid_div04,
		"{{ 1.0 / 0.0 }}",
		json!({})
	),
	(
		overflow01,
		"{{ 9223372036854775807 + 1 }}",
		json!({})
	),
	(
		overflow02,
		"{{ -9223372036854775807 - 2 }}",
		json!({})
	),
	(
		overflow03,
		"{{ 9223372036854775807 * 2 }}",
		json!({})
	),
	(
		overflow04,
		"{{ -9223372036854775807 * 2 }}",
		json!({})
	)
);

macro_tests!(
	test_interpreter,
	(
		variable01,
		"{{ foo }}",
		json!({"foo": "bar"}),
		"bar"
	),
	(
		variable02,
		"{{ foo.bar }}",
		json!({"foo": {"bar": "baz"}}),
		"baz"
	),
	(
		variable03,
		"{{ foo.bar.baz }}",
		json!({"foo": {"bar": {"baz": "qux"}}}),
		"qux"
	),
	(
		variable04,
		"{{ arr[0] }} {{ arr[1] }}",
		json!({"arr": ["foo", "bar"]}),
		"foo bar"
	),
	(
		variable05,
		"{{ arr[0].name }} {{ arr[1].surname }}",
		json!({"arr": [{"name": "foo"}, {"name": "bar", "surname": "baz"}]}),
		"foo baz"
	)
);

macro_tests!(
	expect_error,
	(
		invalid_variable01,
		"{{ foo }}",
		json!({})
	),
	(
		invalid_variable02,
		"{{ foo.bar }}",
		json!({"foo": []})
	),
	(
		invalid_variable03,
		"{{ foo.bar }}",
		json!({"foo": {"other": 42}})
	),
	(
		invalid_variable_index01,
		"{{ arr[0] }}",
		json!({"arr": []})
	),
	(
		invalid_variable_index02,
		"{{ arr[12] }}",
		json!({"arr": [1, 2, 3]})
	)
);

// Stmt
macro_tests!(
	test_interpreter,
	(
		print01,
		"print: {{print 1 }}",
		json!({}),
		"print: "
	),
	(
		if01,
		"{{ if true}}i{{ fi }}",
		json!({}),
		"i"
	),
	(
		if02,
		"{{ if false}}i{{ fi }}",
		json!({}),
		""
	),
	(
		if03,
		"{{ if true}}i{{ else }}e{{ fi }}",
		json!({}),
		"i"
	),
	(
		if04,
		"{{ if false}}i{{ else }}e{{ fi }}",
		json!({}),
		"e"
	),
	(
		if05,
		"{{ if v==1}}if{{elseif v==2}}elseif{{else}}else{{fi}}",
		json!({"v": 1}),
		"if"
	),
	(
		if06,
		"{{ if v==1}}if{{elseif v==2}}elseif{{else}}else{{fi}}",
		json!({"v": 2}),
		"elseif"
	),
	(
		if07,
		"{{ if v==1}}if{{elseif v==2}}elseif{{else}}else{{fi}}",
		json!({"v": 3}),
		"else"
	),
	(
		if08,
		"{{ if v==1}}if{{elseif v==2}}elseif01{{elseif v==3}}elseif02{{else}}else{{fi}}",
		json!({"v": 3}),
		"elseif02"
	),
	(
		if09,
		"{{ if v1}}if{{if v2}}if01{{else}}else01{{fi}}{{else}}else02{{fi}}",
		json!({"v1": true, "v2": true}),
		"ifif01"
	),
	(
		if10,
		"{{ if v1}}if{{if v2}}if01{{else}}else01{{fi}}{{else}}else02{{fi}}",
		json!({"v1": true, "v2": false}),
		"ifelse01"
	),
	(
		if11,
		"{{ if v1}}if{{if v2}}if01{{else}}else01{{fi}}{{else}}else02{{fi}}",
		json!({"v1": false, "v2": true}),
		"else02"
	),
	(
		if12,
		"{{ if v1}}if{{if v2}}if01{{else}}else01{{fi}}{{else}}else02{{fi}}",
		json!({"v1": false, "v2": false}),
		"else02"
	)
);
