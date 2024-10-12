use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct LambdaCallable {
	lambda: Lambda,
}

impl LambdaCallable {
	pub fn new(lambda: Lambda) -> Self {
		Self { lambda }
	}

	pub fn arity(&self) -> usize {
		self.lambda.params().len()
	}

	pub fn call(&self, ctx: &mut Ctx, args: CallableArgs) -> Result<Expr, OsmiaError> {
		let arity = self.arity();
		let mut i = 0;
		while i < arity {
			ctx.set(
				&mut vec![self.lambda.params()[i].name().clone()].iter(),
				(&args[i]).try_into()?,
			)?;
			i += 1;
		}
		Ok(self.lambda.body().clone())
	}
}
