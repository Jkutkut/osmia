use std::cell::RefCell;

use crate::types::*;
use super::{
	Interpreter,
	ExitStatus,
};
use crate::utils::{
	Affirm,
	string_or_none,
	push_op_string,
};
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
		match (&code).accept(self)? {
			(ExitStatus::Okay, r) => Ok(r.unwrap_or("".into())),
			(ExitStatus::Break, _) | (ExitStatus::Continue, _) => Err("Cannot break or continue out of the program".into()),
		}
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

type StmtOutput = (ExitStatus, Option<OsmiaOutput>);
type StmtResult = Result<StmtOutput, OsmiaError>;
type ExprResult = Result<Expr, OsmiaError>;

impl Visitor<StmtResult, ExprResult> for OsmiaInterpreter<'_> {
	fn visit_stmt(&self, stmt: &Stmt) -> StmtResult {
		match stmt {
			Stmt::Raw(s) => Ok((ExitStatus::Okay, Some(s.clone()))),
			Stmt::Block(b) => self.visit_block(b),
			Stmt::Expr(e) => Ok((ExitStatus::Okay, Some(e.accept(self)?.to_string()))),
			Stmt::Comment(_) => Ok((ExitStatus::Okay, None)),
			Stmt::Assign(a) => self.visit_assign(a),
			Stmt::If(i) => self.visit_if(i),
			Stmt::While(w) => self.visit_while(w),
			Stmt::For(f) => self.visit_for(f),
			Stmt::Break => Ok((ExitStatus::Break, None)),
			Stmt::Continue => Ok((ExitStatus::Continue, None)),
			s => unimplemented!("Interpreter for statement: {:?}", s), // TODO
		}
	}

	fn visit_expr(&self, expr: &Expr) -> ExprResult {
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
	fn visit_block(&self, block: &Block) -> StmtResult {
		let mut state = ExitStatus::Okay;
		let mut content = String::new();
		for s in block.stmts() {
			let (status, r) = s.accept(self)?;
			push_op_string(&mut content, r);
			state = status.clone();
			match status {
				ExitStatus::Okay => (),
				ExitStatus::Break | ExitStatus::Continue => break,
			}
		}
		Ok((state, string_or_none(content)))
	}

	fn visit_if(&self, if_stmt: &If) -> StmtResult {
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
		Ok((ExitStatus::Okay, None))
	}

	fn visit_while(&self, while_stmt: &While) -> StmtResult {
		let mut content = String::new();
		while let Some((status, r)) = self.visit_conditional(while_stmt)? {
			push_op_string(&mut content, r);
			match status {
				ExitStatus::Okay | ExitStatus::Continue => (),
				ExitStatus::Break => break,
			}
		}
		Ok((ExitStatus::Okay, string_or_none(content)))
	}

	fn visit_conditional(&self, conditional: &ConditionalStmt) -> Result<Option<StmtOutput>, OsmiaError> {
		match conditional.condition().accept(self)?.to_bool() {
			false => Ok(None),
			true => Ok(Some(conditional.body().accept(self)?)),
		}
	}

	fn visit_for(&self, for_stmt: &For) -> StmtResult {
		let var = Self::variable_to_ctx_variable(for_stmt.variable())?;
		let iterable = self.visit_iterable(for_stmt.iterable())?;
		let body = for_stmt.body();
		let mut content = String::new();
		for e in iterable {
			self.set_variable(&mut var.iter(), (&e).try_into()?)?;
			let (status, r) = body.accept(self)?;
			push_op_string(&mut content, r);
			match status {
				ExitStatus::Okay | ExitStatus::Continue => (),
				ExitStatus::Break => break,
			}
		}
		Ok((ExitStatus::Okay, string_or_none(content)))
	}

	fn visit_assign(&self, assign: &Assign) -> StmtResult {
		let var = Self::variable_to_ctx_variable(assign.variable())?;
		let value: Expr = assign.value().accept(self)?;
		let value = (&value).try_into()?;
		self.set_variable(&mut var.iter(), value)?;
		Ok((ExitStatus::Okay, None))
	}
}

impl OsmiaInterpreter<'_> {
	fn visit_binary(&self, binary: &Binary) -> ExprResult {
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

	fn visit_grouping(&self, grouping: &Grouping) -> ExprResult {
		grouping.expr().accept(self)
	}

	fn visit_unary(&self, unary: &Unary) -> ExprResult {
		match unary.operator() {
			UnaryOp::Plus => Ok(unary.expr().accept(self)?.affirm()?),
			UnaryOp::Minus => Ok((-unary.expr().accept(self)?)?),
			UnaryOp::Not => Ok(!unary.expr().accept(self)?),
		}
	}

	fn visit_array(&self, arr: &Array) -> ExprResult {
		let mut new_arr = Vec::new();
		for e in arr.iter() {
			new_arr.push(e.accept(self)?);
		}
		Ok(Expr::Array(new_arr.into()))
	}

	fn visit_object(&self, obj: &Object) -> ExprResult {
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

	fn visit_iterable(&self, iterable: &Expr) -> Result<Vec<Expr>, OsmiaError> {
		let iterable: Expr = match iterable {
			Expr::Array(arr) => self.visit_array(arr)?,
			Expr::Object(obj) => self.visit_object(obj)?,
			Expr::Variable(v) => match self.visit_variable(v)? {
				Expr::Array(a) => Expr::Array(a),
				Expr::Object(o) => Expr::Object(o),
				_ => return Err(format!("Variable {:?} is not iterable", v)),
			},
			_ => return Err(format!("Cannot iterate over: {:?}", iterable)),
		};
		Ok(match iterable {
			Expr::Array(arr) => (&arr).into(),
			Expr::Object(obj) => obj.entries().iter().map(|(k, v)| {
				Ok(Expr::Object(Object::new_hash(vec![
					(Expr::Str("key".into()), k.clone()),
					(Expr::Str("value".into()), v.clone()),
				])?))
			}).collect::<Result<Vec<Expr>, OsmiaError>>()?,
			_ => unreachable!(),
		})
	}

	fn visit_variable(&self, variable: &Variable) -> ExprResult {
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
	fn get_variable<'a>(&self, variable: &mut impl Iterator<Item = &'a JsonTreeKey<String>>) -> ExprResult {
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
