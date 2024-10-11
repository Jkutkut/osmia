use super::*;

pub struct ModuleCallable {
	pub key: Vec<JsonTreeKey<String>>,
	pub callable: Callable,
}

impl ModuleCallable {
	pub fn new(key: Vec<JsonTreeKey<String>>, callable: Callable) -> Self {
		Self { key, callable }
	}
}

impl From<(&str, Callable)> for ModuleCallable {
	fn from((key, callable): (&str, Callable)) -> Self {
		Self::new(JsonTreeKey::from(key), callable)
	}
}
