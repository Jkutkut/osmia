/// Macro that allows to generate easily a custom Debug implementation
/// for an enum.
///
/// `write!` is used under the hood, allowing custom formatting.
///
/// ## Examples
/// ```rust
/// use osmia::impl_debug;
///
/// enum MyEnum {
///   A,
///   B,
///   SomethingCustom(String),
///   SomethingCustomComplex(String, u32),
/// }
///
/// impl_debug!(
///   MyEnum,
///   (
///     A <=> "a",
///     SomethingCustom(x) <=> "SomethingCustom({x:?})",
///     SomethingCustomComplex(x, y) <=> "SomethingCustomComplex({y}, {x:?})",
///     B <=> "b", // The enum options order can be changed here
///   )
/// );
///
/// let tests = vec![
///   (MyEnum::A, "a"),
///   (MyEnum::B, "b"),
///   (
///       MyEnum::SomethingCustom("test".to_string()),
///       r#"SomethingCustom("test")"#
///   ),
///   (
///       MyEnum::SomethingCustomComplex("test".to_string(), 1),
///       r#"SomethingCustomComplex(1, "test")"#
///   )
/// ];
/// for (token, expected) in tests {
///   println!("Testing: {}", expected);
///   let token_str = format!("{:?}", token);
///   assert_eq!(token_str, expected);
/// }
/// ```
#[macro_export]
macro_rules! impl_debug {
	(
		$enum_name:ident,
		(
			$(
				$variant:ident$(( $($var:ident),* ))? <=> $value:expr $(,)?
			),*
		)
	) => {
		impl std::fmt::Debug for $enum_name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				match self {
					$(
						Self::$variant$(( $($var),* ))? => write!(f, $value),
					)*
				}
			}
		}
	};
}
