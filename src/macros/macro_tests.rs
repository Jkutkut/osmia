/// This macro simplifies running multiple tests with less boilerplate code.
///
/// # Examples
/// ```rust
/// use osmia::macro_tests;
///
/// #[test]
/// fn my_test_function(arg1: i32, arg2: i32) {
///     assert_eq!(arg1, arg2);
/// }
///
/// macro_tests!(my_test_function,
///     (test_1, 1, 1),
///     (test_2, 1 + 1 - 2 + 2, 2)
/// );
/// ```
///
/// The previous code is equivalent to the following:
/// ```rust
/// #[test]
/// fn my_test_function(arg1: i32, arg2: i32) {
///     assert_eq!(arg1, arg2);
/// }
///
/// #[test]
/// fn test_1() {
///     my_test_function(1, 1);
/// }
///
/// #[test]
/// fn test_2() {
///     my_test_function(1 + 1 - 2 + 2, 2);
/// }
/// ```
#[macro_export]
macro_rules! macro_tests {
	(
		$ft:ident,
		$(($test_name:ident, $($ex:expr),*)),*
	) => {
		$(
			#[test]
			fn $test_name() {
				$ft($($ex),*);
			}
		)*
	}
}
