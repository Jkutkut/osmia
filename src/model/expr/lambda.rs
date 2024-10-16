use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Lambda {
	params: Vec<FunctionParam>,
	body: Box<Expr>,
}

impl Lambda {
	pub fn new(params: Vec<FunctionParam>, body: Expr) -> Self {
		Self { params, body: Box::new(body) }
	}

	pub fn params(&self) -> &Vec<FunctionParam> {
		&self.params
	}

	pub fn body(&self) -> &Expr {
		&self.body
	}
}

impl Display for Lambda {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let params = self.params.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ");
		write!(f, "fn ({}) => {}", params, self.body)
	}
}
