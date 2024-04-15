#[derive(Debug, PartialEq, Clone)]
pub enum VariableKey<'a> {
	Key(&'a str),
	Index(usize),
}
