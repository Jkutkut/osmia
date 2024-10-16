use super::*;

pub type BuiltinArg = fn(ctx: &mut Ctx, args: CallableArgs) -> Result<Expr, OsmiaError>;

#[derive(Debug, Clone, PartialEq)]
pub struct Builtin {
	arity: usize,
	call: BuiltinArg,
}

impl Builtin {
	pub fn new(arity: usize, call: BuiltinArg) -> Self {
		Self { arity, call }
	}

	pub fn arity(&self) -> usize {
		self.arity
	}

	pub fn call(&self, ctx: &mut Ctx, args: CallableArgs) -> Result<Expr, OsmiaError> {
		(self.call)(ctx, args)
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
