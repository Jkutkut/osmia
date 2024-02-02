#[derive(Debug, PartialEq)]
pub enum VariableKey<'a> {
	Key(&'a str),
	Index(usize),
}
