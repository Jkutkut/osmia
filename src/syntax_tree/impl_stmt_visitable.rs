use crate::syntax_tree::model::{
	Stmt, Assign, ConditionalBlock, ForEach, If
};
use crate::syntax_tree::{
	StmtVisitable, StmtVisitor
};

impl<T> StmtVisitable<T> for Stmt<'_> {
	fn accept(&self, visitor: &dyn StmtVisitor<T>) -> T {
		match self {
			Stmt::Block(blocks) => visitor.visit_block(blocks),
			Stmt::Raw(raw) => visitor.visit_raw(raw),
			Stmt::Print(print) => visitor.visit_print(print),
			Stmt::Expression(expression) => visitor.visit_expression(expression),
			Stmt::Assign(assign) => assign.accept(visitor),
			Stmt::If(if_stmt) => if_stmt.accept(visitor),
			Stmt::While(while_stmt) => visitor.visit_while(while_stmt),
			Stmt::ForEach(foreach) => foreach.accept(visitor),
			Stmt::Break => visitor.visit_break(),
			Stmt::Continue => visitor.visit_continue(),
		}
	}
}

impl<T> StmtVisitable<T> for Assign<'_> {
	fn accept(&self, visitor: &dyn StmtVisitor<T>) -> T {
		visitor.visit_assign(self)
	}
}

impl<T> StmtVisitable<T> for If<'_> {
	fn accept(&self, visitor: &dyn StmtVisitor<T>) -> T {
		visitor.visit_if(self)
	}
}

impl<T> StmtVisitable<T> for ConditionalBlock<'_> {
	fn accept(&self, visitor: &dyn StmtVisitor<T>) -> T {
		visitor.visit_conditional_block(self)
	}
}

impl<T> StmtVisitable<T> for ForEach<'_> {
	fn accept(&self, visitor: &dyn StmtVisitor<T>) -> T {
		visitor.visit_foreach(self)
	}
}
