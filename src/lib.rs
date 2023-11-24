#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::fmt::Display;

// trait OsmiaBlock<T> {
// 	fn start() -> &'static str;
// 	fn end() -> &'static str;
// 	fn from_str(
// 		block: &str,
// 		ctx: HashMap<&str, T>
// 	) -> Option<Self> where Self: Sized, T: Display;
// 
// 	fn block_size(str: &str) -> Option<usize> {
// 		if !str.starts_with(Self::start()) {
// 			return None;
// 		}
// 		let start = Self::start().len();
// 		let end = str[start..].find(Self::end())?;
// 		Some(end)
// 	}
// 
// 	fn to_string(self) -> String;
// }
// 
// struct RawOsmiaBlock {
// 	value: String
// }
// 
// impl<T> OsmiaBlock<T> for RawOsmiaBlock {
// 	fn start() -> &'static str {
// 		"{{{"
// 	}
// 
// 	fn end() -> &'static str {
// 		"}}}"
// 	}
// 
// 	fn from_str(
// 		block: &str,
// 		_ctx: HashMap<&str, T>
// 	) -> Option<Self> {
// 		let block = block.trim();
// 		Some(RawOsmiaBlock {
// 			value: block.to_string()
// 		})
// 	}
// 
// 	fn to_string(self) -> String {
// 		self.value.clone()
// 	}
// }
// 
// struct ValueOsmiaBlock {
// 	value: String
// }
// 
// impl<T> OsmiaBlock<T> for ValueOsmiaBlock {
// 	fn start() -> &'static str {
// 		"{{"
// 	}
// 
// 	fn end() -> &'static str {
// 		"}}"
// 	}
// 
// 	fn from_str(
// 		block: &str,
// 		ctx: HashMap<&str, T>
// 	) -> Option<Self> where T: Display {
// 		let block = block.trim();
// 		let value = ctx.get(block)?.to_string();
// 		Some(ValueOsmiaBlock {
// 			value: value
// 		})
// 	}
// 
// 	fn to_string(self) -> String {
// 		self.value.clone()
// 	}
// }

/*
trait OsmiaBlockTrait {
	fn new(code: &str) -> Option<(Self, usize)> where Self: Sized;
}

struct LineOsmiaBlock<'a> {

}

impl OsmiaBlock for LineOsmiaBlock {
*/

type OsmiaContext<'a, T> = &'a HashMap<&'a str, T>;
type FtStractBlock<T> = dyn Fn(&str, OsmiaContext<T>) -> Result<Option<(String, usize)>, String>;

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
	syntax.add_simple_block(&|code, ctx| {
		let start = "{{";
		let end = "}}";
		if !code.starts_with(start) {
			return Ok(None);
		}
		let end_index = match code[start.len()..].find(end) {
			Some(end_index) => end_index,
			None => return Err(format!("Missing closing block: {}", start))
		};
		let block = &code[start.len()..end_index + start.len()];
		let value = match ctx.get(block) {
			Some(value) => value.to_string(),
			None => return Err(format!("Key not found: {}", block))
		};
		Ok(Some((value, start.len() + end_index + end.len())))
	});
	return syntax.render(template, ctx);
}


// pub fn render<T>(
// 	template: &str,
// 	ctx: &HashMap<&str, T>
// ) -> Result<String, String>
// where
// 	T: Display
// {
// 	let blocks = Vec::from([
// 		["{{{", "}}}"],
// 		["{{", "}}"]
// 	]);
// 	builder::<T>(template, blocks, &|block| {
// 		match ctx.get(block) {
// 			Some(value) => Ok(value.to_string()),
// 			None => Err(format!("Key not found: {}", block))
// 		}
// 	})
// }
// 
// fn builder<T>(
// 	template: &str,
// 	syntax_blocks: Vec<[&str; 2]>,
// 	on_block: &dyn Fn(&str) -> Result<String, String>
// ) -> Result<String, String>
// where
// 	T: Display
// {
// 	let mut result = String::new();
// 	let mut i = 0;
// 	let mut last_copy = 0;
// 	while i < template.len() {
// 		let mut found = false;
// 		for block in &syntax_blocks {
// 			let block_start = block[0];
// 			let block_end = block[1];
// 			if template[i..].starts_with(block_start) {
// 				let block_start_index = i;
// 				i += block_start.len();
// 				let block_end_index = template[i..].find(block_end);
// 				if let Some(block_end_index) = block_end_index {
// 					let block_end_index = block_end_index + i;
// 					result.push_str(&template[last_copy..block_start_index]);
// 					let block = &template[
// 						block_start_index + block_start.len()..block_end_index
// 					];
// 					let block = match on_block(block) {
// 						Ok(block) => block,
// 						Err(err) => return Err(err)
// 					};
// 					result.push_str(block.as_str());
// 					i = block_end_index + block_end.len();
// 					last_copy = i;
// 					found = true;
// 					break;
// 				}
// 			}
// 		}
// 		if !found {
// 			i += 1;
// 		}
// 	}
// 	result.push_str(&template[last_copy..]);
// 	Ok(result)
// }
