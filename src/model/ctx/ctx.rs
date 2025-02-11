use std::collections::VecDeque;

use super::*;
use crate::stdlib;
use crate::types::OsmiaError;

pub struct Ctx {
	ctx: VecDeque<JsonTree<String, CtxValue>>,
}

impl Ctx {
	pub fn new() -> Self {
		let mut ctx = Self::clean();
		Self::default_libs(&mut ctx);
		ctx.begin_scope();
		ctx
	}

	pub fn from(ctx: JsonTree<String, CtxValue>) -> Self {
		Self { ctx: VecDeque::from([ctx]) }
	}

	pub fn clean() -> Self {
		Self::from(JsonTree::new_obj())
	}

	fn default_libs(ctx: &mut Self) {
		stdlib::import(ctx);
	}

	pub fn begin_scope(&mut self) {
		self.ctx.push_back(JsonTree::new_obj());
	}

	pub fn end_scope(&mut self) {
		self.ctx.pop_back();
	}

	pub fn get<'a>(
		&self,
		key: &Vec<JsonTreeKey<String>>
	) -> Result<&JsonTree<String, CtxValue>, OsmiaError> {
		let mut error: Option<JsonTreeError<JsonTreeKey<String>>> = None;
		for scope in self.ctx.iter().rev() {
			error = match scope.get(&mut key.iter()) {
				Ok(v) => return Ok(v),
				Err(e) => match e {
					JsonTreeError::KeyNotFound(_) => Some(e),
					e => return Err(Self::format_get_error(e)),
				}
			};
		}
		match error {
			Some(e) => Err(Self::format_set_error(e)),
			None => unreachable!(),
		}
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
		key: &Vec<JsonTreeKey<String>>,
		value: JsonTree<String, CtxValue>
	) -> Result<(), OsmiaError> {
		let mut error: Option<JsonTreeError<JsonTreeKey<String>>> = None;
		for scope in self.ctx.iter_mut().rev() {
			error = match scope.set(&mut key.iter(), value.clone()) {
				Ok(_) => return Ok(()),
				Err(e) => match e {
					JsonTreeError::KeyNotFound(_) => Some(e),
					e => return Err(Self::format_set_error(e)),
				}
			};
		}
		match error {
			Some(e) => Err(Self::format_set_error(e)),
			None => unreachable!(),
		}
	}

	fn format_set_error(error: JsonTreeError<JsonTreeKey<String>>) -> OsmiaError {
		match error {
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
		ctx.begin_scope();
		Ok(ctx)
	}
}
