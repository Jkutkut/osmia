/// Describes the possible exit status / reasons for the interpreter to end executing.
#[derive(Debug, PartialEq, Clone)]
pub enum ExitStatus {
	/// The execution was successful and the code executed fully.
	Okay,
	/// The execution ended with a break statement.
	Break,
	/// The execution ended with a continue statement.
	Continue,
}
