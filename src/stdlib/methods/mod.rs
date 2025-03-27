use super::*;
use generics::add_generics;
use crate::OsmiaError;

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
