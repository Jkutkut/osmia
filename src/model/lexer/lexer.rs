pub trait Lexer<T, E> {
	fn lex(&self, code: &str) -> Result<T, E>;
}
