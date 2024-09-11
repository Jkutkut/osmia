pub trait Interpreter<I, T, E> {
	fn interpret(&self, code: I) -> Result<T, E>;
}
