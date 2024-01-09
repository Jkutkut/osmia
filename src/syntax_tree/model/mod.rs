mod expression;
mod binary;
mod grouping;
mod literal;
mod unary;
mod variable;

mod impl_visitor;

pub use binary::Binary;
pub use expression::Expression;
pub use grouping::Grouping;
pub use literal::Literal;
pub use unary::Unary;
pub use variable::Variable;

enum Stmt<'a> {
	Block(Vec<Stmt<'a>>),
	Raw(&'a str),
	Print(Expression<'a>),
	Expression(Expression<'a>),
	Assign(Variable<'a>, Expression<'a>),
	If(Expression<'a>, Box<Stmt<'a>>, Option<Box<Stmt<'a>>>),
	While(Expression<'a>, Box<Stmt<'a>>),
	ForEach(Variable<'a>, Variable<'a>, Box<Stmt<'a>>),
	Break,
	Continue,
}
