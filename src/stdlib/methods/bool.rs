use super::*;

pub fn module() -> Module {
	Module::new()
	.add_value("then", Callable::new(
		3,
		|_, args| {
			match &args[0] {
				Expr::Bool(b) => match *b {
					true => Ok(args[1].clone()),
					false => Ok(args[2].clone()),
				}
				_ => return Err("Element must be a boolean".into()),
			}
		}
	).into())
}
