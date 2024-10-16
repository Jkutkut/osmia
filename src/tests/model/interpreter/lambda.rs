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
	),
	(
		lambda_execution_08_01,
		"{{ (fn (...nbrs) => nbrs)() }}",
		vec![
			(Ctx::new(), Ok("[]")),
		]
	),
	(
		lambda_execution_08_02,
		"{{ (fn (...nbrs) => nbrs)(1) }}",
		vec![
			(Ctx::new(), Ok("[1]")),
		]
	),
	(
		lambda_execution_08_03,
		"{{ (fn (...nbrs) => nbrs)(1, 2) }}",
		vec![
			(Ctx::new(), Ok("[1, 2]")),
		]
	),
	(
		lambda_execution_08_04,
		"{{ (fn (...nbrs) => nbrs)(1, 2, 3) }}",
		vec![
			(Ctx::new(), Ok("[1, 2, 3]")),
		]
	),
	(
		lambda_execution_09_01,
		"{{ (fn (extra, ...nbrs) => extra + nbrs[0])() }}",
		vec![
			(Ctx::try_from(r#"{ "args": [] }"#).unwrap(), Err(vec!["missing", "extra"])),
		]
	),
	(
		lambda_execution_09_02,
		"{{ (fn (extra, ...nbrs) => extra + nbrs[0])(1) }}",
		vec![
			(Ctx::try_from(r#"{ "args": [] }"#).unwrap(), Err(vec!["index", "out", "bounds", "0"])),
		]
	),
	(
		lambda_execution_09_03,
		"{{ (fn (extra, ...nbrs) => extra + nbrs[0])(1, 2) }}",
		vec![
			(Ctx::new(), Ok("3")),
		]
	),
	(
		lambda_execution_09_04,
		"{{ (fn (extra, ...nbrs) => extra + nbrs[0])(10, 20, 3) }}",
		vec![
			(Ctx::new(), Ok("30")),
		]
	)
);
