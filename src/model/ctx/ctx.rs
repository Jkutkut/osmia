use std::collections::HashMap;

use super::*;
use crate::stdlib;
use crate::types::OsmiaError;

pub struct Ctx {
	ctx: JsonTree<String, CtxValue>,
}

impl Ctx {
	pub fn new() -> Self {
		let mut ctx = Self::clean();
		Self::default_libs(&mut ctx);
		ctx
	}

	pub fn from(ctx: JsonTree<String, CtxValue>) -> Self {
		Self { ctx }
	}

	pub fn clean() -> Self {
		Self::from(JsonTree::Object(HashMap::new()))
	}

	fn default_libs(ctx: &mut Self) {
		stdlib::import(ctx);
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
	type Error = OsmiaError;

	fn try_from(json: &'a str) -> Result<Self, Self::Error> {
		let content: JsonTree<String, CtxValue> = match serde_json::from_str(json) {
			Ok(c) => match c {
				JsonTree::Object(_) => c,
				JsonTree::Array(_) => return Err("Cannot use an array as a context".into()),
				_ => return Err("Ctx must be an object".into()),
			}
			Err(e) => return Err(format!("Invalid JSON: {}", e)),
		};
		let mut ctx = Self::from(content);
		Self::default_libs(&mut ctx);
		Ok(ctx)
	}
}

impl Into<JsonTree<String, CtxValue>> for Ctx {
	fn into(self) -> JsonTree<String, CtxValue> {
		self.ctx
	}
}
