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

	pub fn new_variable_args(call: BuiltinArg) -> Self {
		Self::Builtin(Builtin::new_variable_args(call))
	}

	pub fn arity(&self) -> Option<usize> {
		match self {
			Callable::Builtin(f) => f.arity(),
			Callable::Lambda(l) => l.arity(),
			Callable::Function(f) => f.arity(),
		}
	}

	fn argc_error(&self, argc: usize) -> OsmiaError {
		let arity = match self.arity() {
			Some(a) => a.to_string(),
			None => "infinite".to_string(),
		};
		return format!(
			"Expected {} arguments, got {}",
			arity, argc
		);
	}

	pub fn call(&self, ctx: &CtxRef, args: &Vec<Expr>) -> Result<Expr, OsmiaError> {
		if let Some(arity) = self.arity() {
			if args.len() != arity {
				return Err(self.argc_error(args.len()));
			}
		}
		match self {
			Callable::Builtin(f) => f.call(ctx, args),
			Callable::Lambda(l) => l.call(ctx, args),
			Callable::Function(_) => unreachable!("Function should be called with the call_stmt method"),
		}
	}

	pub fn call_stmt(&self, ctx: &CtxRef, args: &Vec<Expr>) -> Result<Stmt, OsmiaError> {
		if let Some(arity) = self.arity() {
			if args.len() != arity {
				return Err(self.argc_error(args.len()));
			}
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
