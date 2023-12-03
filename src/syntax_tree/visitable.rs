use crate::syntax_tree::visitor::Visitor;

pub trait Visitable<T> {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T;
}
