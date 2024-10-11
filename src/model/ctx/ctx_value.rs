use serde::Deserialize;
use crate::model::interpreter::Callable;

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum CtxValue {
	Int(i64),
	Float(f64),
	Str(String),
	Bool(bool),
	Null,
}
