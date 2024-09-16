use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Array {
	arr: Vec<Expr>,
}

impl Array {
	pub fn new(arr: Vec<Expr>) -> Self {
		Self { arr }
	}

	pub fn push(&mut self, expr: Expr) {
		self.arr.push(expr)
	}

	pub fn len(&self) -> usize {
		self.arr.len()
	}

	pub fn iter(&self) -> std::slice::Iter<'_, Expr> {
		self.arr.iter()
	}
}

impl From<Vec<Expr>> for Array {
	fn from(arr: Vec<Expr>) -> Self {
		Self::new(arr)
	}
}

impl Into<Vec<Expr>> for &Array {
	fn into(self) -> Vec<Expr> {
		self.arr.clone()
	}
}

impl Display for Array {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(
			f, "[{}]",
			self.arr.iter()
				.map(|e| e.to_string())
				.collect::<Vec<_>>()
				.join(", ")
		)
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
	obj: Vec<(Expr, Expr)>,
}

impl Object {
	pub fn new(obj: Vec<(Expr, Expr)>) -> Self {
		Self { obj }
	}

	pub fn push(&mut self, e: (Expr, Expr)) {
		self.obj.push(e)
	}

	pub fn len(&self) -> usize {
		self.obj.len()
	}
}

impl From<Vec<(Expr, Expr)>> for Object {
	fn from(obj: Vec<(Expr, Expr)>) -> Self {
		Self::new(obj)
	}
}
