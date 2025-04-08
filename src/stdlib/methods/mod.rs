use super::*;
use generics::add_generics;
use crate::OsmiaError;
use crate::model::expr::{
	Array,
	Object
};
use crate::model::visitor_pattern::Visitor;

#[allow(non_upper_case_globals)]
mod generics;
mod string;
mod int;
mod float;
mod bool;
mod null;
mod array;
mod object;
mod callable;
mod lambda;

// TODO refactor
#[allow(dead_code)]
mod utils {
	use super::*;

	pub fn string_or_fail(expr: &Expr) -> Result<&str, OsmiaError> {
		match expr {
			Expr::Str(s) => Ok(s),
			_ => Err(format!("{} is not a string", expr)),
		}
	}

	pub fn int_or_fail(expr: &Expr) -> Result<i64, OsmiaError> {
		match expr {
			Expr::Int(i) => Ok(*i),
			_ => Err(format!("{} is not an integer", expr)),
		}
	}

	pub fn usize_or_fail(expr: &Expr) -> Result<usize, OsmiaError> {
		match int_or_fail(expr)? {
			i if i >= 0 => Ok(i as usize),
			_ => Err(format!("{} is not a positive integer", expr)),
		}
	}

	pub fn boolean(expr: &Expr) -> bool {
		expr.to_bool()
	}

	pub fn arr_or_fail(expr: &Expr) -> Result<&Array, OsmiaError> {
		match expr {
			Expr::Array(a) => Ok(a),
			_ => Err(format!("{} is not an array", expr)),
		}
	}

	pub fn obj_or_fail(expr: &Expr) -> Result<&Object, OsmiaError> {
		match expr {
			Expr::Object(o) => Ok(o),
			_ => Err(format!("{} is not an object", expr)),
		}
	}

	pub fn callable_or_fail(expr: &Expr) -> Result<&Callable, OsmiaError> {
		match expr {
			Expr::Callable(c) => Ok(c),
			_ => Err(format!("{} is not callable", expr)),
		}
	}
}
pub use utils::*;

pub fn module() -> Module {
	Module::new()
	.add_module(
		MethodExpression::Str.into(),
		add_generics(string::module())
	)
	.add_module(
		MethodExpression::Int.into(),
		add_generics(int::module())
	)
	.add_module(
		MethodExpression::Float.into(),
		add_generics(float::module())
	)
	.add_module(
		MethodExpression::Bool.into(),
		add_generics(bool::module())
	)
	.add_module(
		MethodExpression::Null.into(),
		add_generics(null::module())
	)
	.add_module(
		MethodExpression::Array.into(),
		add_generics(array::module())
	)
	.add_module(
		MethodExpression::Object.into(),
		add_generics(object::module())
	)
	.add_module(
		MethodExpression::Callable.into(),
		add_generics(callable::module())
	)
	.add_module(
		MethodExpression::Lambda.into(),
		add_generics(lambda::module())
	)
}
