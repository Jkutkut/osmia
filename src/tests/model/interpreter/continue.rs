use super::*;

macro_tests!(
	interpreter_test,
	(
		continue_01,
		"{{v = 0}}{{while v < 4}}{{v = v + 1}}{{if v < 3}}{{continue}}{{fi}}{{v}}{{done}}",
		vec![
			(Ctx::try_from("{}").unwrap(), Ok("34")),
		]
	),
	(
		continue_02,
		"{{for i in [1, 2]}}{{i}}{{continue}}{{done}}",
		vec![
			(Ctx::try_from("{}").unwrap(), Ok("12")),
		]
	),
	(
		continue_03,
		"{{for i in [1, 2, 3, 4]}}{{if i % 2 == 1}}{{continue}}{{fi}}{{i}}{{done}}",
		vec![
			(Ctx::try_from("{}").unwrap(), Ok("24")),
		]
	),
	(
		continue_04,
		"{{continue}}",
		vec![
			(Ctx::try_from("{}").unwrap(), Err(vec!["continue", "program"])),
		]
	)
);
