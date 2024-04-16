use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::model::{
	Literal, Expression,
	JsonExpression
};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
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

	pub fn from(expr: &JsonExpression) -> Result<Self, String> {
		let tree = match expr {
			JsonExpression::Expression(expr) => match expr {
				Expression::Literal(literal) => match literal {
					Literal::Str(s) => JsonTree::Str(s.to_string()),
					Literal::Int(i) => JsonTree::Number(*i),
					Literal::Float(f) => JsonTree::Float(*f),
					Literal::Bool(b) => JsonTree::Bool(*b),
					Literal::Null => JsonTree::Null,
				},
				_ => return Err("Only literals are supported".to_string())
			},
			JsonExpression::Array(arr) => JsonTree::Array(
				arr.iter()
					.map(|x| JsonTree::from(x))
					.collect::<Result<Vec<JsonTree>, String>>()?
			),
			JsonExpression::Object(obj) => JsonTree::Object(obj.iter().map(|(k, v)| {
				(k.to_string(), Box::new(JsonTree::from(v).unwrap()))
			}).collect())
		};
		Ok(tree)
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
