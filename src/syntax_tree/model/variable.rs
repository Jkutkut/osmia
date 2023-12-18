use std::collections::LinkedList;

#[derive(Debug, PartialEq)]
pub struct Variable<'a> {
	raw: &'a str,
	keys: LinkedList<VariableKey<'a>>,
}

impl<'a> Variable<'a> {
	pub fn new(raw: &'a str, lst: LinkedList<VariableKey<'a>>) -> Self {
		Self {
			raw,
			keys: lst
		}
	}

	fn get_key_str(value: &'a str, start: usize, end: usize) -> Option<&'a str> {
		if start >= end {
			return None;
		}
		match value[start..end].trim() {
			"" => None,
			result => Some(result)
		}
	}

	fn is_valid_key_char(c: char) -> bool {
		c.is_alphanumeric() || c == '_'
	}

	fn is_valid_key_start(c: char) -> bool {
		c.is_alphabetic() || c == '_'
	}

	fn is_valid_key(value: &'a str) -> bool {
		if value.is_empty() {
			return false;
		}
		let mut chars = value.chars();
		if !Self::is_valid_key_start(chars.next().unwrap()) {
			return false;
		}
		for c in chars {
			if !Self::is_valid_key_char(c) {
				return false;
			}
		}
		true
	}

	fn get_as_key(value: &'a str, start: usize, end: usize) -> Option<VariableKey<'a>> {
		let key = Self::get_key_str(value, start, end)?;
		match Self::is_valid_key(key) {
			true => Some(VariableKey::Key(key)),
			false => None
		}
	}

	fn get_as_index(value: &'a str, start: usize, end: usize) -> Option<VariableKey<'a>> {
		let key = Self::get_key_str(value, start, end)?;
		match key.parse::<usize>() {
			Ok(index) => Some(VariableKey::Index(index)),
			Err(_) => None
		}
	}

	pub fn from_str(raw: &'a str) -> Option<Self> {
		let mut keys = LinkedList::new();
		let mut can_be_index = false;
		let mut i: usize = 0;
		if let Some(last_char) = raw.chars().last() {
			if !Self::is_valid_key_start(last_char) && last_char != ']' {
				return None;
			}
		}
		while i < raw.len() {
			let current_char = raw.chars().nth(i).unwrap(); // TODO
			if current_char == '[' {
				if !can_be_index {
					return None;
				}
				let end: usize = raw[i..].find(']')?;
				let key = Self::get_as_index(raw, i + 1, i + end)?;
				keys.push_back(key);
				i += end + 1;
				match raw.chars().nth(i) {
					None => (),
					Some('.') => {
						can_be_index = false;
						i += 1;
					},
					Some('[') => {
						can_be_index = true;
						i -= 1;
					},
					_ => return None
				}
			}
			else if Self::is_valid_key_start(current_char) {
				let start = i;
				while i < raw.len() && Self::is_valid_key_char(raw.chars().nth(i).unwrap()) {
					i += 1;
				}
				let key = Self::get_as_key(raw, start, i)?;
				keys.push_back(key);
				match raw.chars().nth(i) {
					None => (),
					Some('[') => {
						can_be_index = true;
						i -= 1;
					},
					Some('.') => can_be_index = false,
					_ => return None
				}
			}
			else {
				return None;
			}
			i += 1;
		}
		match keys.is_empty() {
			true => None,
			false => Some(Self::new(raw, keys))
		}
	}
}

impl std::fmt::Display for Variable<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.raw)
	}
}

#[derive(Debug, PartialEq)]
pub enum VariableKey<'a> {
	Key(&'a str),
	Index(usize),
}
