use super::*;

macro_tests!(
	interpreter_test,
	(
		function_01,
		"{{fn add; x, y }}{{return x + y}}{{done}}{{add(1, 2)}}",
		vec![
			(Ctx::new(), Ok("3")),
		]
	),
	(
		function_02,
		"{{fn add; x, y }}{{return x + y}}{{done}}{{add(1, add(2, 3))}}",
		vec![
			(Ctx::new(), Ok("6")),
		]
	),
	(
		function_03,
		"{{fn two_to_the; n }}{{if n <= 0}}{{return 1}}{{else}}{{return two_to_the(n - 1) * 2}}{{fi}}{{done}}{{two_to_the(m)}}",
		vec![
			(Ctx::try_from("{\"m\": 3}").unwrap(), Ok("8")),
			(Ctx::try_from("{\"m\": 10}").unwrap(), Ok("1024")),
			(Ctx::try_from("{\"m\": 0}").unwrap(), Ok("1")),
		]
	),
	(
		function_04,
		r#"{{fn fibonacci; n }}{{if n <= 0}}{{return 0}}{{elseif n == 1}}{{return 1}}{{else}}{{return fibonacci(n - 1) + fibonacci(n - 2)}}{{fi}}{{done}}{{fibonacci(m)}}"#,
		vec![
			(Ctx::try_from(r#"{"m": 0}"#).unwrap(), Ok("0")),
			(Ctx::try_from(r#"{"m": 1}"#).unwrap(), Ok("1")),
			(Ctx::try_from(r#"{"m": 2}"#).unwrap(), Ok("1")),
			(Ctx::try_from(r#"{"m": 3}"#).unwrap(), Ok("2")),
			(Ctx::try_from(r#"{"m": 10}"#).unwrap(), Ok("55")),
		]
	),
	(
		function_05,
		r#"{{fn vecPlus; v}}{{v.x = v.x + 1}}{{return v.x}}{{done}}{{vecPlus({"x": 1})}} vs {{v.x}}"#,
		vec![
			(Ctx::try_from("{\"v\": {\"x\": 1}}").unwrap(), Ok("2 vs 1")),
			(Ctx::try_from("{\"v\": {\"x\": 0}}").unwrap(), Ok("2 vs 0")),
		]
	)
);
