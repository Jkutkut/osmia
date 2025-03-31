use super::*;

const len: BuiltinArg = |_, args| {
	match &args[0] {
		Expr::Str(s) => Ok(Expr::Int(s.len() as i64)),
		Expr::Array(arr) => Ok(Expr::Int(arr.len() as i64)),
		_ => return Err("Cannot get length for this".into()),
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

const to_bool: BuiltinArg = |_, args| {
	Ok(Expr::Bool(args[0].to_bool()))
};

const to_float: BuiltinArg = |_, args| {
	match &args[0].to_float() {
		Ok(f) => Ok(Expr::Float(*f)),
		Err(e) => Err(e.into()),
	}
};

const to_int: BuiltinArg = |_, args| {
	match &args[0].to_int() {
		Ok(i) => Ok(Expr::Int(*i)),
		Err(e) => Err(e.into()),
	}
};

const to_string: BuiltinArg = |_, args| {
	Ok(Expr::Str(args[0].to_string()))
};

const r#type: BuiltinArg = |_, args| {
	Ok(Expr::Str(args[0].r#type()))
};

const switch: BuiltinArg = |_, args| {
	let obj = &args[0];
	let argc = args.len() - 1;
	let else_expr = match argc % 2 == 0 {
		true => Expr::Null,
		false => args[argc].clone(),
	};
	let mut i = 1;
	while i < argc {
		let value = &args[i];
		if obj == value {
			return Ok(args[i + 1].clone());
		}
		i += 2;
	}
	Ok(Expr::Str(else_expr.to_string()))
};

pub fn add_generics(module: Module) -> Module {
	module
	.add_value("len", Callable::new(1, len).into())
	.add_value("has_content", Callable::new(1, has_content).into())
	.add_value("to_bool", Callable::new(1, to_bool).into())
	.add_value("to_float", Callable::new(1, to_float).into())
	.add_value("to_int", Callable::new(1, to_int).into())
	.add_value("to_string", Callable::new(1, to_string).into())
	.add_value("type", Callable::new(1, r#type).into())
	.add_value("switch", Callable::new_variable_args(switch).into())
}
