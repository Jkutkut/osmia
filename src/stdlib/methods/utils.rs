use super::*;

pub fn string_or_fail(expr: &Expr) -> Result<&str, OsmiaError> {
	match expr {
		Expr::Str(s) => Ok(s),
		_ => Err(format!("{} is not a string", expr)),
	}
}

pub fn int_or_fail(expr: &Expr) -> Result<i64, OsmiaError> {
	match expr {
		Expr::Int(i) => Ok(*i),
		_ => Err(format!("{} is not an integer", expr)),
	}
}

pub fn usize_or_fail(expr: &Expr) -> Result<usize, OsmiaError> {
	match int_or_fail(expr)? {
		i if i >= 0 => Ok(i as usize),
		_ => Err(format!("{} is not a positive integer", expr)),
	}
}

pub fn boolean(expr: &Expr) -> bool {
	expr.to_bool()
}

pub fn arr_or_fail(expr: &Expr) -> Result<&Array, OsmiaError> {
	match expr {
		Expr::Array(a) => Ok(a),
		_ => Err(format!("{} is not an array", expr)),
	}
}

pub fn obj_or_fail(expr: &Expr) -> Result<&Object, OsmiaError> {
	match expr {
		Expr::Object(o) => Ok(o),
		_ => Err(format!("{} is not an object", expr)),
	}
}

pub fn callable_or_fail(expr: &Expr) -> Result<&Callable, OsmiaError> {
	match expr {
		Expr::Callable(c) => Ok(c),
		_ => Err(format!("{} is not callable", expr)),
	}
}
