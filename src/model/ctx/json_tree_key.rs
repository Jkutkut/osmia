use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum JsonTreeKey<K: Clone + Display> {
	Index(usize),
	Key(K)
}

impl JsonTreeKey<String> {
	pub fn from(k: &str) -> Vec<Self> {
		k.split(".").map(|s| JsonTreeKey::Key(s.into())).collect()
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
