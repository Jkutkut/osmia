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
			e => unimplemented!("Display for: {:?}", e), // TODO
		}
	}
}
