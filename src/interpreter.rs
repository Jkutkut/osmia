use serde_json::{
	Map,
	Value
};
use crate::syntax_tree::model::Stmt;

pub type Ctx = Map<String, Value>;


#[derive(Debug)]
pub enum InterpreterValue {
	String(String),
	Void
}

type InterpreterResult = Result<(ExitStatus, InterpreterValue), String>;

#[derive(Debug, PartialEq)]
pub enum ExitStatus {
	Okay,
	// Break,
	// Continue
}

pub struct Interpreter<'a> {
	code: Stmt<'a>,
	ctx: &'a mut Ctx 
}

impl<'a> Interpreter<'a> {
	pub fn new(code: Stmt<'a>, ctx: &'a mut Ctx) -> Self {
		Self { code, ctx }
	}

	pub fn run(&mut self) -> InterpreterResult {
		self.ctx.insert("?".to_string(), serde_json::json!(12));
		println!("Code: {:?}", self.code);
		println!("Ctx: {:?}", self.ctx);
		if let Value::Number(n) = self.ctx.get("?").unwrap() {
			println!("Value: {:?}", n);
			if n.as_i64().unwrap() == 11 {
				return Ok((ExitStatus::Okay, InterpreterValue::Void));
			}
		}
		Err("not implemented".to_string())
	}
}

