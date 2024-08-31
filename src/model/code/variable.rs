use crate::model::ctx::JsonTreeKey;

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
	var: Vec<JsonTreeKey<String>>,
}

impl Variable {
	pub fn from_vec(var: Vec<JsonTreeKey<String>>) -> Self {
		Self { var }
	}

	pub fn push(&mut self, key: JsonTreeKey<String>) {
		self.var.push(key)
	}

	pub fn vec(&self) -> &Vec<JsonTreeKey<String>> {
		&self.var
	}
}

impl From<&str> for JsonTreeKey<String> {
	fn from(s: &str) -> Self {
		JsonTreeKey::Key(s.into())
	}
}
