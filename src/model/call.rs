use super::{
	Expression, JsonExpression,
};
use crate::lexer::Token;

// TODO refactor into file
#[derive(Debug, PartialEq, Clone)]
pub enum Callable {
	Call(Box<Call>),
	MethodCall(Box<MethodCall>),
}

impl Callable {
	pub fn new_call(callee: Expression, args: Vec<JsonExpression>) -> Self {
		Self::Call(Box::new(Call::new(callee, args)))
	}

	pub fn new_method_call(expr: Expression, callable: Callable) -> Self {
		Self::MethodCall(Box::new(MethodCall::new(expr, callable)))
	}
}

impl std::fmt::Display for Callable {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Callable::Call(c) => write!(f, "{}", c),
			Callable::MethodCall(m) => write!(f, "{}", m),
		}
	}
}


// TODO refactor into file
#[derive(Debug, PartialEq, Clone)]
pub struct Call {
	callee: Expression,
	args: Vec<JsonExpression>,
}

impl Call {
	pub fn new(callee: Expression, args: Vec<JsonExpression>) -> Self {
		Self { callee, args }
	}

	pub fn callee(&self) -> &Expression {
		&self.callee
	}

	pub fn args(&self) -> &Vec<JsonExpression> {
		&self.args
	}
}

impl std::fmt::Display for Call {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}({})", self.callee, self.args
			.iter()
			.map(|a| a.to_string())
			.collect::<Vec<String>>()
			.join(", "))
	}
}

// TODO refactor into file
#[derive(Debug, PartialEq, Clone)]
pub struct MethodCall {
	expr: Expression,
	call: Callable,
}

impl MethodCall {
	pub fn new(expr: Expression, call: Callable) -> Self {
		Self { expr, call }
	}
}

impl std::fmt::Display for MethodCall {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}{}", self.expr, Token::Question, self.call)
	}
}

// pub trait Callable {
// 	let arity: u8;
// 
// 	fn arity(&self) -> u8 {
// 		self.arity
// 	}
// 
// 	fn call(&self, args: Vec<Expression>) -> Result<Expression, String>;
// }
// 
// pub struct FuctionCall {
// 	ft: String; // TODO
// 	args: Vec<Json>,
// }
