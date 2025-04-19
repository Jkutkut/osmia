/// Utility designed to print a specific line of code,
/// highlighting the given character position.
///
/// * `text` - The code with the specific line. It can
///		contain lines before and after the line.
///	* `idx` - The index of the character position.
///	* `msg` - A message to be printed before the code.
pub fn code_trace(
	text: &str, idx: usize,
	msg: &str
) -> String {
	let mut msg: String = msg.into();
	if !msg.is_empty() {
		msg.push('\n');
	}
	let start_offset = match text[..idx].rfind('\n') {
		Some(idx) => idx + 1,
		None => 0
	};
	let end_offset = text[idx..].find('\n').unwrap_or(text.len() - idx) + idx;
	let code_chunk = &text[start_offset..end_offset];

	let mut trace_arrow: String = "".to_string();
	if !code_chunk.is_empty() && idx >= start_offset {
		trace_arrow.push('\n');
		trace_arrow.push_str(&" ".repeat(idx - start_offset));
		trace_arrow.push('^');
	}

	format!("{}{}{}", msg, code_chunk, trace_arrow)
}
