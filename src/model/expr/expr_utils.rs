use std::fmt::{
	Display, Error,
};

use std::collections::HashMap;
use super::*;
use crate::types::OsmiaError;
use crate::ctx::{
	JsonTree,
	CtxValue,
};

impl Display for Expr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
		match self {
			Expr::Float(f_nbr) => Ok(write!(f, "{f_nbr}")?),
			Expr::Int(i) => Ok(write!(f, "{i}")?),
			Expr::Str(s) => Ok(write!(f, "{s}")?),
			Expr::Bool(b) => Ok(write!(f, "{b}")?),
			Expr::Null => Ok(write!(f, "null")?),
			Expr::Array(arr) => Ok(write!(f, "{arr}")?),
			Expr::Object(obj) => Ok(write!(f, "{obj}")?),
			Expr::Binary(b) => Ok(write!(f, "{b}")?),
			Expr::Grouping(g) => Ok(write!(f, "{g}")?),
			Expr::Unary(u) => Ok(write!(f, "{u}")?),
			Expr::Variable(v) => Ok(write!(f, "{v}")?),
			Expr::Lambda(l) => Ok(write!(f, "{l}")?),
			Expr::Callable(c) => Ok(write!(f, "{c}")?),
			e => unimplemented!("Display for: {:?}", e), // TODO
		}
	}
}

impl Expr {
	pub fn to_bool(&self) -> bool {
		match self {
			Expr::Bool(b) => *b,
			Expr::Float(f) => *f != 0.0,
			Expr::Int(i) => *i != 0,
			Expr::Str(s) => !s.is_empty(),
			Expr::Null => false,
			_ => true
		}
	}

	pub fn to_float(&self) -> Result<f64, OsmiaError> {
		match self {
			Expr::Float(f) => Ok(*f),
			Expr::Int(i) => Ok(*i as f64),
			Expr::Str(n) => match n.parse::<f64>() {
				Ok(f) => Ok(f),
				Err(_) => Err(format!("Cannot convert {:?} to float", n)),
			},
			_ => Err(format!("Cannot convert {} to float", self))
		}
	}

	pub fn to_int(&self) -> Result<i64, OsmiaError> {
		match self {
			Expr::Float(f) => Ok(*f as i64),
			Expr::Int(i) => Ok(*i),
			Expr::Str(s) => match s.parse::<i64>() {
				Ok(i) => Ok(i),
				Err(_) => Err(format!("Cannot convert {:?} to int", s)),
			},
			_ => Err(format!("Cannot convert {} to int", self))
		}
	}

	pub fn print_as_json(&self) -> String {
		match self {
			Expr::Str(_) => format!(r#""{}""#, self.to_string()),
			_ => self.to_string(),
		}
	}
}

impl TryFrom<&JsonTree<String, CtxValue>> for Expr {
	type Error = OsmiaError;

	fn try_from(tree: &JsonTree<String, CtxValue>) -> Result<Self, Self::Error> {
		match tree {
			JsonTree::Value(v) => match v {
				CtxValue::Int(i) => Ok(Expr::Int(*i)),
				CtxValue::Float(f) => Ok(Expr::Float(*f)),
				CtxValue::Str(s) => Ok(Expr::Str(s.clone())),
				CtxValue::Bool(b) => Ok(Expr::Bool(*b)),
				CtxValue::Null => Ok(Expr::Null),
				CtxValue::Callable(c) => Ok(Expr::Callable(c.clone())),
			},
			JsonTree::Object(o) => {
				let mut items: Vec<(Expr, Expr)> = Vec::new();
				for (k, v) in o {
					items.push((
						Expr::Str(k.into()),
						Expr::try_from(v.as_ref())?,
					));
				}
				Ok(Expr::Object(Object::new_hash(items)?))
			},
			JsonTree::Array(a) => {
				let mut arr: Vec<Expr> = Vec::new();
				for v in a {
					arr.push(Expr::try_from(v)?);
				}
				Ok(Expr::Array(arr.into()))
			}
		}
	}
}

impl TryFrom<&Expr> for JsonTree<String, CtxValue> {
	type Error = OsmiaError;

	fn try_from(value: &Expr) -> Result<Self, Self::Error> {
		Ok(match value {
			Expr::Float(v) => JsonTree::Value(CtxValue::Float(*v)),
			Expr::Int(v) => JsonTree::Value(CtxValue::Int(*v)),
			Expr::Str(v) => JsonTree::Value(CtxValue::Str(v.into())),
			Expr::Bool(v) => JsonTree::Value(CtxValue::Bool(*v)),
			Expr::Null => JsonTree::Value(CtxValue::Null),
			Expr::Array(arr) => {
				let mut items: Vec<JsonTree<String, CtxValue>> = Vec::new();
				for e in arr.iter() {
					items.push(e.try_into()?);
				}
				JsonTree::Array(items)
			},
			Expr::Object(obj) => {
				let mut items = HashMap::new();
				for (e, v) in obj.entries() {
					let key = match e {
						Expr::Str(s) => s.into(),
						_ => return Err("Object key must be a string".to_string()),
					};
					let value = Box::new((&v).try_into()?);
					items.insert(key, value);
				}
				JsonTree::Object(items)
			},
			Expr::Callable(c) => JsonTree::Value(CtxValue::Callable(c.clone())),
			_ => return Err(format!("The expression {:?} cannot be stored in the context", value)),
		})
	}
}
