use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::model::{
	Variable, VariableKey,
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
		// self.tree.as_mut_map().unwrap().insert(
		// 	key.to_string(),
		// 	value
		// );
		let mut keys = key.keys().iter();
		let mut var = &mut self.tree;
		let mut next: &VariableKey;
		let mut current_key = keys.next().unwrap();
		loop {
			let next = match keys.next() {
				Some(k) => k,
				None => break
			};
			match var {
				JsonTree::Object(ref mut map) => {
					let current_key = match current_key {
						VariableKey::Key(k) => k,
						VariableKey::Index(i) => return Err(
							format!("Attempted to set index {} in object", i)
						)
					};
					match map.get_mut(current_key.clone()) {
						None => return Err(
							format!("Key {} not found in object", current_key)
						),
						Some(v) => var = v
					};
				},
				JsonTree::Array(ref mut array) => {
					let current_key = match current_key {
						VariableKey::Index(i) => i,
						VariableKey::Key(k) => return Err(
							format!("Attempted to set key {} in array", k)
						)
					};
					match array.get_mut(*current_key) {
						None => return Err(
							format!("Index {} not found in array", current_key)
						),
						Some(v) => var = v
					}
				},
				_ => return Err(
					format!("{} is not an object or array", var)
				)
			};
			current_key = next;
		}
		match (var, current_key) {
			(JsonTree::Object(map), VariableKey::Key(key)) => {
				map.insert(key.to_string(), Box::new(value));
			},
			(JsonTree::Array(array), VariableKey::Index(index)) => {
				while *index > array.len() {
					array.push(JsonTree::Null);
				}
				array.insert(*index, *Box::new(value));
			},
			_ => todo!()
		}
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
	Object(HashMap<String, Box<JsonTree>>),
}

impl JsonTree {
	pub fn from_str(json: &str) -> Result<Self, String> {
		serde_json::from_str(json)
			.map_err(|err| err.to_string())
	}

	pub fn as_mut_map(&mut self) -> Result<&mut HashMap<String, Box<JsonTree>>, String> {
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
