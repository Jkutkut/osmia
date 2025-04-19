use super::*;

macro_tests!(
	interpreter_test,
	(
		if01,
		"{{if true}}i{{fi}}",
		vec![
			(Ctx::try_from(r#"{}"#).unwrap(), Ok("i"))
		]
	),
	(
		if02,
		"{{if false}}i{{fi}}",
		vec![
			(Ctx::try_from(r#"{}"#).unwrap(), Ok(""))
		]
	),
	(
		if03,
		"{{if true}}i{{else}}e{{fi}}",
		vec![
			(Ctx::try_from(r#"{}"#).unwrap(), Ok("i"))
		]
	),
	(
		if04,
		"{{if false}}i{{else}}e{{fi}}",
		vec![
			(Ctx::try_from(r#"{}"#).unwrap(), Ok("e"))
		]
	),
	(
		if05,
		"{{if v==1}}if{{elseif v==2}}elseif{{else}}else{{fi}}",
		vec![
			(Ctx::try_from(r#"{"v": 1}"#).unwrap(), Ok("if"))
		]
	),
	(
		if06,
		"{{if v==1}}if{{elseif v==2}}elseif{{else}}else{{fi}}",
		vec![
			(Ctx::try_from(r#"{"v": 2}"#).unwrap(), Ok("elseif"))
		]
	),
	(
		if07,
		"{{if v==1}}if{{elseif v==2}}elseif{{else}}else{{fi}}",
		vec![
			(Ctx::try_from(r#"{"v": 3}"#).unwrap(), Ok("else"))
		]
	),
	(
		if08,
		"{{if v==1}}if{{elseif v==2}}elseif01{{elseif v==3}}elseif02{{else}}else{{fi}}",
		vec![
			(Ctx::try_from(r#"{"v": 3}"#).unwrap(), Ok("elseif02"))
		]
	),
	(
		if09,
		"{{if v1}}if{{if v2}}if01{{else}}else01{{fi}}{{else}}else02{{fi}}",
		vec![
			(Ctx::try_from(r#"{"v1": true, "v2": true}"#).unwrap(), Ok("ifif01"))
		]
	),
	(
		if10,
		"{{if v1}}if{{if v2}}if01{{else}}else01{{fi}}{{else}}else02{{fi}}",
		vec![
			(Ctx::try_from(r#"{"v1": true, "v2": false}"#).unwrap(), Ok("ifelse01"))
		]
	),
	(
		if11,
		"{{if v1}}if{{if v2}}if01{{else}}else01{{fi}}{{else}}else02{{fi}}",
		vec![
			(Ctx::try_from(r#"{"v1": false, "v2": true}"#).unwrap(), Ok("else02"))
		]
	),
	(
		if12,
		"{{if v1}}if{{if v2}}if01{{else}}else01{{fi}}{{else}}else02{{fi}}",
		vec![
			(Ctx::try_from(r#"{"v1": false, "v2": false}"#).unwrap(), Ok("else02"))
		]
	)
);
