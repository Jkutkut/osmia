use super::Visitor;

pub trait StmtVisitable {
	fn accept<S, E>(&self, visitor: &dyn Visitor<S, E>) -> S;
}
