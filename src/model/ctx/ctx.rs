use std::collections::HashMap;

use super::*;
use crate::types::OsmiaError;
use crate::model::interpreter::Callable;

pub struct Ctx {
	ctx: JsonTree<String, CtxValue>,
}

impl Ctx {
	pub fn new() -> Self {
		Self { ctx: JsonTree::Object(HashMap::new()) }
	}

	pub fn get_callable<'a>(
		&self,
		key: &mut impl Iterator<Item = &'a JsonTreeKey<String>>
	) -> Result<Callable, OsmiaError> {
		match self.ctx.get(key) {
			Ok(JsonTree::Value(CtxValue::Callable(c))) => Ok(c.clone()),
			Ok(_) => Err(format!("Not a callable")),
			Err(e) => Err(Self::format_get_error(e)),
		}
	}

	pub fn get<'a>(
		&self,
		key: &mut impl Iterator<Item = &'a JsonTreeKey<String>>
	) -> Result<&JsonTree<String, CtxValue>, OsmiaError> {
		self.ctx.get(key).map_err(Self::format_get_error)
	}

	fn format_get_error(error: JsonTreeError<JsonTreeKey<String>>) -> OsmiaError {
		match error {
			JsonTreeError::AccessValue(k) => format!("Cannot access a value: {}", k),
			JsonTreeError::ArrayOutOfBounds((idx, len)) => format!(
				"Array index out of bounds. Attempted to access index {} in an array of length {}",
				idx, len
			),
			JsonTreeError::IndexInObject => format!("Cannot get by index from an object"),
			JsonTreeError::KeyInArray => format!("Cannot get by key from an array"),
			JsonTreeError::KeyNotFound(k) => format!("Variable not found: {}", k),
			JsonTreeError::NoKey => unreachable!(),
		}
	}

	pub fn set<'a>(
		&mut self,
		key: &mut impl Iterator<Item = &'a JsonTreeKey<String>>,
		value: JsonTree<String, CtxValue>,
	) -> Result<(), OsmiaError> {
		self.ctx.set(key, value).map_err(|e| {
			match e {
				JsonTreeError::AccessValue(k) => format!("Cannot access a value: {}", k),
				JsonTreeError::ArrayOutOfBounds((idx, len)) => format!(
					"Array index out of bounds. Attempted to access index {} in an array of length {}",
					idx, len
				),
				JsonTreeError::IndexInObject => format!("Cannot set by index from an object"),
				JsonTreeError::KeyInArray => format!("Cannot set by key from an array"),
				JsonTreeError::KeyNotFound(k) => format!("Variable not found: {}", k),
				JsonTreeError::NoKey => unreachable!(),
			}
		})
	}
}

impl<'a> TryFrom<&'a str> for Ctx {
	type Error = serde_json::Error;

	fn try_from(s: &'a str) -> Result<Self, Self::Error> {
		let tree = serde_json::from_str(s)?;
		Ok(Self { ctx: tree })
	}
}

impl Into<JsonTree<String, CtxValue>> for Ctx {
	fn into(self) -> JsonTree<String, CtxValue> {
		self.ctx
	}
}
