use super::*;

pub type BuiltinArg = fn(ctx: &mut Ctx, args: CallableArgs) -> Result<Expr, OsmiaError>;

#[derive(Debug, Clone, PartialEq)]
pub struct Builtin {
	arity: usize,
	call: BuiltinArg,
	params: Option<Vec<FunctionParam>>,
}

impl Builtin {
	pub fn new(arity: usize, call: BuiltinArg) -> Self {
		Self { arity, call, params: None }
	}

	pub fn with_params(arity: usize, call: BuiltinArg, params: Vec<FunctionParam>) -> Self {
		Self { arity, call, params: Some(params) }
	}

	pub fn arity(&self) -> usize {
		self.arity
	}

	pub fn call(&self, ctx: &mut Ctx, args: CallableArgs) -> Result<Expr, OsmiaError> {
		(self.call)(ctx, args)
	}

	pub fn params(&self) -> Option<&Vec<FunctionParam>> {
		self.params.as_ref()
	}
}

impl std::fmt::Display for Builtin {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let args: String = (0..self.arity)
			.map(|idx| format!("arg{idx}"))
			.collect::<Vec<String>>().join(", ");
		write!(f, "{{ fn ({args}) => ... }}")
	}
}
