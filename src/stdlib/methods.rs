use super::*;
use generics::add_generics;

pub fn module() -> Module {
	Module::new()
	.add_module(
		MethodExpression::Str.into(),
		add_generics(string())
	)
	.add_module(
		MethodExpression::Int.into(),
		add_generics(int())
	)
	.add_module(
		MethodExpression::Float.into(),
		add_generics(float())
	)
	.add_module(
		MethodExpression::Bool.into(),
		add_generics(bool())
	)
	.add_module(
		MethodExpression::Null.into(),
		add_generics(null())
	)
	.add_module(
		MethodExpression::Array.into(),
		add_generics(array())
	)
	.add_module(
		MethodExpression::Object.into(),
		add_generics(object())
	)
	.add_module(
		MethodExpression::Callable.into(),
		add_generics(callable())
	)
	.add_module(
		MethodExpression::Lambda.into(),
		add_generics(lambda())
	)
}

#[allow(non_upper_case_globals)]
mod generics {
	use super::*;

	const len: BuiltinArg = |_, args| {
		match &args[0] {
			Expr::Str(s) => Ok(Expr::Int(s.len() as i64)),
			Expr::Array(arr) => Ok(Expr::Int(arr.len() as i64)),
			_ => return Err("Argument must be a string or array".into()),
		}
	};

	const has_content: BuiltinArg = |_, args| {
		match &args[0] {
			Expr::Str(s) => Ok(Expr::Bool(!s.is_empty())),
			Expr::Array(arr) => Ok(Expr::Bool(arr.len() != 0)),
			Expr::Object(obj) => Ok(Expr::Bool(obj.len() != 0)),
			Expr::Int(_) | Expr::Float(_) => Ok(Expr::Bool(true)),
			Expr::Bool(_) => Ok(Expr::Bool(true)),
			Expr::Null => Ok(Expr::Bool(false)),
			Expr::Callable(_) => Ok(Expr::Bool(true)),
			_ => unreachable!()
		}
	};

	pub fn add_generics(module: Module) -> Module {
		module
		.add_value("len", Callable::new(1, len).into())
		.add_value("has_content", Callable::new(1, has_content).into())
	}
}

fn string() -> Module {
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

fn int() -> Module {
	Module::new()
}

fn float() -> Module {
	Module::new()
}

fn bool() -> Module {
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

fn null() -> Module {
	Module::new()
}

fn array() -> Module {
	Module::new()
}

fn object() -> Module {
	Module::new()
}

fn callable() -> Module {
	Module::new()
}

fn lambda() -> Module {
	Module::new()
}
