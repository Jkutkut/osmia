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
		expr01,
		"{{ 1 + 2 }} {{ 1 - 2 }} {{ 3 * 5 }} {{ 3 / 5 }} {{ 3 % 5 }} {{ 3 > 5 }} {{ 3 >= 5 }} {{ 3 < 5 }} {{ 3 <= 5 }} {{ 3 == 5 }} {{ 3 != 5 }}",
		json!({}),
		"3 -1 15 0 3 false false true true false true"
	)/*,
	(
		expr02,
		"{{ 3 && 5 }} {{ 3 || 5 }}",
		json!({}),
		"1 1"
	)
	*/
);
