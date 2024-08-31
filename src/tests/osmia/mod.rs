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

#[cfg(test)]
fn strarr2var(arr: Vec<&str>) -> Variable {
	Variable::from_vec(arr.into_iter().map(|s| s.into()).collect())
}
