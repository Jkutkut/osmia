use super::*;

pub type BuiltinArg = fn(ctx: &CtxRef, args: CallableArgs) -> Result<Expr, OsmiaError>;

#[derive(Debug, Clone, PartialEq)]
pub struct Builtin {
	arity: Option<usize>,
	call: BuiltinArg,
	params: Option<Vec<FunctionParam>>,
}

impl Builtin {
	pub fn new(arity: usize, call: BuiltinArg) -> Self {
		Self { arity: Some(arity), call, params: None }
	}

	pub fn new_variable_args(call: BuiltinArg) -> Self {
		Self { arity: None, call, params: None }
	}

	pub fn with_params(arity: usize, call: BuiltinArg, params: Vec<FunctionParam>) -> Self {
		Self { arity: Some(arity), call, params: Some(params) }
	}

	pub fn arity(&self) -> Option<usize> {
		self.arity
	}

	pub fn call(&self, ctx: &CtxRef, args: CallableArgs) -> Result<Expr, OsmiaError> {
		(self.call)(ctx, args)
	}

	pub fn params(&self) -> Option<&Vec<FunctionParam>> {
		self.params.as_ref()
	}
}

impl std::fmt::Display for Builtin {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{{ fn (...) => ... }}")
	}
}
