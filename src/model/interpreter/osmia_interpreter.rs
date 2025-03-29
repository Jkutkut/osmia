use std::cell::RefCell;

use crate::types::*;
use super::{
	Interpreter,
	ExitStatus,
	callable::*,
	OsmiaResult,
	MethodExpression
};
use crate::utils::{
	Affirm,
};
use crate::constants::METHOD_CTX_LOCATION;
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
			(ExitStatus::Okay, r) => Ok(r.to_string()),
			(ExitStatus::Break, _) | (ExitStatus::Continue, _) => Err("Cannot break or continue out of the program".into()),
			(ExitStatus::Return, r) => Ok(r.to_string()),
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

type StmtOutput = (ExitStatus, OsmiaResult);
type StmtResult = Result<StmtOutput, OsmiaError>;
type ExprResult = Result<Expr, OsmiaError>;

impl Visitor<StmtResult, ExprResult> for OsmiaInterpreter<'_> {
	fn visit_stmt(&self, stmt: &Stmt) -> StmtResult {
		match stmt {
			Stmt::Raw(s) => Ok((ExitStatus::Okay, OsmiaResult::OsmiaOutput(s.into()))),
			Stmt::Block(b) => self.visit_block(b),
			Stmt::Expr(e) => Ok((ExitStatus::Okay, OsmiaResult::Expr(e.accept(self)?))),
			Stmt::Comment(_) => Ok((ExitStatus::Okay, OsmiaResult::None)),
			Stmt::Assign(a) => self.visit_assign(a),
			Stmt::If(i) => self.visit_if(i),
			Stmt::While(w) => self.visit_while(w),
			Stmt::For(f) => self.visit_for(f),
			Stmt::Break => Ok((ExitStatus::Break, OsmiaResult::None)),
			Stmt::Continue => Ok((ExitStatus::Continue, OsmiaResult::None)),
			Stmt::Return(r) => self.visit_return(r),
			Stmt::Function(f) => self.visit_function(f),
			s => unimplemented!("Interpreter for statement: {:?}", s), // TODO
		}
	}

	fn visit_expr(&self, expr: &Expr) -> ExprResult {
		Ok(match expr {
			Expr::Float(_) | Expr::Int(_) | Expr::Str(_) | Expr::Bool(_) | Expr::Null => expr.clone(),
			Expr::Binary(b) => self.visit_binary(b)?,
			Expr::Grouping(g) => self.visit_grouping(g)?,
			Expr::Unary(u) => self.visit_unary(u)?,
			Expr::Array(arr) => self.visit_array(arr)?,
			Expr::Object(obj) => self.visit_object(obj)?,
			Expr::Lambda(l) => self.visit_lambda(l)?,
			Expr::Call(c) => self.visit_call(c)?,
			Expr::MethodCall(m) => self.visit_method_call(m)?,
			Expr::Variable(v) => self.get_variable(v)?,
			Expr::Callable(_) => expr.clone(),
		})
	}
}

impl OsmiaInterpreter<'_> {
	fn visit_block(&self, block: &Block) -> StmtResult {
		let mut state = ExitStatus::Okay;
		let mut content = String::new();
		for s in block.stmts() {
			let (status, r) = s.accept(self)?;
			content += r.to_string().as_str();
			state = status;
			match &state {
				ExitStatus::Okay => (),
				ExitStatus::Break | ExitStatus::Continue => break,
				ExitStatus::Return => match r {
					OsmiaResult::None => break,
					_ => return Ok((state, r)),
				}
			}
		}
		Ok((state, content.into()))
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
		Ok((ExitStatus::Okay, OsmiaResult::None))
	}

	fn visit_while(&self, while_stmt: &While) -> StmtResult {
		let mut content = String::new();
		while let Some((status, r)) = self.visit_conditional(while_stmt)? {
			content.push_str(r.to_string().as_str());
			match status {
				ExitStatus::Okay | ExitStatus::Continue => (),
				ExitStatus::Break => break,
				ExitStatus::Return => match r {
					OsmiaResult::None => break,
					_ => return Ok((status, r)),
				}
			}
		}
		Ok((ExitStatus::Okay, content.into()))
	}

	fn visit_conditional(&self, conditional: &ConditionalStmt) -> Result<Option<StmtOutput>, OsmiaError> {
		match conditional.condition().accept(self)?.to_bool() {
			false => Ok(None),
			true => Ok(Some(conditional.body().accept(self)?)),
		}
	}

	fn visit_for(&self, for_stmt: &For) -> StmtResult {
		let var = Self::var_arr_to_ctx_variable(self.visit_variable(for_stmt.variable())?.vec())?;
		let iterable = self.visit_iterable(for_stmt.iterable())?;
		let body = for_stmt.body();
		let mut content = String::new();
		for e in iterable {
			self.set_variable(&var, (&e).try_into()?)?;
			let (status, r) = body.accept(self)?;
			content += r.to_string().as_str();
			match status {
				ExitStatus::Okay | ExitStatus::Continue => (),
				ExitStatus::Break => break,
				ExitStatus::Return => match r {
					OsmiaResult::None => break,
					_ => return Ok((status, r)),
				}
			}
		}
		Ok((ExitStatus::Okay, content.into()))
	}

	fn visit_assign(&self, assign: &Assign) -> StmtResult {
		let var = Self::var_arr_to_ctx_variable(self.visit_variable(assign.variable())?.vec())?;
		let value: Expr = assign.value().accept(self)?;
		let value = (&value).try_into()?;
		self.set_variable(&var, value)?;
		Ok((ExitStatus::Okay, OsmiaResult::None))
	}

	fn visit_return(&self, r: &Return) -> StmtResult {
		let value = match r.expr() {
			Some(expr) => expr.accept(self)?.into(),
			None => OsmiaResult::None,
		};
		Ok((ExitStatus::Return, value))
	}

	fn visit_function(&self, ft: &Function) -> StmtResult {
		let name = ft.name().clone();
		let params = self.visit_function_params(ft.params())?;
		let ft = Function::new(ft.name().clone(), params, ft.body().clone());
		let callable = Callable::Function(FunctionCallable::new(ft));
		self.set_variable(
			&vec![name],
			(&Expr::Callable(callable)).try_into()?,
		)?;
		Ok((ExitStatus::Okay, OsmiaResult::None))
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
			Object::Hash(h) => Ok(self.visit_object(&Object::new_code(h.entries()))?),
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

	fn visit_method_call(&self, m: &MethodCall) -> ExprResult {
		let obj = m.obj.as_ref().accept(self)?;
		let var_type = MethodExpression::try_from(&obj).unwrap_or_else(|_| unreachable!());
		let var = match m.call.callee() {
			Expr::Variable(v) => v.vec().get(0).unwrap(),
			_ => unreachable!()
		};
		let call_path = vec![
			JsonTreeKeyExpr::JsonTreeKey(JsonTreeKey::Key(METHOD_CTX_LOCATION.into())),
			JsonTreeKeyExpr::JsonTreeKey(JsonTreeKey::Key((&var_type).into())),
			var.clone()
		];
		let mut args = Vec::with_capacity(m.call.args().len() + 1);
		args.push(obj.clone());
		args.extend_from_slice(m.call.args());
		let call_expr: Expr = Variable::from_vec(call_path).into();
		let call = Call::new(call_expr, args);
		self.visit_call(&call)
	}

	fn make_call(&self, call: &Callable, args: &Vec<Expr>) -> ExprResult {
		let args = self.setup_callable_args(args, call)?;
		let expr: Expr = match call {
			Callable::Builtin(_) | Callable::Lambda(_) => call.call(&mut self.ctx.borrow_mut(), &args)?,
			Callable::Function(_) => {
				self.ctx.borrow_mut().begin_scope();
				let ft_body = call.call_stmt(&mut self.ctx.borrow_mut(), &args)?;
				let (status, r) = self.visit_stmt(&ft_body)?;
				let result = match status {
					ExitStatus::Continue | ExitStatus::Break => return Err(format!(
						"Cannot break or continue out of a function"
					)),
					ExitStatus::Okay | ExitStatus::Return => match r {
						OsmiaResult::None => Expr::Null,
						OsmiaResult::Expr(e) => e,
						OsmiaResult::OsmiaOutput(s) => Expr::Str(s),
					}
				};
				self.ctx.borrow_mut().end_scope();
				result
			}
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
			Callable::Lambda(l) => self.setup_callable_args_with_params(
				arguments, args, l.params()
			),
			Callable::Function(f) => self.setup_callable_args_with_params(
				arguments, args, f.params()
			),
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

	fn visit_variable(&self, variable: &Variable) -> Result<Variable, OsmiaError> {
		let mut arr: Vec<JsonTreeKeyExpr> = Vec::with_capacity(variable.vec().len());
		for v in variable.vec() {
			match v {
				JsonTreeKeyExpr::Expr(e) => arr.push(self.visit_expr(e)?.into()),
				k => arr.push(k.clone()),
			}
		}
		Ok(Variable::from_vec(arr))
	}

	fn var_arr_to_ctx_variable(
		var: &Vec<JsonTreeKeyExpr>
	) -> Result<Vec<JsonTreeKey<String>>, OsmiaError> {
		let mut variable_keys: Vec<JsonTreeKey<String>> = Vec::with_capacity(var.len());
		for v in var {
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
		let variable = self.visit_variable(variable)?;
		match variable.vec().get(0) {
			Some(JsonTreeKeyExpr::JsonTreeKey(_)) => self.get_variable_from_ctx(&self.ctx.borrow(), &variable.vec()),
			Some(JsonTreeKeyExpr::Expr(e)) => {
				let context = Ctx::from(e.try_into()?);
				let keys: Vec<JsonTreeKeyExpr> = variable.vec().iter().skip(1).map(|k| k.clone()).collect();
				self.get_variable_from_ctx(&context, &keys)
			},
			None => unreachable!()
		}
	}

	fn get_variable_from_ctx(&self, ctx: &Ctx, variable: &Vec<JsonTreeKeyExpr>) -> ExprResult {
		let keys = Self::var_arr_to_ctx_variable(&variable)?;
		Ok(ctx.get(&keys)?.try_into()?)
	}

	fn set_variable<'a>(
		&self,
		var: &Vec<JsonTreeKey<String>>,
		value: JsonTree<String, CtxValue>
	) -> Result<(), OsmiaError> {
		self.ctx.borrow_mut().set(var, value)
	}
}
