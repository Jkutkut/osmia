use super::*;

macro_tests!(
	interpreter_test,
	(
		plus_int_01,
		"{{ 1 + 2 }}",
		vec![
			(Ctx::new(), Ok("3")),
		]
	),
	(
		plus_int_02,
		"{{ 1 + 2.0 }}",
		vec![
			(Ctx::new(), Ok("3")),
		]
	),
	(
		plus_int_03,
		"{{ 1 + 2.2 }}",
		vec![
			(Ctx::new(), Ok("3")),
		]
	),
	(
		plus_int_overflow_01,
		"{{ 9223372036854775807 + 1 }}",
		vec![
			(Ctx::new(), Err(vec!["overflow"])),
		]
	),
	(
		plus_int_overflow_02,
		"{{ 1 + 9223372036854775807 }}",
		vec![
			(Ctx::new(), Err(vec!["overflow"])),
		]
	),
	(
		plus_invalid_01,
		"{{ 1 + null }}",
		vec![
			(Ctx::new(), Err(vec!["null"])),
		]
	),
	(
		plus_invalid_02,
		"{{ null + 2 }}",
		vec![
			(Ctx::new(), Err(vec!["2", "null", "add"])),
		]
	),
	(
		plus_float_01,
		"{{ 1.1 + 2.3 }}",
		vec![
			(Ctx::new(), Ok("3.4")),
		]
	),
	(
		plus_float_02,
		"{{ 1.1 + 2 }}",
		vec![
			(Ctx::new(), Ok("3.1")),
		]
	)
);

macro_tests!(
	interpreter_test,
	(
		minus_int_01,
		"{{ 1 - 2 }}",
		vec![
			(Ctx::new(), Ok("-1")),
		]
	),
	(
		minus_int_02,
		"{{ 1 - 2.0 }}",
		vec![
			(Ctx::new(), Ok("-1")),
		]
	),
	(
		minus_int_03,
		"{{ 1 - 2.2 }}",
		vec![
			(Ctx::new(), Ok("-1")),
		]
	),
	(
		minus_int_overflow_01,
		"{{ 0 - 9223372036854775807 - 2 }}",
		vec![
			(Ctx::new(), Err(vec!["overflow"])),
		]
	),
	(
		minus_invalid_01,
		"{{ 1 - null }}",
		vec![
			(Ctx::new(), Err(vec!["null"])),
		]
	),
	(
		minus_invalid_02,
		"{{ null - 2 }}",
		vec![
			(Ctx::new(), Err(vec!["2", "null", "subtract"])),
		]
	),
	(
		minus_float_01,
		"{{ 1.2 - 2.4 }}",
		vec![
			(Ctx::new(), Ok("-1.2")),
		]
	),
	(
		minus_float_02,
		"{{ 1.2 - 2 }}",
		vec![
			(Ctx::new(), Ok("-0.8")),
		]
	)
);
