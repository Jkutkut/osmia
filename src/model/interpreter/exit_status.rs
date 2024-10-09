/// Describes the possible exit status / reasons for the interpreter to end executing.
#[derive(Debug, PartialEq)]
pub enum ExitStatus {
	/// The execution was successful and the code executed fully.
	Okay,
}
