use serde::Deserialize;
use crate::model::interpreter::Callable;
use crate::ctx::JsonTree;

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum CtxValue {
	Int(i64),
	Float(f64),
	Str(String),
	Bool(bool),
	Null,
	#[serde(skip)]
	Callable(Callable),
}

impl Into<JsonTree<String, CtxValue>> for CtxValue {
	fn into(self) -> JsonTree<String, CtxValue> {
		JsonTree::Value(self)
	}
}
