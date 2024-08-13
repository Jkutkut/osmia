#[derive(Debug, PartialEq)]
pub enum JsonTreeError<K> {
	AccessValue(K),
	KeyInArray,
	IndexInObject,
	ArrayOutOfBounds((usize, usize)),
	KeyNotFound(K),
}
