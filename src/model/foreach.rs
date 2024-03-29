use crate::model::{Variable, Stmt, ListOrVariable};

#[derive(Debug, PartialEq)]
pub struct ForEach<'a> {
	var: Variable<'a>,
	list: ListOrVariable<'a>,
	body: Box<Stmt<'a>>,
}

impl<'a> ForEach<'a> {
	pub fn new(var: Variable<'a>, list: ListOrVariable<'a>, body: Stmt<'a>) -> Self {
		Self { var, list, body: Box::new(body) }
	}

	pub fn variable(&self) -> &Variable<'a> {
		&self.var
	}

	pub fn list(&self) -> &ListOrVariable<'a> {
		&self.list
	}

	pub fn body(&self) -> &Stmt<'a> {
		&self.body
	}
}

impl std::fmt::Display for ForEach<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "foreach ( {} in {} ) {{\n{}}}", self.var, self.list, self.body)
	}
}
