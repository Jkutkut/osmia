use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::model::{
	Variable,
	Literal
};

#[derive(Debug, PartialEq)]
pub struct Ctx {
	tree: JsonTree,
}

impl Ctx {
	fn new(
		tree: JsonTree
	) -> Ctx {
		Ctx { tree }
	}

	pub fn from_str(
		json: &str
	) -> Result<Ctx, String> {
		let json = JsonTree::from_str(json)?;
		match json {
			JsonTree::Object(_) => Ok(Ctx::new(json)),
			_ => Err("Ctx must be an object".to_string()),
		}
	}

	pub fn set(&mut self, key: Variable, value: Literal) -> Result<(), String> {
		let value = JsonTree::from_literal(&value);
		self.tree.as_mut_map().unwrap().insert(
			key.to_string(),
			value
		);
		Ok(())
	}
}

impl std::fmt::Display for Ctx {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter
	) -> std::fmt::Result {
		serde_json::to_string(&self.tree)
			.unwrap()
			.fmt(f)
	}
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
enum JsonTree {
	Str(String),
	Number(i64),
	Float(f64),
	Bool(bool),
	Null,
	Array(Vec<JsonTree>),
	Object(HashMap<String, JsonTree>),
}

impl JsonTree {
	pub fn from_str(json: &str) -> Result<Self, String> {
		serde_json::from_str(json)
			.map_err(|err| err.to_string())
	}

	pub fn as_mut_map(&mut self) -> Result<&mut HashMap<String, JsonTree>, String> {
		match self {
			JsonTree::Object(map) => Ok(map),
			_ => return Err("Not an object".to_string()),
		}
	}

	pub fn from_literal(literal: &Literal) -> JsonTree {
		match literal {
			Literal::Str(s) => JsonTree::Str(s.to_string()),
			Literal::Int(i) => JsonTree::Number(*i),
			Literal::Float(f) => JsonTree::Float(*f),
			Literal::Bool(b) => JsonTree::Bool(*b),
			Literal::Null => JsonTree::Null
		}
	}
}
