use crate::syntax_tree::StmtVisitor;

pub trait StmtVisitable<T> {
	fn accept(&self, visitor: &dyn StmtVisitor<T>) -> T;
}
