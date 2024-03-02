use crate::Token;
use crate::syntax_tree::{
	StmtVisitor, ExprVisitor, StmtVisitable, ExprVisitable
};
use crate::model::{
	Expression, Literal, Binary, Unary, Grouping, Variable, JsonExpression,
	Stmt, ConditionalBlock, Block, Assign, If, ForEach,
	Ctx
};

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
	Continue,
	False
}

pub struct Interpreter<'a> {
	ctx: &'a mut Ctx
}

impl<'a> Interpreter<'a> {
	pub fn new(ctx: &'a mut Ctx) -> Self {
		Self { ctx }
	}

	pub fn run(&mut self, code: &Stmt<'a>) -> Result<String, String> {
		println!("Running interpreter...");
		let (exit_status, value) = self.visit_stmt(code)?;
		match exit_status {
			ExitStatus::Okay | ExitStatus::False => (),
			ExitStatus::Break | ExitStatus::Continue => return Err(
				"Cannot continue or break from the program".to_string()
			)
		}
		match value {
			InterpreterValue::String(s) => Ok(s),
			InterpreterValue::Void => Ok("".to_string())
		}
	}
}

impl StmtVisitor<InterpreterResult> for Interpreter<'_> {
	fn visit_stmt(&mut self, stmt: &Stmt) -> InterpreterResult {
		stmt.accept(self)
	}

	fn visit_block(&mut self, block: &Block) -> InterpreterResult {
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
			// Continue, False -> do nothing
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
		let expr = print.accept(self)?;
		println!("{}", expr);
		Ok((ExitStatus::False, InterpreterValue::Void))
	}

	fn visit_assign(&mut self, assign: &Assign) -> InterpreterResult {
		match assign.expression() {
			JsonExpression::Expression(expr) => {
				let expr = expr.accept(self)?;
				self.ctx.set(assign.variable(), expr)?;
				Ok((ExitStatus::Okay, InterpreterValue::Void))
			},
			_ => todo!() // TODO implement
		}
	}

	fn visit_if(&mut self, block: &If) -> InterpreterResult {
		let (if_status, if_result) = block.if_block().accept(self)?;
		match if_status {
			ExitStatus::False => (),
			_ => return Ok((if_status, if_result))
		}
		if let Some(elseifs) = block.elseifs() {
			for elseif in elseifs {
				let (elseif_status, elseif_result) = elseif.accept(self)?;
				match elseif_status {
					ExitStatus::False => (),
					_ => return Ok((elseif_status, elseif_result))
				}
			}
		}
		if let Some(else_block) = block.else_block() {
			let (else_status, else_result) = else_block.accept(self)?;
			match else_status {
				ExitStatus::False => (),
				_ => return Ok((else_status, else_result))
			}
		}
		Ok((ExitStatus::False, InterpreterValue::Void))
	}

	fn visit_while(&mut self, block: &ConditionalBlock) -> InterpreterResult {
		let mut string = String::new();
		let mut exit_status = ExitStatus::False;
		loop {
			let condition = block.condition().accept(self)?;
			if !condition.as_bool() {
				break;
			}
			let result = block.body().accept(self)?;
			match result.0 {
				ExitStatus::Continue => continue,
				ExitStatus::Break => {
					exit_status = ExitStatus::Break;
					break;
				},
				_ => {
					exit_status = ExitStatus::Okay;
					if let InterpreterValue::String(v) = result.1 {
						string.push_str(&v);
					}
				}
			}
		}
		let result = match string.is_empty() {
			true => InterpreterValue::Void,
			false => InterpreterValue::String(string)
		};
		Ok((exit_status, result))
	}

	fn visit_foreach(&mut self, block: &ForEach) -> InterpreterResult {
		todo!(); // TODO
	}

	fn visit_conditional_block(&mut self, block: &ConditionalBlock) -> InterpreterResult {
		let condition = block.condition().accept(self)?;
		if condition.as_bool() {
			let result = block.body().accept(self)?;
			return match result.0 {
				ExitStatus::Continue | ExitStatus::Break => Err(
					"Cannot continue or break in conditional block".to_string()
				),
				_ => Ok(result)
			}
		}
		Ok((ExitStatus::False, InterpreterValue::Void))
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

impl ExprVisitor<Result<Literal, String>> for Interpreter<'_> {
	fn visit_expression(&self, expression: &Expression) -> Result<Literal, String> {
		expression.accept(self)
	}

	fn visit_literal(&self, literal: &Literal) -> Result<Literal, String> {
		Ok(literal.clone())
	}

	fn visit_variable(&self, variable: &Variable) -> Result<Literal, String> {
		self.ctx.get(variable)
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
					Token::Divide => {
						if right.as_float()? == 0.0 {
							return Err("Division by zero".to_string())
						}
						return Ok(Literal::Float(left.as_float()? / right.as_float()?))
					},
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
					Token::Plus => return Ok(Literal::Int(
						left.as_int()?
						.checked_add(right.as_int()?)
						.ok_or("Overflow in addition".to_string())?
					)),
					Token::Minus => return Ok(Literal::Int(
						left.as_int()?
						.checked_sub(right.as_int()?)
						.ok_or("Overflow in subtraction".to_string())?
					)),
					Token::Multiply => return Ok(Literal::Int(
						left.as_int()?
						.checked_mul(right.as_int()?)
						.ok_or("Overflow in multiplication".to_string())?
					)),
					Token::Divide => {
						if right.as_int()? == 0 {
							return Err("Division by zero".to_string())
						}
						return Ok(Literal::Int(left.as_int()? / right.as_int()?))
					}
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
