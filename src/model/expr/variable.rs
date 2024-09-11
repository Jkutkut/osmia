use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
	var: Vec<JsonTreeKeyExpr>,
}

impl Variable {
	pub fn from_vec(var: Vec<JsonTreeKeyExpr>) -> Self {
		Self { var }
	}

	pub fn push(&mut self, key: JsonTreeKeyExpr) {
		self.var.push(key)
	}

	pub fn extend(&mut self, keys: Vec<JsonTreeKeyExpr>) {
		self.var.extend(keys)
	}

	pub fn vec(&self) -> &Vec<JsonTreeKeyExpr> {
		&self.var
	}
}

impl From<Variable> for Vec<JsonTreeKeyExpr> {
	fn from(var: Variable) -> Self {
		var.var
	}
}
