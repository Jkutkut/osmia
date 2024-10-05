use std::cmp::{
	PartialOrd,
	Ordering
};
use std::ops::{
	Add, Sub,
	Mul, Div,
	Rem,
	BitAnd, BitOr, BitXor,
	Shl, Shr
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
			(s, rhs) => Err(format!("Don't know how to subtract {} and {}", s, rhs))
		}
	}
}

/// Multiplication
impl Mul for Expr {
	type Output = Result<Expr, OsmiaError>;

	/// ```rust
	/// use osmia::Osmia;
	///
	/// let mut osmia = Osmia::default();
	/// assert_eq!(osmia.run_code("{{ 1 * 2 }}").unwrap(), "2");
	/// assert_eq!(osmia.run_code("{{ 1.2 * 2 }}").unwrap(), "2.4");
	/// assert_eq!(osmia.run_code("{{ 1 * 2.1 }}").unwrap(), "2");
	/// ```
	fn mul(self, rhs: Self) -> Self::Output {
		match (self, &rhs) {
			(Expr::Float(n1), n2) => Ok(Expr::Float(n1 * n2.to_float()?)),
			(Expr::Int(n1), n2) => Ok(Expr::Int(
				n1.checked_mul(n2.to_int()?)
				.ok_or(format!(
					"Cannot multiply {} and {}: It will overflow",
					n1, n2
				))?
			)),
			(s, rhs) => Err(format!("Don't know how to multiply {} and {}", s, rhs))
		}
	}
}

/// Division
impl Div for Expr {
	type Output = Result<Expr, OsmiaError>;

	/// ```rust
	/// use osmia::Osmia;
	///
	/// let mut osmia = Osmia::default();
	/// assert_eq!(osmia.run_code("{{ 1 / 2 }}").unwrap(), "0");
	/// assert_eq!(osmia.run_code("{{ 1.2 / 2 }}").unwrap(), "0.6");
	/// assert_eq!(osmia.run_code("{{ 1 / 2.0 }}").unwrap(), "0.5");
	/// assert!(osmia.run_code("{{ 1 / 0 }}").unwrap_err().contains("divide"));
	/// assert!(osmia.run_code("{{ 0 / 0 }}").unwrap_err().contains("divide"));
	/// ```
	fn div(self, rhs: Self) -> Self::Output {
		match (self, &rhs) {
			(Expr::Float(n1), n2) => Ok(Expr::Float(n1 / n2.to_float()?)),
			(n1, Expr::Float(n2)) => Ok(Expr::Float(n1.to_float()? / n2)),
			(Expr::Int(n1), n2) => Ok(Expr::Int(
				n1.checked_div(n2.to_int()?)
				.ok_or(format!(
					"Cannot divide {} and {}",
					n1, n2
				))?
			)),
			(s, rhs) => Err(format!("Don't know how to divide {} and {}", s, rhs))
		}
	}
}

/// Modulo
impl Rem for Expr {
	type Output = Result<Expr, OsmiaError>;

	/// ```rust
	/// use osmia::Osmia;
	///
	/// let mut osmia = Osmia::default();
	/// assert_eq!(osmia.run_code("{{ 1 % 2 }}").unwrap(), "1");
	/// assert_eq!(osmia.run_code("{{ 1.2 % 2 }}").unwrap(), "1.2");
	/// assert_eq!(osmia.run_code("{{ 1 % 2.1 }}").unwrap(), "1");
	/// assert_eq!(osmia.run_code("{{ 1.2 % 2.1 }}").unwrap(), "1.2");
	/// assert!(osmia.run_code("{{ 1 % 0 }}").unwrap_err().contains("modulo"));
	/// assert!(osmia.run_code("{{ 0 % 0 }}").unwrap_err().contains("modulo"));
	/// ```
	fn rem(self, rhs: Self) -> Self::Output {
		match (self, &rhs) {
			(Expr::Float(n1), n2) => Ok(Expr::Float(n1 % n2.to_float()?)),
			(n1, Expr::Float(n2)) => Ok(Expr::Float(n1.to_float()? % n2)),
			(Expr::Int(n1), n2) => Ok(Expr::Int(
				n1.checked_rem(n2.to_int()?)
				.ok_or(format!(
					"Cannot modulo {} and {}",
					n1, n2
				))?
			)),
			(s, rhs) => Err(format!("Don't know how to modulo {} and {}", s, rhs))
		}
	}
}

/// Comparison
impl PartialOrd for Expr {

	/// ```rust
	/// use osmia::Osmia;
	///
	/// let mut osmia = Osmia::default();
	/// assert_eq!(osmia.run_code("{{ 1 < 2 }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ 1 < 2.1 }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ 1.2 < 2.1 }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ 1.2 < 2 }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ \"a\" < \"b\" }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ \"1\" < 2 }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ \"2\" < true }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ \"2\" < null }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ true < true }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ true < false }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ true < null }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ null < null }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ null < false }}").unwrap(), "false");
	/// ```
	/// ```rust
	/// use osmia::Osmia;
	///
	/// let mut osmia = Osmia::default();
	/// assert_eq!(osmia.run_code("{{ 1 > 2 }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ 1 > 2.1 }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ 1.2 > 2.1 }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ 1.2 > 2 }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ \"a\" > \"b\" }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ \"1\" > 2 }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ \"2\" > true }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ \"2\" > null }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ true > true }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ true > false }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ true > null }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ null > null }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ null > false }}").unwrap(), "false");
	/// ```
	/// ```rust
	/// use osmia::Osmia;
	///
	/// let mut osmia = Osmia::default();
	/// assert_eq!(osmia.run_code("{{ 1 <= 2 }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ 1 <= 2.1 }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ 1.2 <= 2.1 }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ 1.2 <= 2 }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ \"a\" <= \"b\" }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ \"1\" <= 2 }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ \"2\" <= true }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ \"2\" <= null }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ true <= true }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ true <= false }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ true <= null }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ null <= null }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ null <= false }}").unwrap(), "false");
	/// ```
	/// ```rust
	/// use osmia::Osmia;
	///
	/// let mut osmia = Osmia::default();
	/// assert_eq!(osmia.run_code("{{ 1 >= 2 }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ 1 >= 2.1 }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ 1.2 >= 2.1 }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ 1.2 >= 2 }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ \"a\" >= \"b\" }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ \"1\" >= 2 }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ \"2\" >= true }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ \"2\" >= null }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ true >= true }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ true >= false }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ true >= null }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ null >= null }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ null >= false }}").unwrap(), "false");
	/// ```
	/// ```rust
	/// use osmia::Osmia;
	///
	/// let mut osmia = Osmia::default();
	/// assert_eq!(osmia.run_code("{{ 1 == 2 }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ 1 == 2.1 }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ 1.2 == 2.1 }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ 1.2 == 2 }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ \"a\" == \"b\" }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ \"1\" == 2 }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ \"1\" == 1 }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ \"2\" == true }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ \"2\" == null }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ true == true }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ true == false }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ true == null }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ null == null }}").unwrap(), "true");
	/// ```
	/// ```rust
	/// use osmia::Osmia;
	///
	/// let mut osmia = Osmia::default();
	/// assert_eq!(osmia.run_code("{{ 1 != 2 }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ 1 != 2.1 }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ 1.2 != 2.1 }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ 1.2 != 2 }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ \"a\" != \"b\" }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ \"1\" != 2 }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ \"1\" != 1 }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ \"2\" != true }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ \"2\" != null }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ true != true }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ true != false }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ true != null }}").unwrap(), "true");
	/// assert_eq!(osmia.run_code("{{ null != null }}").unwrap(), "false");
	/// assert_eq!(osmia.run_code("{{ null != false }}").unwrap(), "true");
	/// ```
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			(Expr::Int(n1), Expr::Int(n2)) => n1.partial_cmp(n2),
			(Expr::Int(n1), Expr::Float(_)) => n1.partial_cmp(&other.to_int().unwrap()),
			(Expr::Float(n1), Expr::Float(n2)) => n1.partial_cmp(n2),
			(Expr::Float(n1), Expr::Int(_)) => n1.partial_cmp(&other.to_float().unwrap()),
			(Expr::Str(s1), s2) => s1.partial_cmp(&s2.to_string()),
			(Expr::Bool(b1), b2) => b1.partial_cmp(&b2.to_bool()),
			_ => None
		}
	}
}

fn cast_int_for_operation(e: Expr, operation: &str) -> Result<i64, OsmiaError> {
	match e.to_int() {
		Ok(i) => Ok(i),
		Err(e) => Err(format!(
			"Cannot execute operation {} on non-integer value: {}",
			operation, e
		))
	}
}

/// BitAnd
impl BitAnd for Expr {
	type Output = Result<Expr, OsmiaError>;

	/// ```rust
	/// use osmia::Osmia;
	///
	/// let mut osmia = Osmia::default();
	/// assert_eq!(osmia.run_code("{{ 1 & 2 }}").unwrap(), "0");
	/// assert_eq!(osmia.run_code("{{ 1.2 & 2.1 }}").unwrap(), "0");
	/// assert_eq!(osmia.run_code("{{ 1.2 & 2 }}").unwrap(), "0");
	/// assert_eq!(osmia.run_code("{{ 1 & 2.1 }}").unwrap(), "0");
	/// assert_eq!(osmia.run_code("{{ 1.2 & 2.1 }}").unwrap(), "0");
	/// assert_eq!(osmia.run_code("{{ 1 & 2 }}").unwrap(), "0");
	/// ```
	fn bitand(self, rhs: Expr) -> Self::Output {
		const OPERATION: &str = "bitand (&)";
		Ok(Expr::Int(
			cast_int_for_operation(self, OPERATION)? &
			cast_int_for_operation(rhs, OPERATION)?
		))
	}
}

/// BitOr
impl BitOr for Expr {
	type Output = Result<Expr, OsmiaError>;

	/// ```rust
	/// use osmia::Osmia;
	///
	/// let mut osmia = Osmia::default();
	/// assert_eq!(osmia.run_code("{{ 7 | 4 }}").unwrap(), "7");
	/// assert_eq!(osmia.run_code("{{ 1.2 | 2.1 }}").unwrap(), "3");
	/// assert_eq!(osmia.run_code("{{ 1.2 | 2 }}").unwrap(), "3");
	/// assert_eq!(osmia.run_code("{{ 1 | 2.1 }}").unwrap(), "3");
	/// assert_eq!(osmia.run_code("{{ 1.2 | 2.1 }}").unwrap(), "3");
	/// assert_eq!(osmia.run_code("{{ 1 | 2 }}").unwrap(), "3");
	/// ```
	fn bitor(self, rhs: Expr) -> Self::Output {
		const OPERATION: &str = "bitor (|)";
		Ok(Expr::Int(
			cast_int_for_operation(self, OPERATION)? |
			cast_int_for_operation(rhs, OPERATION)?
		))
	}
}

/// BitXor
impl BitXor for Expr {
	type Output = Result<Expr, OsmiaError>;

	/// ```rust
	/// use osmia::Osmia;
	///
	/// let mut osmia = Osmia::default();
	/// assert_eq!(osmia.run_code("{{ 7 ^ 4 }}").unwrap(), "3");
	/// assert_eq!(osmia.run_code("{{ 1.2 ^ 2.1 }}").unwrap(), "3");
	/// assert_eq!(osmia.run_code("{{ 1.2 ^ 2 }}").unwrap(), "3");
	/// assert_eq!(osmia.run_code("{{ 1 ^ 2.1 }}").unwrap(), "3");
	/// assert_eq!(osmia.run_code("{{ 1.2 ^ 2.1 }}").unwrap(), "3");
	/// assert_eq!(osmia.run_code("{{ 1 ^ 2 }}").unwrap(), "3");
	/// ```
	fn bitxor(self, rhs: Expr) -> Self::Output {
		const OPERATION: &str = "bitxor (^)";
		Ok(Expr::Int(
			cast_int_for_operation(self, OPERATION)? ^
			cast_int_for_operation(rhs, OPERATION)?
		))
	}
}

/// Shl
impl Shl for Expr {
	type Output = Result<Expr, OsmiaError>;

	/// ```rust
	/// use osmia::Osmia;
	///
	/// let mut osmia = Osmia::default();
	/// assert_eq!(osmia.run_code("{{ 7 << 4 }}").unwrap(), "112");
	/// assert_eq!(osmia.run_code("{{ 1.2 << 2.1 }}").unwrap(), "4");
	/// assert_eq!(osmia.run_code("{{ 1.2 << 2 }}").unwrap(), "4");
	/// assert_eq!(osmia.run_code("{{ 1 << 2.1 }}").unwrap(), "4");
	/// assert_eq!(osmia.run_code("{{ 1.2 << 2.1 }}").unwrap(), "4");
	/// assert_eq!(osmia.run_code("{{ 1 << 2 }}").unwrap(), "4");
	/// assert_eq!(osmia.run_code("{{ 1 << 7 }}").unwrap(), "128");
	/// assert_eq!(osmia.run_code("{{ 1 << 63 }}").unwrap(), "-9223372036854775808");
	/// ```
	fn shl(self, rhs: Expr) -> Self::Output {
		const OPERATION: &str = "shl (<<)";
		Ok(Expr::Int(
			cast_int_for_operation(self, OPERATION)? <<
			cast_int_for_operation(rhs, OPERATION)?
		))
	}
}

/// Shr
impl Shr for Expr {
	type Output = Result<Expr, OsmiaError>;

	/// ```rust
	/// use osmia::Osmia;
	///
	/// let mut osmia = Osmia::default();
	/// assert_eq!(osmia.run_code("{{ 7 >> 4 }}").unwrap(), "0");
	/// assert_eq!(osmia.run_code("{{ 1.2 >> 2.1 }}").unwrap(), "0");
	/// assert_eq!(osmia.run_code("{{ 1.2 >> 2 }}").unwrap(), "0");
	/// assert_eq!(osmia.run_code("{{ 1 >> 2.1 }}").unwrap(), "0");
	/// assert_eq!(osmia.run_code("{{ 1.2 >> 2.1 }}").unwrap(), "0");
	/// assert_eq!(osmia.run_code("{{ 1 >> 2 }}").unwrap(), "0");
	/// assert_eq!(osmia.run_code("{{ 128 >> 7 }}").unwrap(), "1");
	/// assert_eq!(osmia.run_code("{{ 1024 >> 10 }}").unwrap(), "1");
	/// ```
	fn shr(self, rhs: Expr) -> Self::Output {
		const OPERATION: &str = "shr (>>)";
		Ok(Expr::Int(
			cast_int_for_operation(self, OPERATION)? >>
			cast_int_for_operation(rhs, OPERATION)?
		))
	}
}
