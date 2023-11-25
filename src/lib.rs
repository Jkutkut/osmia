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
//!		"Hello, {{name}}!",
//!		&serde_json::from_str::<Json>(r#"{ "name": "world" }"#).unwrap()
//!	).unwrap();
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
		self,
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
	/*.add_complex_block(&|code, ctx, syntax| {
		let result = match detect_simple_block("{{for", "}}", code) {
			Ok(None) => return Ok(None),
			Err(err) => return Err(err),
			Ok(Some((block, block_size))) => Ok(Some((block, block_size)))
		};
		let (block, block_size) = result.clone().unwrap().unwrap();
		let block_arr = block.trim().split_whitespace().collect::<Vec<&str>>();
		if block_arr.len() != 3 || block_arr[1] != "in" {
			return Err(format!("Invalid for block: {}", block));
		}
		let key = block_arr[0];
		let value = match ctx.get(block_arr[2]) {
			Some(value) => value,
			None => return Err(format!("Key not found: {}", block_arr[2]))
		};
		let iterator = match value.as_array() {
			Some(iterator) => iterator,
			None => return Err(format!("\"{}\" is not iterable", block_arr[2]))
		};
		let mut i = block_size;
		while i < code.len() && !code[i..].starts_with("{{end}}") {
			i += 1;
		}
		if i >= code.len() || !code[i..].starts_with("{{end}}") {
			return Err(format!("Missing closing block: {{end}}"));
		}
		let new_code = &code[block_size..i];
		let mut new_ctx = ctx.clone();
		new_ctx.insert(key, value);
		let mut new_block = String::new();
		for (i, value) in iterator.iter().enumerate() {
			match syntax.render(new_code, ctx) {
				Err(err) => return Err(err),
				Ok(rendered) => new_block.push_str(rendered.as_str())
			}
		}

		let new_block_size = i + "{{end}}".len();
		let result = Ok(Some((new_block, new_block_size)));
		result
	})*/
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
			let value = match ctx.get(block.as_str()) {
				Some(value) => match value.as_str() {
					Some(value) => value.to_owned(),
					None => return Err(format!("\"{}\" is not a string", block))
				},
				None => return Err(format!("Key not found: {}", block))
			};
			return Ok(Some((value, *block_size)));
		}
		result
	});
	syntax
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
