use std::cell::RefCell;

use crate::types::*;
use super::Interpreter;
use crate::utils::Affirm;
use crate::ctx::{
	JsonTree,
	JsonTreeKey,
	JsonTreeError,
	CtxValue,
};

pub struct OsmiaInterpreter<'ctx> {
	#[allow(dead_code)]
	ctx: RefCell<&'ctx mut Ctx>,
}


impl<'ctx> OsmiaInterpreter<'ctx> {
	pub fn new(ctx: &'ctx mut Ctx) -> Self {
		Self {
			ctx: RefCell::new(ctx),
		}
	}
}

impl Interpreter<ParserCode, OsmiaOutput, OsmiaError> for OsmiaInterpreter<'_> {
	fn interpret(&self, code: ParserCode) -> Result<OsmiaOutput, OsmiaError> {
		(&code).accept(self)
	}
}

use crate::model::visitor_pattern::{
	Visitor,
	ExprVisitable,
	StmtVisitable
};
use crate::model::{
	stmt::*,
	expr::*,
};

impl Visitor<Result<OsmiaOutput, OsmiaError>, Result<Expr, OsmiaError>> for OsmiaInterpreter<'_> {
	fn visit_stmt(&self, stmt: &Stmt) -> Result<OsmiaOutput, OsmiaError> {
		match stmt {
			Stmt::Raw(s) => Ok(s.clone()),
			Stmt::Block(b) => self.visit_block(b),
			Stmt::Expr(e) => Ok(e.accept(self)?.to_string()),
			Stmt::Comment(_) => Ok("".to_string()),
			Stmt::Assign(a) => self.visit_assign(a),
			Stmt::If(i) => self.visit_if(i),
			s => unimplemented!("Interpreter for statement: {:?}", s), // TODO
		}
	}

	fn visit_expr(&self, expr: &Expr) -> Result<Expr, OsmiaError> {
		match expr {
			Expr::Float(_) | Expr::Int(_) | Expr::Str(_) | Expr::Bool(_) | Expr::Null => Ok(expr.clone()),
			Expr::Binary(b) => Ok(self.visit_binary(b)?),
			Expr::Grouping(g) => Ok(self.visit_grouping(g)?),
			Expr::Unary(u) => Ok(self.visit_unary(u)?),
			Expr::Array(arr) => Ok(self.visit_array(arr)?),
			Expr::Object(obj) => Ok(self.visit_object(obj)?),
			Expr::Variable(v) => Ok(self.visit_variable(v)?),
			_ => unimplemented!("Interpreter for expr: {:?}", expr), // TODO
		}
	}
}

impl OsmiaInterpreter<'_> {
	fn visit_block(&self, block: &Block) -> Result<OsmiaOutput, OsmiaError> {
		// TODO this will change with flow breaking statements
		let stmts = &block.stmts;
		Ok(stmts.into_iter()
			.map(|s| self.visit_stmt(&s))
			.collect::<Result<Vec<String>, OsmiaError>>()?
			.join("")
		)
	}

	fn visit_if(&self, if_stmt: &If) -> Result<OsmiaOutput, OsmiaError> {
		if let Some(content) = self.visit_conditional(if_stmt.conditional())? {
			return Ok(content);
		}
		if let Some(else_ifs) = if_stmt.elseifs() {
			for e in else_ifs {
				if let Some(content) = self.visit_conditional(e)? {
					return Ok(content);
				}
			}
		}
		if let Some(e) = if_stmt.else_block() {
			let e: &Stmt = &*e;
			return Ok(e.accept(self)?);
		}
		Ok("".to_string())
	}

	fn visit_while(&self, while_stmt: &While) -> Result<OsmiaOutput, OsmiaError> {
		let mut content = String::new();
		while let Some(c) = self.visit_conditional(while_stmt)? {
			content.push_str(&c);
		}
		Ok(content)
	}

	fn visit_conditional(&self, conditional: &ConditionalStmt) -> Result<Option<OsmiaOutput>, OsmiaError> {
		match conditional.condition().accept(self)?.to_bool() {
			false => Ok(None),
			true => Ok(Some(conditional.body().accept(self)?)),
		}
	}

	fn visit_assign(&self, assign: &Assign) -> Result<OsmiaOutput, OsmiaError> {
		let var = Self::variable_to_ctx_variable(assign.variable())?;
		let value: Expr = assign.value().accept(self)?;
		let value = (&value).try_into()?;
		self.set_variable(&mut var.iter(), value)?;
		Ok("".to_string())
	}
}

impl OsmiaInterpreter<'_> {
	fn visit_binary(&self, binary: &Binary) -> Result<Expr, OsmiaError> {
		let left = binary.left().accept(self)?;
		let right = binary.right().accept(self)?;
		match binary.operator() {
			BinaryOp::Plus => left + right,
			BinaryOp::Minus => left - right,
			BinaryOp::Mult => left * right,
			BinaryOp::Div => left / right,
			BinaryOp::Mod => left % right,
			BinaryOp::Equal => Ok(Expr::Bool(left == right)),
			BinaryOp::NotEqual => Ok(Expr::Bool(left != right)),
			BinaryOp::Greater => Ok(Expr::Bool(left > right)),
			BinaryOp::GreaterEqual => Ok(Expr::Bool(left >= right)),
			BinaryOp::Less => Ok(Expr::Bool(left < right)),
			BinaryOp::LessEqual => Ok(Expr::Bool(left <= right)),
			BinaryOp::BitAnd => left & right,
			BinaryOp::BitOr => left | right,
			BinaryOp::BitXor => left ^ right,
			BinaryOp::BitShiftLeft => left << right,
			BinaryOp::BitShiftRight => left >> right,
			BinaryOp::And => Ok(Expr::Bool(left.to_bool() && right.to_bool())),
			BinaryOp::Or => Ok(Expr::Bool(left.to_bool() || right.to_bool())),
		}
	}

	fn visit_grouping(&self, grouping: &Grouping) -> Result<Expr, OsmiaError> {
		grouping.expr().accept(self)
	}

	fn visit_unary(&self, unary: &Unary) -> Result<Expr, OsmiaError> {
		match unary.operator() {
			UnaryOp::Plus => Ok(unary.expr().accept(self)?.affirm()?),
			UnaryOp::Minus => Ok((-unary.expr().accept(self)?)?),
			UnaryOp::Not => Ok(!unary.expr().accept(self)?),
		}
	}

	fn visit_array(&self, arr: &Array) -> Result<Expr, OsmiaError> {
		let mut new_arr = Vec::new();
		for e in arr.iter() {
			new_arr.push(e.accept(self)?);
		}
		Ok(Expr::Array(new_arr.into()))
	}

	fn visit_object(&self, obj: &Object) -> Result<Expr, OsmiaError> {
		match obj {
			Object::Code(_) => {
				let items: Vec<(Expr, Expr)> = obj.into();
				let mut new_obj = Vec::new();
				for (e, v) in items {
					new_obj.push((self.visit_expr(&e)?, self.visit_expr(&v)?));
				}
				Ok(Expr::Object(Object::new_hash(new_obj)?))
			},
			Object::Hash(h) => unreachable!("Interpreter for hash object: {:?}", h),
		}
	}

	fn visit_variable(&self, variable: &Variable) -> Result<Expr, OsmiaError> {
		self.get_variable(&mut Self::variable_to_ctx_variable(variable)?.iter())
	}

	fn variable_to_ctx_variable(variable: &Variable) -> Result<Vec<JsonTreeKey<String>>, OsmiaError> {
		let mut variable_keys: Vec<JsonTreeKey<String>> = Vec::new();
		for v in variable.vec() {
			match v {
				JsonTreeKeyExpr::JsonTreeKey(k) => variable_keys.push(k.clone()),
				JsonTreeKeyExpr::Expr(e) => {
					let key: JsonTreeKey<String> = match e {
						Expr::Str(s) => JsonTreeKey::Key(s.into()),
						Expr::Int(i) => {
							if *i < 0 {
								return Err(format!("Invalid variable index: {:?}", e));
							}
							JsonTreeKey::Index(*i as usize)
						},
						_ => return Err(format!("Invalid variable key: {:?}", e)),
					};
					variable_keys.push(key);
				},
			}
		}
		Ok(variable_keys)
	}
}

impl OsmiaInterpreter<'_> {
	fn get_variable<'a>(&self, variable: &mut impl Iterator<Item = &'a JsonTreeKey<String>>) -> Result<Expr, OsmiaError> {
		match self.ctx.borrow().get(variable) {
			Ok(r) => Ok(r.try_into()?),
			Err(e) => return Err(match e {
				JsonTreeError::AccessValue(k) => format!("Cannot access a value: {}", k),
				JsonTreeError::ArrayOutOfBounds((idx, len)) => format!("Array index out of bounds. Attempted to access index {} in an array of length {}", idx, len),
				JsonTreeError::IndexInObject => format!("Cannot get by index from an object"),
				JsonTreeError::KeyInArray => format!("Cannot get by key from an array"),
				JsonTreeError::KeyNotFound(k) => format!("Variable not found: {}", k),
				JsonTreeError::NoKey => unreachable!(),
			})
		}
	}

	fn set_variable<'a>(&self, var: &mut impl Iterator<Item = &'a JsonTreeKey<String>>, value: JsonTree<String, CtxValue>) -> Result<(), OsmiaError> {
		match self.ctx.borrow_mut().set(var, value) {
			Err(e) => Err(match e {
				JsonTreeError::AccessValue(k) => format!("Cannot access a value: {}", k),
				JsonTreeError::ArrayOutOfBounds((idx, len)) => format!("Array index out of bounds. Attempted to access index {} in an array of length {}", idx, len),
				JsonTreeError::IndexInObject => format!("Cannot set by index from an object"),
				JsonTreeError::KeyInArray => format!("Cannot set by key from an array"),
				JsonTreeError::KeyNotFound(k) => format!("Variable not found: {}", k),
				JsonTreeError::NoKey => unreachable!(),
			}),
			Ok(_) => Ok(()),
		}
	}
}
