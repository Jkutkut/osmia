mod json_tree;
mod json_tree_error;
mod json_tree_key;

pub use json_tree::JsonTree;
pub use json_tree_error::JsonTreeError;
pub use json_tree_key::JsonTreeKey;

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
