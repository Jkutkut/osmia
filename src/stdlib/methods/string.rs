use super::*;

pub fn module() -> Module {
	Module::new()
	.add_value("lower", Callable::new(
		1,
		|_, args| {
			match &args[0] {
				Expr::Str(s) => Ok(Expr::Str(s.to_lowercase().into())),
				_ => return Err("Argument must be a string".into()),
			}
		}
	).into())
	.add_value("upper", Callable::new(
		1,
		|_, args| {
			match &args[0] {
				Expr::Str(s) => Ok(Expr::Str(s.to_uppercase().into())),
				_ => return Err("Argument must be a string".into()),
			}
		}
	).into())
	.add_value("trim", Callable::new(
		1,
		|_, args| {
			match &args[0] {
				Expr::Str(s) => Ok(Expr::Str(s.trim().into())),
				_ => return Err("Argument must be a string".into()),
			}
		}
	).into())
}
