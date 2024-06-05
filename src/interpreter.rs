use std::collections::HashMap;
use crate::lexer::Token;
use crate::syntax_tree::{
	StmtVisitor, ExprVisitor, StmtVisitable, ExprVisitable
};
use crate::model::{
	Expression, Literal, Binary, Unary, Grouping, Variable,
	JsonExpression, ListOrVariable, JsonTree,
	Stmt, ConditionalBlock, Block, Assign, If, ForEach,
	Ctx,
	ExitStatus, InterpreterValue, InterpreterResult
};

pub struct Interpreter {
	ctx: Ctx
}

impl Interpreter {
	pub fn new(ctx: Ctx) -> Self {
		Self { ctx }
	}

	pub fn run(&mut self, code: &Stmt) -> Result<String, String> {
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

impl StmtVisitor<InterpreterResult> for Interpreter {
	fn visit_stmt(&mut self, stmt: &Stmt) -> InterpreterResult {
		stmt.accept(self)
	}

	fn visit_block(&mut self, block: &Block) -> InterpreterResult {
		let mut exit_status = ExitStatus::Okay;
		let mut s = String::new();
		for stmt in block.stmts() {
			let (block_exit_status, r) = self.visit_stmt(stmt)?;
			Self::add2result(&mut s, r);
			match block_exit_status {
				ExitStatus::Continue | ExitStatus::Break => {
					exit_status = block_exit_status;
					break;
				}
				_ => ()
			}
		}
		Ok((exit_status, InterpreterValue::from(s)))
	}

	fn visit_raw(&self, raw: &str) -> InterpreterResult {
		Ok((
			ExitStatus::Okay,
			InterpreterValue::from(raw.to_string())
		))
	}

	fn visit_print(&self, print: &Expression) -> InterpreterResult {
		let expr = print.accept(self)?;
		println!("{}", expr);
		Ok((ExitStatus::False, InterpreterValue::Void))
	}

	fn visit_assign(&mut self, assign: &Assign) -> InterpreterResult {
		let value = self.eval_json(assign.expression())?;
		let tree = JsonTree::from(&value)?;
		self.ctx.set(assign.variable(), tree)?;
		Ok((ExitStatus::Okay, InterpreterValue::Void))
	}

	fn visit_if(&mut self, block: &If) -> InterpreterResult {
		match block.if_block().accept(self)? {
			(ExitStatus::False, _) => (),
			r => return Ok(r)
		}
		if let Some(elseifs) = block.elseifs() {
			for elseif in elseifs {
				match elseif.accept(self)? {
					(ExitStatus::False, _) => (),
					r => return Ok(r)
				}
			}
		}
		if let Some(else_block) = block.else_block() {
			match else_block.accept(self)? {
				(ExitStatus::False, _) => (),
				r => return Ok(r)
			}
		}
		Ok((ExitStatus::False, InterpreterValue::Void))
	}

	fn visit_while(&mut self, block: &ConditionalBlock) -> InterpreterResult {
		let mut s = String::new();
		let mut exit_status = ExitStatus::False;
		while block.condition().accept(self)?.as_bool() {
			exit_status = ExitStatus::Okay;
			let (block_exit_status, r) = block.body().accept(self)?;
			Self::add2result(&mut s, r);
			match block_exit_status {
				ExitStatus::Break => break,
				_ => ()
			}
		}
		Ok((exit_status, InterpreterValue::from(s)))
	}

	fn visit_foreach(&mut self, for_block: &ForEach) -> InterpreterResult {
		let mut s = String::new();
		let mut exit_status = ExitStatus::False;
		let iterable_obj = match for_block.list() { // TODO use refs
			ListOrVariable::List(json) => match json {
				JsonExpression::Array(arr) => arr.clone(),
				JsonExpression::Object(_) => return Err("Cannot iterate over object".to_string()),
				JsonExpression::Expression(_) => return Err("Cannot iterate over expression".to_string())
			},
			ListOrVariable::Variable(var) => {
				let json_tree = self.ctx.get(var)?;
				let json_expr = JsonExpression::from(json_tree);
				match json_expr {
					JsonExpression::Array(arr) => arr.clone(),
					JsonExpression::Object(_) => return Err("Cannot iterate over object".to_string()),
					JsonExpression::Expression(_) => return Err("Cannot iterate over expression".to_string())
				}
			}
		};
		let mut iterable = iterable_obj.iter();
		while let Some(item) = iterable.next() {
			let item = self.eval_json(item)?;
			let item = JsonTree::from(&item)?;
			self.ctx.set(for_block.variable(), item)?;
			exit_status = ExitStatus::Okay;
			let (block_exit_status, r) = for_block.body().accept(self)?;
			Self::add2result(&mut s, r);
			match block_exit_status {
				ExitStatus::Break => break,
				_ => ()
			}
		}
		Ok((exit_status, InterpreterValue::from(s)))
	}

	fn visit_conditional_block(&mut self, block: &ConditionalBlock) -> InterpreterResult {
		if block.condition().accept(self)?.as_bool() {
			return block.body().accept(self);
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
		Ok((
			ExitStatus::Okay,
			InterpreterValue::from(
				expression.accept(self)?.to_string()
			)
		))
	}
}

impl ExprVisitor<Result<Literal, String>> for Interpreter {
	fn visit_expression(&self, expression: &Expression) -> Result<Literal, String> {
		expression.accept(self)
	}

	fn visit_literal(&self, literal: &Literal) -> Result<Literal, String> {
		Ok(literal.clone())
	}

	fn visit_variable(&self, variable: &Variable) -> Result<Literal, String> {
		self.ctx.get_literal(variable)
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

impl Interpreter {
	pub fn eval_json(&self, tree: &JsonExpression) -> Result<JsonExpression, String> {
		let new_tree: JsonExpression = match tree {
			JsonExpression::Expression(expr) => JsonExpression::Expression(Expression::Literal(expr.accept(self)?)),
			JsonExpression::Array(arr) => {
				let mut new_arr = Vec::new();
				for item in arr {
					new_arr.push(self.eval_json(item)?);
				}
				JsonExpression::Array(new_arr)
			},
			JsonExpression::Object(obj) => {
				let mut new_obj = HashMap::new();
				for (key, value) in obj {
					new_obj.insert(key.to_string(), self.eval_json(value)?);
				}
				JsonExpression::Object(new_obj)
			}
		};
		Ok(new_tree)
	}

	fn add2result(result: &mut String, value: InterpreterValue) {
		if let InterpreterValue::String(s) = value {
			result.push_str(&s);
		}
	}
}
