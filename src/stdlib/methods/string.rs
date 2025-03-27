use super::*;

fn string_or_fail(expr: &Expr) -> Result<&str, OsmiaError> {
	match expr {
		Expr::Str(s) => Ok(s),
		_ => Err("This method should be called on a string".into()),
		// TODO unreachable
	}
}

pub fn module() -> Module {
	Module::new()
	.add_value("lower", Callable::new(1,
		|_, args| Ok(Expr::Str(string_or_fail(&args[0])?.to_lowercase().into()))
	).into())
	.add_value("upper", Callable::new(1,
		|_, args| Ok(Expr::Str(string_or_fail(&args[0])?.to_uppercase().into()))
	).into())
	.add_value("trim", Callable::new(1,
		|_, args| Ok(Expr::Str(string_or_fail(&args[0])?.trim().into()))
	).into())
	.add_value("capitalize", Callable::new(1,
		|_, args| {
			let mut s = string_or_fail(&args[0])?.chars();
			let mut result = String::new();
			let mut first = true;
			while let Some(c) = s.next() {
				if c.is_alphabetic() && first {
					result.push(c.to_uppercase().next().unwrap());
					first = false;
				} else {
					result.push(c);
					first = !c.is_alphabetic();
				}
			}
			Ok(Expr::Str(result))
		}
	).into())
	.add_value("starts_with", Callable::new(2,
		|_, args| Ok(Expr::Bool(string_or_fail(&args[0])?.starts_with(string_or_fail(&args[1])?).into()))
	).into())
	.add_value("ends_with", Callable::new(2,
		|_, args| Ok(Expr::Bool(string_or_fail(&args[0])?.ends_with(string_or_fail(&args[1])?).into()))
	).into())
	.add_value("ensure_starts_with", Callable::new(2,
		|_, args| {
			let s = string_or_fail(&args[0])?;
			let prefix = string_or_fail(&args[1])?;
			match s.starts_with(prefix) {
				true => Ok(Expr::Str(s.into())),
				false => Ok(Expr::Str(format!("{}{}", prefix, s).into()))
			}
		}
	).into())
	.add_value("ensure_ends_with", Callable::new(2,
		|_, args| {
			let s = string_or_fail(&args[0])?;
			let suffix = string_or_fail(&args[1])?;
			match s.ends_with(suffix) {
				true => Ok(Expr::Str(s.into())),
				false => Ok(Expr::Str(format!("{}{}", s, suffix).into()))
			}
		}
	).into())
	.add_value("index_of", Callable::new(2,
		|_, args| match string_or_fail(&args[0])?.find(string_or_fail(&args[1])?) {
			Some(i) => Ok(Expr::Int(i as i64)),
			None => Ok(Expr::Int(-1))
		}
	).into())
	.add_value("last_index_of", Callable::new(2,
		|_, args| match string_or_fail(&args[0])?.rfind(string_or_fail(&args[1])?) {
			Some(i) => Ok(Expr::Int(i as i64)),
			None => Ok(Expr::Int(-1))
		}
	).into())
}
