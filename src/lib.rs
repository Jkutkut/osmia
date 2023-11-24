#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::fmt::Display;

type OsmiaContext<'a, T> = &'a HashMap<&'a str, T>;
type OsmiaResult = Result<Option<(String, usize)>, String>;
type FtStractBlock<T> = dyn Fn(&str, OsmiaContext<T>) -> OsmiaResult;

struct RenderSyntax<'a, T> {
	simple_blocks: Vec<&'a FtStractBlock<T>>,
	//complex_blocks: Vec<()>, // TODO
}

impl<'a, T> RenderSyntax<'a, T> {
	pub fn new() -> Self {
		return RenderSyntax {
			simple_blocks: Vec::new(),
			// complex_blocks: Vec::new()
		};
	}

	pub fn add_simple_block(
		&mut self,
		render_function: &'a FtStractBlock<T>
	) {
		self.simple_blocks.push(render_function);
	}

	pub fn render(
		self,
		template: &str,
		ctx: OsmiaContext<T>
	) -> Result<String, String>
	where
		T: Display
	{
		let mut result = String::new();
		let mut i = 0;
		let mut last_copy = 0;
		while i < template.len() {
			let mut found = false;
			for render_function in &self.simple_blocks {
				match render_function(&template[i..], ctx) {
					Ok(None) => {},
					Ok(Some((block, block_size))) => {
						result.push_str(&template[last_copy..i]);
						result.push_str(block.as_str());
						i += block_size;
						last_copy = i;
						found = true;
						break;
					},
					Err(err) => return Err(err)
				}
			}
			if !found {
				i += 1;
			}
		}
		result.push_str(&template[last_copy..]);
		Ok(result)
	}
}

pub fn render<T>(
	template: &str,
	ctx: OsmiaContext<T>
) -> Result<String, String>
where
	T: Display
{
	let mut syntax: RenderSyntax<'_, T> = RenderSyntax::new();
	syntax.add_simple_block(&|code, _ctx| {
		let result = detect_simple_block("{{{", "}}}", code);
		if let Ok(Some((block, block_size))) = &result {
			let block = "{{".to_owned() + block.as_str() + "}}";
			return Ok(Some((block, *block_size)));
		}
		result
	});
	syntax.add_simple_block(&|code, ctx| {
		let result = detect_simple_block("{{", "}}", code);
		if let Ok(Some((block, block_size))) = &result {
			let value = match ctx.get(block.as_str()) {
				Some(value) => value.to_string(),
				None => return Err(format!("Key not found: {}", block))
			};
			return Ok(Some((value, *block_size)));
		}
		result
	});
	
	return syntax.render(template, ctx);
}

fn detect_simple_block(
	start: &str,
	end: &str,
	code: &str
) -> OsmiaResult {
	if !code.starts_with(start) {
		return Ok(None);
	}
	let end_index = match code[start.len()..].find(end) {
		Some(end_index) => end_index,
		None => return Err(format!("Missing closing block: {}", start))
	};
	let block = &code[start.len()..end_index + start.len()];
	Ok(Some((block.to_owned(), start.len() + end_index + end.len())))
}

