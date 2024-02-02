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
