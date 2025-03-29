use super::*;

pub fn module() -> Module {
	Module::new()
	.add_value("keys", Callable::new(1,
		|_, args| Ok(Expr::Array(obj_or_fail(&args[0])?.keys().into()))
	).into())
	.add_value("values", Callable::new(1,
		|_, args| Ok(Expr::Array(obj_or_fail(&args[0])?.values().into()))
	).into())
	.add_value("entries", Callable::new(1,
		|_, args| {
			Ok(Expr::Array(obj_or_fail(&args[0])?.entries().iter().map(|(k, v)| {
				Expr::Object(Object::new(vec![
					(Expr::new_str("key"), k.clone()),
					(Expr::new_str("value"), v.clone()),
				]))
			}).collect::<Vec<Expr>>().into()))
		}
	).into())
}
