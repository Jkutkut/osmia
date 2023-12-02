use crate::token::Token;

pub enum Literal {
	Float(f64),
	Int(i64),
	Str(String),
	Bool(bool),
	Null
}

pub struct Grouping<'a> {
	expression: Box<Expression<'a>>
}

impl<'a> Grouping<'a> {
	pub fn new(expression: Expression<'a>) -> Grouping<'a> {
		Grouping {
			expression: Box::new(expression)
		}
	}
}

pub struct Unary<'a> {
	operator: Token<'a>,
	right: Box<Expression<'a>>
}

impl<'a> Unary<'a> {
	pub fn new(operator: Token<'a>, right: Expression<'a>) -> Result<Unary<'a>, String> {
		match operator {
			Token::Minus | Token::Not => Ok(Unary {
				operator: operator,
				right: Box::new(right)
			}),
			_ => return Err(format!("Invalid unary operator: {}", operator))
		}
	}
}

pub struct Binary<'a> {
	left: Box<Expression<'a>>,
	operator: Token<'a>,
	right: Box<Expression<'a>>
}

impl<'a> Binary<'a> {
	pub fn new(
		left: Expression<'a>,
		operator: Token<'a>,
		right: Expression<'a>
	) -> Result<Binary<'a>, String> {
		if !operator.is_binary_operator() {
			return Err(format!("Invalid binary operator: {}", operator));
		}
		Ok(Binary {
			left: Box::new(left),
			operator: operator,
			right: Box::new(right)
		})
	}
}

pub enum Expression<'a> {
	Literal(Literal),
	Grouping(Grouping<'a>),
	Unary(Unary<'a>),
	Binary(Binary<'a>)
}

// Visitor
pub trait Visitor<T> {
	fn visit_expression(&self, expression: &Expression) -> T;
	fn visit_literal(&self, literal: &Literal) -> T;
	fn visit_grouping(&self, grouping: &Grouping) -> T;
	fn visit_unary(&self, unary: &Unary) -> T;
	fn visit_binary(&self, binary: &Binary) -> T;
}

pub trait Visitable<T> {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T;
}

impl<T> Visitable<T> for Expression<'_> {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		match self {
			Expression::Literal(literal) => visitor.visit_literal(literal),
			Expression::Grouping(grouping) => visitor.visit_grouping(grouping),
			Expression::Unary(unary) => visitor.visit_unary(unary),
			Expression::Binary(binary) => visitor.visit_binary(binary),
		}
	}
}

impl<T> Visitable<T> for Literal {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_literal(self)
	}
}

impl<T> Visitable<T> for Grouping<'_> {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_grouping(self)
	}
}

impl<T> Visitable<T> for Unary<'_> {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_unary(self)
	}
}

impl<T> Visitable<T> for Binary<'_> {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_binary(self)
	}
}

// Printer
pub struct SyntaxTreePrinter;

impl Visitor<String> for SyntaxTreePrinter {
	fn visit_expression(&self, expression: &Expression) -> String {
		expression.accept(self)
	}

	fn visit_literal(&self, literal: &Literal) -> String {
		match literal {
			Literal::Float(f) => f.to_string(),
			Literal::Int(i) => i.to_string(),
			Literal::Str(s) => s.to_string(),
			Literal::Bool(b) => b.to_string(),
			Literal::Null => "null".to_string()
		}
	}

	fn visit_grouping(&self, grouping: &Grouping) -> String {
		format!("({})", &grouping.expression.accept(self))
	}

	fn visit_unary(&self, unary: &Unary) -> String {
		format!("{}{}", &unary.operator, &unary.right.accept(self))
	}

	fn visit_binary(&self, binary: &Binary) -> String {
		format!("{} {} {}", &binary.left.accept(self), &binary.operator, &binary.right.accept(self))
	}
}
