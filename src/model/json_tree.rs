use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::model::{
	Literal
};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum JsonTree {
	Str(String),
	Number(i64),
	Float(f64),
	Bool(bool),
	Null,
	Array(Vec<JsonTree>),
	Object(HashMap<String, Box<JsonTree>>),
}

impl JsonTree {
	pub fn from_str(json: &str) -> Result<Self, String> {
		serde_json::from_str(json)
			.map_err(|err| err.to_string())
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

impl std::fmt::Display for JsonTree {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter
	) -> std::fmt::Result {
		serde_json::to_string(&self)
			.unwrap()
			.fmt(f)
	}
}
