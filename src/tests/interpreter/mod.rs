use crate::{
	Token,
	Parser,
	Lexer
};
use crate::interpreter::{
	Interpreter,
};
use crate::model::{
	Ctx
};
use crate::macro_tests;

#[cfg(test)]
fn run_interpreter(
	code: &str,
	mut ctx: Ctx
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
	let mut interpreter = Interpreter::new(&mut ctx);
	interpreter.run(&code)
}

#[cfg(test)]
fn test_interpreter(code: &str, ctx: &str, expected: &str) { 
	let ctx = Ctx::from_str(ctx).unwrap();
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
fn expect_error(code: &str, ctx: &str) {
	let ctx = Ctx::from_str(ctx).unwrap();
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
		r#"{}"#,
		"hello world!"
	),
	(
		int01,
		"{{ 1 + 2 }} {{ 1 - 2 }} {{ 3 * 5 }} {{ 3 / 5 }} {{ 3 % 5 }} {{ 7 && 5 }} {{ 7 || 5 }}",
		r#"{}"#,
		"3 -1 15 0 3 5 7"
	),
	(
		float01,
		"{{ 1.2 + 2.0 }} {{ 1.2 - 2.0 }} {{ 3.2 * 5.2 }} {{ 3.2 / 4.2 }} {{ 3.2 % 5.0 }} {{ 7.2 && 5.0 }} {{ 7.0 || 5.2 }}",
		r#"{}"#,
		"3.2 -0.8 16.64 0.7619047619047619 3.2 5 7"
	),
	(
		int02,
		"{{ 3 > 5 }} {{ 3 >= 5 }} {{ 3 < 5 }} {{ 3 <= 5 }} {{ 3 == 5 }} {{ 3 != 5 }}",
		r#"{}"#,
		"false false true true false true"
	),
	(
		float02,
		"{{ 3.0 > 5.0 }} {{ 3.0 >= 5.0 }} {{ 3.0 < 5.0 }} {{ 3.0 <= 5.0 }} {{ 3.0 == 5.0 }} {{ 3.0 != 5.0 }}",
		r#"{}"#,
		"false false true true false true"
	),
	(
		int_float01,
		"{{ 1 + 2.0 }} {{ 1.0 + 2 }} {{ 1 - 2.0 }} {{ 1.0 - 2 }} {{ 1 * 2.0 }} {{ 1.0 * 2 }} {{ 1 / 2.0 }} {{ 1.0 / 2 }}",
		r#"{}"#,
		"3 3 -1 -1 2 2 0.5 0.5"
	),
	(
		division,
		"{{ 1 / 2 }} {{ 1.0 / 2 }} {{ 1 / 2.0 }} {{ 1.0 / 2.0 }}",
		r#"{}"#,
		"0 0.5 0.5 0.5"
	),
	(
		bool01,
		"{{ true && true }} {{ true && false }} {{ false && true }} {{ false && false }}",
		r#"{}"#,
		"true false false false"
	),
	(
		bool02,
		"{{ true || true }} {{ true || false }} {{ false || true }} {{ false || false }}",
		r#"{}"#,
		"true true true false"
	),
	(
		str01,
		r#"{{ "hello" + " world" }},{{"null: " + null}},{{"true: " + true}},{{"false: " + false}},{{"int: " + 1}},{{"float: " + 1.1}}"#,
		r#"{}"#,
		"hello world,null: null,true: true,false: false,int: 1,float: 1.1"
	),
	(
		str02,
		r#"{{null + "null"}},{{true + "true"}},{{false + "false"}},{{1 + "int"}},{{1.6 + "float"}}"#,
		r#"{}"#,
		"nullnull,truetrue,falsefalse,1int,1.6float"
	),
	(
		unary01,
		"{{ !true }} {{ !false }} {{ !!true }} {{ !!false }}",
		r#"{}"#,
		"false true true false"
	),
	(
		unary02,
		"{{ -+-3 }} {{-2}}",
		r#"{}"#,
		"3 -2"
	),
	(
		bool03,
		"{{ 1 + true }} {{ 1 + false }} {{ 0 + true }} {{ 0 + false }} {{ 3 + true }} {{ 3 + false }}",
		r#"{}"#,
		"true true true false true true"
	),
	(
		bool04,
		"{{ 1 - true }} {{ 1 - false }} {{ 0 - true }} {{ 0 - false }} {{ 3 - true }} {{ 3 - false }}",
		r#"{}"#,
		"true false false false true false"
	),
	(
		unary03,
		"{{ !!3 }} {{ !!-3 }} {{ !!0 }}",
		r#"{}"#,
		"true true false"
	),
	(
		grouping01,
		"{{ (1 + 2) * 3 }}",
		r#"{}"#,
		"9"
	),
	(
		grouping02,
		"{{ 1 + (2 * 3) }}",
		r#"{}"#,
		"7"
	),
	(
		grouping03,
		"{{ (1 + 2) * (3 + 4) }}",
		r#"{}"#,
		"21"
	)
);

macro_tests!(
	expect_error,
	(
		invalid_div01,
		"{{ 1 / 0 }}",
		r#"{}"#
	),
	(
		invalid_div02,
		"{{ 1.0 / 0 }}",
		r#"{}"#
	),
	(
		invalid_div03,
		"{{ 1 / 0.0 }}",
		r#"{}"#
	),
	(
		invalid_div04,
		"{{ 1.0 / 0.0 }}",
		r#"{}"#
	),
	(
		overflow01,
		"{{ 9223372036854775807 + 1 }}",
		r#"{}"#
	),
	(
		overflow02,
		"{{ -9223372036854775807 - 2 }}",
		r#"{}"#
	),
	(
		overflow03,
		"{{ 9223372036854775807 * 2 }}",
		r#"{}"#
	),
	(
		overflow04,
		"{{ -9223372036854775807 * 2 }}",
		r#"{}"#
	)
);

macro_tests!(
	test_interpreter,
	(
		variable01,
		"{{ foo }}",
		r#"{"foo": "bar"}"#,
		"bar"
	),
	(
		variable02,
		"{{ foo.bar }}",
		r#"{"foo": {"bar": "baz"}}"#,
		"baz"
	),
	(
		variable03,
		"{{ foo.bar.baz }}",
		r#"{"foo": {"bar": {"baz": "qux"}}}"#,
		"qux"
	),
	(
		variable04,
		"{{ arr[0] }} {{ arr[1] }}",
		r#"{"arr": ["foo", "bar"]}"#,
		"foo bar"
	),
	(
		variable05,
		"{{ arr[0].name }} {{ arr[1].surname }}",
		r#"{"arr": [{"name": "foo"}, {"name": "bar", "surname": "baz"}]}"#,
		"foo baz"
	)
);

macro_tests!(
	expect_error,
	(
		invalid_variable01,
		"{{ foo }}",
		r#"{}"#
	),
	(
		invalid_variable02,
		"{{ foo.bar }}",
		r#"{"foo": []}"#
	),
	(
		invalid_variable03,
		"{{ foo.bar }}",
		r#"{"foo": {"other": 42}}"#
	),
	(
		invalid_variable_index01,
		"{{ arr[0] }}",
		r#"{"arr": []}"#
	),
	(
		invalid_variable_index02,
		"{{ arr[12] }}",
		r#"{"arr": [1, 2, 3]}"#
	)
);

// Stmt
macro_tests!(
	test_interpreter,
	(
		print01,
		"print: {{print 1 }}",
		r#"{}"#,
		"print: "
	),
	(
		if01,
		"{{ if true}}i{{ fi }}",
		r#"{}"#,
		"i"
	),
	(
		if02,
		"{{ if false}}i{{ fi }}",
		r#"{}"#,
		""
	),
	(
		if03,
		"{{ if true}}i{{ else }}e{{ fi }}",
		r#"{}"#,
		"i"
	),
	(
		if04,
		"{{ if false}}i{{ else }}e{{ fi }}",
		r#"{}"#,
		"e"
	),
	(
		if05,
		"{{ if v==1}}if{{elseif v==2}}elseif{{else}}else{{fi}}",
		r#"{"v": 1}"#,
		"if"
	),
	(
		if06,
		"{{ if v==1}}if{{elseif v==2}}elseif{{else}}else{{fi}}",
		r#"{"v": 2}"#,
		"elseif"
	),
	(
		if07,
		"{{ if v==1}}if{{elseif v==2}}elseif{{else}}else{{fi}}",
		r#"{"v": 3}"#,
		"else"
	),
	(
		if08,
		"{{ if v==1}}if{{elseif v==2}}elseif01{{elseif v==3}}elseif02{{else}}else{{fi}}",
		r#"{"v": 3}"#,
		"elseif02"
	),
	(
		if09,
		"{{ if v1}}if{{if v2}}if01{{else}}else01{{fi}}{{else}}else02{{fi}}",
		r#"{"v1": true, "v2": true}"#,
		"ifif01"
	),
	(
		if10,
		"{{ if v1}}if{{if v2}}if01{{else}}else01{{fi}}{{else}}else02{{fi}}",
		r#"{"v1": true, "v2": false}"#,
		"ifelse01"
	),
	(
		if11,
		"{{ if v1}}if{{if v2}}if01{{else}}else01{{fi}}{{else}}else02{{fi}}",
		r#"{"v1": false, "v2": true}"#,
		"else02"
	),
	(
		if12,
		"{{ if v1}}if{{if v2}}if01{{else}}else01{{fi}}{{else}}else02{{fi}}",
		r#"{"v1": false, "v2": false}"#,
		"else02"
	)
);

macro_tests!(
	test_interpreter,
	(
		assign01,
		"{{assign v = 1 }}{{ v }}",
		r#"{}"#,
		"1"
	),
	(
		assign02,
		"{{assign v = 1 }}{{ v }}",
		r#"{"v": 2}"#,
		"1"
	),
	(
		assign_string,
		"{{assign v = \"foo\" }}{{ v }}",
		r#"{}"#,
		"foo"
	),
	(
		assign_int,
		"{{assign v = 1 }}{{ v }}",
		r#"{}"#,
		"1"
	),
	(
		assign_float,
		"{{assign v = 1.1 }}{{ v }}",
		r#"{}"#,
		"1.1"
	),
	(
		assign_bool,
		"{{assign v = true }}{{ v }} -- {{assign v = false }}{{ v }}",
		r#"{}"#,
		"true -- false"
	),
	(
		assign_null,
		"{{assign v = null }}{{ v }}",
		r#"{}"#,
		"null"
	),
	(
		assign_array,
		"{{assign v[2] = 2 }}{{ v[0] }}{{ v[1] }}{{ v[2] }}",
		r#"{"v": [1, 2, 3]}"#,
		"122"
	),
	(
		assign_override01,
		"{{assign v = 1 }}{{v}}",
		r#"{"v": {}}"#,
		"1"
	),
	(
		assign_override02,
		"{{assign v = 1 }}{{v}}",
		r#"{"v": [123, 2, 3]}"#,
		"1"
	),
	(
		assign_variable01,
		"{{assign foo = bar }}{{foo}}",
		r#"{"bar": 2}"#,
		"2"
	),
	(
		assign_variable02,
		"{{assign foo = bar * foo }}{{foo}}",
		r#"{"bar": 2, "foo": 2}"#,
		"4"
	)
);

macro_tests!(
	expect_error,
	(
		invalid_assign01,
		"{{assign v.foo = 1 }}",
		r#"{}"#
	),
	(
		invalid_assign02,
		"{{assign v[1] = 1 }}",
		r#"{}"#
	),
	(
		invalid_assign03,
		"{{assign v[1] = 1 }}",
		r#"{"v": {"h": 2}}"#
	),
	(
		invalid_assign04,
		"{{assign v.foo = 1 }}",
		r#"{"v": [123, 2, 3]}"#
	)
);

macro_tests!(
	test_interpreter,
	(
		while01,
		"{{while v < 3}}{{ v }}{{assign v = v + 1}}{{done}}",
		r#"{"v": 0}"#,
		"012"
	),
	(
		while02,
		"{{while v != 0}}{{v % 2}}{{assign v = v - 1}}{{done}}",
		r#"{"v": 10}"#,
		"0101010101"
	),
	(
		whilefalse,
		"{{while false}}123{{done}}",
		r#"{}"#,
		""
	),
	(
		while03,
		"{{while v != 10000}}{{assign v = v + 1}}{{done}}{{v}}",
		r#"{"v": 0}"#,
		"10000"
	)
);

macro_tests!(
	test_interpreter,
	(
		foreach01,
		"{{for v in [1, 2, 3]}}{{ v }}{{done}}",
		r#"{}"#,
		"123"
	),
	(
		foreach02,
		"{{for v in [1, 2, 3]}}{{ v }}{{done}}",
		r#"{"v": 3}"#,
		"123"
	),
	(
		foreach03,
		r#"{{for v in [true, false, null, "hello world"]}}{{ v }}{{done}}"#,
		r#"{"v": 3}"#,
		"truefalsenullhello world"
	),
	(
		foreach04,
		"{{for v in []}}{{ v }}{{done}}",
		r#"{}"#,
		""
	),
	(
		foreach05,
		"{{for v in [1 + v, 2, 3]}}{{ v }}{{done}}",
		r#"{"v": 3}"#,
		"423"
	),
	(
		foreachvariable01,
		"{{for v in arr}}{{ v }}{{done}}",
		r#"{"arr": [1, 2, 3]}"#,
		"123"
	),
	(
		foreachvariable02,
		"{{for v in arr}} {{ v }}{{done}}",
		r#"{"arr": [true, "2", null]}"#,
		" true 2 null"
	),
	(
		foreachvariable03,
		"{{for v in arr}}{{ v }}{{done}}",
		r#"{"arr": []}"#,
		""
	),
	(
		foreachvariable04,
		"{{for v in arr}}{{ v }}{{done}}",
		r#"{"arr": [12.3]}"#,
		"12.3"
	),
	(
		foreach06,
		"{{for v in [[1, 2], [3, 4]]}}{{ v[0] }} -- {{ v[1] }},{{done}}",
		"{}",
		"1 -- 2,3 -- 4,"
	),
	(
		foreach07,
		r#"{{for v in [{"name": "foo"}, {"name": "bar" + extra}]}}{{ v.name }},{{done}}"#,
		r#"{"extra": 12}"#,
		"foo,bar12,"
	)
);


// TODO: Allow this advanced tests
// macro_tests!(
// 	test_interpreter,
// 	(
// 		advanced_json_control01,
// 		r#"{{ foo[v] }}"#,
// 		r#"{"foo": [1, 2, 3], "v": 1}"#,
// 		"2"
// 	),
// 	(
// 		advanced_json_control02,
// 		r#"{{ foo["bar"] }}"#,
// 		r#"{"foo": {"bar": "baz"}, "v": 1}"#,
// 		"baz"
// 	),
// 	(
// 		advanced_json_control03,
// 		r#"{{ foo['bar'] }}"#,
// 		r#"{"foo": {"bar": "baz"}, "v": 1}"#,
// 		"baz"
// 	),
// 	(
// 		advanced_json_control04,
// 		r#"{{ foo[v] }}"#,
// 		r#"{"foo": {"bar": "baz"}, "v": "bar"}"#,
// 		"baz"
// 	)
// );
