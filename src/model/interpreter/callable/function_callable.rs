use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCallable {
	ft: Function,
}

impl FunctionCallable {
	pub fn new(ft: Function) -> Self {
		Self { ft }
	}

	pub fn arity(&self) -> usize {
		self.ft.params().len()
	}

	pub fn call(&self, ctx: &mut Ctx, args: CallableArgs) -> Result<Stmt, OsmiaError> {
		let arity = self.arity();
		let mut i = 0;
		while i < arity {
			ctx.set(
				&mut vec![self.ft.params()[i].name().clone()].iter(),
				(&args[i]).try_into()?,
			)?;
			i += 1;
		}
		Ok(self.ft.body().clone())
	}
}
