use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
	var: Vec<JsonTreeKeyExpr>,
}

impl Variable {
	pub fn from_vec(var: Vec<JsonTreeKeyExpr>) -> Self {
		Self { var }
	}

	pub fn from_name(name: JsonTreeKeyExpr) -> Self {
		Self::from_vec(vec![name])
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

impl std::fmt::Display for Variable {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut brackets;
		for (i, v) in self.var.iter().enumerate() {
			brackets = false;
			if i > 0 {
				match v {
					JsonTreeKeyExpr::JsonTreeKey(JsonTreeKey::Key(_)) => write!(f, ".")?,
					_ => {
						brackets = true;
						write!(f, "[")?;
					}
				}
			}
			write!(f, "{v}")?;
			if brackets {
				write!(f, "]")?;
			}
		}
		Ok(())
	}
}
