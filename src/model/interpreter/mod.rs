mod exit_status;
mod interpreter_value;

pub use exit_status::ExitStatus;
pub use interpreter_value::InterpreterValue;

pub type InterpreterResult = Result<(ExitStatus, InterpreterValue), String>;
