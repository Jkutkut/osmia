use std::hash::Hash;
use std::collections::HashMap;
use std::iter::Peekable;
use std::fmt::Display;
use serde::Deserialize;
use super::{
	JsonTreeKey,
	JsonTreeError,
};

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum JsonTree<K: Eq + Hash + Clone + Display, T> {
	Value(T),
	Array(Vec<JsonTree<K, T>>),
	Object(HashMap<K, Box<JsonTree<K, T>>>),
}

impl<K: Eq + Hash + Clone + Display, T: Clone> JsonTree<K, T> {
	pub fn get<'a>(
		&self,
		keys: &mut impl Iterator<Item = &'a JsonTreeKey<K>>
	) -> Result<&JsonTree<K, T>, JsonTreeError<JsonTreeKey<K>>> where K: 'a {
		let key = match keys.next() {
			Some(k) => k,
			None => return Ok(self),
		};
		match self {
			JsonTree::Value(_) => Err(JsonTreeError::AccessValue(key.clone())),
			JsonTree::Array(arr) => match key {
				JsonTreeKey::Key(_) => Err(JsonTreeError::KeyInArray),
				JsonTreeKey::Index(i) => arr.get(*i)
					.ok_or(JsonTreeError::ArrayOutOfBounds((*i, arr.len())))
					.and_then(|v| v.get(keys))
			},
			JsonTree::Object(obj) => match key {
				JsonTreeKey::Key(k) => obj.get(k)
					.ok_or(JsonTreeError::KeyNotFound(key.clone()))
					.and_then(|v| v.get(keys)),
				JsonTreeKey::Index(_) => Err(JsonTreeError::IndexInObject),
			}
		}
	}

	pub fn set<'a>(
		&mut self,
		keys: &mut impl Iterator<Item = &'a JsonTreeKey<K>>,
		value: JsonTree<K, T>
	) -> Result<(), JsonTreeError<JsonTreeKey<K>>> where K: 'a {
		self.set_recursive(
			keys.peekable(),
			value
		)
	}

	fn set_recursive<'a>(
		&mut self,
		mut keys: Peekable<impl Iterator<Item = &'a JsonTreeKey<K>>>,
		value: JsonTree<K, T>
	) -> Result<(), JsonTreeError<JsonTreeKey<K>>> where K: 'a {
		let current = match keys.next() {
			None => return Err(JsonTreeError::NoKey),
			Some(k) => k,
		};
		let is_last = keys.peek().is_none();
		if !is_last {
			return match self {
				JsonTree::Value(_) => Err(JsonTreeError::AccessValue(current.clone())),
				JsonTree::Array(arr) => match current {
					JsonTreeKey::Key(_) => Err(JsonTreeError::KeyInArray),
					JsonTreeKey::Index(i) => {
						let arr_size = arr.len();
						let next = arr.get_mut(*i)
							.ok_or(JsonTreeError::ArrayOutOfBounds((*i, arr_size)))?;
						next.set_recursive(keys, value)
					}
				},
				JsonTree::Object(obj) => match current {
					JsonTreeKey::Index(_) => Err(JsonTreeError::IndexInObject),
					JsonTreeKey::Key(k) => obj.get_mut(k)
						.ok_or(JsonTreeError::KeyNotFound(current.clone()))?
						.set_recursive(keys, value)
				}
			}
		}
		match self {
			JsonTree::Value(_) => return Err(JsonTreeError::AccessValue(current.clone())),
			JsonTree::Array(arr) => match current {
				JsonTreeKey::Key(_) => return Err(JsonTreeError::KeyInArray),
				JsonTreeKey::Index(i) => {
					while *i > arr.len() {
						arr.push(value.clone());
					}
					arr.insert(*i, value);
				}
			},
			JsonTree::Object(obj) => match current {
				JsonTreeKey::Index(_) => return Err(JsonTreeError::IndexInObject),
				JsonTreeKey::Key(k) => {obj.insert(k.clone(), Box::new(value));}
			}
		};
		Ok(())
	}
}
