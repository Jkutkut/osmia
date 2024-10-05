use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Grouping {
	pub expr: Box<Expr>
}

impl Grouping {
	pub fn new(expr: Expr) -> Self {
		Self { expr: Box::new(expr) }
	}
}

impl Display for Grouping {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "({})", self.expr)
	}
}
