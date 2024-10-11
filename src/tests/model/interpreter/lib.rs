use super::*;

macro_tests!(
	interpreter_test,
	(
		contant_01,
		"{{ math.pi }}",
		vec![
			(Ctx::new(), Ok("3.141592653589793")]),
			(Ctx::empty(), Err(vec!["not", "found"]))
		]
	)
);
