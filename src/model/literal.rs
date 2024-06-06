use std::cmp::{
	PartialEq, PartialOrd,
};
use std::ops::{
	Add, Sub, Mul, Div,
	Rem, Neg, Not,
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Literal {
	Float(f64),
	Int(i64),
	Str(String),
	Bool(bool),
	Null
}

impl Literal {
	pub fn from_str(s: &str) -> Option<Literal> {
		if s == "null" {
			return Some(Literal::Null);
		}
		if let Ok(i) = s.parse::<i64>() {
			return Some(Literal::Int(i));
		}
		if let Ok(f) = s.parse::<f64>() {
			return Some(Literal::Float(f));
		}
		if s == "true" {
			return Some(Literal::Bool(true));
		}
		if s == "false" {
			return Some(Literal::Bool(false));
		}
		if s.starts_with('"') && s.ends_with('"') {
			return Some(Literal::Str(s[1..s.len()-1].to_string()));
		}
		None
	}

	pub fn is_bool(&self) -> bool {
		match self {
			Literal::Bool(_) => true,
			_ => false
		}
	}

	pub fn is_numeric(&self) -> bool {
		match self {
			Literal::Float(_) => true,
			Literal::Int(_) => true,
			_ => false
		}
	}

	pub fn is_int(&self) -> bool {
		match self {
			Literal::Int(_) => true,
			_ => false
		}
	}

	pub fn is_float(&self) -> bool {
		match self {
			Literal::Float(_) => true,
			_ => false
		}
	}

	pub fn is_str(&self) -> bool {
		match self {
			Literal::Str(_) => true,
			_ => false
		}
	}

	pub fn as_bool(&self) -> bool {
		match self {
			Literal::Float(f) => *f != 0.0,
			Literal::Int(i) => *i != 0,
			Literal::Str(s) => !s.is_empty(),
			Literal::Bool(b) => *b,
			Literal::Null => false
		}
	}

	pub fn as_float(&self) -> Result<f64, String> {
		match self {
			Literal::Float(f) => Ok(*f),
			Literal::Int(i) => Ok(*i as f64),
			_ => Err(format!("Cannot convert {} to float", self))
		}
	}

	pub fn as_int(&self) -> Result<i64, String> {
		match self {
			Literal::Float(f) => Ok(*f as i64),
			Literal::Int(i) => Ok(*i),
			_ => Err(format!("Cannot convert {} to int", self))
		}
	}

	pub fn as_str(&self) -> String {
		match self {
			Literal::Str(s) => s.clone(),
			Literal::Int(i) => i.to_string(),
			Literal::Float(f) => f.to_string(),
			Literal::Bool(b) => b.to_string(),
			Literal::Null => "null".to_string()
		}
	}
}

impl std::fmt::Display for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Literal::Float(fl) => write!(f, "{}", fl),
			Literal::Int(i) => write!(f, "{}", i),
			Literal::Str(s) => write!(f, "{}", s),
			Literal::Bool(b) => write!(f, "{}", b),
			Literal::Null => write!(f, "null")
		}
	}
}

// Operations

impl Add for Literal {
	type Output = Result<Literal, String>;

	fn add(self, rhs: Literal) -> Self::Output {
		match (self, &rhs) {
			(Literal::Str(s1), s2) => Ok(Literal::Str(format!("{}{}", s1, s2.as_str()))),
			(t, Literal::Str(s)) => Ok(Literal::Str(format!("{}{}", t.as_str(), s))),
			(Literal::Bool(b1), b2) => Ok(Literal::Bool(b1 || b2.as_bool())),
			(b1, Literal::Bool(b2)) => Ok(Literal::Bool(b1.as_bool() || *b2)),
			(Literal::Float(n1), n2) => Ok(Literal::Float(n1 + n2.as_float()?)),
			(Literal::Int(n1), n2) => Ok(Literal::Int(
				n1.checked_add(n2.as_int()?)
				.ok_or("Overflow in addition".to_string())?
			)),
			(Literal::Null, Literal::Null) => Ok(Literal::Null),
			(s, rhs) => Err(format!("Cannot add {} and {}", s, rhs))
		}
	}
}


impl Sub for Literal {
	type Output = Result<Literal, String>;

	fn sub(self, rhs: Literal) -> Self::Output {
		match (self, &rhs) {
			(Literal::Bool(b1), b2) => Ok(Literal::Bool(b1 ^ b2.as_bool())),
			(b1, Literal::Bool(b2)) => Ok(Literal::Bool(!b1.as_bool() ^ *b2)),
			(Literal::Float(n1), n2) => Ok(Literal::Float(n1 - n2.as_float()?)),
			(Literal::Int(n1), n2) => Ok(Literal::Int(
				n1.checked_sub(n2.as_int()?)
				.ok_or("Overflow in subtraction".to_string())?
			)),
			(Literal::Str(_), _) => Err(format!("Cannot subtract {} from string", rhs)),
			(s, rhs) => Err(format!("Cannot subtract {} from {}", s, rhs))
		}
	}
}

impl Mul for Literal {
	type Output = Result<Literal, String>;

	fn mul(self, rhs: Literal) -> Self::Output {
		match (self, &rhs) {
			(Literal::Float(n1), n2) => Ok(Literal::Float(n1 * n2.as_float()?)),
			(Literal::Int(n1), n2) => Ok(Literal::Int(
				n1.checked_mul(n2.as_int()?)
				.ok_or("Overflow in multiplication".to_string())?
			)),
			(Literal::Str(s), Literal::Int(n)) => match *n < 0 {
				false => Err(format!("Cannot repeat string {} {} times", s, n)),
				true => Ok(Literal::Str(s.repeat(n.abs() as usize)))
			},
			(s, rhs) => Err(format!("Cannot multiply {} by {}", s, rhs))
		}
	}
}

impl Div for Literal {
	type Output = Result<Literal, String>;

	fn div(self, rhs: Literal) -> Self::Output {
		match (self, rhs) {
			(Literal::Float(n1), n2) => Ok(Literal::Float(n1 / n2.as_float()?)),
			(n1, Literal::Float(n2)) => Ok(Literal::Float(n1.as_float()? / n2)),
			(Literal::Int(n1), n2) => Ok(Literal::Int(
				n1.checked_div(n2.as_int()?)
				.ok_or("Overflow in division".to_string())?
			)),
			(s, rhs) => Err(format!("Cannot divide {} by {}", s, rhs))
		}
	}
}

impl Rem for Literal {
	type Output = Result<Literal, String>;

	fn rem(self, rhs: Literal) -> Self::Output {
		match (self, &rhs) {
			(Literal::Float(n1), n2) => Ok(Literal::Float(n1 % n2.as_float()?)),
			(Literal::Int(n1), n2) => Ok(Literal::Int(n1 % n2.as_int()?)),
			(Literal::Str(_), _) => Err(format!("Cannot modulo string by {}", rhs)),
			(Literal::Bool(_), _) => Err(format!("Cannot modulo bool by {}", rhs)),
			(s, rhs) => Err(format!("Cannot modulo {} by {}", s, rhs))
		}
	}
}

// Unary
impl Neg for Literal {
	type Output = Result<Literal, String>;

	fn neg(self) -> Self::Output {
		match self {
			Literal::Float(f) => Ok(Literal::Float(-f)),
			Literal::Int(i) => Ok(Literal::Int(-i)),
			_ => Err(format!("Cannot negate {}", self))
		}
	}
}

impl Not for Literal {
	type Output = Result<Literal, String>;

	fn not(self) -> Self::Output {
		match self {
			Literal::Int(_) => Ok(Literal::Bool(!self.as_bool())),
			Literal::Float(_) => Ok(Literal::Bool(!self.as_bool())),
			Literal::Bool(b) => Ok(Literal::Bool(!b)),
			_ => Err(format!("Cannot logic negate {}", self))
		}
	}
}
