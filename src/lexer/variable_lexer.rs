use std::collections::LinkedList;
use crate::syntax_tree::model::VariableKey;

pub struct VariableLexer;

impl<'a> VariableLexer {
	pub fn lex(raw: &'a str) -> Option<LinkedList<VariableKey<'a>>> {
		VariableLexer::lex_str(raw)
	}
}

impl<'a> VariableLexer {
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

	/// Attempts to get a key as a string.
	/// The key is validated by the `is_valid_key` function.
	fn get_as_key(value: &'a str, start: usize, end: usize) -> Option<VariableKey<'a>> {
		let key = Self::get_key_str(value, start, end)?;
		match Self::is_valid_key(key) {
			true => Some(VariableKey::Key(key)),
			false => None
		}
	}

	/// Attempts to get a key as a index.
	///
	/// ## Example:
	/// If the variable is `foo[2]`, the only index key is `2`.
	fn get_as_index(value: &'a str, start: usize, end: usize) -> Option<VariableKey<'a>> {
		let key = Self::get_key_str(value, start, end)?;
		match key.parse::<usize>() {
			Ok(index) => Some(VariableKey::Index(index)),
			Err(_) => None
		}
	}

	/// Lexes a variable.
	/// During lexing, the variable is validated.
	fn lex_str(raw: &'a str) -> Option<LinkedList<VariableKey<'a>>> {
		let mut keys = LinkedList::new();
		let mut should_be_index = false;
		let mut i: usize = 0;
		let mut current_char: char;
		let chars = raw.chars().collect::<Vec<char>>();
		while i < raw.len() {
			current_char = chars[i];
			if current_char == '[' {
				if !should_be_index {
					return None;
				}
				let end: usize = raw[i..].find(']')?;
				keys.push_back(
					Self::get_as_index(raw, i + 1, i + end)?
				);
				i += end + 1;
			}
			else if Self::is_valid_key_start(current_char) {
				let start = i;
				while i < raw.len() && Self::is_valid_key_char(chars[i]) {
					i += 1;
				}
				keys.push_back(
					Self::get_as_key(raw, start, i)?
				);
			}
			else {
				return None;
			}
			match chars.get(i) {
				None => (),
				Some('.') => should_be_index = false,
				Some('[') => {
					should_be_index = true;
					i -= 1;
				},
				_ => return None
			}
			i += 1;
		}
		if let Some(last_char) = chars.last() {
			if !Self::is_valid_key_start(*last_char) && last_char != &']' {
				return None;
			}
		}
		match keys.is_empty() {
			true => None,
			false => Some(keys)
		}
	}
}
