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
//! use osmia::{osmia_syntax, RenderSyntax};
//! use std::collections::HashMap;
//!
//! let template = "Hello, {{name}}!";
//! let result = osmia_syntax().render(
//!		"Hello, {{name}}!",
//!		&HashMap::from([("name", "world")])
//!	).unwrap();
//! assert_eq!(result, "Hello, world!".to_string());
//! ```

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::fmt::Display;

/// Alias for the context of the template.
/// It's a HashMap with a string as key and anything implementing the trait Display.
type OsmiaContext<'a, T> = &'a HashMap<&'a str, T>;

/// Internal type to handle the result of the render function.
///
/// If there's a block, it returns the rendered block 
/// as a String and the size used from the code (`(String, usize)`).
///
/// If there's no block, it returns `None`.
///
/// If there's an error, it returns the error as a `String`.
pub type OsmiaResult = Result<Option<(String, usize)>, String>;

/// Type of the functions detecting the syntax blocks.
type FtStractBlock<T> = dyn Fn(&str, OsmiaContext<T>) -> OsmiaResult;

/// Struct to store the syntax of the template.
pub struct RenderSyntax<'a, T> {
	simple_blocks: Vec<&'a FtStractBlock<T>>,
	//complex_blocks: Vec<()>, // TODO
}

impl<'a, T> RenderSyntax<'a, T> {
	/// Create a new RenderSyntax.
	///
	/// It follows the Builder pattern.
	pub fn new() -> Self {
		return RenderSyntax {
			simple_blocks: Vec::new(),
			// complex_blocks: Vec::new()
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
	/// let mut syntax: RenderSyntax<'_, String> = RenderSyntax::new();
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
		render_function: &'a FtStractBlock<T>
	) -> &mut Self {
		self.simple_blocks.push(render_function);
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
	///
	/// ```rust
	/// use osmia::osmia_syntax;
	/// use std::collections::HashMap;
	///
	/// let result = osmia_syntax().render(
	///		"Hello, {{name}}!",
	/// 	&HashMap::from([("name", "world")])
	/// ).unwrap();
	/// assert_eq!(result, "Hello, world!".to_string());
	/// ```
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
/// use osmia::{osmia_syntax, RenderSyntax};
/// use std::collections::HashMap;
///
/// let template = "Hello, {{name}}!";
/// let cxt = HashMap::from([("name", "world")]);
/// let expected = "Hello, world!".to_string();
/// let result = osmia_syntax().render(template, &cxt).unwrap();
/// assert_eq!(result, expected);
/// ```
pub fn osmia_syntax<'a, T>() -> RenderSyntax<'a, T>
where
	T: Display
{
	let mut syntax: RenderSyntax<'_, T> = RenderSyntax::new();
	syntax.add_simple_block(&|code, _ctx| {
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
				Some(value) => value.to_string(),
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
/// use osmia::{detect_simple_block, RenderSyntax, OsmiaResult};
/// use std::collections::HashMap;
///
/// let mut syntax: RenderSyntax<'_, String> = RenderSyntax::new();
/// syntax.add_simple_block(&|code, _ctx| {
/// 	let result = detect_simple_block("{*", "*}", code);
/// 	if let Ok(Some((_block, block_size))) = &result {
/// 		return Ok(Some((String::new(), *block_size)));
/// 	}
/// 	result
/// });
/// 
/// let result = syntax.render("Hello, {* comment *}!", &HashMap::new()).unwrap();
/// assert_eq!(result, "Hello, !");
/// ```
/// 
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
