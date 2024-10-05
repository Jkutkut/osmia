use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Grouping {
	expr: Box<Expr>
}

impl Grouping {
	pub fn new(expr: Expr) -> Self {
		Self { expr: Box::new(expr) }
	}

	pub fn expr(&self) -> &Expr {
		&self.expr
	}
}

impl Display for Grouping {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "({})", self.expr)
	}
}
