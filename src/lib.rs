//! # Osmia
//! This library is a minimal template engine built in Rust.
//!
//! ## Features
//! * Simple syntax
//! * No dependencies
//! * Easy to use
//! * Customizable
//! * Fast
//!
//! ## Usage
//! ```rust
//! use osmia::{osmia_syntax, RenderSyntax, Json};
//!
//! let template = "Hello, {{name}}!";
//! let result = osmia_syntax().render(
//! 	"Hello, {{name}}!",
//! 	&serde_json::from_str::<Json>(r#"{ "name": "world" }"#).unwrap()
//! ).unwrap();
//! assert_eq!(result, "Hello, world!".to_string());
//! ```

#[cfg(test)]
mod tests;

/// Alias for the serde_json::Value type.
///
/// It's used as the context of the template.
pub type Json = serde_json::Value;

/// Internal type to handle the context of the template.
type JsonRef<'a> = &'a Json;

/// Internal type to handle the result of the render function.
///
/// If there's a block, it returns the rendered block 
/// as a String and the size used from the code (`(String, usize)`).
///
/// If there's no block, it returns `None`.
///
/// If there's an error, it returns the error as a `String`.
pub type OsmiaResult = Result<Option<(String, usize)>, String>;

/// Type of the functions used to detect simple blocks.
///
/// A simple block is a block without any logic, like a comment or a variable.
///
/// # Arguments
///
/// * `code` - The code to parse.
///
/// * `ctx` - The context of the template.
type FtSimpleBlock = dyn Fn(&str, JsonRef) -> OsmiaResult;

/// Type of the functions used to detect complex blocks.
///
/// A complex block is a block with logic, like a for loop.
///
/// # Arguments
///
/// * `code` - The code to parse.
///
/// * `ctx` - The context of the template.
///
/// * `syntax` - The syntax of the template.
type FtComplexBlock = dyn Fn(&str, JsonRef, &RenderSyntax) -> OsmiaResult;

/// Struct to store the syntax of the template.
///
/// It stores the functions used to detect the blocks.
pub struct RenderSyntax<'a> {
	simple_blocks: Vec<&'a FtSimpleBlock>,
	complex_blocks: Vec<&'a FtComplexBlock>
}

impl<'a> RenderSyntax<'a> {
	/// Create a new RenderSyntax struct.
	pub fn new() -> Self {
		return RenderSyntax {
			simple_blocks: Vec::new(),
			complex_blocks: Vec::new()
		};
	}

	/// Add a simple block to the syntax. Can be chained with the Builder pattern.
	///
	/// A simple block is a block without any logic, like a comment or a variable.
	///
	/// # Arguments
	///
	/// * `render_function` - The function detecting the block.
	///
	/// # Example
	/// ```
	/// use osmia::{osmia_syntax, RenderSyntax};
	///
	/// let mut syntax = RenderSyntax::new();
	/// syntax.add_simple_block(&|code, _ctx| {
	///		// Logic
	///		Ok(None)
	///	})
	///	.add_simple_block(&|code, _ctx| {
	///		// Logic
	///		Ok(None)
	///	});
	///	```
	pub fn add_simple_block(
		&mut self,
		render_function: &'a FtSimpleBlock
	) -> &mut Self {
		self.simple_blocks.push(render_function);
		self
	}

	pub fn add_complex_block(
		&mut self,
		render_function: &'a FtComplexBlock
	) -> &mut Self {
		self.complex_blocks.push(render_function);
		self
	}

	/// Renders the template with the configured syntax.
	///
	/// # Arguments
	///
	/// * `template` - The template to render.
	///
	/// * `ctx` - The context of the template.
	///
	/// # Example
	/// ```rust
	/// use osmia::{osmia_syntax, Json};
	///
	/// let result = osmia_syntax().render(
	/// 	"Hello, {{name}}!",
	/// 	&serde_json::from_str::<Json>(r#"{ "name": "world" }"#).unwrap()
	/// ).unwrap();
	/// assert_eq!(result, "Hello, world!".to_string());
	/// ```
	pub fn render(
		&self,
		template: &str,
		ctx: JsonRef
	) -> Result<String, String> {
		match ctx.as_object() {
			Some(ctx) => ctx,
			None => return Err("Context should be an object".to_string())
		};
		let mut result = String::new();
		let mut i = 0;
		let mut last_copy = 0;
		while i < template.len() {
			let mut found = false;
			for render_function in &self.complex_blocks {
				match render_function(&template[i..], ctx, &self) { // TODO refactor
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

/// Preconfigured syntax for the Osmia template engine.
///
/// It's consider as the default syntax and showcases the basic features of the engine.
///
/// ## Syntax
///
/// * `{{name}}` - Variable
/// * `{{{name}}}` - Print literal `{{name}}`
/// * `{* comment *}` - Comment (not rendered)
///
/// # Example
/// ```
/// use osmia::{osmia_syntax, RenderSyntax, Json};
///
/// let template = "Hello, {{name}}!";
/// let cxt = serde_json::from_str::<Json>(r#"{ "name": "world" }"#).unwrap();
/// let expected = "Hello, world!".to_string();
/// let result = osmia_syntax().render(template, &cxt).unwrap();
/// assert_eq!(result, expected);
/// ```
pub fn osmia_syntax<'a>() -> RenderSyntax<'a> {
	let mut syntax: RenderSyntax<'_> = RenderSyntax::new();
	syntax
	// .add_complex_block(&|code, ctx, syntax| {
	// TODO
	// })
	.add_complex_block(&|code, ctx, syntax| {
		let (condition, body, block_size) = match detect_complex_block(
			"{{for", "}}", "{{end}}", code
		) {
			Ok(None) => return Ok(None),
			Err(err) => return Err(err),
			Ok(Some((condition, body, block_size))) => (condition, body, block_size)
		};
		let condition_arr: Vec<&str> = Tokenizer::new(&condition).collect();
		if condition_arr.len() != 3 || condition_arr[1] != "in" {
			return Err(format!("Invalid for block: {}", condition_arr.join(" ")));
		}
		let key = &condition_arr[0];
		let value = match ctx.get(&condition_arr[2]) {
			Some(value) => value,
			None => return Err(format!("Key not found: \"{}\"", condition_arr[2]))
		};
		let iterator = match value.as_array() {
			Some(iterator) => iterator,
			None => return Err(format!("\"{}\" is not iterable", condition_arr[2]))
		};
		let mut new_block = String::new();
		for value in iterator.iter() {
			let mut new_ctx = ctx.clone();
			new_ctx[key] = value.clone();
			match syntax.render(&body, &new_ctx) {
				Err(err) => return Err(err),
				Ok(rendered) => new_block.push_str(rendered.as_str())
			}
		}
		let result: OsmiaResult = Ok(Some((new_block, block_size)));
		result
	})
	.add_simple_block(&|code, _ctx| {
		let result = detect_simple_block("{*", "*}", code);
		if let Ok(Some((_block, block_size))) = &result {
			return Ok(Some((String::new(), *block_size)));
		}
		result
	})
	.add_simple_block(&|code, _ctx| {
		let result = detect_simple_block("{{{", "}}}", code);
		if let Ok(Some((block, block_size))) = &result {
			let block = "{{".to_owned() + block.as_str() + "}}";
			return Ok(Some((block, *block_size)));
		}
		result
	})
	.add_simple_block(&|code, ctx| {
		let result = detect_simple_block("{{", "}}", code);
		if let Ok(Some((block, block_size))) = &result {
			let value = match get_json_value(ctx, block.trim()) {
				Ok(value) => value,
				Err(err) => return Err(err)
			};
			return Ok(Some((value, *block_size)));
		}
		result
	});
	syntax
}

fn get_json_value<'a>(
	ctx: JsonRef<'a>,
	key: &str
) -> Result<String, String> {
	let mut key_index: usize = 0;
	while key_index < key.len() {
		let c = key.chars().nth(key_index).unwrap();
		if c == '.' || c == '[' {
			break;
		}
		key_index += 1;
	}
	let value = match ctx.get(key[..key_index].to_owned()) {
		Some(value) => value,
		None => return Err(format!(
			"Key not found: \"{}\" in \"{}\"",
			key[..key_index].to_owned(),
			serde_json::to_string(&ctx).unwrap()
		))
	};
	if key_index >= key.len() {
		return json_as_str(value);
	}
	let c = key.chars().nth(key_index).unwrap();
	if c == '.' {
		match value {
			Json::Object(_) => return get_json_value(value, &key[key_index + 1..]),
			_ => Err(format!("\"{}\" is not an object", key[..key_index].to_owned()))
		}
	}
	else {
		match value {
			Json::Array(_) => (),
			_ => return Err(format!("\"{}\" is not an array", key[..key_index].to_owned()))
		}
		let mut key_end_index = key_index + 1;
		while key_end_index < key.len() {
			let c = key.chars().nth(key_end_index).unwrap();
			if c == ']' {
				break;
			}
			key_end_index += 1;
		}
		if key_end_index >= key.len() || key.chars().nth(key_end_index).unwrap() != ']' {
			return Err(format!("Missing closing bracket: {}", key));
		}
		let index = match key[key_index + 1..key_end_index].parse::<usize>() {
			Ok(index) => index,
			Err(err) => return Err(format!("Invalid index: {}", err))
		};
		let value = match value.as_array() {
			Some(value) => value,
			None => return Err(format!("\"{}\" is not an array", key[..key_index].to_owned()))
		};
		if index >= value.len() {
			return Err(format!("Index out of bounds: {}", index));
		}
		if key_end_index < key.len() - 1 && key.chars().nth(key_end_index + 1).unwrap() == '.' {
			key_end_index += 1;
		}
		if key_end_index + 1 >= key.len() {
			return json_as_str(&value[index]);
		}
		return get_json_value(&value[index], &key[key_end_index + 1..]);
	}
}

/// Converts a Json value to a String.
///
/// It prevents serde_json from adding quotes to the string.
///
/// # Arguments
///
/// * `value` - The Json value to convert.
fn json_as_str(value: JsonRef) -> Result<String, String> {
	match value {
		Json::String(value) => Ok(value.to_string()),
		e => serde_json::to_string(&e).map_err(|err| err.to_string())
	}
}

/// Detects a simple block.
///
/// # Arguments
///
/// * `start` - The start of the block.
/// * `end` - The end of the block.
/// * `code` - The code to parse.
///
/// # Example
/// ```
/// use osmia::{detect_simple_block, RenderSyntax, OsmiaResult, Json};
/// use std::collections::HashMap;
/// use serde_json::Map;
///
/// let mut syntax = RenderSyntax::new();
/// syntax.add_simple_block(&|code, _ctx| {
/// 	let result = detect_simple_block("{*", "*}", code);
/// 	if let Ok(Some((_block, block_size))) = &result {
/// 		return Ok(Some((String::new(), *block_size)));
/// 	}
/// 	result
/// });
/// let ctx: Json = serde_json::from_str(r#"{}"#).unwrap();
/// let result = syntax.render("Hello, {* comment *}!", &ctx).unwrap();
/// assert_eq!(result, "Hello, !");
/// ```
pub fn detect_simple_block(
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



struct Tokenizer<'a> {
	text: &'a str,
	start: usize,
	current: usize,
	in_quotes: Option<char>,
}

impl<'a> Tokenizer<'a> {
	fn new(text: &'a str) -> Self {
		Self {
			text,
			start: 0,
			current: 0,
			in_quotes: None
		}
	}
}

impl<'a> std::iter::Iterator for Tokenizer<'a> {
	type Item = &'a str;

	fn next(&mut self) -> Option<Self::Item> {
		while self.current < self.text.len() {
			let c = self.text.chars().nth(self.current).unwrap();
			if c == '"' || c == '\'' {
				match self.in_quotes {
					None => self.in_quotes = Some(c),
					Some(q) => {
						if q == c {
							self.in_quotes = None;
						}
					}
				}
			}
			if c.is_whitespace() && self.in_quotes.is_none() {
				let token = &self.text[self.start..self.current].trim();
				self.start = self.current;
				if token.len() > 0 {
					return Some(token);
				}
			}
			self.current += 1;
		}
		if self.current >= self.text.len() && self.current != self.start {
			if self.in_quotes.is_some() {
				panic!("Unclosed quotes!");
			}
			let token = &self.text[self.start..self.current].trim();
			self.start = self.current;
			if token.len() > 0 {
				return Some(token);
			}
		}
		None
	}
}

pub fn detect_complex_block<'a>(
	start_block_start: &'a str,
	start_block_end: &'a str,
	end_block: &'a str,
	code: &'a str
) -> Result<Option<(String, String, usize)>, String> {
	let (block, block_size) = match detect_simple_block(
		start_block_start, start_block_end, code
	) {
		Ok(None) => return Ok(None),
		Err(err) => return Err(err),
		Ok(Some((block, block_size))) => (block, block_size)
	};
	let mut i = block_size;
	while i < code.len() && !code[i..].starts_with(end_block) {
		i += 1;
	}
	if i >= code.len() || !code[i..].starts_with(end_block) {
		return Err(format!("Missing closing block: {}", end_block));
	}
	let new_code = &code[block_size..i].to_owned();
	Ok(Some((
		block.to_owned(),
		new_code.to_owned(),
		i + end_block.len()
	)))
}
