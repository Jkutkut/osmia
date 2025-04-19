use crate::model::expr::Expr;

#[derive(Debug)]
pub enum MethodExpression {
	Str,
	Int,
	Float,
	Bool,
	Null,
	Array,
	Object,
	Callable,
	Lambda
}

impl TryFrom<&Expr> for MethodExpression {
	type Error = String;
	fn try_from(expr: &Expr) -> Result<Self, Self::Error> {
		match expr {
			Expr::Str(_) => Ok(MethodExpression::Str),
			Expr::Int(_) => Ok(MethodExpression::Int),
			Expr::Float(_) => Ok(MethodExpression::Float),
			Expr::Bool(_) => Ok(MethodExpression::Bool),
			Expr::Null => Ok(MethodExpression::Null),
			Expr::Array(_) => Ok(MethodExpression::Array),
			Expr::Object(_) => Ok(MethodExpression::Object),
			Expr::Callable(_) => Ok(MethodExpression::Callable),
			Expr::Lambda(_) => Ok(MethodExpression::Lambda),
			_ => Err(format!("Could not convert expression {} to method expression", expr)),
		}
	}
}

impl<'a> Into<&'a str> for MethodExpression {
	fn into(self) -> &'a str {
		match self {
			MethodExpression::Str => "str",
			MethodExpression::Int => "int",
			MethodExpression::Float => "float",
			MethodExpression::Bool => "bool",
			MethodExpression::Null => "null",
			MethodExpression::Array => "array",
			MethodExpression::Object => "object",
			MethodExpression::Callable => "callable",
			MethodExpression::Lambda => "lambda",
		}
	}
}

impl Into<String> for &MethodExpression {
	fn into(self) -> String {
		match self {
			MethodExpression::Str => "str".into(),
			MethodExpression::Int => "int".into(),
			MethodExpression::Float => "float".into(),
			MethodExpression::Bool => "bool".into(),
			MethodExpression::Null => "null".into(),
			MethodExpression::Array => "array".into(),
			MethodExpression::Object => "object".into(),
			MethodExpression::Callable => "callable".into(),
			MethodExpression::Lambda => "lambda".into(),
		}
	}
}
