use super::{
	StmtVisitable,
	ExprVisitable,
	Visitor
};
use crate::model::stmt::Stmt;
use crate::model::expr::Expr;


impl ExprVisitable for &Expr {
	fn accept<S, E>(&self, visitor: &dyn Visitor<S, E>) -> E {
		visitor.visit_expr(self)
	}
}

impl StmtVisitable for &Stmt {
	fn accept<S, E>(&self, visitor: &dyn Visitor<S, E>) -> S {
		visitor.visit_stmt(self)
	}
}
