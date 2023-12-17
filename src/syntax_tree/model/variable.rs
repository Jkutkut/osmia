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

	pub fn from_str(raw: &'a str) -> Option<Self> {
		{
			println!("\nfrom_str: {}", raw);
		}
		if let Some(fist_char) = raw.chars().nth(0) {
			if !Self::is_valid_key_start(fist_char) {
				return None;
			}
		}
		let mut keys = LinkedList::new();
		let mut i: usize = 0;
		while i < raw.len() {
			let current_char = raw.chars().nth(i).unwrap(); // TODO
			{
				println!("current_char: {}", current_char);
			}
			if current_char == '[' {
				if i == 0 {
					return None;
				}
				{
					println!("  i: {} -- raw.len(): {}", i, raw.len());
					println!("  current: {:?}", raw[i..].to_string());
				}
				let end: usize = raw[i..].find(']')?;
				{
					println!("  end: {}", end);
				}
				match Self::get_key(raw, i + 1, i + end, true) {
					Some(VariableKey::Index(index)) => {
						{
							println!("index: {}", index);
						}
						keys.push_back(VariableKey::Index(index));
						i += end + 1;
					},
					_ => return None
				};
				{
					println!("arr key: {:?}", &keys.back());
					println!("  current_char: {:?}", raw.chars().nth(i));
					println!("  i: {} -- raw.len(): {}", i, raw.len());
				}
				if let Some(next_char) = raw.chars().nth(i) {
					if next_char == '.' {
						i += 1;
					}
				}
			}
			else {
				let start = i;
				while i < raw.len() && Self::is_valid_key_char(raw.chars().nth(i).unwrap()) {
					i += 1;
				}
				let key = Self::get_key(raw, start, i, false)?;
				{
					println!("key: {:?}", &key);
					println!("  current_char: {:?}", raw.chars().nth(i));
					println!("  i: {} -- raw.len(): {}", i, raw.len());
				}
				keys.push_back(key);
			}
			if let Some(current_char) = raw.chars().nth(i) {
				if current_char == '.' {
					i += 1;
				}
			}
		}
		if keys.is_empty() {
			return None;
		}
		let last_char = raw.chars().last().unwrap();
		if last_char == '.' || last_char == '[' {
			if !Self::is_valid_key_start(last_char) {
				return None;
			}
		}
		Some(Self::new(raw, keys))
	}

	fn is_valid_key_char(c: char) -> bool {
		c.is_alphanumeric() || c == '_'
	}

	fn is_valid_key_start(c: char) -> bool {
		c.is_alphabetic() || c == '_'
	}

	fn get_key(value: &str, start: usize, end: usize, validate: bool) -> Option<VariableKey> {
		if start >= end {
			return None;
		}
		let result = value[start..end].trim();
		{
			println!("  -> get_key: {}", result);

		}
		if result.is_empty() {
			return None;
		}
		if let Ok(index) = result.parse::<usize>() {
			return Some(VariableKey::Index(index));
		}
		// Validate key
		if validate {
			for c in result.chars() {
				if !Self::is_valid_key_char(c) {
					return None;
				}
			}
		}
		Some(VariableKey::Key(result))
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
