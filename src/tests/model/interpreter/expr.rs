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

macro_tests!(
	interpreter_test,
	(
		mul_int_01,
		"{{ 1 * 2 }}",
		vec![
			(Ctx::new(), Ok("2")),
		]
	),
	(
		mul_int_02,
		"{{ 1 * 2.0 }}",
		vec![
			(Ctx::new(), Ok("2")),
		]
	),
	(
		mul_int_03,
		"{{ 1 * 2.2 }}",
		vec![
			(Ctx::new(), Ok("2")),
		]
	),
	(
		mul_int_overflow_01,
		"{{ 9223372036854775807 * 2 }}",
		vec![
			(Ctx::new(), Err(vec!["overflow"])),
		]
	),
	(
		mul_invalid_01,
		"{{ 1 * null }}",
		vec![
			(Ctx::new(), Err(vec!["null"])),
		]
	),
	(
		mul_invalid_02,
		"{{ null * 2 }}",
		vec![
			(Ctx::new(), Err(vec!["2", "null", "multiply"])),
		]
	),
	(
		mul_float_01,
		"{{ 1.2 * 2.4 }}",
		vec![
			(Ctx::new(), Ok("2.88")),
		]
	),
	(
		mul_float_02,
		"{{ 1.2 * 2 }}",
		vec![
			(Ctx::new(), Ok("2.4")),
		]
	)
);

macro_tests!(
	interpreter_test,
	(
		div_int_01,
		"{{ 1 / 2 }}",
		vec![
			(Ctx::new(), Ok("0")),
		]
	),
	(
		div_int_02,
		"{{ 1 / 2.0 }}",
		vec![
			(Ctx::new(), Ok("0.5")),
		]
	),
	(
		div_int_03,
		"{{ 1 / 2.0 }}",
		vec![
			(Ctx::new(), Ok("0.5")),
		]
	),
	(
		div_int_04,
		"{{ 9223372036854775807 / 2 }}",
		vec![
			(Ctx::new(), Ok("4611686018427387903")),
		]
	),
	(
		div_invalid_01,
		"{{ 1 / null }}",
		vec![
			(Ctx::new(), Err(vec!["null"])),
		]
	),
	(
		div_invalid_02,
		"{{ null / 2 }}",
		vec![
			(Ctx::new(), Err(vec!["2", "null", "divide"])),
		]
	),
	(
		div_float_01,
		"{{ 1.2 / 2.4 }}",
		vec![
			(Ctx::new(), Ok("0.5")),
		]
	),
	(
		div_float_02,
		"{{ 1.2 / 2 }}",
		vec![
			(Ctx::new(), Ok("0.6")),
		]
	)
);

macro_tests!(
	interpreter_test,
	(
		rem_int_01,
		"{{ 1 % 2 }}",
		vec![
			(Ctx::new(), Ok("1")),
		]
	),
	(
		rem_int_02,
		"{{ 1 % 2.0 }}",
		vec![
			(Ctx::new(), Ok("1")),
		]
	),
	(
		rem_int_03,
		"{{ 1 % 2.0 }}",
		vec![
			(Ctx::new(), Ok("1")),
		]
	),
	(
		rem_int_04,
		"{{ 9223372036854775807 % 2 }}",
		vec![
			(Ctx::new(), Ok("1")),
		]
	),
	(
		rem_invalid_01,
		"{{ 1 % null }}",
		vec![
			(Ctx::new(), Err(vec!["null"])),
		]
	),
	(
		rem_invalid_02,
		"{{ null % 2 }}",
		vec![
			(Ctx::new(), Err(vec!["2", "null", "modulo"])),
		]
	),
	(
		rem_float_01,
		"{{ 1.2 % 2.4 }}",
		vec![
			(Ctx::new(), Ok("1.2")),
		]
	),
	(
		rem_float_02,
		"{{ 1.2 % 2 }}",
		vec![
			(Ctx::new(), Ok("1.2")),
		]
	)
);
