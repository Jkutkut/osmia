#[derive(Debug)]
pub enum InterpreterValue {
	String(String),
	Void
}

impl From<String> for InterpreterValue {
	fn from(s: String) -> Self {
		match s.is_empty() {
			true => Self::Void,
			false => Self::String(s)
		}
	}
}
