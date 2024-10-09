use super::*;

macro_tests!(
	interpreter_test,
	(
		break_01,
		"{{while true}}foo{{break}}{{done}}",
		vec![
			(Ctx::try_from("{}").unwrap(), Ok("foo")),
		]
	),
	(
		break_02,
		"{{v = 0}}{{while true}}{{v}}{{v = v + 1}}{{if v == 3}}{{break}}{{else}}-{{fi}}{{done}}",
		vec![
			(Ctx::try_from("{}").unwrap(), Ok("0-1-2")),
		]
	),
	(
		break_03,
		"{{for i in [1, 2]}}{{i}}{{break}}{{done}}",
		vec![
			(Ctx::try_from("{}").unwrap(), Ok("1")),
		]
	),
	(
		break_04,
		"{{for i in [1, 2, 3, 4]}}{{i}}{{if i == 3}}{{break}}{{fi}}-{{done}}",
		vec![
			(Ctx::try_from("{}").unwrap(), Ok("1-2-3")),
		]
	)
);
