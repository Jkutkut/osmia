#[allow(dead_code)]
pub trait StmtVisitable {
	fn accept<S, E>(&self, visitor: &dyn Visitor<S, E>) -> S;
}

#[allow(dead_code)]
pub trait ExprVisitable {
	fn accept<S, E>(&self, visitor: &dyn Visitor<S, E>) -> E;
}

#[allow(dead_code)]
pub trait Visitor<S, E> {
	fn visit_stmt(&mut self, stmt: &String) -> S;
	fn visit_expr(&mut self, expr: &String) -> E;
}
