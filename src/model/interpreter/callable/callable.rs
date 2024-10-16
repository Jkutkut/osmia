use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
	Builtin(Builtin),
	Lambda(LambdaCallable),
	Function(FunctionCallable),
}

impl Callable {
	pub fn new(arity: usize, call: BuiltinArg) -> Self {
		Self::Builtin(Builtin::new(arity, call))
	}

	pub fn arity(&self) -> usize {
		match self {
			Callable::Builtin(f) => f.arity(),
			Callable::Lambda(l) => l.arity(),
			Callable::Function(f) => f.arity(),
		}
	}

	fn argc_error(&self, argc: usize) -> OsmiaError {
		return format!(
			"Expected {} arguments, got {}",
			self.arity(), argc
		);
	}

	pub fn call(&self, ctx: &mut Ctx, args: &Vec<Expr>) -> Result<Expr, OsmiaError> {
		if args.len() != self.arity() {
			return Err(self.argc_error(args.len()));
		}
		match self {
			Callable::Builtin(f) => f.call(ctx, args),
			Callable::Lambda(l) => l.call(ctx, args),
			Callable::Function(_) => unreachable!("Function should be called with the call_stmt method"),
		}
	}

	pub fn call_stmt(&self, ctx: &mut Ctx, args: &Vec<Expr>) -> Result<Stmt, OsmiaError> {
		if args.len() != self.arity() {
			return Err(self.argc_error(args.len()));
		}
		match self {
			Callable::Builtin(f) => Ok(Stmt::Expr(f.call(ctx, args)?)),
			Callable::Lambda(l) => Ok(Stmt::Expr(l.call(ctx, args)?)),
			Callable::Function(f) => f.call(ctx, args),
		}
	}
}

impl std::fmt::Display for Callable {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Callable::Builtin(b) => write!(f, "{b}"),
			Callable::Lambda(l) => write!(f, "{l}"),
			Callable::Function(ft) => write!(f, "{ft}"),
		}
	}
}
