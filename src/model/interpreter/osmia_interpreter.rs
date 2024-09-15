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
		match stmt {
			Stmt::Raw(s) => Ok(s.clone()),
			Stmt::Block(b) => self.visit_block(b),
			Stmt::Expr(e) => Ok(match e.accept(self)? {
				Expr::Float(f) => f.to_string(),
				Expr::Int(i) => i.to_string(),
				Expr::Str(s) => s.to_string(),
				Expr::Bool(b) => b.to_string(),
				Expr::Null => "null".to_string(),
				Expr::Binary(_) | Expr::Grouping(_) | Expr::Unary(_) => unreachable!(),
				Expr::Call(_) | Expr::MethodCall(_) => unreachable!(),
				e => unimplemented!("Interpreter for expr: {:?}", e), // TODO
			}),
			Stmt::Comment(_) => Ok("".to_string()),
			s => unimplemented!("Interpreter for statement: {:?}", s), // TODO
		}
	}

	fn visit_expr(&self, expr: &Expr) -> Result<Expr, OsmiaError> {
		match expr {
			Expr::Float(_) | Expr::Int(_) | Expr::Str(_) | Expr::Bool(_) | Expr::Null => Ok(expr.clone()),
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
}
