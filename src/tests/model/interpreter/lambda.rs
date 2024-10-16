use super::*;

macro_tests!(
	interpreter_test,
	(
		lambda_01,
		"{{ fn (x) => x }}",
		vec![
			(Ctx::new(), Ok("fn (x) => x")),
		]
	),
	(
		lambda_02,
		"{{ fn (x, y = 10) => x - y }}",
		vec![
			(Ctx::new(), Ok("fn (x, y = 10) => x - y")),
		]
	),
	(
		lambda_03,
		"{{ fn (x, y = 5 + 5) => x - y }}",
		vec![
			(Ctx::new(), Ok("fn (x, y = 10) => x - y")),
		]
	),
	(
		lambda_execution_01,
		"{{ x = fn (x) => x }}{{x(10)}}",
		vec![
			(Ctx::new(), Ok("10")),
		]
	),
	(
		lambda_execution_02,
		"{{ x = fn (x, y = 10) => x - y }}{{x(10)}}",
		vec![
			(Ctx::new(), Ok("0")),
		]
	),
	(
		lambda_execution_03,
		"{{ x = fn (x, y = z + 5) => x - y }}{{x(10)}}",
		vec![
			(Ctx::try_from(r#"{ "z": 5 }"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{ "z": 10 }"#).unwrap(), Ok("-5")),
		]
	),
	(
		lambda_execution_04,
		"{{ (fn (x) => x)(10) }}",
		vec![
			(Ctx::new(), Ok("10")),
		]
	),
	(
		lambda_execution_05,
		"{{ (fn (x) => fn(x) => x)(123)(10) }}",
		vec![
			(Ctx::new(), Ok("10")),
		]
	),
	(
		lambda_execution_06,
		"{{ (fn () => fn(x) => x)()(10) }}",
		vec![
			(Ctx::new(), Ok("10")),
		]
	),
	(
		lambda_execution_07,
		"{{ (fn () => fn(x) => x)() }}",
		vec![
			(Ctx::new(), Ok("fn (x) => x")),
		]
	)
);
