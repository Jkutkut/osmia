#[derive(Debug, PartialEq, Clone)]
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
