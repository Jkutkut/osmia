use std::cell::RefCell;

use crate::types::*;
use super::{
	Interpreter,
	ExitStatus,
	callable::*,
};
use crate::utils::{
	Affirm,
	string_or_none,
	push_op_string,
};
use crate::ctx::{
	JsonTree,
	JsonTreeKey,
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
			Expr::Lambda(l) => Ok(self.visit_lambda(l)?),
			Expr::Call(c) => Ok(self.visit_call(c)?),
			Expr::Variable(v) => Ok(self.get_variable(v)?),
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

	fn visit_lambda(&self, lambda: &Lambda) -> ExprResult {
		let params = self.visit_function_params(lambda.params())?;
		let lambda = Lambda::new(params, lambda.body().clone());
		let callable = Callable::Lambda(LambdaCallable::new(lambda));
		Ok(Expr::Callable(callable))
	}

	fn visit_function_params(&self, params: &Vec<FunctionParam>) -> Result<Vec<FunctionParam>, OsmiaError> {
		let mut new_params = Vec::new();
		for p in params {
			match p {
				FunctionParam::Param(p, default) => {
					let default = match default {
						None => None,
						Some(d) => Some(self.visit_expr(d)?),
					};
					new_params.push(FunctionParam::Param(p.clone(), default));
				},
				FunctionParam::Spread(_) => new_params.push(p.clone()),
			}
		}
		Ok(new_params)
	}

	fn visit_call(&self, call: &Call) -> ExprResult {
		match call.callee().accept(self)? {
			Expr::Callable(c) => self.visit_expr(&self.make_call(&c, call.args())?),
			e => Err(format!("Expression {} is not callable", e)),
		}
	}

	fn make_call(&self, call: &Callable, args: &Vec<Expr>) -> ExprResult {
		let args = self.setup_callable_args(args, call)?;
		let ctx: &mut Ctx = &mut self.ctx.borrow_mut();
		let expr: Expr = match call {
			Callable::Builtin(_) | Callable::Lambda(_) => call.call(ctx, &args)?,
			Callable::Function(_) => todo!(), // TODO
		};
		Ok(expr)
	}

	fn setup_callable_args(&self, args: &Vec<Expr>, call: &Callable) -> Result<Vec<Expr>, OsmiaError> {
		let call_arity = call.arity();
		let mut arguments: Vec<Expr> = Vec::with_capacity(call_arity);
		match call {
			Callable::Builtin(b) => match b.params() {
				Some(params) => self.setup_callable_args_with_params(arguments, args, params),
				None => {
					let mut i = 0;
					while i < call_arity && i < args.len() {
						arguments.push(self.visit_expr(&args[i])?);
						i += 1;
					}
					Ok(arguments)
				},
			},
			Callable::Lambda(l) => {
				let lambda_params = l.params();
				self.setup_callable_args_with_params(arguments, args, lambda_params)
			},
			Callable::Function(_) => todo!(), // TODO
		}
	}

	fn setup_callable_args_with_params(
		&self,
		mut arguments: Vec<Expr>,
		args: &Vec<Expr>,
		ft_params: &Vec<FunctionParam>
	) -> Result<Vec<Expr>, OsmiaError> {
		let mut i = 0;
		loop {
			match (args.get(i), ft_params.get(i)) {
				(_, None) => break,
				(arg, Some(p)) => match (arg, p) {
					(_, FunctionParam::Spread(_)) => {
						arguments.push(Expr::Array(args[i..].to_vec().into()));
						break;
					},
					(Some(arg), _) => arguments.push(self.visit_expr(arg)?),
					(None, _) => match p {
						FunctionParam::Param(p, None) => return Err(format!(
							"Missing argument: {}", p
						)),
						FunctionParam::Param(_, Some(d)) => arguments.push(self.visit_expr(d)?),
						FunctionParam::Spread(_) => unreachable!(),
					}
				}
			}
			i += 1;
		}
		Ok(arguments)
	}

	fn visit_iterable(&self, iterable: &Expr) -> Result<Vec<Expr>, OsmiaError> {
		let iterable: Expr = match iterable {
			Expr::Array(arr) => self.visit_array(arr)?,
			Expr::Object(obj) => self.visit_object(obj)?,
			Expr::Variable(v) => match self.get_variable(v)? {
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
	fn get_variable<'a>(&self, variable: &Variable) -> ExprResult {
		let variable = Self::variable_to_ctx_variable(variable)?;
		Ok(self.ctx.borrow().get(&mut variable.iter())?.try_into()?)
	}

	fn set_variable<'a>(&self, var: &mut impl Iterator<Item = &'a JsonTreeKey<String>>, value: JsonTree<String, CtxValue>) -> Result<(), OsmiaError> {
		self.ctx.borrow_mut().set(var, value)
	}
}
