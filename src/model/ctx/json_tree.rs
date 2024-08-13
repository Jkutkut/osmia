use std::hash::Hash;
use std::collections::HashMap;
use serde::Deserialize;
use super::{
	JsonTreeKey,
	JsonTreeError,
};

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum JsonTree<K: Eq + Hash + Clone, T> {
	Value(T),
	Array(Vec<JsonTree<K, T>>),
	Object(HashMap<K, Box<JsonTree<K, T>>>),
}

impl<K: Eq + Hash + Clone, T> JsonTree<K, T> where T: Clone {
	pub fn get<'a>(
		&self,
		keys: &mut impl Iterator<Item = &'a JsonTreeKey<K>>
	) -> Result<&JsonTree<K, T>, JsonTreeError<JsonTreeKey<K>>> where K: 'a
	{
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

	#[allow(unused_variables)]
	pub fn set<'a>(
		&mut self,
		keys: &mut impl Iterator<Item = &'a JsonTreeKey<K>>,
		value: JsonTree<K, T>
	) -> Result<(), JsonTreeError<JsonTreeKey<K>>> where K: 'a
	{
		todo!(); // TODO
	}
}
