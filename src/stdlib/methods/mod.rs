use super::*;
use generics::add_generics;
use crate::OsmiaError;
use crate::model::expr::{
	Array,
	Object
};
use crate::model::visitor_pattern::Visitor;

#[allow(non_upper_case_globals)]
mod generics;
mod string;
mod int;
mod float;
mod bool;
mod null;
mod array;
mod object;
mod callable;
mod lambda;
#[allow(dead_code)]
mod utils;

pub use utils::*;

/// # math module
/// ## Constants:
/// ```rust
/// use osmia::Osmia;
///
/// let mut osmia = Osmia::default();
/// osmia.run_code("{{ math.PI }}").unwrap();
/// osmia.run_code("{{ math.E }}").unwrap();
/// ```
///
/// ## Methods:
///
/// ### String:
/// ```rust
/// use osmia::Osmia;
///
/// let mut osmia = Osmia::default();
///
/// // Generics
/// assert_eq!(osmia.run_code(r#"{{ "hello"?len() }}"#).unwrap(), "5");
/// assert_eq!(osmia.run_code(r#"{{ "hello"?has_content() }}"#).unwrap(), "true");
/// assert_eq!(osmia.run_code(r#"{{ "hello"?to_bool() }}"#).unwrap(), "true");
/// assert!(osmia.run_code(r#"{{ "hello"?to_float() }}"#).is_err());
/// assert!(osmia.run_code(r#"{{ "hello"?to_int() }}"#).is_err());
/// assert_eq!(osmia.run_code(r#"{{ "hello"?to_string() }}"#).unwrap(), "hello");
/// assert_eq!(osmia.run_code(r#"{{ "hello"?type() }}"#).unwrap(), "string");
/// assert_eq!(osmia.run_code(r#"{{ "hi"?switch("bye", 1, "hi", 2) }}"#).unwrap(), "2");
///
/// // String only
/// assert_eq!(osmia.run_code(r#"{{ "HELLO"?lower() }}"#).unwrap(), "hello");
/// assert_eq!(osmia.run_code(r#"{{ "hello"?upper() }}"#).unwrap(), "HELLO");
/// assert_eq!(osmia.run_code(r#"{{ "  hi   "?trim() }}"#).unwrap(), "hi");
/// assert_eq!(osmia.run_code(r#"{{ "hello world"?capitalize() }}"#).unwrap(), "Hello World");
/// assert_eq!(osmia.run_code(r#"{{ "hello"?starts_with("h") }}"#).unwrap(), "true");
/// assert_eq!(osmia.run_code(r#"{{ "hello"?ends_with("o") }}"#).unwrap(), "true");
/// assert_eq!(osmia.run_code(r#"{{ "hi"?ensure_starts_with("¡") }}"#).unwrap(), "¡hi");
/// assert_eq!(osmia.run_code(r#"{{ "hi"?ensure_ends_with("!") }}"#).unwrap(), "hi!");
/// assert_eq!(osmia.run_code(r#"{{ "hello"?index_of("l") }}"#).unwrap(), "2");
/// assert_eq!(osmia.run_code(r#"{{ "hello"?last_index_of("l") }}"#).unwrap(), "3");
/// assert_eq!(osmia.run_code(r#"{{ "h1-F"?match("^\w\d-\w$") }}"#).unwrap(), "true");
/// assert_eq!(osmia.run_code(r#"{{ "hello"?replace("l", "1") }}"#).unwrap(), "he1lo");
/// assert_eq!(osmia.run_code(r#"{{ "hello"?replace_all("l", "1") }}"#).unwrap(), "he11o");
/// assert_eq!(osmia.run_code(r#"{{ "hello"?split("") }}"#).unwrap(), r#"["", "h", "e", "l", "l", "o", ""]"#);
/// assert_eq!(osmia.run_code(r#"{{ "hello"?substring(1, 3) }}"#).unwrap(), r#"el"#);
/// assert_eq!(osmia.run_code(r#"{{ "hello"?truncate(2) }}..."#).unwrap(), r#"he..."#);
/// ```
///
/// ### Int:
/// ```rust
/// use osmia::Osmia;
///
/// let mut osmia = Osmia::default();
/// 
/// // Generics
/// assert_eq!(osmia.run_code(r#"{{ 1?has_content() }}"#).unwrap(), "true");
/// assert_eq!(osmia.run_code(r#"{{ 1?to_bool() }}"#).unwrap(), "true");
/// assert_eq!(osmia.run_code(r#"{{ 1?to_float() }}"#).unwrap(), "1");
/// assert_eq!(osmia.run_code(r#"{{ 1?to_int() }}"#).unwrap(), "1");
/// assert_eq!(osmia.run_code(r#"{{ 1?to_string() }}"#).unwrap(), "1");
/// assert_eq!(osmia.run_code(r#"{{ 1?type() }}"#).unwrap(), "int");
/// assert_eq!(osmia.run_code(r#"{{ 1?switch(1, "one", 2, "two") }}"#).unwrap(), "one");
///
/// // Int only
/// // There are no other methods
/// ```
///
/// ### Float:
/// ```rust
/// use osmia::Osmia;
///
/// let mut osmia = Osmia::default();
///
/// // Generics
/// assert_eq!(osmia.run_code(r#"{{ 1.1?has_content() }}"#).unwrap(), "true");
/// assert_eq!(osmia.run_code(r#"{{ 1.1?to_bool() }}"#).unwrap(), "true");
/// assert_eq!(osmia.run_code(r#"{{ 1.1?to_float() }}"#).unwrap(), "1.1");
/// assert_eq!(osmia.run_code(r#"{{ 1.1?to_int() }}"#).unwrap(), "1");
/// assert_eq!(osmia.run_code(r#"{{ 1.1?to_string() }}"#).unwrap(), "1.1");
/// assert_eq!(osmia.run_code(r#"{{ 1.1?type() }}"#).unwrap(), "float");
/// assert_eq!(osmia.run_code(r#"{{ 1.1?switch(1, "one", 2, "two", null) }}"#).unwrap(), "null");
/// ```
///
/// ### Bool:
/// ```rust
/// use osmia::Osmia;
///
/// let mut osmia = Osmia::default();
/// 
/// // Generics
/// assert_eq!(osmia.run_code(r#"{{ true?has_content() }}"#).unwrap(), "true");
/// assert_eq!(osmia.run_code(r#"{{ true?to_bool() }}"#).unwrap(), "true");
/// assert_eq!(osmia.run_code(r#"{{ true?to_string() }}"#).unwrap(), "true");
/// assert_eq!(osmia.run_code(r#"{{ true?type() }}"#).unwrap(), "bool");
/// assert_eq!(osmia.run_code(r#"{{ true?switch(true, "t", false, "f") }}"#).unwrap(), "t");
///
/// // Bool only
/// assert_eq!(osmia.run_code(r#"{{ true?then("t", "f") }}"#).unwrap(), "t");
/// assert_eq!(osmia.run_code(r#"{{ false?not() }}"#).unwrap(), "true");
/// assert_eq!(osmia.run_code(r#"{{ false?and(true) }}"#).unwrap(), "false");
/// assert_eq!(osmia.run_code(r#"{{ true?or(false) }}"#).unwrap(), "true");
/// assert_eq!(osmia.run_code(r#"{{ true?nand(true) }}"#).unwrap(), "false");
/// assert_eq!(osmia.run_code(r#"{{ true?nor(false) }}"#).unwrap(), "false");
/// assert_eq!(osmia.run_code(r#"{{ true?xor(false) }}"#).unwrap(), "true");
/// assert_eq!(osmia.run_code(r#"{{ true?xnor(false) }}"#).unwrap(), "false");
/// ```
///
/// ### Null:
/// ```rust
/// use osmia::Osmia;
///
/// let mut osmia = Osmia::default();
///
/// // Generics
/// assert_eq!(osmia.run_code(r#"{{ null?has_content() }}"#).unwrap(), "false");
/// assert_eq!(osmia.run_code(r#"{{ null?to_bool() }}"#).unwrap(), "false");
/// assert_eq!(osmia.run_code(r#"{{ null?to_string() }}"#).unwrap(), "null");
/// assert_eq!(osmia.run_code(r#"{{ null?type() }}"#).unwrap(), "null");
/// assert_eq!(osmia.run_code(r#"{{ null?switch(1, "one", 2, "two", "null") }}"#).unwrap(), "null");
///
/// // Null only
/// // There are no other methods
/// ```
///
/// ### Array:
/// ```rust
/// use osmia::Osmia;
///
/// let mut osmia = Osmia::default();
///
/// // Generics
/// assert_eq!(osmia.run_code(r#"{{ [1, 2, 3]?len() }}"#).unwrap(), "3");
/// assert_eq!(osmia.run_code(r#"{{ [1, 2, 3]?has_content() }}"#).unwrap(), "true");
/// assert_eq!(osmia.run_code(r#"{{ [1, 2, 3]?to_bool() }}"#).unwrap(), "true");
/// assert_eq!(osmia.run_code(r#"{{ [1, 2, 3]?to_string() }}"#).unwrap(), "[1, 2, 3]");
/// assert_eq!(osmia.run_code(r#"{{ [1, 2, 3]?type() }}"#).unwrap(), "array");
/// assert_eq!(osmia.run_code(r#"{{ [1, 2, 3]?switch(1, "one", 2, "two", null) }}"#).unwrap(), "null");
/// assert_eq!(osmia.run_code(r#"{{ [1, 2, 3]?keys() }}"#).unwrap(), "[0, 1, 2]");
/// assert_eq!(osmia.run_code(r#"{{ [1, 2, 3]?values() }}"#).unwrap(), "[1, 2, 3]");
/// assert_eq!(osmia.run_code(r#"{{ [1, 2, 3]?entries() }}"#).unwrap(), r#"[{"key": 0, "value": 1}, {"key": 1, "value": 2}, {"key": 2, "value": 3}]"#);
/// assert_eq!(osmia.run_code(r#"{{ [1, 2, 3]?get(4, null) }}"#).unwrap(), "null");
///
/// // Array only
/// assert_eq!(osmia.run_code(r#"{{ [3, 2, 1]?sort() }}"#).unwrap(), "[1, 2, 3]");
/// assert_eq!(osmia.run_code(r#"{{ [2, 3, 1]?sort_by(fn (a, b) => b - a ) }}"#).unwrap(), "[3, 2, 1]");
/// assert_eq!(osmia.run_code(r#"{{ [1, 3, 2]?map(fn (a) => a * 2 - 1) }}"#).unwrap(), "[1, 5, 3]");
/// assert_eq!(osmia.run_code(r#"{{ [1, 3, 2]?for_each(fn (a) => a * 2) }}"#).unwrap(), "");
/// assert_eq!(osmia.run_code(r#"{{ [1, 3, 2]?for_each_index(fn (a, idx) => a * idx) }}"#).unwrap(), "");
/// assert_eq!(osmia.run_code(r#"{{ [1, 3, 2]?reverse() }}"#).unwrap(), "[2, 3, 1]");
/// assert_eq!(osmia.run_code(r#"{{ [1, 2, 3, 4]?filter(fn (a) => a % 2 == 0) }}"#).unwrap(), "[2, 4]");
/// assert_eq!(osmia.run_code(r#"{{ [1, 2, 3, 4]?filter_index(fn (a, idx) => idx % 2 == 0) }}"#).unwrap(), "[1, 3]");
/// assert_eq!(osmia.run_code(r#"{{ [1, 2, 3, 4]?reduce(fn (a, b) => a + b, 0) }}"#).unwrap(), "10");
/// assert_eq!(osmia.run_code(r#"{{ [1, 2, 3]?join(",") }}"#).unwrap(), "1,2,3");
/// ```
///
/// ### Object:
/// ```rust
/// use osmia::Osmia;
///
/// let mut osmia = Osmia::default();
///
/// // Generics
/// assert_eq!(osmia.run_code(r#"{{ {"a": 1, "b": 2}?has_content() }}"#).unwrap(), "true");
/// assert_eq!(osmia.run_code(r#"{{ {"a": 1, "b": 2}?to_bool() }}"#).unwrap(), "true");
/// assert_eq!(osmia.run_code(r#"{{ {"a": 1, "b": 2}?to_string() }}"#).unwrap(), "{\"a\": 1, \"b\": 2}");
/// assert_eq!(osmia.run_code(r#"{{ {"a": 1, "b": 2}?type() }}"#).unwrap(), "object");
/// assert_eq!(osmia.run_code(r#"{{ {}?switch({"a": 1}, "empty", {"b": null}, "b", null) }}"#).unwrap(), "null");
/// assert_eq!(osmia.run_code(r#"{{ {"a": 1, "b": 2}?keys()?sort() }}"#).unwrap(), "[\"a\", \"b\"]");
/// assert_eq!(osmia.run_code(r#"{{ {"a": 1, "b": 2}?values()?sort() }}"#).unwrap(), "[1, 2]");
/// assert_eq!(osmia.run_code(
///		r#"{{ {"a": 1, "b": 2}?entries()?sort_by(fn (a, b) => b.key - a.key) }}"#).unwrap(),
///		r#"[{"key": "a", "value": 1}, {"key": "b", "value": 2}]"#
///	);
/// assert_eq!(osmia.run_code(r#"{{ {"a": 1, "b": 2}?get("b", null) }}"#).unwrap(), "2");
/// assert_eq!(osmia.run_code(r#"{{ {"a": 1, "b": 2}?get("c", null) }}"#).unwrap(), "null");
///
/// // Object only
/// // There are no other methods
/// ```
///
/// ## Sub modules:
/// There are no sub modules for this module
pub fn module() -> Module {
	Module::new()
	.add_module(
		MethodExpression::Str.into(),
		add_generics(string::module())
	)
	.add_module(
		MethodExpression::Int.into(),
		add_generics(int::module())
	)
	.add_module(
		MethodExpression::Float.into(),
		add_generics(float::module())
	)
	.add_module(
		MethodExpression::Bool.into(),
		add_generics(bool::module())
	)
	.add_module(
		MethodExpression::Null.into(),
		add_generics(null::module())
	)
	.add_module(
		MethodExpression::Array.into(),
		add_generics(array::module())
	)
	.add_module(
		MethodExpression::Object.into(),
		add_generics(object::module())
	)
	.add_module(
		MethodExpression::Callable.into(),
		add_generics(callable::module())
	)
	.add_module(
		MethodExpression::Lambda.into(),
		add_generics(lambda::module())
	)
}
