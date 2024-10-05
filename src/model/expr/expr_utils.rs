use std::fmt::{
	Display, Error,
};

use super::*;
use crate::types::OsmiaError;

impl Display for Expr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
		match self {
			Expr::Float(f_nbr) => Ok(write!(f, "{f_nbr}")?),
			Expr::Int(i) => Ok(write!(f, "{i}")?),
			Expr::Str(s) => Ok(write!(f, "{s}")?),
			Expr::Bool(b) => Ok(write!(f, "{b}")?),
			Expr::Null => Ok(write!(f, "null")?),
			Expr::Array(arr) => Ok(write!(f, "{arr}")?),
			Expr::Binary(b) => Ok(write!(f, "{b}")?),
			Expr::Grouping(g) => Ok(write!(f, "{g}")?),
			Expr::Unary(u) => Ok(write!(f, "{u}")?),
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
			Expr::Str(s) => Err(format!("Cannot convert {:?} to float", s)),
			_ => Err(format!("Cannot convert {} to float", self))
		}
	}

	pub fn to_int(&self) -> Result<i64, OsmiaError> {
		match self {
			Expr::Float(f) => Ok(*f as i64),
			Expr::Int(i) => Ok(*i),
			Expr::Str(s) => Err(format!("Cannot convert {:?} to int", s)),
			_ => Err(format!("Cannot convert {} to int", self))
		}
	}
}
