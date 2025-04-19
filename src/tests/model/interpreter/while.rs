use super::*;

macro_tests!(
	interpreter_test,
	(
		while01,
		"{{while v < 3}}{{ v }}{{v = v + 1}}{{done}}",
		vec![
			(Ctx::try_from(r#"{"v": 0}"#).unwrap(), Ok("012"))
		]
	),
	(
		while02,
		"{{while v != 0}}{{v % 2}}{{v = v - 1}}{{done}}",
		vec![
			(Ctx::try_from(r#"{"v": 10}"#).unwrap(), Ok("0101010101"))
		]
	),
	(
		whilefalse,
		"{{while false}}123{{done}}",
		vec![
			(Ctx::try_from(r#"{}"#).unwrap(), Ok(""))
		]
	),
	(
		while03,
		"{{while v != 10000}}{{v = v + 1}}{{done}}{{v}}",
		vec![
			(Ctx::try_from(r#"{"v": 0}"#).unwrap(), Ok("10000"))
		]
	)
);
