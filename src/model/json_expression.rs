use std::collections::HashMap;

use crate::model::{
	Expression
};

#[derive(Debug, PartialEq)]
pub enum JsonExpression<'a> {
	Expression(Expression<'a>),
	Array(Vec<JsonExpression<'a>>),
	Object(HashMap<String, JsonExpression<'a>>)
}

impl std::fmt::Display for JsonExpression<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Expression(e) => write!(f, "{}", e),
			Self::Array(a) => write!(
				f, "[{}]",
				a.iter()
					.map(|e| format!("{}", e))
					.collect::<Vec<String>>()
					.join(",")
			),
			Self::Object(o) => write!(
				f, "{{{}}}",
				o.iter()
					.map(|(k, v)| format!("{}:{}", k, v))
					.collect::<Vec<String>>()
					.join(",")
			)
		}
	}
}
