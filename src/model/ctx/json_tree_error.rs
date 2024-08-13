/// Collection of errors that can occur while traversing a JsonTree.
/// When the error contains any value, it represents the error location.
#[derive(Debug, PartialEq)]
pub enum JsonTreeError<K> {
	/// Attempted to access a value.
	/// 
	/// ## Examples:
	/// ```json
	/// {"foo": "bar"}
	/// ```
	/// ```txt
	/// foo.something_not_valid
	/// foo[1]
	/// ```
	AccessValue(K),

	/// Attempted access an array with an index that is out of bounds.
	/// By default, the first element is the index attempted while
	/// the second is the size of the array.
	///
	/// ## Examples:
	/// ```json
	/// {"foo": ["bar"]}
	/// ```
	/// ```txt
	/// foo[1]
	/// foo[123]
	/// ```
	ArrayOutOfBounds((usize, usize)),

	/// Attempted to access an object with an index.
	///
	/// ## Examples:
	/// ```json
	/// {"foo": {"bar": "baz"}}
	/// ```
	/// ```txt
	/// foo[1]
	/// foo.bar[0]
	/// ```
	IndexInObject,

	/// Attempted to access an array with a key.
	///
	/// ## Example:
	/// ```json
	/// {"foo": ["bar"]}
	/// ```
	/// ```txt
	/// foo.something_not_valid
	/// ```
	KeyInArray,

	/// Attempted to access an object with a key that does not exist.
	///
	/// ## Example:
	/// ```json
	/// {"foo": {"bar": "baz"}}
	/// ```
	/// ```txt
	/// foo.something_not_valid
	/// ```
	KeyNotFound(K),

	/// When no key has been provided.
	/// *Note*: In practice, this should never happen.
	///
	/// ## Example:
	/// Imagine we have:
	/// ```json
	/// {"foo": "bar"}
	/// ```
	/// And we attempt to assign a new value to the whole object.
	/// In that case, this error will be returned.
	NoKey,
}
