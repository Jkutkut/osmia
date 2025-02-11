use crate::{
	macro_tests,
	model::ctx::Ctx,
	tests::test,
};

mod lib;
mod assign;
mod expr;
mod get_ctx;
mod r#if;
mod r#for;
mod r#while;
mod r#break;
mod r#continue;
mod lambda;
mod function;

fn interpreter_test(
	code: &str,
	execution: Vec<(Ctx, Result<&str, Vec<&str>>)>
) {
	test(Some(code), None, None, Some(execution));
}
