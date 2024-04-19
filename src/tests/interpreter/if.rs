use crate::macro_tests;
use super::{test_interpreter};

macro_tests!(
	test_interpreter,
	(
		if01,
		"{{ if true}}i{{ fi }}",
		r#"{}"#,
		"i"
	),
	(
		if02,
		"{{ if false}}i{{ fi }}",
		r#"{}"#,
		""
	),
	(
		if03,
		"{{ if true}}i{{ else }}e{{ fi }}",
		r#"{}"#,
		"i"
	),
	(
		if04,
		"{{ if false}}i{{ else }}e{{ fi }}",
		r#"{}"#,
		"e"
	),
	(
		if05,
		"{{ if v==1}}if{{elseif v==2}}elseif{{else}}else{{fi}}",
		r#"{"v": 1}"#,
		"if"
	),
	(
		if06,
		"{{ if v==1}}if{{elseif v==2}}elseif{{else}}else{{fi}}",
		r#"{"v": 2}"#,
		"elseif"
	),
	(
		if07,
		"{{ if v==1}}if{{elseif v==2}}elseif{{else}}else{{fi}}",
		r#"{"v": 3}"#,
		"else"
	),
	(
		if08,
		"{{ if v==1}}if{{elseif v==2}}elseif01{{elseif v==3}}elseif02{{else}}else{{fi}}",
		r#"{"v": 3}"#,
		"elseif02"
	),
	(
		if09,
		"{{ if v1}}if{{if v2}}if01{{else}}else01{{fi}}{{else}}else02{{fi}}",
		r#"{"v1": true, "v2": true}"#,
		"ifif01"
	),
	(
		if10,
		"{{ if v1}}if{{if v2}}if01{{else}}else01{{fi}}{{else}}else02{{fi}}",
		r#"{"v1": true, "v2": false}"#,
		"ifelse01"
	),
	(
		if11,
		"{{ if v1}}if{{if v2}}if01{{else}}else01{{fi}}{{else}}else02{{fi}}",
		r#"{"v1": false, "v2": true}"#,
		"else02"
	),
	(
		if12,
		"{{ if v1}}if{{if v2}}if01{{else}}else01{{fi}}{{else}}else02{{fi}}",
		r#"{"v1": false, "v2": false}"#,
		"else02"
	)
);
