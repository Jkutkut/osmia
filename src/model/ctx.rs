use std::collections::HashMap;

use crate::model::{
	Variable, VariableKey,
	Literal, JsonTree
};

#[derive(Debug, PartialEq)]
pub struct Ctx {
	tree: JsonTree,
}

impl Ctx {
	fn new(tree: JsonTree) -> Ctx {
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

	pub fn get<'a>(&'a self, key: &'a Variable) -> Result<&'a JsonTree, String> {
		self.get_value(key)
	}

	pub fn get_literal(&self, key: &Variable) -> Result<Literal, String> {
		self.get_value_literal(key)
	}

	pub fn set(&mut self, key: &Variable, value: JsonTree) -> Result<(), String> {
		self.set_value(key, value)
	}

	pub fn set_literal(&mut self, key: &Variable, value: Literal) -> Result<(), String> {
		let value = JsonTree::from_literal(&value);
		self.set_value(key, value)
	}
}

impl Ctx {
	fn get_value_literal(&self, key: &Variable) -> Result<Literal, String> {
		match self.get_value(key)? {
			JsonTree::Str(s) => Ok(Literal::Str(s.to_string())),
			JsonTree::Number(n) => Ok(Literal::Int(*n)),
			JsonTree::Float(f) => Ok(Literal::Float(*f)),
			JsonTree::Null => Ok(Literal::Null),
			JsonTree::Bool(b) => Ok(Literal::Bool(*b)),
			_ => Err("Cannot get an array or object as a literal".to_string())
		}
	}

	fn get_value<'a>(&'a self, key: &'a Variable) -> Result<&'a JsonTree, String> {
		let keys = &mut key.keys().iter();
		let mut var = &self.tree;
		let mut current_key = keys.next().unwrap();
		loop {
			match var {
				JsonTree::Object(ref map) => var = match Self::visit_obj(current_key, map)? {
					Some(v) => v,
					None => return match keys.next() {
						Some(_) => Err(format!("Key {} not found in object", current_key)),
						None => Ok(&JsonTree::Null)
					}
				},
				JsonTree::Array(ref array) =>
					var = Self::visit_arr(current_key, array)?,
				_ => return Err(
					format!("{} is not an object or array", var)
				)
			};
			current_key = match keys.next() {
				Some(k) => k,
				None => break
			};
		}
		Ok(var)
	}

	fn set_value(&mut self, key: &Variable, value: JsonTree) -> Result<(), String> {
		let mut keys = key.keys().iter();
		let mut var = &mut self.tree;
		let mut current_key = keys.next().unwrap();
		loop {
			let next = match keys.next() {
				Some(k) => k,
				None => break
			};
			match var {
				JsonTree::Object(ref mut map) =>
					var = Self::visit_mut_obj(current_key, map)?,
				JsonTree::Array(ref mut array) =>
					var = Self::visit_mut_arr(current_key, array)?,
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
			(JsonTree::Array(_), VariableKey::Key(key)) => {
				return Err(
					format!("Attempted to use key {} in array", key)
				);
			},
			(JsonTree::Object(_), VariableKey::Index(index)) => {
				return Err(
					format!("Attempted to use index {} in object", index)
				);
			},
			_ => return Err(
				"Error while attempting to set a non object or array".to_string()
			)
		}
		Ok(())
	}
}

impl Ctx {
	fn visit_obj<'a>(
		key: &'a VariableKey,
		map: &'a HashMap<String, Box<JsonTree>>
	) -> Result<Option<&'a Box<JsonTree>>, String> {
		let key = match key {
			VariableKey::Key(k) => k,
			VariableKey::Index(i) => return Err(
				format!("Attempted to use index {} in object", i)
			)
		};
		Ok(map.get(key))
	}

	fn visit_mut_obj<'a>(
		key: &'a VariableKey,
		map: &'a mut HashMap<String, Box<JsonTree>>
	) -> Result<&'a mut JsonTree, String> {
		let key = match key {
			VariableKey::Key(k) => k,
			VariableKey::Index(i) => return Err(
				format!("Attempted to use index {} in object", i)
			)
		};
		match map.get_mut(key) {
			None => Err(
				format!("Key {} not found in object", key)
			),
			Some(v) => Ok(v)
		}
	}

	fn visit_arr<'a>(
		key: &'a VariableKey,
		array: &'a Vec<JsonTree>
	) -> Result<&'a JsonTree, String> {
		let idx = match key {
			VariableKey::Index(i) => i,
			VariableKey::Key(k) => return Err(
				format!("Attempted to use key {} in array", k)
			)
		};
		match array.get(*idx) {
			None => Err(
				format!("Index {} not found in array", idx)
			),
			Some(v) => Ok(v)
		}
	}

	fn visit_mut_arr<'a>(
		key: &'a VariableKey,
		array: &'a mut Vec<JsonTree>
	) -> Result<&'a mut JsonTree, String> {
		let idx = match key {
			VariableKey::Index(i) => i,
			VariableKey::Key(k) => return Err(
				format!("Attempted to use key {} in array", k)
			)
		};
		match array.get_mut(*idx) {
			None => Err(
				format!("Index {} not found in array", idx)
			),
			Some(v) => Ok(v)
		}
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
