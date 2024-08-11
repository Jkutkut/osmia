use std::collections::HashMap;
use std::convert::TryFrom;
use crate::lexer::Token;
use crate::syntax_tree::{
	Visitor, StmtVisitor, ExprVisitor, StmtVisitable, ExprVisitable
};
use crate::model::{
	Expression, Literal, Binary, Unary, Grouping, Variable,
	JsonExpression, ListOrVariable, JsonTree,
	Stmt, ConditionalBlock, Block, Assign, If, ForEach,
	Ctx,
	ExitStatus, InterpreterValue, InterpreterResult,
	Call, MethodCall
};
use crate::tree_walker::SyntaxTreePrinter;

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

	fn visit_print(&self, content: &JsonExpression) -> InterpreterResult {
		let expr = content.accept(self)?;
		println!("{}", expr);
		Ok((ExitStatus::False, InterpreterValue::Void))
	}

	fn visit_assign(&mut self, assign: &Assign) -> InterpreterResult {
		let value = self.eval_json(assign.expression())?;
		let tree = JsonTree::try_from(&value)?;
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
		for item in self.get_iterable(for_block.list())? {
			let item = self.eval_json(&item)?;
			let item = JsonTree::try_from(&item)?;
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

	fn visit_expression(&self, expression: &JsonExpression) -> InterpreterResult {
		Ok((
			ExitStatus::Okay,
			InterpreterValue::from(
				expression.accept(self)?.to_string()
			)
		))
	}
}

impl ExprVisitor<Result<Literal, String>> for Interpreter {
	// Json
	fn visit_array(&self, arr: &Vec<JsonExpression>) -> Result<Literal, String> {
		Ok(Literal::Str(SyntaxTreePrinter::new().visit_array(arr)))
	}

	fn visit_object(&self, obj: &HashMap<String, JsonExpression>) -> Result<Literal, String> {
		Ok(Literal::Str(SyntaxTreePrinter::new().visit_object(obj)))
	}

	// Callable
	fn visit_call(&self, call: &Call) -> Result<Literal, String> {
		todo!() // TODO
	}

	fn visit_method_call(&self, method_call: &MethodCall) -> Result<Literal, String> {
		todo!() // TODO
	}

	// Expression
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
			Token::Minus => Ok((-right)?),
			Token::Plus => Ok(right),
			Token::Not => Ok((!right)?),
			_ => Err(format!("Unsupported unary operation: {}{}", unary.operator, right))
		}
	}

	fn visit_binary(&self, binary: &Binary) -> Result<Literal, String> {
		let left = binary.left.accept(self)?;
		let right = binary.right.accept(self)?;
		match binary.operator {
			Token::Plus => Ok((left + right)?),
			Token::Minus => Ok((left - right)?),
			Token::Multiply => Ok((left * right)?),
			Token::Divide => Ok((left / right)?),
			Token::Modulo => Ok((left % right)?),
			Token::And => Ok(Literal::Bool(left.as_bool() && right.as_bool())),
			Token::Or => Ok(Literal::Bool(left.as_bool() || right.as_bool())),
			Token::BitAnd => Ok((left & right)?),
			Token::BitOr => Ok((left | right)?),
			Token::BitXor => Ok((left ^ right)?),
			Token::BitShiftLeft => Ok((left << right)?),
			Token::BitShiftRight => Ok((left >> right)?),
			Token::Equal => Ok(Literal::Bool(left == right)),
			Token::NotEqual => Ok(Literal::Bool(left != right)),
			Token::GreaterThan => Ok(Literal::Bool(left > right)),
			Token::GreaterEqual => Ok(Literal::Bool(left >= right)),
			Token::LessThan => Ok(Literal::Bool(left < right)),
			Token::LessEqual => Ok(Literal::Bool(left <= right)),
			_ => Err(format!("Unsupported binary operation: {} {} {}", left, binary.operator, right))
		}
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

	fn get_iterable(&self, lst_or_var: &ListOrVariable) -> Result<Vec<JsonExpression>, String> {
		let ite = match lst_or_var {
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
		Ok(ite)
	}
}
