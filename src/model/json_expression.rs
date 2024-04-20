use std::collections::HashMap;

use crate::model::{
	Expression,
	JsonTree, Literal
};

#[derive(Debug, PartialEq, Clone)]
pub enum JsonExpression {
	Expression(Expression),
	Array(Vec<JsonExpression>),
	Object(HashMap<String, JsonExpression>)
}

impl std::fmt::Display for JsonExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

impl From<&JsonTree> for JsonExpression {
	fn from(tree: &JsonTree) -> Self {
		match tree {
			JsonTree::Number(n) => Self::Expression(Expression::Literal(Literal::Int(*n))),
			JsonTree::Float(f) => Self::Expression(Expression::Literal(Literal::Float(*f))),
			JsonTree::Str(s) => Self::Expression(Expression::Literal(Literal::Str(s.to_string()))),
			JsonTree::Bool(b) => Self::Expression(Expression::Literal(Literal::Bool(*b))),
			JsonTree::Null => Self::Expression(Expression::Literal(Literal::Null)),
			JsonTree::Array(array) => Self::Array(array.iter().map(|e| (&e.clone()).into()).collect()),
			JsonTree::Object(obj) => Self::Object(obj.iter().map(|(k, v)| (k.to_string(), JsonExpression::from(&**v))).collect()),
		}
	}
}
