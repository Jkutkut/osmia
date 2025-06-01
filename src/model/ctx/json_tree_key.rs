use std::fmt::Display;
use crate::model::lexer::{
	OsmiaLexerScanner,
	Token
};

#[derive(Debug, PartialEq, Clone)]
pub enum JsonTreeKey<K: Clone + Display> {
	Index(usize),
	Key(K)
}

impl JsonTreeKey<String> {
	pub fn try_parse(k: &str) -> Result<Vec<Self>, String> {
		let var_iter = OsmiaLexerScanner::new(k).scan_stmt()?;
		if var_iter.len() == 0 {
			return Err("Empty".into());
		}
		let mut var_iter = var_iter.into_iter();
		let mut keys = Vec::new();
		let mut first = true;
		while let Some(token) = var_iter.next() {
			if first {
				match token {
					Token::Alpha(_) => (),
					Token::Null => (),
					_ => return Err("Invalid key".into())
				}
				first = false;
			}
			match token {
				Token::Alpha(s) => keys.push(JsonTreeKey::Key(s.into())),
				Token::Null => keys.push(JsonTreeKey::Key("null".into())),
				Token::Dot => {
					if let Some(Token::Alpha(a)) = var_iter.next() {
						keys.push(JsonTreeKey::Key(a.into()));
					}
					else {
						return Err("Invalid dot '.' in key".into());
					}
				},
				Token::ArrayStart => {
					match var_iter.next() {
						Some(Token::Number(n)) => {
							let n = match n.parse::<usize>() {
								Ok(n) => n,
								Err(_) => return Err("Expected positive integer".into()),
							};
							keys.push(JsonTreeKey::Index(n));
						},
						None => return Err("Unclosed array".into()),
						_ => return Err("Expected index number".into())
					}
					match var_iter.next() {
						Some(Token::ArrayEnd) => (),
						_ => return Err("Unclosed array".into())
					}
				},
				#[cfg(debug_assertions)]
				t => return Err(format!("Invalid token in key: {:?}", t).into()),
				#[cfg(not(debug_assertions))]
				t => return Err("Invalid token in key".into())
			}
		}
		Ok(keys)
	}
}

impl<K: Clone + Display> Display for JsonTreeKey<K> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsonTreeKey::Index(i) => write!(f, "{}", i),
			JsonTreeKey::Key(k) => write!(f, "{}", k),
		}
	}
}
