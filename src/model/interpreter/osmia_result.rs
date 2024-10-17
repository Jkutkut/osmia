use crate::{
	types::OsmiaOutput,
	model::expr::Expr,
};

pub enum OsmiaResult {
	None,
	Expr(Expr),
	OsmiaOutput(OsmiaOutput),
}

impl std::fmt::Display for OsmiaResult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			OsmiaResult::None => Ok(()),
			OsmiaResult::Expr(e) => write!(f, "{}", e),
			OsmiaResult::OsmiaOutput(o) => write!(f, "{}", o),
		}
	}
}

impl Into<OsmiaResult> for OsmiaOutput {
	fn into(self) -> OsmiaResult {
		OsmiaResult::OsmiaOutput(self)
	}
}

impl Into<OsmiaResult> for Expr {
	fn into(self) -> OsmiaResult {
		OsmiaResult::Expr(self)
	}
}
