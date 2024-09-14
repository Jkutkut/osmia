use std::cell::RefCell;

use crate::types::*;
use super::Interpreter;

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
	#[allow(unused_variables)]
	fn interpret(&self, code: ParserCode) -> Result<OsmiaOutput, OsmiaError> {
		(&code).accept(self)
	}
}

use crate::model::visitor_pattern::{
	Visitor,
	ExprVisitable,
	StmtVisitable
};
use crate::model::stmt::*;
use crate::model::expr::*;

impl Visitor<Result<OsmiaOutput, OsmiaError>, Result<Expr, OsmiaError>> for OsmiaInterpreter<'_> {
	fn visit_stmt(&self, stmt: &Stmt) -> Result<OsmiaOutput, OsmiaError> {
		println!("{:?}", stmt);
		todo!()
	}

	fn visit_expr(&self, expr: &Expr) -> Result<Expr, OsmiaError> {
		expr.accept(self)
	}
}
