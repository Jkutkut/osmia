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
		},
		#[cfg(feature = "detailed-dumper")]
		"Returns either value or else"
	).into())
	.add_value("not", Callable::new(1,
		|_, args| Ok(Expr::Bool(!boolean(&args[0]))),
		#[cfg(feature = "detailed-dumper")]
		"Returns the boolean oposite"
	).into())
	.add_value("and", Callable::new(2,
		|_, args| Ok(Expr::Bool(boolean(&args[0]) && boolean(&args[1]))),
		#[cfg(feature = "detailed-dumper")]
		"Returns true if both are true"
	).into())
	.add_value("or", Callable::new(2,
		|_, args| Ok(Expr::Bool(boolean(&args[0]) || boolean(&args[1]))),
		#[cfg(feature = "detailed-dumper")]
		"Returns true if at least one is true"
	).into())
	.add_value("nand", Callable::new(2,
		|_, args| Ok(Expr::Bool(!(boolean(&args[0]) && boolean(&args[1])))),
		#[cfg(feature = "detailed-dumper")]
		"Returns true if both are false"
	).into())
	.add_value("nor", Callable::new(2,
		|_, args| Ok(Expr::Bool(!(boolean(&args[0]) || boolean(&args[1])))),
		#[cfg(feature = "detailed-dumper")]
		"Returns true if at least one is false"
	).into())
	.add_value("xor", Callable::new(2,
		|_, args| Ok(Expr::Bool(boolean(&args[0]) ^ boolean(&args[1]))),
		#[cfg(feature = "detailed-dumper")]
		"Returns true if exactly one is true"
	).into())
	.add_value("xnor", Callable::new(2,
		|_, args| Ok(Expr::Bool(!(boolean(&args[0]) ^ boolean(&args[1])))),
		#[cfg(feature = "detailed-dumper")]
		"Returns true if both have the same value"
	).into())
}
