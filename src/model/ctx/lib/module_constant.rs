use super::*;

pub struct ModuleConstant {
	pub key: Vec<JsonTreeKey<String>>,
	pub value: CtxValue,
}

impl ModuleConstant {
	pub fn new(key: Vec<JsonTreeKey<String>>, value: CtxValue) -> Self {
		Self { key, value }
	}
}

impl From<(&str, CtxValue)> for ModuleConstant {
	fn from((key, value): (&str, CtxValue)) -> Self {
		Self::new(JsonTreeKey::from(key), value)
	}
}
