mod json_tree;
mod json_tree_error;
mod json_tree_key;

pub use json_tree::JsonTree;
pub use json_tree_error::JsonTreeError;
pub use json_tree_key::JsonTreeKey;

pub struct Ctx {
	ctx: JsonTree<String, CtxValue>,
}

impl Ctx {
	pub fn new() -> Self {
		Self { ctx: JsonTree::Object(
			std::collections::HashMap::new()
		) }
	}

	pub fn get<'a>(
		&self,
		key: &mut impl Iterator<Item = &'a JsonTreeKey<String>>
	) -> Result<&JsonTree<String, CtxValue>, JsonTreeError<JsonTreeKey<String>>> {
		self.ctx.get(key)
	}

	pub fn set<'a>(
		&mut self,
		key: &mut impl Iterator<Item = &'a JsonTreeKey<String>>,
		value: CtxValue
	) -> Result<(), JsonTreeError<JsonTreeKey<String>>> {
		self.ctx.set(key, JsonTree::Value(value))
	}
}

use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum CtxValue {
	Int(i64),
	Float(f64),
	String(String),
	Bool(bool),
	Null,
}
