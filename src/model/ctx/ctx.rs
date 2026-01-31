//! # Ctx
//! The context can be loaded with information from external sources by the use of the available
//! utilities. You can feed the context with JSON or YAML.
//!
//! **Note**: Keep in mind that the osmia context uses a JSON-like structure, even if the context
//! was fed with YAML.
//!
//! As you can load any JSON or YAML, the logic only permits objects and arrays as the root
//! elements of the context.
//! - If you load an object, each key-value of the object will become a variable in the context.
//! - If you load an array, how would you use it? What is its name? Osmia defines it as a variable
//! called `ctx`.
//! - If you try to load a primitive (number, text...) as a root context, how would you request
//! that information from Osmia? Osmia does not allow this.
//!
#![doc = include_str!("../../../docs/ctx.md")]
//!

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
					e => return Err(e.format_get_error()),
				}
			};
		}
		match error {
			Some(e) => Err(e.format_get_error()),
			None => unreachable!(),
		}
	}

	pub fn set_in_current_scope<'a>(
		&mut self,
		key: &Vec<JsonTreeKey<String>>,
		value: JsonTree<String, CtxValue>
	) -> Result<(), OsmiaError> {
		let current_scope = self.ctx.back_mut().unwrap_or_else(|| unreachable!());
		match current_scope.set(&mut key.iter(), value) {
			Ok(_) => Ok(()),
			Err(e) => match e {
				JsonTreeError::KeyNotFound(_) => Err(e.format_set_error()),
				e => Err(e.format_set_error()),
			}
		}
	}

	pub fn set<'a>(
		&mut self,
		key: &Vec<JsonTreeKey<String>>,
		value: JsonTree<String, CtxValue>
	) -> Result<(), OsmiaError> {
		let root_variable: Vec<JsonTreeKey<String>> = vec![key.get(0).unwrap_or(&JsonTreeKey::Key("".into())).clone()];
		for scope in self.ctx.iter_mut().rev() {
			let get_result = scope.get(&mut root_variable.iter());
			match get_result {
				Ok(_) => return scope.set(&mut key.iter(), value).map_err(|e| e.format_set_error()),
				Err(e) => match e {
					JsonTreeError::KeyNotFound(_) => (),
					e => return Err(e.format_set_error()),
				}
			}
		}
		self.set_in_current_scope(key, value)
	}

	pub fn raw(&self) -> &VecDeque<JsonTree<String, CtxValue>> {
		&self.ctx
	}

	fn try_from_json_tree(
		jt: JsonTree<String, CtxValue>,
	) -> Result<Self, OsmiaError> {
		let content = match jt {
			JsonTree::Object(_) => jt,
			JsonTree::Array(_) => {
				let mut obj = JsonTree::new_obj();
				let key = JsonTreeKey::try_parse("ctx").unwrap();
				obj.set(&mut key.iter(), jt).unwrap();
				obj
			},
			_ => return Err("Ctx must be an object or array".into()),
		};
		let mut ctx = Self::from(content);
		Self::default_libs(&mut ctx);
		ctx.begin_scope();
		Ok(ctx)
	}
	
	pub fn try_from_json(json: &str) -> Result<Self, OsmiaError> {
		match serde_json::from_str(json) {
			Ok(jt) => Self::try_from_json_tree(jt),
			Err(e) => Err(format!("Invalid JSON: {}", e)),
		}
	}

	pub fn try_from_yaml(yaml: &str) -> Result<Self, OsmiaError> {
		match serde_yml::from_str(yaml) {
			Ok(jt) => Self::try_from_json_tree(jt),
			Err(e) => Err(format!("Invalid JSON: {}", e)),
		}
	}
}

impl<'a> TryFrom<&'a str> for Ctx {
	type Error = OsmiaError;

	fn try_from(json: &'a str) -> Result<Self, Self::Error> {
		Self::try_from_json(json)
	}
}


impl JsonTreeError<JsonTreeKey<String>> {
	fn format_get_error(self) -> OsmiaError {
		match self {
			JsonTreeError::AccessValue(k) => format!("Cannot access a value: {}", k),
			JsonTreeError::ArrayOutOfBounds((idx, len)) => format!(
				"Array index out of bounds. Attempted to access index {} in an array of length {}",
				idx, len
			),
			JsonTreeError::IndexInObject => format!("Cannot get by index from an object"),
			JsonTreeError::KeyInArray => format!("Cannot get by key from an array"),
			JsonTreeError::KeyNotFound(k) => format!("{} not found", k),
			JsonTreeError::NoKey => unreachable!(),
		}
	}

	fn format_set_error(self) -> OsmiaError {
		match self {
			JsonTreeError::AccessValue(k) => format!("Cannot access a value: {}", k),
			JsonTreeError::ArrayOutOfBounds((idx, len)) => format!(
				"Array index out of bounds. Attempted to access index {} in an array of length {}",
				idx, len
			),
			JsonTreeError::IndexInObject => format!("Cannot set by index from an object"),
			JsonTreeError::KeyInArray => format!("Cannot set by key from an array"),
			JsonTreeError::KeyNotFound(k) => format!("{} not found", k),
			JsonTreeError::NoKey => unreachable!(),
		}
	}
}
