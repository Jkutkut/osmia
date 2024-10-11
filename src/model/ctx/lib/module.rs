use super::*;

pub struct Module {
	constants: Vec<ModuleConstant>,
	callables: Vec<ModuleCallable>,
}

impl Module {
	pub fn new(
		constants: Vec<ModuleConstant>,
		callables: Vec<ModuleCallable>,
	) -> Self {
		Self {
			constants,
			callables,
		}
	}
}

impl Into<JsonTree<String, CtxValue>> for Module {
	fn into(self) -> JsonTree<String, CtxValue> {
		let mut ctx = Ctx::clean();
		for ModuleConstant { key, value } in self.constants {
			ctx.set(
				&mut key.iter(),
				JsonTree::Value(value),
			).unwrap();
		}
		for ModuleCallable { key, callable } in self.callables {
			ctx.set(
				&mut key.iter(),
				JsonTree::Value(CtxValue::Callable(callable)),
			).unwrap();
		}
		ctx.into()
	}
}
