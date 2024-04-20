#[derive(Debug, PartialEq, Clone)]
pub enum VariableKey {
	Key(String),
	Index(usize),
}
