#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::fmt::Display;


pub fn render<T>(
	template: &str,
	ctx: &HashMap<&str, T>
) -> String
where
	T: Display
{
	let syntax = Vec::from([
		["${", "}"]
	]);
	builder::<T>(template, syntax, &|block| {
		// TODO handle invalid types
		ctx.get(block).unwrap().to_string()
	})
}

fn builder<T>(
	template: &str,
	syntax_blocks: Vec<[&str; 2]>,
	on_block: &dyn Fn(&str) -> String
) -> String
where
	T: Display
{
	let mut result = String::new();
	let mut i = 0;
	let mut last_copy = 0;
	

	// search for blocks
	while i < template.len() {
		let mut found = false;
		for block in &syntax_blocks {
			let block_start = block[0];
			let block_end = block[1];

			if template[i..].starts_with(block_start) {
				let block_start_index = i;
				i += block_start.len();
				let block_end_index = template[i..].find(block_end);
				if let Some(block_end_index) = block_end_index {
					let block_end_index = block_end_index + i;
					result.push_str(&template[last_copy..block_start_index]);
					let block = &template[
						block_start_index + block_start.len()..block_end_index
					];
					result.push_str(on_block(block).as_str());
					i = block_end_index + block_end.len();
					last_copy = i;
					found = true;
					break;
				}
			}
		}
		if !found {
			i += 1;
		}
	}
	result.push_str(&template[last_copy..]);
	result
}
