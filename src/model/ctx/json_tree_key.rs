#[derive(Debug, PartialEq, Clone)]
pub enum JsonTreeKey<K: Clone> {
	Index(usize),
	Key(K)
}

impl JsonTreeKey<String> {
	pub fn from(k: &str) -> Vec<Self> {
		// TODO do properly
		k.split(".").map(|s| JsonTreeKey::Key(s.into())).collect()
	}
}

impl std::fmt::Display for JsonTreeKey<String> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsonTreeKey::Index(i) => write!(f, "{}", i),
			JsonTreeKey::Key(k) => write!(f, "{}", k),
		}
	}
}
