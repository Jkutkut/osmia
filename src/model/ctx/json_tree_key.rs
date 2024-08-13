#[derive(Debug, PartialEq, Clone)]
pub enum JsonTreeKey<K: Clone> {
	Index(usize),
	Key(K)
}
