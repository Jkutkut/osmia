#[derive(Debug, PartialEq, Clone)]
pub enum VariableKey {
	Key(String),
	Index(usize),
}

impl std::fmt::Display for VariableKey {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			VariableKey::Key(k) => write!(f, "{}", k),
			VariableKey::Index(i) => write!(f, "{}", i),
		}
	}
}
