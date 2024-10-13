use super::*;

macro_tests!(
	interpreter_test,
	(
		contant_01,
		"{{ math.PI }}",
		vec![
			(Ctx::new(), Ok("3.141592653589793")),
			(Ctx::clean(), Err(vec!["not", "found"]))
		]
	),
	(
		contant_call_01,
		"{{ math.PI() }}",
		vec![
			(Ctx::new(), Err(vec!["not", "callable"])),
		]
	),
	(
		builtin_01,
		"{{ math.pow(2, 10) }}",
		vec![
			(Ctx::new(), Ok("1024")),
			(Ctx::clean(), Err(vec!["not", "found"]))
		]
	),
	(
		builtin_02,
		"{{ math.abs(v) }}",
		vec![
			(Ctx::try_from(r#"{"v": 1}"#).unwrap(), Ok("1")),
			(Ctx::try_from(r#"{"v": -1}"#).unwrap(), Ok("1")),
			(Ctx::clean(), Err(vec!["not", "found"]))
		]
	),
	(
		builtin_03,
		"{{ math.pow() }}",
		vec![
			(Ctx::new(), Err(vec!["2", "arguments", "got", "0"])),
			(Ctx::clean(), Err(vec!["not", "found", "math"])),
		]
	),
	(
		builtin_04,
		"{{ math.pow(2, 3, 4) }}",
		vec![
			(Ctx::new(), Ok("8")),
			(Ctx::clean(), Err(vec!["not", "found", "math"])),
		]
	),
	(
		builtin_05,
		"{{ math.pow(2) }}",
		vec![
			(Ctx::new(), Err(vec!["2", "arguments", "got", "1"])),
		]
	),
	(
		builtin_06,
		"{{ math.pow(10 / 2 - 3, 5 * (--2)) }}",
		vec![
			(Ctx::new(), Ok("1024")),
		]
	)
);
