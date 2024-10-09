pub fn push_op_string(s1: &mut String, s2: Option<String>) {
	if let Some(s) = s2 {
		s1.push_str(&s);
	}
}
