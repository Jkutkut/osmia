mod json_tree;
mod json_tree_error;
mod json_tree_key;

pub use json_tree::JsonTree;
pub use json_tree_error::JsonTreeError;
pub use json_tree_key::JsonTreeKey;

type CtxKey = String;
type CtxValue = String;

pub struct Ctx {
	ctx: JsonTree<CtxKey, CtxValue>,
}

impl Ctx {
	#[allow(dead_code)]
	pub fn new() -> Self {
		Self { ctx: JsonTree::Object(
			std::collections::HashMap::new()
		) }
	}

	#[allow(dead_code)]
	#[allow(unused_variables)]
	pub fn get<'a>(
		&self,
		key: &mut impl Iterator<Item = &'a JsonTreeKey<CtxKey>>
	) -> Result<&JsonTree<CtxKey, CtxValue>, JsonTreeError<JsonTreeKey<CtxKey>>> {
		self.ctx.get(key)
	}

	#[allow(dead_code)]
	#[allow(unused_variables)]
	pub fn set<'a>(
		&mut self,
		key: &mut impl Iterator<Item = &'a JsonTreeKey<CtxKey>>,
		value: CtxValue
	) -> Result<(), JsonTreeError<JsonTreeKey<CtxKey>>> {
		self.ctx.set(key, JsonTree::Value(value))
	}
}

// TODO Remove once real Ctx is implemented
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum JsonValue {
	Int(i64),
	Float(f64),
	String(String),
	Bool(bool),
	Null,
}
