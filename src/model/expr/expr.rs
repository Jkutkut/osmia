use super::*;

/// Expressions
///
/// A expression is the smallest building block of the language.
///
/// # Operations:
/// - [Addition](#method.add)
/// - [Subtraction](#method.sub)
/// - [Multiplication](#method.mul)
/// - [Division](#method.div)
/// - [Module / Remainder](#method.rem)
/// - [Comparison](#method.partial_cmp)
/// - Bitwise operations: [and](#method.bitand), [or](#method.bitor), [xor](#method.bitxor), [shl](#method.shl), [shr](#method.shr), [not](#method.not), [neg](#method.neg)
///
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
	Binary(Binary),
	Grouping(Grouping),
	Unary(Unary),

	Lambda(Lambda),
	Object(Object),
	Array(Array),
	Call(Call),
	MethodCall(MethodCall),
	Variable(Variable),

	Float(f64),
	Int(i64),
	Str(String),
	Bool(bool),
	Null,

	Callable(Callable),
}

impl Expr {
	pub fn new_str(s: &str) -> Self {
		Self::Str(s.to_string())
	}

	pub fn r#type(&self) -> String {
		match self {
			Expr::Binary(_) => "binary",
			Expr::Grouping(_) => "grouping",
			Expr::Unary(_) => "unary",
			Expr::Lambda(_) => "lambda",
			Expr::Object(_) => "object",
			Expr::Array(_) => "array",
			Expr::Call(_) => "call",
			Expr::MethodCall(_) => "method_call",
			Expr::Variable(_) => "variable",
			Expr::Float(_) => "float",
			Expr::Int(_) => "int",
			Expr::Str(_) => "string",
			Expr::Bool(_) => "bool",
			Expr::Null => "null",
			Expr::Callable(c) => match c {
				Callable::Function(_) | Callable::Builtin(_) => "function",
				Callable::Lambda(_) => "lambda",
			}
		}.to_string()
	}
}

impl From<Binary> for Expr {
	fn from(b: Binary) -> Self {
		Self::Binary(b)
	}
}

impl From<Grouping> for Expr {
	fn from(g: Grouping) -> Self {
		Self::Grouping(g)
	}
}

impl From<Unary> for Expr {
	fn from(u: Unary) -> Self {
		Self::Unary(u)
	}
}

impl From<Lambda> for Expr {
	fn from(l: Lambda) -> Self {
		Self::Lambda(l)
	}
}

impl From<Object> for Expr {
	fn from(o: Object) -> Self {
		Self::Object(o)
	}
}

impl From<Array> for Expr {
	fn from(a: Array) -> Self {
		Self::Array(a)
	}
}

impl From<Call> for Expr {
	fn from(c: Call) -> Self {
		Self::Call(c)
	}
}

impl From<MethodCall> for Expr {
	fn from(m: MethodCall) -> Self {
		Self::MethodCall(m)
	}
}

impl From<Variable> for Expr {
	fn from(v: Variable) -> Self {
		Self::Variable(v)
	}
}
