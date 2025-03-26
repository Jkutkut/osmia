use super::*;

pub fn module() -> Module {
	Module::new()
	// Constants
	.add_value("PI", CtxValue::Float(std::f64::consts::PI))
	.add_value("E", CtxValue::Float(std::f64::consts::E))
	// Methods
	.add_value("abs", Callable::new(
		1,
		|_, args| {
			Ok(Expr::Float(args[0].to_float()?.abs()))
		}
	).into())
	.add_value("ceil", Callable::new(
		1,
		|_, args| {
			Ok(Expr::Float(args[0].to_float()?.ceil()))
		}
	).into())
	.add_value("floor", Callable::new(
		1,
		|_, args| {
			Ok(Expr::Float(args[0].to_float()?.floor()))
		}
	).into())
	.add_value("pow", Callable::new(
		2,
		|_, args| {
			Ok(Expr::Float(args[0].to_float()?.powf(args[1].to_float()?)))
		}
	).into())
	.add_value("sqrt", Callable::new(
		1,
		|_, args| {
			Ok(Expr::Float(args[0].to_float()?.sqrt()))
		}
	).into())
	.add_value("round", Callable::new(
		1,
		|_, args| {
			Ok(Expr::Float(args[0].to_float()?.round()))
		}
	).into())
}
