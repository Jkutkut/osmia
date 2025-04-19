use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCallable {
	ft: Function,
}

impl FunctionCallable {
	pub fn new(ft: Function) -> Self {
		Self { ft }
	}

	pub fn arity(&self) -> Option<usize> {
		Some(self.ft.params().len())
	}

	pub fn call(&self, intpr: &OsmiaInterpreter<'_>, args: CallableArgs) -> Result<Stmt, OsmiaError> {
		let arity = self.arity().unwrap_or(args.len());
		let mut i = 0;
		while i < arity {
			intpr.ctx.borrow_mut().set_in_current_scope(
				&vec![self.ft.params()[i].name().clone()],
				(&args[i]).try_into()?,
			)?;
			i += 1;
		}
		Ok(self.ft.body().clone())
	}

	pub fn params(&self) -> &Vec<FunctionParam> {
		self.ft.params()
	}
}

impl std::fmt::Display for FunctionCallable {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.ft)
	}
}
