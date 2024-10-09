pub fn string_or_none(s: String) -> Option<String> {
	match s.is_empty() {
		false => Some(s),
		true => None,
	}
}
