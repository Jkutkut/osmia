use std::ops::{
	Add, Sub,
};

use super::*;
use crate::types::OsmiaError;

/// Addition
impl Add for Expr {
	type Output = Result<Expr, OsmiaError>;

	/// ```rust
	/// use osmia::Osmia;
	///
	/// let mut osmia = Osmia::default();
	/// assert_eq!(osmia.run_code("{{ \"Hello \" + \"World\" }}").unwrap(), "Hello World");
	/// assert_eq!(osmia.run_code("{{ \"Hello \" + 123 }}").unwrap(), "Hello 123");
	/// assert_eq!(osmia.run_code("{{ true + \" Hello\" }}").unwrap(), "true Hello");
	/// assert_eq!(osmia.run_code("{{ true + false }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ 1 + false }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ 1.1 + 2 }}").unwrap(), "3.1");
	/// assert_eq!(osmia.run_code("{{ 1 + 2.1 }}").unwrap(), "3");
	/// assert_eq!(osmia.run_code("{{ null + null }}").unwrap(), "null");
	/// assert_eq!(osmia.run_code("{{ [1, 2] + [3, 4] }}").unwrap(), "[1, 2, 3, 4]");
	//  assert_eq!(osmia.run_code(r#"{{ {"a": 1, "b": 2} + {"b": 3, "c": 4} }}"#).unwrap(), r#"{"a": 1, "b": 3, "c": 4}"#);
	/// ```
	fn add(self, rhs: Self) -> Self::Output {
		match (self, &rhs) {
			(Expr::Str(s1), s2) => Ok(Expr::Str(format!("{}{}", s1, s2.to_string()))),
			(s1, Expr::Str(s2)) => Ok(Expr::Str(format!("{}{}", s1.to_string(), s2))),
			(Expr::Bool(b1), b2) => Ok(Expr::Bool(b1 || b2.to_bool())),
			(b1, Expr::Bool(b2)) => Ok(Expr::Bool(b1.to_bool() || *b2)),
			(Expr::Float(n1), n2) => Ok(Expr::Float(n1 + n2.to_float()?)),
			(Expr::Int(n1), n2) => Ok(Expr::Int(
				n1.checked_add(n2.to_int()?)
				.ok_or(format!(
					"Cannot add {} and {}: It will overflow",
					n1, n2
				))?
			)),
			(Expr::Null, Expr::Null) => Ok(Expr::Null),
			(Expr::Array(arr1), Expr::Array(arr2)) => Ok(Expr::Array(&arr1 + arr2)),
			// (Expr::Object(obj1), Expr::Object(obj2)) => Ok(Expr::Object(&obj1 + obj2)), // TODO
			(s, rhs) => Err(format!("Don't know how to add {} and {}", s, rhs))
		}
	}
}

impl Add for &Array {
	type Output = Array;

	fn add(self, rhs: Self) -> Self::Output {
		let mut arr: Vec<_> = self.into();
		arr.append(&mut rhs.into());
		Array::new(arr)
	}
}

impl Add for &Object {
	type Output = Object;

	fn add(self, rhs: Self) -> Self::Output {
		let mut obj: Vec<_> = self.into();
		obj.append(&mut rhs.into());
		Object::new(obj)
	}
}

/// Subtraction
impl Sub for Expr {
	type Output = Result<Expr, OsmiaError>;

	/// ```rust
	/// use osmia::Osmia;
	///
	/// let mut osmia = Osmia::default();
	/// assert_eq!(osmia.run_code("{{ 1 - 2 }}").unwrap(), "-1");
	/// assert_eq!(osmia.run_code("{{ 1.2 - 2 }}").unwrap(), "-0.8");
	/// assert_eq!(osmia.run_code("{{ 1 - 2.1 }}").unwrap(), "-1");
	/// ```
	fn sub(self, rhs: Self) -> Self::Output {
		match (self, &rhs) {
			(Expr::Float(n1), n2) => Ok(Expr::Float(n1 - n2.to_float()?)),
			(Expr::Int(n1), n2) => Ok(Expr::Int(
				n1.checked_sub(n2.to_int()?)
				.ok_or(format!(
					"Cannot subtract {} and {}: It will overflow",
					n1, n2
				))?
			)),
			(Expr::Null, Expr::Null) => Ok(Expr::Null),
			(s, rhs) => Err(format!("Don't know how to subtract {} and {}", s, rhs))
		}
	}
}
