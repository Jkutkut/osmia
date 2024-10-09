mod json_tree;
mod json_tree_error;
mod json_tree_key;

pub use json_tree::JsonTree;
pub use json_tree_error::JsonTreeError;
pub use json_tree_key::JsonTreeKey;

use serde::Deserialize;
use std::collections::HashMap;

pub struct Ctx {
	ctx: JsonTree<String, CtxValue>,
}

impl Ctx {
	pub fn new() -> Self {
		Self { ctx: JsonTree::Object(
			HashMap::new()
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
		value: JsonTree<String, CtxValue>,
	) -> Result<(), JsonTreeError<JsonTreeKey<String>>> {
		self.ctx.set(key, value)
	}
}

impl<'a> TryFrom<&'a str> for Ctx {
	type Error = serde_json::Error;

	fn try_from(s: &'a str) -> Result<Self, Self::Error> {
		let tree = serde_json::from_str(s)?;
		Ok(Self { ctx: tree })
	}
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum CtxValue {
	Int(i64),
	Float(f64),
	Str(String),
	Bool(bool),
	Null,
}
