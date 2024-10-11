use std::fmt::Debug;

use crate::types::{
	Ctx,
	OsmiaError,
};
use crate::model::{
	expr::Expr,
};

type CallableArgs<'c> = &'c Vec<Expr>;
type CallableFt = fn(ctx: &mut Ctx, args: CallableArgs) -> Result<Expr, OsmiaError>;

#[derive(Debug, Clone, PartialEq)]
pub struct Callable {
	arity: usize,
	call: CallableFt,
}

impl Callable {
	pub fn new(arity: usize, call: CallableFt) -> Self {
		Self { arity, call }
	}

	fn arity(&self) -> usize {
		self.arity
	}

	fn argc_error(&self, argc: usize) -> OsmiaError {
		return format!(
			"Expected {} arguments, got {}",
			self.arity, argc
		);
	}

	pub fn call(&self, ctx: &mut Ctx, args: CallableArgs) -> Result<Expr, OsmiaError> {
		if args.len() != self.arity() {
			return Err(self.argc_error(args.len()));
		}
		(self.call)(ctx, args)
	}
}
