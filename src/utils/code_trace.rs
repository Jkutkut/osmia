pub fn code_trace(
	text: &str, idx: usize,
	msg: &str
) -> String {
	let start_offset = match text[..idx].rfind('\n') {
		Some(idx) => idx + 1,
		None => 0
	};
	let end_offset = text[idx..].find('\n').unwrap_or(text.len() - idx) + idx;
	let code_chunk = &text[start_offset..end_offset];
	format!(
		"{}\n{}\n{}^",
		msg,
		code_chunk,
		" ".repeat(idx - start_offset - 1)
	)
}
