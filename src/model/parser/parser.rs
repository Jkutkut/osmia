pub trait Parser<I, T, E> {
	fn parse(&self, code: I) -> Result<T, E>;
}
