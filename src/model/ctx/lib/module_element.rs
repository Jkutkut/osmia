use super::*;

pub struct ModuleElement {
	pub key: Vec<JsonTreeKey<String>>,
	pub value: ModuleValue,
}

impl ModuleElement {
	pub fn new(key: &str, value: ModuleValue) -> Self {
		Self {
			key: JsonTreeKey::try_parse(key).unwrap(),
			value,
		}
	}
}
