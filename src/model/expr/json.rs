use super::*;
use std::collections::HashMap;
use crate::OsmiaError;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone)]
pub struct Array {
	arr: Vec<Expr>,
}

impl Array {
	pub fn new(arr: Vec<Expr>) -> Self {
		Self { arr }
	}

	pub fn push(&mut self, expr: Expr) {
		self.arr.push(expr)
	}

	pub fn len(&self) -> usize {
		self.arr.len()
	}

	pub fn iter(&self) -> std::slice::Iter<'_, Expr> {
		self.arr.iter()
	}

	pub fn sort(&self) -> Result<Self, OsmiaError> {
		Ok(self.sort_by(|a, b| match a.partial_cmp(b) {
			Some(o) => o,
			None => Ordering::Equal
		}))
	}

	pub fn sort_by(&self, func: impl Fn(&Expr, &Expr) -> Ordering) -> Self {
		let mut sorted = self.arr.clone();
		sorted.sort_by(func);
		Self::new(sorted)
	}
}

impl From<Vec<Expr>> for Array {
	fn from(arr: Vec<Expr>) -> Self {
		Self::new(arr)
	}
}

impl Into<Vec<Expr>> for &Array {
	fn into(self) -> Vec<Expr> {
		self.arr.clone()
	}
}

impl Display for Array {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(
			f, "[{}]",
			self.arr.iter()
				.map(|e| e.print_as_json())
				.collect::<Vec<_>>()
				.join(", ")
		)
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
	Code(CodeObject),
	Hash(HashObject),
}

impl Object {
	pub fn new(obj: Vec<(Expr, Expr)>) -> Self {
		Self::new_code(obj)
	}

	pub fn new_code(obj: Vec<(Expr, Expr)>) -> Self {
		Self::Code(CodeObject::new(obj))
	}

	pub fn new_hash(obj: Vec<(Expr, Expr)>) -> Result<Self, String> {
		Ok(Self::Hash(HashObject::new(obj)?))
	}

	pub fn push(&mut self, e: (Expr, Expr)) -> Result<(), String> {
		match self {
			Object::Code(c) => c.push(e),
			Object::Hash(h) => h.push(e)?,
		}
		Ok(())
	}

	pub fn contains_key(&self, key: &Expr) -> bool {
		match self {
			Object::Code(c) => c.contains_key(key),
			Object::Hash(h) => h.contains_key(key),
		}
	}

	pub fn len(&self) -> usize {
		match self {
			Object::Code(c) => c.len(),
			Object::Hash(h) => h.len(),
		}
	}

	pub fn entries(&self) -> Vec<(Expr, Expr)> {
		match self {
			Object::Code(c) => c.entries(),
			Object::Hash(h) => h.entries(),
		}
	}

	pub fn keys(&self) -> Vec<Expr> {
		match self {
			Object::Code(c) => c.keys(),
			Object::Hash(h) => h.keys(),
		}
	}

	pub fn values(&self) -> Vec<Expr> {
		match self {
			Object::Code(c) => c.values(),
			Object::Hash(h) => h.values(),
		}
	}
}

impl From<Vec<(Expr, Expr)>> for Object {
	fn from(obj: Vec<(Expr, Expr)>) -> Self {
		Self::new_code(obj)
	}
}

impl Into<Vec<(Expr, Expr)>> for &Object {
	fn into(self) -> Vec<(Expr, Expr)> {
		match self {
			Object::Code(c) => c.entries(),
			Object::Hash(h) => h.entries(),
		}
	}
}

impl Display for Object {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		match self {
			Object::Code(c) => write!(f, "{}", c),
			Object::Hash(h) => write!(f, "{}", h),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct CodeObject {
	obj: Vec<(Expr, Expr)>,
}

impl CodeObject {
	pub fn new(obj: Vec<(Expr, Expr)>) -> Self {
		Self { obj }
	}

	pub fn push(&mut self, e: (Expr, Expr)) {
		self.obj.push(e)
	}

	pub fn contains_key(&self, key: &Expr) -> bool {
		self.obj.iter().any(|(k, _)| k == key)
	}

	pub fn len(&self) -> usize {
		self.obj.len()
	}

	pub fn entries(&self) -> Vec<(Expr, Expr)> {
		self.obj.clone()
	}

	pub fn keys(&self) -> Vec<Expr> {
		self.obj.iter().map(|(k, _)| k.clone()).collect()
	}

	pub fn values(&self) -> Vec<Expr> {
		self.obj.iter().map(|(_, v)| v.clone()).collect()
	}
}

impl Display for CodeObject {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(
			f, "{{{}}}",
			self.obj.iter()
				.map(|(k, v)| format!("\"{}\": {}", k.to_string(), v.print_as_json()))
				.collect::<Vec<String>>().join(", ")
		)
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct HashObject {
	obj: HashMap<String, Expr>,
}

impl HashObject {
	pub fn empty() -> Self {
		Self { obj: HashMap::new() }
	}

	pub fn new(obj: Vec<(Expr, Expr)>) -> Result<Self, String> {
		let mut new_obj = Self::empty();
		for entry in obj {
			new_obj.push(entry)?;
		}
		Ok(new_obj)
	}

	pub fn push(&mut self, e: (Expr, Expr)) -> Result<(), String> {
		let (key, value) = e;
		let key_string: String = match key {
			Expr::Str(s) => s,
			_ => return Err("Object key must be a string".to_string()),
		};
		self.obj.insert(key_string, value);
		Ok(())
	}

	pub fn contains_key(&self, key: &Expr) -> bool {
		match key {
			Expr::Str(k) => self.obj.contains_key(k),
			_ => false,
		}
	}

	pub fn len(&self) -> usize {
		self.obj.len()
	}

	pub fn entries(&self) -> Vec<(Expr, Expr)> {
		let mut entries = self.obj.iter()
			.map(|(k, v)| (Expr::Str(k.clone()), v.clone()))
			.collect::<Vec<_>>();
		entries.sort_by(|a, b| a.0.to_string().cmp(&b.0.to_string()));
		entries
	}

	pub fn keys(&self) -> Vec<Expr> {
		self.obj.iter().map(|(k, _)| Expr::Str(k.clone())).collect()
	}

	pub fn values(&self) -> Vec<Expr> {
		self.obj.iter().map(|(_, v)| v.clone()).collect()
	}
}

impl Display for HashObject {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(
			f, "{{{}}}",
			self.entries().iter()
				.map(|(k, v)| format!("\"{}\": {}", k, v.print_as_json()))
				.collect::<Vec<String>>().join(", ")
		)
	}
}
