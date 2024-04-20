use crate::model::{Variable, Stmt, ListOrVariable};

#[derive(Debug, PartialEq)]
pub struct ForEach {
	var: Variable,
	list: ListOrVariable,
	body: Box<Stmt>,
}

impl ForEach {
	pub fn new(var: Variable, list: ListOrVariable, body: Stmt) -> Self {
		Self { var, list, body: Box::new(body) }
	}

	pub fn variable(&self) -> &Variable {
		&self.var
	}

	pub fn list(&self) -> &ListOrVariable {
		&self.list
	}

	pub fn body(&self) -> &Stmt {
		&self.body
	}
}

impl std::fmt::Display for ForEach {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "foreach ( {} in {} ) {{\n{}}}", self.var, self.list, self.body)
	}
}
