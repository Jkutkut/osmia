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
		todo!(); // TODO
	}

	fn visit_binary(&self, binary: &Binary) -> Result<Literal, String> {
		let left = binary.left.accept(self)?;
		let right = binary.right.accept(self)?;

		// TODO int-int, int-float, float-int, float-float
		if let (Literal::Int(l), Literal::Int(r)) = (&left, &right) {
			Ok(match binary.operator {
				Token::Plus => Literal::Int(l + r),
				Token::Minus => Literal::Int(l - r),
				Token::Multiply => Literal::Int(l * r),
				Token::Divide => Literal::Int(l / r),
				Token::Modulo => Literal::Int(l % r),
				Token::GreaterThan => Literal::Bool(l > r),
				Token::GreaterEqual => Literal::Bool(l >= r),
				Token::LessThan => Literal::Bool(l < r),
				Token::LessEqual => Literal::Bool(l <= r),
				Token::Equal => Literal::Bool(l == r),
				Token::NotEqual => Literal::Bool(l != r),
				Token::And => Literal::Int(l & r),
				Token::Or => Literal::Int(l | r),
				_ => return Err(format!("Unsupported operator: {} {} {}", l, binary.operator, r))
			})
		}
		else if let Literal::Str(l) = &left {
			let r = right.to_string();
			Ok(Literal::Str(match binary.operator {
				Token::Plus => l.to_owned() + &r,
				_ => return Err(format!("Unsupported operation: {} {} {}", l, binary.operator, r))
			}))
		}
		else if let Literal::Str(r) = &right {
			let l = left.to_string();
			Ok(Literal::Str(match binary.operator {
				Token::Plus => l.to_owned() + &r,
				_ => return Err(format!("Unsupported operation: {} {} {}", l, binary.operator, r))
			}))
		}
		else if let (Literal::Bool(l), Literal::Bool(r)) = (left, right) {
			Ok(Literal::Bool(match binary.operator {
				Token::Equal=> l == r,
				Token::NotEqual => l != r,
				Token::And => l && r,
				Token::Or => l || r,
				_ => return Err(format!("Unsupported operation: {} {} {}", l, binary.operator, r))
			}))
		}
		else {
			Err(format!("Unsupported operation: {} {} {}", binary.left, binary.operator, binary.right))
		}
	}
}
