use crate::{
	macro_tests,
	model::ctx::Ctx,
	tests::test,
};

mod expr;
mod ctx_get;
mod r#if;

fn interpreter_test(
	code: &str,
	execution: Vec<(Ctx, Result<&str, Vec<&str>>)>
) {
	test(Some(code), None, None, Some(execution));
}
