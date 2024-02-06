use serde_json::{
	Map,
	Value
};
use crate::Token;
use crate::syntax_tree::{
	StmtVisitor, ExprVisitor, StmtVisitable, ExprVisitable
};
use crate::syntax_tree::model::{
	Expression, Literal, Binary, Unary, Grouping, Variable,
	Stmt, ConditionalBlock, Block, Assign, If, ForEach
};

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
	Break,
	Continue
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
		println!("Running interpreter...");
		self.visit_stmt(&self.code)
	}
}

impl StmtVisitor<InterpreterResult> for Interpreter<'_> {
	fn visit_stmt(&self, stmt: &Stmt) -> InterpreterResult {
		stmt.accept(self)
	}

	fn visit_block(&self, block: &Block) -> InterpreterResult {
		let mut s = String::new();
		for stmt in block.stmts() {
			let (exit_status, value) = self.visit_stmt(stmt)?;
			if exit_status == ExitStatus::Okay {
				if let InterpreterValue::String(v) = value {
					s.push_str(&v);
				}
			}
			else if exit_status == ExitStatus::Break {
				break;
			}
			// Continue
		}
		Ok((ExitStatus::Okay, InterpreterValue::String(s)))
	}

	fn visit_raw(&self, raw: &str) -> InterpreterResult {
		Ok((
			ExitStatus::Okay,
			InterpreterValue::String(raw.to_string())
		))
	}

	fn visit_print(&self, print: &Expression) -> InterpreterResult {
		todo!(); // TODO
	}

	fn visit_assign(&self, assign: &Assign) -> InterpreterResult {
		todo!(); // TODO
	}

	fn visit_if(&self, block: &If) -> InterpreterResult {
		todo!(); // TODO
	}

	fn visit_while(&self, block: &ConditionalBlock) -> InterpreterResult {
		todo!(); // TODO
	}

	fn visit_foreach(&self, block: &ForEach) -> InterpreterResult {
		todo!(); // TODO
	}

	fn visit_conditional_block(&self, block: &ConditionalBlock) -> InterpreterResult {
		todo!(); // TODO
	}

	fn visit_break(&self) -> InterpreterResult {
		Ok((ExitStatus::Break, InterpreterValue::Void))
	}

	fn visit_continue(&self) -> InterpreterResult {
		Ok((ExitStatus::Continue, InterpreterValue::Void))
	}

	fn visit_expression(&self, expression: &Expression) -> InterpreterResult {
		let expr = expression.accept(self)?;
		Ok((ExitStatus::Okay, InterpreterValue::String(expr.to_string())))
	}
}

/*
	Float(f64),
	Int(i64),
	Str(String),
	Bool(bool),
	Null
 * */
impl ExprVisitor<Result<Literal, String>> for Interpreter<'_> {
	fn visit_expression(&self, expression: &Expression) -> Result<Literal, String> {
		expression.accept(self)
	}

	fn visit_literal(&self, literal: &Literal) -> Result<Literal, String> {
		Ok(literal.clone())
	}

	fn visit_variable(&self, literal: &Variable) -> Result<Literal, String> {
		todo!(); // TODO
	}

	fn visit_grouping(&self, grouping: &Grouping) -> Result<Literal, String> {
		grouping.expression.accept(self)
	}

	fn visit_unary(&self, unary: &Unary) -> Result<Literal, String> {
		let right = unary.right.accept(self)?;
		match unary.operator {
			Token::Minus => match right {
				Literal::Int(i) => return Ok(Literal::Int(-i)),
				Literal::Float(f) => return Ok(Literal::Float(-f)),
				_ => ()
			},
			Token::Plus => match right {
				Literal::Int(_) | Literal::Float(_) => return Ok(right),
				_ => ()
			},
			Token::Not => return Ok(Literal::Bool(!right.as_bool())),
			_ => ()
		};
		Err(format!("Unsupported unary operation: {}{}", unary.operator, right))
	}

	fn visit_binary(&self, binary: &Binary) -> Result<Literal, String> {
		let left = binary.left.accept(self)?;
		let right = binary.right.accept(self)?;
		let err = format!("Unsupported binary operation: {} {} {}", left, binary.operator, right);
		if left.is_numeric() && right.is_numeric() {
			match binary.operator {
				Token::And => return Ok(Literal::Int(left.as_int()? & right.as_int()?)),
				Token::Or  => return Ok(Literal::Int(left.as_int()? | right.as_int()?)),
				_ => ()
			};
			if left.is_float() || right.is_float() {
				match binary.operator {
					Token::Plus => return Ok(Literal::Float(left.as_float()? + right.as_float()?)),
					Token::Minus => return Ok(Literal::Float(left.as_float()? - right.as_float()?)),
					Token::Multiply => return Ok(Literal::Float(left.as_float()? * right.as_float()?)),
					Token::Divide => return Ok(Literal::Float(left.as_float()? / right.as_float()?)),
					Token::Modulo => return Ok(Literal::Float(left.as_float()? % right.as_float()?)),
					Token::GreaterThan => return Ok(Literal::Bool(left.as_float()? > right.as_float()?)),
					Token::GreaterEqual => return Ok(Literal::Bool(left.as_float()? >= right.as_float()?)),
					Token::LessThan => return Ok(Literal::Bool(left.as_float()? < right.as_float()?)),
					Token::LessEqual => return Ok(Literal::Bool(left.as_float()? <= right.as_float()?)),
					Token::Equal => return Ok(Literal::Bool(left.as_float()? == right.as_float()?)),
					Token::NotEqual => return Ok(Literal::Bool(left.as_float()? != right.as_float()?)),
					_ => ()
				};
			}
			else {
				match binary.operator {
					Token::Plus => return Ok(Literal::Int(left.as_int()? + right.as_int()?)),
					Token::Minus => return Ok(Literal::Int(left.as_int()? - right.as_int()?)),
					Token::Multiply => return Ok(Literal::Int(left.as_int()? * right.as_int()?)),
					Token::Divide => return Ok(Literal::Int(left.as_int()? / right.as_int()?)),
					Token::Modulo => return Ok(Literal::Int(left.as_int()? % right.as_int()?)),
					Token::GreaterThan => return Ok(Literal::Bool(left.as_int()? > right.as_int()?)),
					Token::GreaterEqual => return Ok(Literal::Bool(left.as_int()? >= right.as_int()?)),
					Token::LessThan => return Ok(Literal::Bool(left.as_int()? < right.as_int()?)),
					Token::LessEqual => return Ok(Literal::Bool(left.as_int()? <= right.as_int()?)),
					Token::Equal => return Ok(Literal::Bool(left.as_int()? == right.as_int()?)),
					Token::NotEqual => return Ok(Literal::Bool(left.as_int()? != right.as_int()?)),
					_ => ()
				};
			}
		}
		else if (left.is_bool() || right.is_bool()) && !(left.is_str() || right.is_str()) {
			return Ok(Literal::Bool(match binary.operator {
				Token::Equal=> left.as_bool() == right.as_bool(),
				Token::NotEqual => left.as_bool() != right.as_bool(),
				Token::And | Token::Minus => left.as_bool() && right.as_bool(),
				Token::Or | Token::Plus => left.as_bool() || right.as_bool(),
				_ => Err(err)?
			}));
		}
		else if left.is_str() || right.is_str() {
			return Ok(Literal::Str(match binary.operator {
				Token::Plus => left.as_str() + &right.as_str(),
				_ => Err(err)?
			}));
		}
		Err(err)
	}
}
