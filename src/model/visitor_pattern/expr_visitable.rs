use super::Visitor;

pub trait ExprVisitable {
	fn accept<S, E>(&self, visitor: &dyn Visitor<S, E>) -> E;
}
