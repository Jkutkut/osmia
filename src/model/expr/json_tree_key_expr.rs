use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum JsonTreeKeyExpr {
	JsonTreeKey(JsonTreeKey<String>),
	Expr(Expr),
}

impl From<&str> for JsonTreeKey<String> {
	fn from(s: &str) -> Self {
		JsonTreeKey::Key(s.into())
	}
}

impl From<JsonTreeKey<String>> for JsonTreeKeyExpr {
	fn from(key: JsonTreeKey<String>) -> Self {
		JsonTreeKeyExpr::JsonTreeKey(key)
	}
}

impl From<Expr> for JsonTreeKeyExpr {
	fn from(expr: Expr) -> Self {
		JsonTreeKeyExpr::Expr(expr)
	}
}
