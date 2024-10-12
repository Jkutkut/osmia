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
