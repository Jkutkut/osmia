use super::*;

pub struct Module {
	elements: Vec<ModuleElement>,
}

impl Module {
	pub fn new() -> Self {
		Self {
			elements: vec![],
		}
	}

	fn add(mut self, module_element: ModuleElement) -> Self {
		self.elements.push(module_element);
		self
	}

	pub fn add_value(self, key: &str, value: CtxValue) -> Self {
		self.add(ModuleElement::new(key, ModuleValue::Value(value)))
	}

	pub fn add_module(self, key: &str, module: Module) -> Self {
		self.add(ModuleElement::new(key, ModuleValue::Module(module)))
	}
}


impl Into<JsonTree<String, CtxValue>> for Module {
	fn into(self) -> JsonTree<String, CtxValue> {
		let mut obj = JsonTree::new_obj();
		for ModuleElement { key, value } in self.elements {
			match value {
				ModuleValue::Value(value) => {
					obj.set(
						&mut key.iter(),
						JsonTree::Value(value),
					).unwrap();
				}
				ModuleValue::Module(module) => {
					obj.set(
						&mut key.iter(),
						module.into(),
					).unwrap();
				}
			}
		}
		obj
	}
}
