use crate::model::stmt::Stmt;
use crate::model::expr::Expr;

pub trait Visitor<S, E> {
	fn visit_stmt(&self, stmt: &Stmt) -> S;
	fn visit_expr(&self, expr: &Expr) -> E;
}
