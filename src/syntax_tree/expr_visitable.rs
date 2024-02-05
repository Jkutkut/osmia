use crate::syntax_tree::ExprVisitor;

pub trait ExprVisitable<T> {
	fn accept(&self, visitor: &dyn ExprVisitor<T>) -> T;
}
