pub trait Affirm {
	type Output;
	fn affirm(self) -> Self::Output;
}
