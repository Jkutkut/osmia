use super::*;

pub fn module() -> Module {
	Module::new(
		vec![
			("PI", CtxValue::Float(std::f64::consts::PI)).into(),
			("E", CtxValue::Float(std::f64::consts::E)).into(),
		],
		vec![
			("abs", Callable::new(
				1,
				|_, args| {
					Ok(Expr::Float(args[0].to_float()?.abs()))
				}
			)).into(),
			("ceil", Callable::new(
				1,
				|_, args| {
					Ok(Expr::Float(args[0].to_float()?.ceil()))
				}
			)).into(),
			("floor", Callable::new(
				1,
				|_, args| {
					Ok(Expr::Float(args[0].to_float()?.floor()))
				}
			)).into(),
			("pow", Callable::new(
				2,
				|_, args| {
					Ok(Expr::Float(args[0].to_float()?.powf(args[1].to_float()?)))
				}
			)).into(),
			("sqrt", Callable::new(
				1,
				|_, args| {
					Ok(Expr::Float(args[0].to_float()?.sqrt()))
				}
			)).into(),
			("round", Callable::new(
				1,
				|_, args| {
					Ok(Expr::Float(args[0].to_float()?.round()))
				}
			)).into(),
		]
	)
}
