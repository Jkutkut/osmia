mod get;
mod set;

use serde::Deserialize;
use crate::ctx::JsonTree;

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum JsonValue {
	Int(i64),
	Float(f64),
	String(String),
	Bool(bool),
	Null,
}

type Ctx = JsonTree<String, JsonValue>;

#[cfg(test)]
fn valid_json() -> Ctx {
	serde_json::from_str(r#"
		{
			"foo": 1,
			"bar": "hello",
			"baz": true,
			"qux": null,
			"quux": [
				{
					"usr": "Marvin",
					"pwd": "admin"
				}
			]
		}
	"#).unwrap()
}
