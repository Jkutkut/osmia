mod get;
mod set;

use crate::model::ctx::{
	Ctx,
	JsonValue
};

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
