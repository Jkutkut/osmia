use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
	Builtin(Builtin),
	Lambda(LambdaCallable),
	Function(FunctionCallable),
}

impl Callable {
	pub fn new(
		arity: usize, call: BuiltinArg,
		#[cfg(feature = "detailed-dumper")] description: &str
	) -> Self {
		Self::Builtin(Builtin::new(
			arity, call,
			#[cfg(feature = "detailed-dumper")]
			description
		))
	}

	pub fn new_variable_args(
		call: BuiltinArg,
		#[cfg(feature = "detailed-dumper")] description: &str
	) -> Self {
		Self::Builtin(Builtin::new_variable_args(
			call,
			#[cfg(feature = "detailed-dumper")]
			description
		))
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

	pub fn call(&self, intpr: &OsmiaInterpreter<'_>, args: &Vec<Expr>) -> Result<Expr, OsmiaError> {
		if let Some(arity) = self.arity() {
			if args.len() != arity {
				return Err(self.argc_error(args.len()));
			}
		}
		match self {
			Callable::Builtin(f) => f.call(intpr, args),
			Callable::Lambda(l) => l.call(intpr, args),
			Callable::Function(_) => unreachable!("Function should be called with the call_stmt method"),
		}
	}

	pub fn call_stmt(&self, intpr: &OsmiaInterpreter<'_>, args: &Vec<Expr>) -> Result<Stmt, OsmiaError> {
		if let Some(arity) = self.arity() {
			if args.len() != arity {
				return Err(self.argc_error(args.len()));
			}
		}
		match self {
			Callable::Builtin(f) => Ok(Stmt::Expr(f.call(intpr, args)?)),
			Callable::Lambda(l) => Ok(Stmt::Expr(l.call(intpr, args)?)),
			Callable::Function(f) => f.call(intpr, args),
		}
	}

	#[cfg(feature = "detailed-dumper")]
	pub fn description(&self) -> Option<String> {
		match self {
			Callable::Builtin(b) => Some(b.description().into()),
			_ => None,
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
