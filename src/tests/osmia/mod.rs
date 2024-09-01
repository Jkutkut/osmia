mod assign;
mod basic;
mod callable;
mod r#for;
mod function;
mod get_ctx;
mod gh;
mod r#if;
mod lambda;
mod loop_blocks;
mod operations;
mod print;
mod scripts;
mod r#while;

use crate::macro_tests;
use super::test;
use crate::model::lexer::Token;
use crate::model::code::*;

fn new_binary(left: Expr, op: Token, right: Expr) -> Expr {
	let op: Option<BinaryOp> = (&op).into();
	Binary::new(left, op.unwrap(), right).into()
}
